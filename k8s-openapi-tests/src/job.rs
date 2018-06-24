#[test]
fn create() {
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::api::v1 as api;
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::kubernetes::pkg::apis::batch::v1 as batch;
	#[cfg(feature = "v1_7")] use ::k8s_openapi::v1_7::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::core::v1 as api;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::api::batch::v1 as batch;
	#[cfg(feature = "v1_8")] use ::k8s_openapi::v1_8::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::core::v1 as api;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::api::batch::v1 as batch;
	#[cfg(feature = "v1_9")] use ::k8s_openapi::v1_9::apimachinery::pkg::apis::meta::v1 as meta;

	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::core::v1 as api;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::api::batch::v1 as batch;
	#[cfg(feature = "v1_10")] use ::k8s_openapi::v1_10::apimachinery::pkg::apis::meta::v1 as meta;

	let client = ::Client::new().expect("couldn't create client");

	let job_spec = batch::JobSpec {
		template: api::PodTemplateSpec {
			spec: Some(api::PodSpec {
				containers: vec![
					api::Container {
						name: "k8s-openapi-tests-create-job".to_string(),
						image: "alpine".to_string().into(),
						command: Some(vec![
							"sh".to_string(),
							"-c".to_string(),
							"exit $TEST_ARG".to_string(),
						]),
						env: Some(vec![
							api::EnvVar {
								name: "TEST_ARG".to_string(),
								value: Some("5".to_string()),
								..Default::default()
							},
						]),
						..Default::default()
					},
				],
				restart_policy: Some("Never".to_string()),
				..Default::default()
			}),
			..Default::default()
		},
		..Default::default()
	};
	#[cfg(not(feature = "v1_7"))]
	let job_spec = batch::JobSpec {
		backoff_limit: Some(0),
		..job_spec
	};

	let job = batch::Job {
		metadata: Some(meta::ObjectMeta {
			name: Some("k8s-openapi-tests-create-job".to_string()),
			..Default::default()
		}),
		spec: Some(job_spec),
		..Default::default()
	};

	let request =
		batch::Job::create_batch_v1_namespaced_job("default", &job, None)
		.expect("couldn't create job");
	let response = client.execute(request).expect("couldn't create job");
	let job: batch::Job =
		::get_single_value(response, |response, _status_code, _response_body| match response {
			#[cfg(any(feature = "v1_7", feature = "v1_8"))] Ok(batch::CreateBatchV1NamespacedJobResponse::Other) if _status_code == ::http::StatusCode::CREATED =>
				match ::serde_json::from_slice(_response_body) {
					Ok(job) => Ok(::SingleValueResult::GotValue(job)),
					Err(ref err) if err.is_eof() => Ok(::SingleValueResult::NeedMoreData),
					Err(err) => Err(err.into()),
				},
			#[cfg(not(any(feature = "v1_7", feature = "v1_8")))] Ok(batch::CreateBatchV1NamespacedJobResponse::Created(job)) =>
				Ok(::SingleValueResult::GotValue(job)),
			Ok(other) => Err(format!("{:?}", other).into()),
			Err(::k8s_openapi::ResponseError::NeedMoreData) => Ok(::SingleValueResult::NeedMoreData),
			Err(err) => Err(err.into()),
		}).expect("couldn't create job");

	let job_image =
		job
		.spec.expect("couldn't get job spec")
		.template
		.spec.expect("couldn't get job spec template spec")
		.containers.into_iter().next().expect("couldn't get job container spec")
		.image;
	#[cfg(not(feature = "v1_7"))]
	let job_image = job_image.expect("couldn't get job container image");
	assert_eq!(job_image, "alpine");

	let (job_self_link, job_uid) = {
		let metadata = job.metadata.expect("couldn't get job metadata");
		(metadata.self_link.expect("couldn't get job self link"), metadata.uid.expect("couldn't get job uid"))
	};

	// Wait for job to fail
	loop {
		let response = client.get(&job_self_link).expect("couldn't get job");
		let job: batch::Job =
			::get_single_value(response, |response, _, _| match response {
				Ok(batch::ReadBatchV1NamespacedJobResponse::Ok(job)) => Ok(::SingleValueResult::GotValue(job)),
				Ok(other) => Err(format!("{:?}", other).into()),
				Err(::k8s_openapi::ResponseError::NeedMoreData) => Ok(::SingleValueResult::NeedMoreData),
				Err(err) => Err(err.into()),
			}).expect("couldn't get job");

		let job_status =
			job
			.status.expect("couldn't get job status");

		if job_status.failed == Some(1) {
			break;
		}

		::std::thread::sleep(::std::time::Duration::from_secs(1));
	}

	// Find a pod of the failed job using owner reference
	let job_pod_status = loop {
		#[cfg(feature = "v1_7")] let request =
			api::Pod::list_core_v1_namespaced_pod("default", None, None, None, None, None, None, None);
		#[cfg(not(feature = "v1_7"))] let request =
			api::Pod::list_core_v1_namespaced_pod("default", None, None, None, None, None, None, None, None, None);
		let request = request.expect("couldn't list pods");
		let response = client.execute(request).expect("couldn't list pods");;
		let pod_list =
			::get_single_value(response, |response, _, _| match response {
				Ok(api::ListCoreV1NamespacedPodResponse::Ok(pod_list)) => Ok(::SingleValueResult::GotValue(pod_list)),
				Ok(other) => Err(format!("{:?}", other).into()),
				Err(::k8s_openapi::ResponseError::NeedMoreData) => Ok(::SingleValueResult::NeedMoreData),
				Err(err) => Err(err.into()),
			}).expect("couldn't list pods");

		let job_pod_status =
			pod_list
			.items.into_iter()
			.find(|pod|
				pod.metadata.as_ref()
				.and_then(|metadata| metadata.owner_references.as_ref())
				.and_then(|owner_references| owner_references.first())
				.map(|owner_reference| owner_reference.uid.as_ref()) == Some(&*job_uid))
			.and_then(|job_pod| job_pod.status);

		if let Some(job_pod_status) = job_pod_status {
			if job_pod_status.phase == Some("Failed".to_string()) {
				break job_pod_status;
			}
		}

		::std::thread::sleep(::std::time::Duration::from_secs(1));
	};

	let job_pod_container_state_terminated =
		job_pod_status
		.container_statuses.expect("couldn't get job pod container statuses")
		.into_iter().next().expect("couldn't get job pod container status")
		.state.expect("couldn't get job pod container state")
		.terminated.expect("couldn't get job pod container termination info");
	assert_eq!(job_pod_container_state_terminated.exit_code, 5);

	let response = client.delete(&job_self_link).expect("couldn't delete job");
	::get_single_value::<batch::DeleteBatchV1NamespacedJobResponse, _, _>(response, |_, status_code, _| match status_code {
		::http::StatusCode::OK => Ok(::SingleValueResult::GotValue(())),
		other => Err(other.to_string().into()),
	}).expect("couldn't delete job");

	// Delete all pods of the job using label selector
	#[cfg(feature = "v1_7")] let request =
		api::Pod::list_core_v1_namespaced_pod("default", None, None, Some("job-name=k8s-openapi-tests-create-job"), None, None, None, None);
	#[cfg(not(feature = "v1_7"))] let request =
		api::Pod::list_core_v1_namespaced_pod("default", None, None, None, Some("job-name=k8s-openapi-tests-create-job"), None, None, None, None, None);
	let request = request.expect("couldn't list pods");
	let response = client.execute(request).expect("couldn't list pods");;
	let pod_list =
		::get_single_value(response, |response, _, _| match response {
			Ok(api::ListCoreV1NamespacedPodResponse::Ok(pod_list)) => Ok(::SingleValueResult::GotValue(pod_list)),
			Ok(other) => Err(format!("{:?}", other).into()),
			Err(::k8s_openapi::ResponseError::NeedMoreData) => Ok(::SingleValueResult::NeedMoreData),
			Err(err) => Err(err.into()),
		}).expect("couldn't list pods");

	for pod in pod_list.items {
		let self_link =
			pod.metadata.expect("couldn't get job pod metadata")
			.self_link.expect("couldn't get job pod self link");
		let response = client.delete(&self_link).expect("couldn't delete job pod");
		::get_single_value::<api::DeleteCoreV1NamespacedPodResponse, _, _>(response, |_, status_code, _| match status_code {
			::http::StatusCode::OK => Ok(::SingleValueResult::GotValue(())),
			other => Err(other.to_string().into()),
		}).expect("couldn't delete job pod");
	}
}
