#[test]
fn list() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::core::v1 as api;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::core::v1 as api;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::core::v1 as api;

	let client = ::Client::new().expect("couldn't create client");

	#[cfg(feature = "v1_7")] let request =
		api::Pod::list_core_v1_namespaced_pod("kube-system", None, None, None, None, None, None, None);
	#[cfg(not(feature = "v1_7"))] let request =
		api::Pod::list_core_v1_namespaced_pod("kube-system", None, None, None, None, None, None, None, None, None);
	let request = request.expect("couldn't list pods");
	let response = client.execute(request).expect("couldn't list pods");;
	let pod_list =
		::get_single_value(response, |response, _, _| match response {
			Ok(api::ListCoreV1NamespacedPodResponse::Ok(pod_list)) => Ok(::SingleValueResult::GotValue(pod_list)),
			Ok(other) => Err(format!("{:?}", other).into()),
			Err(::k8s_openapi::ResponseError::NeedMoreData) => Ok(::SingleValueResult::NeedMoreData),
			Err(err) => Err(err.into()),
		}).expect("couldn't list pods");

	assert_eq!(pod_list.kind, Some("PodList".to_string()));

	let addon_manager_pod =
		pod_list
		.items.into_iter()
		.find(|pod| pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")))
		.expect("couldn't find addon-manager pod");

	let addon_manager_container_spec =
		addon_manager_pod
		.spec.expect("couldn't get addon-manager pod spec")
		.containers
		.into_iter().next().expect("couldn't get addon-manager container spec");
	assert_eq!(addon_manager_container_spec.name, "kube-addon-manager");

	let addon_manager_pod_status = addon_manager_pod.status.expect("couldn't get addon-manager pod status");
	assert_eq!(addon_manager_pod_status.phase, Some("Running".to_string()));
}
