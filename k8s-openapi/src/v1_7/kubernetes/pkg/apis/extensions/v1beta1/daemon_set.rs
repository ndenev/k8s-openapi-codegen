// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.DaemonSet

/// DaemonSet represents the configuration of a daemon set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DaemonSet {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// The desired behavior of this daemon set. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub spec: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSetSpec>,

    /// The current status of this daemon set. This data may be out of date by some window of time. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    pub status: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSetStatus>,
}

// Begin extensions/v1beta1/DaemonSet

// Generated from operation createExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// create a DaemonSet
    pub fn create_extensions_v1beta1_namespaced_daemon_set(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
pub enum CreateExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for CreateExtensionsV1beta1NamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                CreateExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => CreateExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation deleteExtensionsV1beta1CollectionNamespacedDaemonSet

impl DaemonSet {
    /// delete collection of DaemonSet
    pub fn delete_extensions_v1beta1_collection_namespaced_daemon_set(
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
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
pub enum DeleteExtensionsV1beta1CollectionNamespacedDaemonSetResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteExtensionsV1beta1CollectionNamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteExtensionsV1beta1CollectionNamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteExtensionsV1beta1CollectionNamespacedDaemonSetResponse::Unauthorized,
            _ => DeleteExtensionsV1beta1CollectionNamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation deleteExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// delete a DaemonSet
    pub fn delete_extensions_v1beta1_namespaced_daemon_set(
        // name of the DaemonSet
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
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
pub enum DeleteExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteExtensionsV1beta1NamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => DeleteExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation listExtensionsV1beta1DaemonSetForAllNamespaces

impl DaemonSet {
    /// list or watch objects of kind DaemonSet
    pub fn list_extensions_v1beta1_daemon_set_for_all_namespaces(
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
        let __url = format!("/apis/extensions/v1beta1/daemonsets?");
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
pub enum ListExtensionsV1beta1DaemonSetForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSetList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListExtensionsV1beta1DaemonSetForAllNamespacesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListExtensionsV1beta1DaemonSetForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListExtensionsV1beta1DaemonSetForAllNamespacesResponse::Unauthorized,
            _ => ListExtensionsV1beta1DaemonSetForAllNamespacesResponse::Other,
        })
    }
}

// Generated from operation listExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// list or watch objects of kind DaemonSet
    pub fn list_extensions_v1beta1_namespaced_daemon_set(
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
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
pub enum ListExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSetList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListExtensionsV1beta1NamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => ListExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// partially update the specified DaemonSet
    pub fn patch_extensions_v1beta1_namespaced_daemon_set(
        // name of the DaemonSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
pub enum PatchExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for PatchExtensionsV1beta1NamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                PatchExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => PatchExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation patchExtensionsV1beta1NamespacedDaemonSetStatus

impl DaemonSet {
    /// partially update status of the specified DaemonSet
    pub fn patch_extensions_v1beta1_namespaced_daemon_set_status(
        // name of the DaemonSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}/status?", name = name, namespace = namespace);
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
pub enum PatchExtensionsV1beta1NamespacedDaemonSetStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for PatchExtensionsV1beta1NamespacedDaemonSetStatusResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                PatchExtensionsV1beta1NamespacedDaemonSetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchExtensionsV1beta1NamespacedDaemonSetStatusResponse::Unauthorized,
            _ => PatchExtensionsV1beta1NamespacedDaemonSetStatusResponse::Other,
        })
    }
}

// Generated from operation readExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// read the specified DaemonSet
    pub fn read_extensions_v1beta1_namespaced_daemon_set(
        // name of the DaemonSet
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
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
pub enum ReadExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadExtensionsV1beta1NamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReadExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => ReadExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation readExtensionsV1beta1NamespacedDaemonSetStatus

impl DaemonSet {
    /// read status of the specified DaemonSet
    pub fn read_extensions_v1beta1_namespaced_daemon_set_status(
        // name of the DaemonSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}/status?", name = name, namespace = namespace);
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
pub enum ReadExtensionsV1beta1NamespacedDaemonSetStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadExtensionsV1beta1NamespacedDaemonSetStatusResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReadExtensionsV1beta1NamespacedDaemonSetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadExtensionsV1beta1NamespacedDaemonSetStatusResponse::Unauthorized,
            _ => ReadExtensionsV1beta1NamespacedDaemonSetStatusResponse::Other,
        })
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// replace the specified DaemonSet
    pub fn replace_extensions_v1beta1_namespaced_daemon_set(
        // name of the DaemonSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
pub enum ReplaceExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReplaceExtensionsV1beta1NamespacedDaemonSetResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => ReplaceExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation replaceExtensionsV1beta1NamespacedDaemonSetStatus

impl DaemonSet {
    /// replace status of the specified DaemonSet
    pub fn replace_extensions_v1beta1_namespaced_daemon_set_status(
        // name of the DaemonSet
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/extensions/v1beta1/namespaces/{namespace}/daemonsets/{name}/status?", name = name, namespace = namespace);
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
pub enum ReplaceExtensionsV1beta1NamespacedDaemonSetStatusResponse {
    Ok(::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSet),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReplaceExtensionsV1beta1NamespacedDaemonSetStatusResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceExtensionsV1beta1NamespacedDaemonSetStatusResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceExtensionsV1beta1NamespacedDaemonSetStatusResponse::Unauthorized,
            _ => ReplaceExtensionsV1beta1NamespacedDaemonSetStatusResponse::Other,
        })
    }
}

// Generated from operation watchExtensionsV1beta1DaemonSetListForAllNamespaces

impl DaemonSet {
    /// watch individual changes to a list of DaemonSet
    pub fn watch_extensions_v1beta1_daemon_set_list_for_all_namespaces(
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
        let __url = format!("/apis/extensions/v1beta1/watch/daemonsets?");
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
pub enum WatchExtensionsV1beta1DaemonSetListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchExtensionsV1beta1DaemonSetListForAllNamespacesResponse {
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
                WatchExtensionsV1beta1DaemonSetListForAllNamespacesResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchExtensionsV1beta1DaemonSetListForAllNamespacesResponse::Unauthorized,
            _ => WatchExtensionsV1beta1DaemonSetListForAllNamespacesResponse::Other,
        })
    }
}

// Generated from operation watchExtensionsV1beta1NamespacedDaemonSet

impl DaemonSet {
    /// watch changes to an object of kind DaemonSet
    pub fn watch_extensions_v1beta1_namespaced_daemon_set(
        // name of the DaemonSet
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
        let __url = format!("/apis/extensions/v1beta1/watch/namespaces/{namespace}/daemonsets/{name}?", name = name, namespace = namespace);
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
pub enum WatchExtensionsV1beta1NamespacedDaemonSetResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchExtensionsV1beta1NamespacedDaemonSetResponse {
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
                WatchExtensionsV1beta1NamespacedDaemonSetResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchExtensionsV1beta1NamespacedDaemonSetResponse::Unauthorized,
            _ => WatchExtensionsV1beta1NamespacedDaemonSetResponse::Other,
        })
    }
}

// Generated from operation watchExtensionsV1beta1NamespacedDaemonSetList

impl DaemonSet {
    /// watch individual changes to a list of DaemonSet
    pub fn watch_extensions_v1beta1_namespaced_daemon_set_list(
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
        let __url = format!("/apis/extensions/v1beta1/watch/namespaces/{namespace}/daemonsets?", namespace = namespace);
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
pub enum WatchExtensionsV1beta1NamespacedDaemonSetListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchExtensionsV1beta1NamespacedDaemonSetListResponse {
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
                WatchExtensionsV1beta1NamespacedDaemonSetListResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchExtensionsV1beta1NamespacedDaemonSetListResponse::Unauthorized,
            _ => WatchExtensionsV1beta1NamespacedDaemonSetListResponse::Other,
        })
    }
}

// End extensions/v1beta1/DaemonSet

impl<'de> ::serde::Deserialize<'de> for DaemonSet {
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
            type Value = DaemonSet;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct DaemonSet")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSetSpec> = None;
                let mut value_status: Option<::v1_7::kubernetes::pkg::apis::extensions::v1beta1::DaemonSetStatus> = None;

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

                Ok(DaemonSet {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "DaemonSet",
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

impl ::serde::Serialize for DaemonSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "DaemonSet",
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
