// Generated from definition io.k8s.api.storage.v1beta1.StorageClass

/// StorageClass describes the parameters for a class of storage for which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class according to etcd is in ObjectMeta.Name.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct StorageClass {
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    pub allow_volume_expansion: Option<bool>,

    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    pub metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. \["ro", "soft"\]. Not validated - mount of the PVs will simply fail if one is invalid.
    pub mount_options: Option<Vec<String>>,

    /// Parameters holds the parameters for the provisioner that should create volumes of this storage class.
    pub parameters: Option<::std::collections::BTreeMap<String, String>>,

    /// Provisioner indicates the type of the provisioner.
    pub provisioner: String,

    /// Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete.
    pub reclaim_policy: Option<String>,

    /// VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is alpha-level and is only honored by servers that enable the VolumeScheduling feature.
    pub volume_binding_mode: Option<String>,
}

// Begin storage.k8s.io/v1beta1/StorageClass

// Generated from operation createStorageV1beta1StorageClass

impl StorageClass {
    /// create a StorageClass
    pub fn create_storage_v1beta1_storage_class(
        body: &::v1_10::api::storage::v1beta1::StorageClass,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses?");
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
pub enum CreateStorageV1beta1StorageClassResponse {
    Ok(::v1_10::api::storage::v1beta1::StorageClass),
    Created(::v1_10::api::storage::v1beta1::StorageClass),
    Accepted(::v1_10::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for CreateStorageV1beta1StorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                CreateStorageV1beta1StorageClassResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                CreateStorageV1beta1StorageClassResponse::Created(result)
            },
            ::http::StatusCode::ACCEPTED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                CreateStorageV1beta1StorageClassResponse::Accepted(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateStorageV1beta1StorageClassResponse::Unauthorized,
            _ => CreateStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation deleteStorageV1beta1CollectionStorageClass

impl StorageClass {
    /// delete collection of StorageClass
    pub fn delete_storage_v1beta1_collection_storage_class(
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
pub enum DeleteStorageV1beta1CollectionStorageClassResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteStorageV1beta1CollectionStorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteStorageV1beta1CollectionStorageClassResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteStorageV1beta1CollectionStorageClassResponse::Unauthorized,
            _ => DeleteStorageV1beta1CollectionStorageClassResponse::Other,
        })
    }
}

// Generated from operation deleteStorageV1beta1StorageClass

impl StorageClass {
    /// delete a StorageClass
    pub fn delete_storage_v1beta1_storage_class(
        // name of the StorageClass
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy. Acceptable values are: 'Orphan' - orphan the dependents; 'Background' - allow the garbage collector to delete the dependents in the background; 'Foreground' - a cascading policy that deletes all dependents in the foreground.
        propagation_policy: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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
pub enum DeleteStorageV1beta1StorageClassResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteStorageV1beta1StorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteStorageV1beta1StorageClassResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteStorageV1beta1StorageClassResponse::Unauthorized,
            _ => DeleteStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation listStorageV1beta1StorageClass

impl StorageClass {
    /// list or watch objects of kind StorageClass
    pub fn list_storage_v1beta1_storage_class(
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
pub enum ListStorageV1beta1StorageClassResponse {
    Ok(::v1_10::api::storage::v1beta1::StorageClassList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListStorageV1beta1StorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListStorageV1beta1StorageClassResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListStorageV1beta1StorageClassResponse::Unauthorized,
            _ => ListStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation patchStorageV1beta1StorageClass

impl StorageClass {
    /// partially update the specified StorageClass
    pub fn patch_storage_v1beta1_storage_class(
        // name of the StorageClass
        name: &str,
        body: &::v1_10::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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
pub enum PatchStorageV1beta1StorageClassResponse {
    Ok(::v1_10::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for PatchStorageV1beta1StorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                PatchStorageV1beta1StorageClassResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchStorageV1beta1StorageClassResponse::Unauthorized,
            _ => PatchStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation readStorageV1beta1StorageClass

impl StorageClass {
    /// read the specified StorageClass
    pub fn read_storage_v1beta1_storage_class(
        // name of the StorageClass
        name: &str,
        // Should the export be exact.  Exact export maintains cluster-specific fields like 'Namespace'.
        exact: Option<bool>,
        // Should this value be exported.  Export strips fields that a user can not specify.
        export: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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
pub enum ReadStorageV1beta1StorageClassResponse {
    Ok(::v1_10::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadStorageV1beta1StorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReadStorageV1beta1StorageClassResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadStorageV1beta1StorageClassResponse::Unauthorized,
            _ => ReadStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation replaceStorageV1beta1StorageClass

impl StorageClass {
    /// replace the specified StorageClass
    pub fn replace_storage_v1beta1_storage_class(
        // name of the StorageClass
        name: &str,
        body: &::v1_10::api::storage::v1beta1::StorageClass,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/storageclasses/{name}?", name = name);
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
pub enum ReplaceStorageV1beta1StorageClassResponse {
    Ok(::v1_10::api::storage::v1beta1::StorageClass),
    Created(::v1_10::api::storage::v1beta1::StorageClass),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReplaceStorageV1beta1StorageClassResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceStorageV1beta1StorageClassResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceStorageV1beta1StorageClassResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceStorageV1beta1StorageClassResponse::Unauthorized,
            _ => ReplaceStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation watchStorageV1beta1StorageClass

impl StorageClass {
    /// watch changes to an object of kind StorageClass
    pub fn watch_storage_v1beta1_storage_class(
        // name of the StorageClass
        name: &str,
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/watch/storageclasses/{name}?", name = name);
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
pub enum WatchStorageV1beta1StorageClassResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchStorageV1beta1StorageClassResponse {
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
                WatchStorageV1beta1StorageClassResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchStorageV1beta1StorageClassResponse::Unauthorized,
            _ => WatchStorageV1beta1StorageClassResponse::Other,
        })
    }
}

// Generated from operation watchStorageV1beta1StorageClassList

impl StorageClass {
    /// watch individual changes to a list of StorageClass
    pub fn watch_storage_v1beta1_storage_class_list(
        // The continue option should be set when retrieving more results from the server. Since this value is server defined, clients may only use the continue value from a previous query result with identical query parameters (except for the value of continue) and the server may reject a continue value it does not recognize. If the specified continue value is no longer valid whether due to expiration (generally five to fifteen minutes) or a configuration change on the server the server will respond with a 410 ResourceExpired error indicating the client must restart their list without the continue field. This field is not supported when watch is true. Clients may start a watch from the last resourceVersion value returned by the server and not miss any modifications.
        continue_: Option<&str>,
        // A selector to restrict the list of returned objects by their fields. Defaults to everything.
        field_selector: Option<&str>,
        // If true, partially initialized resources are included in the response.
        include_uninitialized: Option<bool>,
        // A selector to restrict the list of returned objects by their labels. Defaults to everything.
        label_selector: Option<&str>,
        // limit is a maximum number of responses to return for a list call. If more items exist, the server will set the `continue` field on the list metadata to a value that can be used with the same initial query to retrieve the next set of results. Setting a limit may return fewer than the requested amount of items (up to zero items) in the event all requested objects are filtered out and clients should only use the presence of the continue field to determine whether more results are available. Servers may choose not to support the limit argument and will return all of the available results. If limit is specified and the continue field is empty, clients may assume that no more results are available. This field is not supported if watch is true.
        //
        // The server guarantees that the objects returned when using continue will be identical to issuing a single list call without a limit - that is, no objects created, modified, or deleted after the first request is issued will be included in any subsequent continued requests. This is sometimes referred to as a consistent snapshot, and ensures that a client that is using limit to receive smaller chunks of a very large result can ensure they see all possible objects. If objects are updated during a chunked list the version of the object that was present at the time the first list result was calculated is returned.
        limit: Option<i64>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // When specified with a watch call, shows changes that occur after that particular version of a resource. Defaults to changes from the beginning of history. When specified for list: - if unset, then the result is returned from remote storage based on quorum-read flag; - if it's 0, then we simply return what we currently have in cache, no guarantee; - if set to non zero, then the result is at least as fresh as given rv.
        resource_version: Option<&str>,
        // Timeout for the list/watch call. This limits the duration of the call, regardless of any activity or inactivity.
        timeout_seconds: Option<i64>,
        // Watch for changes to the described resources and return them as a stream of add, update, and remove notifications. Specify resourceVersion.
        watch: Option<bool>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/storage.k8s.io/v1beta1/watch/storageclasses?");
        let mut __query_pairs = ::url::form_urlencoded::Serializer::new(__url);
        if let Some(continue_) = continue_ {
            __query_pairs.append_pair("continue", &continue_);
        }
        if let Some(field_selector) = field_selector {
            __query_pairs.append_pair("fieldSelector", &field_selector);
        }
        if let Some(include_uninitialized) = include_uninitialized {
            __query_pairs.append_pair("includeUninitialized", &include_uninitialized.to_string());
        }
        if let Some(label_selector) = label_selector {
            __query_pairs.append_pair("labelSelector", &label_selector);
        }
        if let Some(limit) = limit {
            __query_pairs.append_pair("limit", &limit.to_string());
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
pub enum WatchStorageV1beta1StorageClassListResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchStorageV1beta1StorageClassListResponse {
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
                WatchStorageV1beta1StorageClassListResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchStorageV1beta1StorageClassListResponse::Unauthorized,
            _ => WatchStorageV1beta1StorageClassListResponse::Other,
        })
    }
}

// End storage.k8s.io/v1beta1/StorageClass

impl<'de> ::serde::Deserialize<'de> for StorageClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_allow_volume_expansion,
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_mount_options,
            Key_parameters,
            Key_provisioner,
            Key_reclaim_policy,
            Key_volume_binding_mode,
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
                            "allowVolumeExpansion" => Field::Key_allow_volume_expansion,
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "mountOptions" => Field::Key_mount_options,
                            "parameters" => Field::Key_parameters,
                            "provisioner" => Field::Key_provisioner,
                            "reclaimPolicy" => Field::Key_reclaim_policy,
                            "volumeBindingMode" => Field::Key_volume_binding_mode,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StorageClass;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct StorageClass")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_allow_volume_expansion: Option<bool> = None;
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_10::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_mount_options: Option<Vec<String>> = None;
                let mut value_parameters: Option<::std::collections::BTreeMap<String, String>> = None;
                let mut value_provisioner: Option<String> = None;
                let mut value_reclaim_policy: Option<String> = None;
                let mut value_volume_binding_mode: Option<String> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_allow_volume_expansion => value_allow_volume_expansion = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_mount_options => value_mount_options = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_parameters => value_parameters = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_provisioner => value_provisioner = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_reclaim_policy => value_reclaim_policy = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_volume_binding_mode => value_volume_binding_mode = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(StorageClass {
                    allow_volume_expansion: value_allow_volume_expansion,
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    mount_options: value_mount_options,
                    parameters: value_parameters,
                    provisioner: value_provisioner.ok_or_else(|| ::serde::de::Error::missing_field("provisioner"))?,
                    reclaim_policy: value_reclaim_policy,
                    volume_binding_mode: value_volume_binding_mode,
                })
            }
        }

        deserializer.deserialize_struct(
            "StorageClass",
            &[
                "allowVolumeExpansion",
                "apiVersion",
                "kind",
                "metadata",
                "mountOptions",
                "parameters",
                "provisioner",
                "reclaimPolicy",
                "volumeBindingMode",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for StorageClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "StorageClass",
            0 +
            self.allow_volume_expansion.as_ref().map_or(0, |_| 1) +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            self.mount_options.as_ref().map_or(0, |_| 1) +
            self.parameters.as_ref().map_or(0, |_| 1) +
            1 +
            self.reclaim_policy.as_ref().map_or(0, |_| 1) +
            self.volume_binding_mode.as_ref().map_or(0, |_| 1),
        )?;
        if let Some(value) = &self.allow_volume_expansion {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "allowVolumeExpansion", value)?;
        }
        if let Some(value) = &self.api_version {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "apiVersion", value)?;
        }
        if let Some(value) = &self.kind {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "kind", value)?;
        }
        if let Some(value) = &self.metadata {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "metadata", value)?;
        }
        if let Some(value) = &self.mount_options {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "mountOptions", value)?;
        }
        if let Some(value) = &self.parameters {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "parameters", value)?;
        }
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "provisioner", &self.provisioner)?;
        if let Some(value) = &self.reclaim_policy {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "reclaimPolicy", value)?;
        }
        if let Some(value) = &self.volume_binding_mode {
            ::serde::ser::SerializeStruct::serialize_field(&mut state, "volumeBindingMode", value)?;
        }
        ::serde::ser::SerializeStruct::end(state)
    }
}
