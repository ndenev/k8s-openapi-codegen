// Generated from definition io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.Role

/// Role is a namespaced, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Role {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata.
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Rules holds all the PolicyRules for this Role
    pub rules: Vec<::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::PolicyRule>,
}

// Begin rbac.authorization.k8s.io/v1alpha1/Role

// Generated from operation createRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// create a Role
    pub fn create_rbac_authorization_v1alpha1_namespaced_role(
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Role,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles?", namespace = namespace);
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
pub enum CreateRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Role),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for CreateRbacAuthorizationV1alpha1NamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                CreateRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => CreateRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1CollectionNamespacedRole

impl Role {
    /// delete collection of Role
    pub fn delete_rbac_authorization_v1alpha1_collection_namespaced_role(
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles?", namespace = namespace);
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
pub enum DeleteRbacAuthorizationV1alpha1CollectionNamespacedRoleResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteRbacAuthorizationV1alpha1CollectionNamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteRbacAuthorizationV1alpha1CollectionNamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteRbacAuthorizationV1alpha1CollectionNamespacedRoleResponse::Unauthorized,
            _ => DeleteRbacAuthorizationV1alpha1CollectionNamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// delete a Role
    pub fn delete_rbac_authorization_v1alpha1_namespaced_role(
        // name of the Role
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name}?", name = name, namespace = namespace);
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
pub enum DeleteRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for DeleteRbacAuthorizationV1alpha1NamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                DeleteRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => DeleteRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation listRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// list or watch objects of kind Role
    pub fn list_rbac_authorization_v1alpha1_namespaced_role(
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles?", namespace = namespace);
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
pub enum ListRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::RoleList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListRbacAuthorizationV1alpha1NamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => ListRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation listRbacAuthorizationV1alpha1RoleForAllNamespaces

impl Role {
    /// list or watch objects of kind Role
    pub fn list_rbac_authorization_v1alpha1_role_for_all_namespaces(
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/roles?");
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
pub enum ListRbacAuthorizationV1alpha1RoleForAllNamespacesResponse {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::RoleList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ListRbacAuthorizationV1alpha1RoleForAllNamespacesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ListRbacAuthorizationV1alpha1RoleForAllNamespacesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListRbacAuthorizationV1alpha1RoleForAllNamespacesResponse::Unauthorized,
            _ => ListRbacAuthorizationV1alpha1RoleForAllNamespacesResponse::Other,
        })
    }
}

// Generated from operation patchRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// partially update the specified Role
    pub fn patch_rbac_authorization_v1alpha1_namespaced_role(
        // name of the Role
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name}?", name = name, namespace = namespace);
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
pub enum PatchRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Role),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for PatchRbacAuthorizationV1alpha1NamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                PatchRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => PatchRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation readRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// read the specified Role
    pub fn read_rbac_authorization_v1alpha1_namespaced_role(
        // name of the Role
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name}?", name = name, namespace = namespace);
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
pub enum ReadRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Role),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReadRbacAuthorizationV1alpha1NamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReadRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => ReadRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation replaceRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// replace the specified Role
    pub fn replace_rbac_authorization_v1alpha1_namespaced_role(
        // name of the Role
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Role,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/namespaces/{namespace}/roles/{name}?", name = name, namespace = namespace);
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
pub enum ReplaceRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Role),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for ReplaceRbacAuthorizationV1alpha1NamespacedRoleResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                ReplaceRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => ReplaceRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1NamespacedRole

impl Role {
    /// watch changes to an object of kind Role
    pub fn watch_rbac_authorization_v1alpha1_namespaced_role(
        // name of the Role
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/roles/{name}?", name = name, namespace = namespace);
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
pub enum WatchRbacAuthorizationV1alpha1NamespacedRoleResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchRbacAuthorizationV1alpha1NamespacedRoleResponse {
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
                WatchRbacAuthorizationV1alpha1NamespacedRoleResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchRbacAuthorizationV1alpha1NamespacedRoleResponse::Unauthorized,
            _ => WatchRbacAuthorizationV1alpha1NamespacedRoleResponse::Other,
        })
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1NamespacedRoleList

impl Role {
    /// watch individual changes to a list of Role
    pub fn watch_rbac_authorization_v1alpha1_namespaced_role_list(
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/namespaces/{namespace}/roles?", namespace = namespace);
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
pub enum WatchRbacAuthorizationV1alpha1NamespacedRoleListResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchRbacAuthorizationV1alpha1NamespacedRoleListResponse {
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
                WatchRbacAuthorizationV1alpha1NamespacedRoleListResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchRbacAuthorizationV1alpha1NamespacedRoleListResponse::Unauthorized,
            _ => WatchRbacAuthorizationV1alpha1NamespacedRoleListResponse::Other,
        })
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1RoleListForAllNamespaces

impl Role {
    /// watch individual changes to a list of Role
    pub fn watch_rbac_authorization_v1alpha1_role_list_for_all_namespaces(
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
        let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/roles?");
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
pub enum WatchRbacAuthorizationV1alpha1RoleListForAllNamespacesResponse {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent, usize),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for WatchRbacAuthorizationV1alpha1RoleListForAllNamespacesResponse {
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
                WatchRbacAuthorizationV1alpha1RoleListForAllNamespacesResponse::Ok(result, byte_offset)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchRbacAuthorizationV1alpha1RoleListForAllNamespacesResponse::Unauthorized,
            _ => WatchRbacAuthorizationV1alpha1RoleListForAllNamespacesResponse::Other,
        })
    }
}

// End rbac.authorization.k8s.io/v1alpha1/Role

impl<'de> ::serde::Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_rules,
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
                            "rules" => Field::Key_rules,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = Role;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Role")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_rules: Option<Vec<::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::PolicyRule>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_rules => value_rules = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(Role {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    rules: value_rules.ok_or_else(|| ::serde::de::Error::missing_field("rules"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "Role",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "rules",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Role",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1,
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
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "rules", &self.rules)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
