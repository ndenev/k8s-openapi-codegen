// Generated from definition io.k8s.kubernetes.pkg.apis.policy.v1beta1.PodDisruptionBudget

/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PodDisruptionBudget {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Specification of the desired behavior of the PodDisruptionBudget.
    pub spec: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetSpec>,

    /// Most recently observed status of the PodDisruptionBudget.
    pub status: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetStatus>,
}

// Begin policy/v1beta1/PodDisruptionBudget

// Generated from operation createPolicyV1beta1NamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// create a PodDisruptionBudget
    pub fn create_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.post(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => CreatePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation deletePolicyV1beta1CollectionNamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// delete collection of PodDisruptionBudget
    pub fn delete_policy_v1beta1_collection_namespaced_pod_disruption_budget<C>(
        __client: &C,
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
    ) -> Result<DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => DeletePolicyV1beta1CollectionNamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation deletePolicyV1beta1NamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// delete a PodDisruptionBudget
    pub fn delete_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
        // name of the PodDisruptionBudget
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
    ) -> Result<DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.delete(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => DeletePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation listPolicyV1beta1NamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    pub fn list_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
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
    ) -> Result<ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => ListPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation listPolicyV1beta1PodDisruptionBudgetForAllNamespaces

#[derive(Debug)]
pub enum ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// list or watch objects of kind PodDisruptionBudget
    pub fn list_policy_v1beta1_pod_disruption_budget_for_all_namespaces<C>(
        __client: &C,
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
    ) -> Result<ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/poddisruptionbudgets")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse::Unauthorized(response),
            other => ListPolicyV1beta1PodDisruptionBudgetForAllNamespacesResponse::Other(other, response),
        })
    }
}

// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// partially update the specified PodDisruptionBudget
    pub fn patch_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
        // name of the PodDisruptionBudget
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => PatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation patchPolicyV1beta1NamespacedPodDisruptionBudgetStatus

#[derive(Debug)]
pub enum PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// partially update status of the specified PodDisruptionBudget
    pub fn patch_policy_v1beta1_namespaced_pod_disruption_budget_status<C>(
        __client: &C,
        // name of the PodDisruptionBudget
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.patch(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Unauthorized(response),
            other => PatchPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// read the specified PodDisruptionBudget
    pub fn read_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
        // name of the PodDisruptionBudget
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(exact) = exact {
                __query_pairs.append_pair("exact", &exact.to_string());
            }
            if let Some(export) = export {
                __query_pairs.append_pair("export", &export.to_string());
            }
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => ReadPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation readPolicyV1beta1NamespacedPodDisruptionBudgetStatus

#[derive(Debug)]
pub enum ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// read status of the specified PodDisruptionBudget
    pub fn read_policy_v1beta1_namespaced_pod_disruption_budget_status<C>(
        __client: &C,
        // name of the PodDisruptionBudget
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Unauthorized(response),
            other => ReadPolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudget

#[derive(Debug)]
pub enum ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// replace the specified PodDisruptionBudget
    pub fn replace_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
        // name of the PodDisruptionBudget
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => ReplacePolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation replacePolicyV1beta1NamespacedPodDisruptionBudgetStatus

#[derive(Debug)]
pub enum ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// replace status of the specified PodDisruptionBudget
    pub fn replace_policy_v1beta1_namespaced_pod_disruption_budget_status<C>(
        __client: &C,
        // name of the PodDisruptionBudget
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudget,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/namespaces/{namespace}/poddisruptionbudgets/{name}/status", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
            if let Some(pretty) = pretty {
                __query_pairs.append_pair("pretty", &pretty);
            }
        }

        let response = __client.put(__url, &body).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Unauthorized(response),
            other => ReplacePolicyV1beta1NamespacedPodDisruptionBudgetStatusResponse::Other(other, response),
        })
    }
}

// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudget

pub enum WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// watch changes to an object of kind PodDisruptionBudget
    pub fn watch_policy_v1beta1_namespaced_pod_disruption_budget<C>(
        __client: &C,
        // name of the PodDisruptionBudget
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
    ) -> Result<WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/watch/namespaces/{namespace}/poddisruptionbudgets/{name}", name = name, namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Unauthorized(response),
            other => WatchPolicyV1beta1NamespacedPodDisruptionBudgetResponse::Other(other, response),
        })
    }
}

// Generated from operation watchPolicyV1beta1NamespacedPodDisruptionBudgetList

pub enum WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// watch individual changes to a list of PodDisruptionBudget
    pub fn watch_policy_v1beta1_namespaced_pod_disruption_budget_list<C>(
        __client: &C,
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
    ) -> Result<WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/watch/namespaces/{namespace}/poddisruptionbudgets", namespace = namespace)).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse::Unauthorized(response),
            other => WatchPolicyV1beta1NamespacedPodDisruptionBudgetListResponse::Other(other, response),
        })
    }
}

// Generated from operation watchPolicyV1beta1PodDisruptionBudgetListForAllNamespaces

pub enum WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl PodDisruptionBudget {
    /// watch individual changes to a list of PodDisruptionBudget
    pub fn watch_policy_v1beta1_pod_disruption_budget_list_for_all_namespaces<C>(
        __client: &C,
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
    ) -> Result<WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/policy/v1beta1/watch/poddisruptionbudgets")).map_err(::Error::URL)?;
        {
            let mut __query_pairs = __url.query_pairs_mut();
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
        }

        let response = __client.get(__url).map_err(::Error::Client)?;

        Ok(match ::Response::status_code(&response) {
            ::http::StatusCode::OK => {
                let result = ::serde_json::Deserializer::from_reader(response).into_iter();
                WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse::Unauthorized(response),
            other => WatchPolicyV1beta1PodDisruptionBudgetListForAllNamespacesResponse::Other(other, response),
        })
    }
}

// End policy/v1beta1/PodDisruptionBudget

impl<'de> ::serde::Deserialize<'de> for PodDisruptionBudget {
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
            type Value = PodDisruptionBudget;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct PodDisruptionBudget")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::policy::v1beta1::PodDisruptionBudgetStatus> = None;

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

                Ok(PodDisruptionBudget {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "PodDisruptionBudget",
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

impl ::serde::Serialize for PodDisruptionBudget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "PodDisruptionBudget",
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
