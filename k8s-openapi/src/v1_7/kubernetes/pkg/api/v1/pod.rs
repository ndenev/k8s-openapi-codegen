// Generated from definition io.k8s.kubernetes.pkg.api.v1.Pod

/// Pod is a collection of containers that can run on a host. This resource is created by clients and scheduled onto hosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pod {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the pod. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_7::kubernetes::pkg::api::v1::PodSpec>,

    /// Most recently observed status of the pod. This data may not be up to date. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<::v1_7::kubernetes::pkg::api::v1::PodStatus>,
}

// Begin /v1/Pod

// Generated from operation connectCoreV1DeleteNamespacedPodProxy

impl Pod {
    /// connect DELETE requests to proxy of Pod
    pub fn connect_core_v1_delete_namespaced_pod_proxy(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", &path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedPodProxyResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1DeleteNamespacedPodProxyResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1DeleteNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNamespacedPodProxyResponse::Unauthorized,
            _ => ConnectCoreV1DeleteNamespacedPodProxyResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1DeleteNamespacedPodProxyWithPath

impl Pod {
    /// connect DELETE requests to proxy of Pod
    pub fn connect_core_v1_delete_namespaced_pod_proxy_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", &path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Unauthorized,
            _ => ConnectCoreV1DeleteNamespacedPodProxyWithPathResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedPodAttach

impl Pod {
    /// connect GET requests to attach of Pod
    pub fn connect_core_v1_get_namespaced_pod_attach(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
        stderr: Option<bool>,
        // Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
        tty: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/attach?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", &container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodAttachResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1GetNamespacedPodAttachResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1GetNamespacedPodAttachResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodAttachResponse::Unauthorized,
            _ => ConnectCoreV1GetNamespacedPodAttachResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedPodExec

impl Pod {
    /// connect GET requests to exec of Pod
    pub fn connect_core_v1_get_namespaced_pod_exec(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Command is the remote command to execute. argv array. Not executed within a shell.
        command: Option<&str>,
        // Container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Redirect the standard error stream of the pod for this call. Defaults to true.
        stderr: Option<bool>,
        // Redirect the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Redirect the standard output stream of the pod for this call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
        tty: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/exec?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(command) = command {
            __query_pairs.append_pair("command", &command);
        }
        if let Some(container) = container {
            __query_pairs.append_pair("container", &container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodExecResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1GetNamespacedPodExecResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1GetNamespacedPodExecResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodExecResponse::Unauthorized,
            _ => ConnectCoreV1GetNamespacedPodExecResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedPodPortforward

impl Pod {
    /// connect GET requests to portforward of Pod
    pub fn connect_core_v1_get_namespaced_pod_portforward(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // List of ports to forward Required when using WebSockets
        ports: Option<i64>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodPortforwardResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1GetNamespacedPodPortforwardResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1GetNamespacedPodPortforwardResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodPortforwardResponse::Unauthorized,
            _ => ConnectCoreV1GetNamespacedPodPortforwardResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedPodProxy

impl Pod {
    /// connect GET requests to proxy of Pod
    pub fn connect_core_v1_get_namespaced_pod_proxy(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", &path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodProxyResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1GetNamespacedPodProxyResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1GetNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodProxyResponse::Unauthorized,
            _ => ConnectCoreV1GetNamespacedPodProxyResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1GetNamespacedPodProxyWithPath

impl Pod {
    /// connect GET requests to proxy of Pod
    pub fn connect_core_v1_get_namespaced_pod_proxy_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", &path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1GetNamespacedPodProxyWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1GetNamespacedPodProxyWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Unauthorized,
            _ => ConnectCoreV1GetNamespacedPodProxyWithPathResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PatchNamespacedPodProxy

impl Pod {
    /// connect PATCH requests to proxy of Pod
    pub fn connect_core_v1_patch_namespaced_pod_proxy(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", &path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedPodProxyResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PatchNamespacedPodProxyResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PatchNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNamespacedPodProxyResponse::Unauthorized,
            _ => ConnectCoreV1PatchNamespacedPodProxyResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PatchNamespacedPodProxyWithPath

impl Pod {
    /// connect PATCH requests to proxy of Pod
    pub fn connect_core_v1_patch_namespaced_pod_proxy_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", &path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PatchNamespacedPodProxyWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PatchNamespacedPodProxyWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Unauthorized,
            _ => ConnectCoreV1PatchNamespacedPodProxyWithPathResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedPodAttach

impl Pod {
    /// connect POST requests to attach of Pod
    pub fn connect_core_v1_post_namespaced_pod_attach(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Stderr if true indicates that stderr is to be redirected for the attach call. Defaults to true.
        stderr: Option<bool>,
        // Stdin if true, redirects the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Stdout if true indicates that stdout is to be redirected for the attach call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the attach call. This is passed through the container runtime so the tty is allocated on the worker node by the container runtime. Defaults to false.
        tty: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/attach?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", &container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodAttachResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PostNamespacedPodAttachResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PostNamespacedPodAttachResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodAttachResponse::Unauthorized,
            _ => ConnectCoreV1PostNamespacedPodAttachResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedPodExec

impl Pod {
    /// connect POST requests to exec of Pod
    pub fn connect_core_v1_post_namespaced_pod_exec(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Command is the remote command to execute. argv array. Not executed within a shell.
        command: Option<&str>,
        // Container in which to execute the command. Defaults to only container if there is only one container in the pod.
        container: Option<&str>,
        // Redirect the standard error stream of the pod for this call. Defaults to true.
        stderr: Option<bool>,
        // Redirect the standard input stream of the pod for this call. Defaults to false.
        stdin: Option<bool>,
        // Redirect the standard output stream of the pod for this call. Defaults to true.
        stdout: Option<bool>,
        // TTY if true indicates that a tty will be allocated for the exec call. Defaults to false.
        tty: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/exec?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(command) = command {
            __query_pairs.append_pair("command", &command);
        }
        if let Some(container) = container {
            __query_pairs.append_pair("container", &container);
        }
        if let Some(stderr) = stderr {
            __query_pairs.append_pair("stderr", &stderr.to_string());
        }
        if let Some(stdin) = stdin {
            __query_pairs.append_pair("stdin", &stdin.to_string());
        }
        if let Some(stdout) = stdout {
            __query_pairs.append_pair("stdout", &stdout.to_string());
        }
        if let Some(tty) = tty {
            __query_pairs.append_pair("tty", &tty.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodExecResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PostNamespacedPodExecResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PostNamespacedPodExecResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodExecResponse::Unauthorized,
            _ => ConnectCoreV1PostNamespacedPodExecResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedPodPortforward

impl Pod {
    /// connect POST requests to portforward of Pod
    pub fn connect_core_v1_post_namespaced_pod_portforward(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // List of ports to forward Required when using WebSockets
        ports: Option<i64>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/portforward?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(ports) = ports {
            __query_pairs.append_pair("ports", &ports.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodPortforwardResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PostNamespacedPodPortforwardResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PostNamespacedPodPortforwardResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodPortforwardResponse::Unauthorized,
            _ => ConnectCoreV1PostNamespacedPodPortforwardResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedPodProxy

impl Pod {
    /// connect POST requests to proxy of Pod
    pub fn connect_core_v1_post_namespaced_pod_proxy(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", &path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodProxyResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PostNamespacedPodProxyResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PostNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodProxyResponse::Unauthorized,
            _ => ConnectCoreV1PostNamespacedPodProxyResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PostNamespacedPodProxyWithPath

impl Pod {
    /// connect POST requests to proxy of Pod
    pub fn connect_core_v1_post_namespaced_pod_proxy_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", &path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PostNamespacedPodProxyWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PostNamespacedPodProxyWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Unauthorized,
            _ => ConnectCoreV1PostNamespacedPodProxyWithPathResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PutNamespacedPodProxy

impl Pod {
    /// connect PUT requests to proxy of Pod
    pub fn connect_core_v1_put_namespaced_pod_proxy(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path) = path {
            __query_pairs.append_pair("path", &path);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedPodProxyResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PutNamespacedPodProxyResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PutNamespacedPodProxyResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNamespacedPodProxyResponse::Unauthorized,
            _ => ConnectCoreV1PutNamespacedPodProxyResponse::Other,
        })
    }
}

// Generated from operation connectCoreV1PutNamespacedPodProxyWithPath

impl Pod {
    /// connect PUT requests to proxy of Pod
    pub fn connect_core_v1_put_namespaced_pod_proxy_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
        // Path is the URL path to use for the current proxy request to pod.
        path_: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/proxy/{path}?", name = name, namespace = namespace, path = path);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(path_) = path_ {
            __query_pairs.append_pair("path", &path_);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ConnectCoreV1PutNamespacedPodProxyWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ConnectCoreV1PutNamespacedPodProxyWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Unauthorized,
            _ => ConnectCoreV1PutNamespacedPodProxyWithPathResponse::Other,
        })
    }
}

// Generated from operation createCoreV1NamespacedPod

impl Pod {
    /// create a Pod
    pub fn create_core_v1_namespaced_pod(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Pod,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::post(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum CreateCoreV1NamespacedPodResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for CreateCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                CreateCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateCoreV1NamespacedPodResponse::Unauthorized,
            _ => CreateCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation deleteCoreV1CollectionNamespacedPod

impl Pod {
    /// delete collection of Pod
    pub fn delete_core_v1_collection_namespaced_pod(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", &resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum DeleteCoreV1CollectionNamespacedPodResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteCoreV1CollectionNamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteCoreV1CollectionNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1CollectionNamespacedPodResponse::Unauthorized,
            _ => DeleteCoreV1CollectionNamespacedPodResponse::Other,
        })
    }
}

// Generated from operation deleteCoreV1NamespacedPod

impl Pod {
    /// delete a Pod
    pub fn delete_core_v1_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(grace_period_seconds) = grace_period_seconds {
            __query_pairs.append_pair("gracePeriodSeconds", &grace_period_seconds.to_string());
        }
        if let Some(orphan_dependents) = orphan_dependents {
            __query_pairs.append_pair("orphanDependents", &orphan_dependents.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(propagation_policy) = propagation_policy {
            __query_pairs.append_pair("propagationPolicy", &propagation_policy);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum DeleteCoreV1NamespacedPodResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteCoreV1NamespacedPodResponse::Unauthorized,
            _ => DeleteCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation listCoreV1NamespacedPod

impl Pod {
    /// list or watch objects of kind Pod
    pub fn list_core_v1_namespaced_pod(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", &resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ListCoreV1NamespacedPodResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::PodList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1NamespacedPodResponse::Unauthorized,
            _ => ListCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation listCoreV1PodForAllNamespaces

impl Pod {
    /// list or watch objects of kind Pod
    pub fn list_core_v1_pod_for_all_namespaces(
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/pods?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", &resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ListCoreV1PodForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::PodList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListCoreV1PodForAllNamespacesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListCoreV1PodForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListCoreV1PodForAllNamespacesResponse::Unauthorized,
            _ => ListCoreV1PodForAllNamespacesResponse::Other,
        })
    }
}

// Generated from operation patchCoreV1NamespacedPod

impl Pod {
    /// partially update the specified Pod
    pub fn patch_core_v1_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum PatchCoreV1NamespacedPodResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for PatchCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                PatchCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedPodResponse::Unauthorized,
            _ => PatchCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation patchCoreV1NamespacedPodStatus

impl Pod {
    /// partially update status of the specified Pod
    pub fn patch_core_v1_namespaced_pod_status(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::patch(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum PatchCoreV1NamespacedPodStatusResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for PatchCoreV1NamespacedPodStatusResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                PatchCoreV1NamespacedPodStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchCoreV1NamespacedPodStatusResponse::Unauthorized,
            _ => PatchCoreV1NamespacedPodStatusResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1DELETENamespacedPod

impl Pod {
    /// proxy DELETE requests to Pod
    pub fn proxy_core_v1_delete_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedPodResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1DELETENamespacedPodResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1DELETENamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1DELETENamespacedPodResponse::Unauthorized,
            _ => ProxyCoreV1DELETENamespacedPodResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1DELETENamespacedPodWithPath

impl Pod {
    /// proxy DELETE requests to Pod
    pub fn proxy_core_v1_delete_namespaced_pod_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::delete(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1DELETENamespacedPodWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1DELETENamespacedPodWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1DELETENamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1DELETENamespacedPodWithPathResponse::Unauthorized,
            _ => ProxyCoreV1DELETENamespacedPodWithPathResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1GETNamespacedPod

impl Pod {
    /// proxy GET requests to Pod
    pub fn proxy_core_v1_get_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedPodResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1GETNamespacedPodResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1GETNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1GETNamespacedPodResponse::Unauthorized,
            _ => ProxyCoreV1GETNamespacedPodResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1GETNamespacedPodWithPath

impl Pod {
    /// proxy GET requests to Pod
    pub fn proxy_core_v1_get_namespaced_pod_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1GETNamespacedPodWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1GETNamespacedPodWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1GETNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1GETNamespacedPodWithPathResponse::Unauthorized,
            _ => ProxyCoreV1GETNamespacedPodWithPathResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1PATCHNamespacedPod

impl Pod {
    /// proxy PATCH requests to Pod
    pub fn proxy_core_v1_patch_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedPodResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1PATCHNamespacedPodResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1PATCHNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PATCHNamespacedPodResponse::Unauthorized,
            _ => ProxyCoreV1PATCHNamespacedPodResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1PATCHNamespacedPodWithPath

impl Pod {
    /// proxy PATCH requests to Pod
    pub fn proxy_core_v1_patch_namespaced_pod_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::patch(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1PATCHNamespacedPodWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1PATCHNamespacedPodWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1PATCHNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PATCHNamespacedPodWithPathResponse::Unauthorized,
            _ => ProxyCoreV1PATCHNamespacedPodWithPathResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1POSTNamespacedPod

impl Pod {
    /// proxy POST requests to Pod
    pub fn proxy_core_v1_post_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedPodResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1POSTNamespacedPodResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1POSTNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1POSTNamespacedPodResponse::Unauthorized,
            _ => ProxyCoreV1POSTNamespacedPodResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1POSTNamespacedPodWithPath

impl Pod {
    /// proxy POST requests to Pod
    pub fn proxy_core_v1_post_namespaced_pod_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::post(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1POSTNamespacedPodWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1POSTNamespacedPodWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1POSTNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1POSTNamespacedPodWithPathResponse::Unauthorized,
            _ => ProxyCoreV1POSTNamespacedPodWithPathResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1PUTNamespacedPod

impl Pod {
    /// proxy PUT requests to Pod
    pub fn proxy_core_v1_put_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}", name = name, namespace = namespace);

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedPodResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1PUTNamespacedPodResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1PUTNamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PUTNamespacedPodResponse::Unauthorized,
            _ => ProxyCoreV1PUTNamespacedPodResponse::Other,
        })
    }
}

// Generated from operation proxyCoreV1PUTNamespacedPodWithPath

impl Pod {
    /// proxy PUT requests to Pod
    pub fn proxy_core_v1_put_namespaced_pod_with_path(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // path to the resource
        path: &str,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/proxy/namespaces/{namespace}/pods/{name}/{path}", name = name, namespace = namespace, path = path);

        let mut __request = ::http::Request::put(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ProxyCoreV1PUTNamespacedPodWithPathResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ProxyCoreV1PUTNamespacedPodWithPathResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ProxyCoreV1PUTNamespacedPodWithPathResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ProxyCoreV1PUTNamespacedPodWithPathResponse::Unauthorized,
            _ => ProxyCoreV1PUTNamespacedPodWithPathResponse::Other,
        })
    }
}

// Generated from operation readCoreV1NamespacedPod

impl Pod {
    /// read the specified Pod
    pub fn read_core_v1_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(exact) = exact {
            __query_pairs.append_pair("exact", &exact.to_string());
        }
        if let Some(export) = export {
            __query_pairs.append_pair("export", &export.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReadCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedPodResponse::Unauthorized,
            _ => ReadCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation readCoreV1NamespacedPodLog

impl Pod {
    /// read log of the specified Pod
    pub fn read_core_v1_namespaced_pod_log(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // The container for which to stream logs. Defaults to only container if there is one container in the pod.
        container: Option<&str>,
        // Follow the log stream of the pod. Defaults to false.
        follow: Option<bool>,
        // If set, the number of bytes to read from the server before terminating the log output. This may not display a complete final line of logging, and may return slightly more or slightly less than the specified limit.
        limit_bytes: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Return previous terminated container logs. Defaults to false.
        previous: Option<bool>,
        // A relative time in seconds before the current time from which to show logs. If this value precedes the time a pod was started, only logs since the pod start will be returned. If this value is in the future, no logs will be returned. Only one of sinceSeconds or sinceTime may be specified.
        since_seconds: Option<i64>,
        // If set, the number of lines from the end of the logs to show. If not specified, logs are shown from the creation of the container or sinceSeconds or sinceTime
        tail_lines: Option<i64>,
        // If true, add an RFC3339 or RFC3339Nano timestamp at the beginning of every line of log output. Defaults to false.
        timestamps: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/log?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(container) = container {
            __query_pairs.append_pair("container", &container);
        }
        if let Some(follow) = follow {
            __query_pairs.append_pair("follow", &follow.to_string());
        }
        if let Some(limit_bytes) = limit_bytes {
            __query_pairs.append_pair("limitBytes", &limit_bytes.to_string());
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(previous) = previous {
            __query_pairs.append_pair("previous", &previous.to_string());
        }
        if let Some(since_seconds) = since_seconds {
            __query_pairs.append_pair("sinceSeconds", &since_seconds.to_string());
        }
        if let Some(tail_lines) = tail_lines {
            __query_pairs.append_pair("tailLines", &tail_lines.to_string());
        }
        if let Some(timestamps) = timestamps {
            __query_pairs.append_pair("timestamps", &timestamps.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodLogResponse<'a> {
    Ok(&'a str),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadCoreV1NamespacedPodLogResponse<'a> {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::std::str::from_utf8(buf) {
                    Ok(s) => s,
                    Err(err) if err.error_len().is_none() => {
                        let valid_up_to = err.valid_up_to();
                        unsafe { ::std::str::from_utf8_unchecked(&buf[..valid_up_to]) }
                    },
                    Err(err) => return Err(::ResponseError::Utf8(err)),
                };
                ReadCoreV1NamespacedPodLogResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedPodLogResponse::Unauthorized,
            _ => ReadCoreV1NamespacedPodLogResponse::Other,
        })
    }
}

// Generated from operation readCoreV1NamespacedPodStatus

impl Pod {
    /// read status of the specified Pod
    pub fn read_core_v1_namespaced_pod_status(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ReadCoreV1NamespacedPodStatusResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadCoreV1NamespacedPodStatusResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReadCoreV1NamespacedPodStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadCoreV1NamespacedPodStatusResponse::Unauthorized,
            _ => ReadCoreV1NamespacedPodStatusResponse::Other,
        })
    }
}

// Generated from operation replaceCoreV1NamespacedPod

impl Pod {
    /// replace the specified Pod
    pub fn replace_core_v1_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Pod,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPodResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReplaceCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceCoreV1NamespacedPodResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedPodResponse::Unauthorized,
            _ => ReplaceCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation replaceCoreV1NamespacedPodStatus

impl Pod {
    /// replace status of the specified Pod
    pub fn replace_core_v1_namespaced_pod_status(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::api::v1::Pod,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/namespaces/{namespace}/pods/{name}/status?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::put(__url);
        let __body = ::serde_json::to_vec(&body).map_err(::RequestError::Json)?;
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum ReplaceCoreV1NamespacedPodStatusResponse {
    Ok(::v1_7::kubernetes::pkg::api::v1::Pod),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReplaceCoreV1NamespacedPodStatusResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceCoreV1NamespacedPodStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceCoreV1NamespacedPodStatusResponse::Unauthorized,
            _ => ReplaceCoreV1NamespacedPodStatusResponse::Other,
        })
    }
}

// Generated from operation watchCoreV1NamespacedPod

impl Pod {
    /// watch changes to an object of kind Pod
    pub fn watch_core_v1_namespaced_pod(
        // name of the Pod
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/pods/{name}?", name = name, namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", &resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum WatchCoreV1NamespacedPodResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchCoreV1NamespacedPodResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                WatchCoreV1NamespacedPodResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedPodResponse::Unauthorized,
            _ => WatchCoreV1NamespacedPodResponse::Other,
        })
    }
}

// Generated from operation watchCoreV1NamespacedPodList

impl Pod {
    /// watch individual changes to a list of Pod
    pub fn watch_core_v1_namespaced_pod_list(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/watch/namespaces/{namespace}/pods?", namespace = namespace);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", &resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum WatchCoreV1NamespacedPodListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchCoreV1NamespacedPodListResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                WatchCoreV1NamespacedPodListResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1NamespacedPodListResponse::Unauthorized,
            _ => WatchCoreV1NamespacedPodListResponse::Other,
        })
    }
}

// Generated from operation watchCoreV1PodListForAllNamespaces

impl Pod {
    /// watch individual changes to a list of Pod
    pub fn watch_core_v1_pod_list_for_all_namespaces(
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/api/v1/watch/pods?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(pretty) = pretty {
            __query_pairs.append_pair("pretty", &pretty);
        }
        if let Some(resource_version) = resource_version {
            __query_pairs.append_pair("resourceVersion", &resource_version);
        }
        if let Some(timeout_seconds) = timeout_seconds {
            __query_pairs.append_pair("timeoutSeconds", &timeout_seconds.to_string());
        }
        if let Some(watch) = watch {
            __query_pairs.append_pair("watch", &watch.to_string());
        }
        let __url = __query_pairs.finish();

        let mut __request = ::http::Request::get(__url);
        let __body = vec![];
        __request.body(__body).map_err(::RequestError::Http)
    }
}

#[derive(Debug)]
pub enum WatchCoreV1PodListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchCoreV1PodListForAllNamespacesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let mut deserializer = ::serde_json::Deserializer::from_slice(buf).into_iter();
                let (result, byte_offset) = match deserializer.next() {
                    Some(Ok(value)) => (value, deserializer.byte_offset()),
                    Some(Err(ref err)) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Some(Err(err)) => return Err(::ResponseError::Json(err)),
                    None => return Err(::ResponseError::NeedMoreData),
                };
                WatchCoreV1PodListForAllNamespacesResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchCoreV1PodListForAllNamespacesResponse::Unauthorized,
            _ => WatchCoreV1PodListForAllNamespacesResponse::Other,
        })
    }
}

// End /v1/Pod

impl<'de> ::serde::Deserialize<'de> for Pod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> ::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(f, "field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Pod;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Pod")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::api::v1::PodSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::api::v1::PodStatus> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Pod {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Pod",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "spec",
                "status",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Pod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Pod",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.spec.as_ref().map_or(0, |_| 1) +
            self.status.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.spec {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "spec", value)?;
        }
        if let Some(value) = &self.status {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
