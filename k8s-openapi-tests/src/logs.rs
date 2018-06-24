#[test]
fn get() {
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

	let addon_manager_pod =
		pod_list
		.items.into_iter()
		.find(|pod| pod.metadata.as_ref().and_then(|metadata| metadata.name.as_ref()).map_or(false, |name| name.starts_with("kube-addon-manager-")))
		.expect("couldn't find addon-manager pod");

	let addon_manager_pod_name =
		addon_manager_pod
		.metadata.as_ref().expect("couldn't get addon-manager pod metadata")
		.name.as_ref().expect("couldn't get addon-manager pod name");

	let request =
		api::Pod::read_core_v1_namespaced_pod_log(
			addon_manager_pod_name, "kube-system", Some("kube-addon-manager"), None, None, None, None, None, None, None)
		.expect("couldn't get addon-manager pod logs");
	let response = client.execute(request).expect("couldn't get addon-manager pod logs");
	get_borrowed_value!(response, |response, _, _| match response {
		Ok(api::ReadCoreV1NamespacedPodLogResponse::Ok(addon_manager_logs)) =>
			if addon_manager_logs.contains("INFO: == Kubernetes addon manager started at") {
				Ok(::SingleValueResult::GotValue(()))
			}
			else if addon_manager_logs.len() > 4096 {
				Err(format!("did not find expected text in addon-manager pod logs: {}", addon_manager_logs).into())
			}
			else {
				Ok(::SingleValueResult::NeedMoreData)
			},

		Ok(other) => Err(format!("{:?}", other).into()),
		Err(::k8s_openapi::ResponseError::NeedMoreData) => unreachable!(),
		Err(err) => Err(err.into()),
	}).expect("couldn't get addon-manager pod logs");
}
