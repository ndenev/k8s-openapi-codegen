#[test]
fn list() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7 as k8s;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8 as k8s;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9 as k8s;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10 as k8s;

	let client = ::Client::new().expect("couldn't create client");

	let request = k8s::get_api_versions().expect("couldn't get API versions");
	let response = client.execute(request).expect("couldn't get API versions");
	let api_versions =
		::get_single_value(response, |response, _, _| match response {
			Ok(k8s::GetAPIVersionsResponse::Ok(api_versions)) => Ok(::SingleValueResult::GotValue(api_versions)),
			Ok(other) => Err(format!("{:?}", other).into()),
			Err(::k8s_openapi::ResponseError::NeedMoreData) => Ok(::SingleValueResult::NeedMoreData),
			Err(err) => Err(err.into()),
		}).expect("couldn't get API versions");

	assert_eq!(api_versions.kind, Some("APIGroupList".to_string()));
}
