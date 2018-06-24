#![cfg(test)]

extern crate backtrace;
extern crate bytes;
extern crate k8s_openapi;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;

use k8s_openapi::http;
use k8s_openapi::serde_json;

#[cfg(windows)] #[path = "client_winapi.rs"] mod client;
#[cfg(not(windows))] #[path = "client_openssl.rs"] mod client;

struct Error(Box<std::error::Error>, backtrace::Backtrace);

impl<E> From<E> for Error where E: Into<Box<std::error::Error>> {
	fn from(value: E) -> Self {
		Error(value.into(), backtrace::Backtrace::new())
	}
}

impl std::fmt::Debug for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f, "{}", self.0)?;
		write!(f, "{:?}", self.1)?;
		Ok(())
	}
}

#[derive(Debug)]
struct Client {
	inner: reqwest::Client,
	server: http::Uri,
}

impl Client {
	fn new() -> Result<Self, Error> {
		let kubeconfig: KubeConfig = {
			let mut kubeconfig_path = std::env::home_dir().ok_or("can't find home directory")?;
			kubeconfig_path.push(".kube");
			kubeconfig_path.push("config");
			serde_yaml::from_reader(std::io::BufReader::new(std::fs::File::open(kubeconfig_path)?))?
		};

		let context = std::env::var("K8S_CONTEXT").unwrap_or(kubeconfig.current_context);

		let KubeConfigContext { cluster, user } =
			kubeconfig.contexts.into_iter()
			.find(|c| c.name == context).unwrap_or_else(|| panic!("couldn't find context named {}", context))
			.context;

		let KubeConfigCluster { certificate_authority, server } =
			kubeconfig.clusters.into_iter()
			.find(|c| c.name == cluster).unwrap_or_else(|| panic!("couldn't find cluster named {}", cluster))
			.cluster;

		let certificate_authority = client::x509_from_pem(&certificate_authority)?;

		let server: http::Uri = http::HttpTryFrom::try_from(server).map_err(|err| format!("couldn't parse server URL: {}", err))?;
		if let Some(path_and_query) = server.path_and_query() {
			if path_and_query != "/" {
				return Err(format!("server URL {} has path and query {}", server, path_and_query).into());
			}
		}

		let KubeConfigUser { client_certificate, client_key } =
			kubeconfig.users.into_iter()
			.find(|u| u.name == user).unwrap_or_else(|| panic!("couldn't find user named {}", user))
			.user;

		let client_key = client::pkcs12(&client_certificate, &client_key)?;

		let mut inner = reqwest::Client::builder();
		inner.danger_disable_hostname_verification();
		inner.add_root_certificate(reqwest::Certificate::from_der(&certificate_authority)?);
		inner.identity(reqwest::Identity::from_pkcs12_der(&client_key, "")?);
		let inner = inner.build()?;

		Ok(Client {
			inner,
			server,
		})
	}

	fn get(&self, path: &str) -> Result<reqwest::Response, Error> {
		let url = {
			let mut url: http::uri::Parts = self.server.clone().into();
			let path: bytes::Bytes = path.into();
			url.path_and_query = Some(http::uri::PathAndQuery::from_shared(path)?);
			http::Uri::from_parts(url)?.to_string()
		};

		let response =
			self.inner
			.get(&url)
			.header(reqwest::header::Accept::json())
			.send()?;

		Ok(response)
	}

	fn delete(&self, path: &str) -> Result<reqwest::Response, Error> {
		let url = {
			let mut url: http::uri::Parts = self.server.clone().into();
			let path: bytes::Bytes = path.into();
			url.path_and_query = Some(http::uri::PathAndQuery::from_shared(path)?);
			http::Uri::from_parts(url)?.to_string()
		};

		let response =
			self.inner
			.delete(&url)
			.header(reqwest::header::Accept::json())
			.send()?;

		Ok(response)
	}

	fn execute(&self, request: http::Request<Vec<u8>>) -> Result<reqwest::Response, Error> {
		let (method, url, body) = {
			let (mut parts, body) = request.into_parts();
			let mut url: http::uri::Parts = parts.uri.into();
			let path = url.path_and_query.take().expect("request doesn't have path and query");
			let mut url: http::uri::Parts = self.server.clone().into();
			url.path_and_query = Some(path);
			let url = http::Uri::from_parts(url)?.to_string();

			(parts.method, url.to_string(), body)
		};

		let mut request = match method {
			http::Method::GET => self.inner.get(&url),
			http::Method::POST => self.inner.post(&url),
			other => panic!("{}", other),
		};

		request.body(body);

		Ok(request.send()?)
	}
}

fn get_single_value<R, F, T>(mut response: reqwest::Response, mut f: F) -> Result<T, Error> where
	for<'r> R: k8s_openapi::Response<'r>,
	F: FnMut(Result<R, k8s_openapi::ResponseError>, http::StatusCode, &[u8]) -> Result<SingleValueResult<T>, Error>,
{
	let mut response_body = bytes::BytesMut::new();

	let status_code = response.status();
	let status_code =
		http::StatusCode::from_u16(status_code.into())
		.map_err(|err| format!("couldn't convert reqwest StatusCode {} to http StatusCode: {}", status_code, err))?;

	loop {
		let read = std::io::Read::read(&mut response, unsafe { bytes::BufMut::bytes_mut(&mut response_body) })?;
		if read == 0 {
			return Err("unexpected EOF".into());
		}
		unsafe { bytes::BufMut::advance_mut(&mut response_body, read); }

		{
			let response_buf = &*response_body;
			let response = <R as k8s_openapi::Response>::try_from_slice(status_code, response_buf);
			match f(response, status_code, response_buf)? {
				SingleValueResult::GotValue(result) => return Ok(result),
				SingleValueResult::NeedMoreData => (),
			}
		}

		response_body.reserve(4096);
	}
}

enum SingleValueResult<T> {
	GotValue(T),
	NeedMoreData,
}

macro_rules! get_borrowed_value {
	($response:ident, $f:expr) => { {
		let result: Result<_, ::Error> = loop {
			let mut response = $response;
			let mut response_body = ::bytes::BytesMut::new();

			let status_code = response.status();
			let status_code = match ::http::StatusCode::from_u16(status_code.into()) {
				Ok(status_code) => status_code,
				Err(err) => break Err(format!("couldn't convert reqwest StatusCode {} to http StatusCode: {}", status_code, err).into()),
			};

			break loop {
				let read = match ::std::io::Read::read(&mut response, unsafe { ::bytes::BufMut::bytes_mut(&mut response_body) }) {
					Ok(0) =>
						if response_body.is_empty() {
							break Ok(());
						}
						else {
							break Err("unexpected EOF".into());
						},

					Ok(read) => read,

					Err(err) => break Err(err.into()),
				};
				unsafe { ::bytes::BufMut::advance_mut(&mut response_body, read); }

				{
					let response_buf = &*response_body;
					let response = ::k8s_openapi::Response::try_from_slice(status_code, response_buf);
					match ($f)(response, status_code, response_buf) {
						Ok(::SingleValueResult::GotValue(result)) => break Ok(result),
						Ok(::SingleValueResult::NeedMoreData) => (),
						Err(err) => break Err(err),
					}
				}

				response_body.reserve(4096);
			};
		};

		result
	} };
}

fn get_multiple_values<R, F, T>(response: reqwest::Response, f: F) -> Result<MultipleValuesIterator<R, F, T>, Error> {
	MultipleValuesIterator::new(response, f)
}

struct MultipleValuesIterator<R, F, T> {
	response: reqwest::Response,
	f: F,
	status_code: http::StatusCode,
	response_body: bytes::BytesMut,
	_pd: std::marker::PhantomData<(R, T)>,
}

impl<R, F, T> MultipleValuesIterator<R, F, T> {
	fn new(response: reqwest::Response, f: F) -> Result<Self, Error> {
		let status_code = response.status();
		let status_code =
			http::StatusCode::from_u16(status_code.into())
			.map_err(|err| format!("couldn't convert reqwest::StatusCode {} to http::StatusCode: {}", status_code, err))?;

		Ok(MultipleValuesIterator {
			response,
			f,
			status_code,
			response_body: bytes::BytesMut::new(),
			_pd: Default::default(),
		})
	}
}

impl<R, F, T> Iterator for MultipleValuesIterator<R, F, T> where
	for<'r> R: k8s_openapi::Response<'r>,
	F: FnMut(Result<R, k8s_openapi::ResponseError>) -> Result<MultipleValuesResult<T>, Error>,
{
	type Item = Result<T, Error>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let read = match std::io::Read::read(&mut self.response, unsafe { bytes::BufMut::bytes_mut(&mut self.response_body) }) {
				Ok(0) =>
					if self.response_body.is_empty() {
						return None;
					}
					else {
						return Some(Err("unexpected EOF".into()));
					},

				Ok(read) => read,

				Err(err) => return Some(Err(err.into())),
			};

			unsafe { bytes::BufMut::advance_mut(&mut self.response_body, read); }

			let response = <R as k8s_openapi::Response>::try_from_slice(self.status_code, &*self.response_body);
			match (self.f)(response) {
				Ok(MultipleValuesResult::GotValue(result, byte_offset)) => {
					self.response_body.advance(byte_offset);
					return Some(Ok(result));
				},

				Ok(MultipleValuesResult::NeedMoreData) => self.response_body.reserve(4096),

				Err(err) => return Some(Err(err)),
			}
		}
	}
}

enum MultipleValuesResult<T> {
	GotValue(T, usize),
	NeedMoreData,
}

#[derive(Deserialize)]
struct KubeConfig {
	clusters: Vec<KubeConfigClusterEntry>,
	contexts: Vec<KubeConfigContextEntry>,
	#[serde(rename = "current-context")]
	current_context: String,
	users: Vec<KubeConfigUserEntry>,
}

#[derive(Deserialize)]
struct KubeConfigClusterEntry {
	cluster: KubeConfigCluster,
	name: String,
}

#[derive(Deserialize)]
struct KubeConfigCluster {
	#[serde(rename = "certificate-authority")]
	certificate_authority: std::path::PathBuf,
	server: String,
}

#[derive(Deserialize)]
struct KubeConfigContextEntry {
	context: KubeConfigContext,
	name: String,
}

#[derive(Deserialize)]
struct KubeConfigContext {
	cluster: String,
	user: String,
}

#[derive(Deserialize)]
struct KubeConfigUserEntry {
	name: String,
	user: KubeConfigUser,
}

#[derive(Deserialize)]
struct KubeConfigUser {
	#[serde(rename = "client-certificate")]
	client_certificate: std::path::PathBuf,
	#[serde(rename = "client-key")]
	client_key: std::path::PathBuf,
}

mod api_versions;

#[cfg(not(feature = "v1_7"))] // CRDs not supported in v1.7
mod custom_resource_definition;

mod deployment;

mod job;

mod logs;

mod pod;

mod special_idents;

mod watch_event;
