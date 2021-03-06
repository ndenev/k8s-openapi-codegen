// Generated from definition io.k8s.kubernetes.pkg.apis.rbac.v1alpha1.ClusterRoleBinding

/// ClusterRoleBinding references a ClusterRole, but not contain it.  It can reference a ClusterRole in the global namespace, and adds who information via Subject.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ClusterRoleBinding {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object's metadata.
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// RoleRef can only reference a ClusterRole in the global namespace. If the RoleRef cannot be resolved, the Authorizer must return an error.
    pub role_ref: ::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::RoleRef,

    /// Subjects holds references to the objects the role applies to.
    pub subjects: Vec<::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Subject>,
}

// Begin rbac.authorization.k8s.io/v1alpha1/ClusterRoleBinding

// Generated from operation createRbacAuthorizationV1alpha1ClusterRoleBinding

#[derive(Debug)]
pub enum CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// create a ClusterRoleBinding
    pub fn create_rbac_authorization_v1alpha1_cluster_role_binding<C>(
        __client: &C,
        body: &::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBinding,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings")).map_err(::Error::URL)?;
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
                CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => CreateRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1ClusterRoleBinding

#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// delete a ClusterRoleBinding
    pub fn delete_rbac_authorization_v1alpha1_cluster_role_binding<C>(
        __client: &C,
        // name of the ClusterRoleBinding
        name: &str,
        // The duration in seconds before the object should be deleted. Value must be non-negative integer. The value zero indicates delete immediately. If this value is nil, the default grace period for the specified type will be used. Defaults to a per object value if not specified. zero means delete immediately.
        grace_period_seconds: Option<i64>,
        // Deprecated: please use the PropagationPolicy, this field will be deprecated in 1.7. Should the dependent objects be orphaned. If true/false, the "orphan" finalizer will be added to/removed from the object's finalizers list. Either this field or PropagationPolicy may be set, but not both.
        orphan_dependents: Option<bool>,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
        // Whether and how garbage collection will be performed. Either this field or OrphanDependents may be set, but not both. The default policy is decided by the existing finalizer set in the metadata.finalizers and the resource-specific default policy.
        propagation_policy: Option<&str>,
    ) -> Result<DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}", name = name)).map_err(::Error::URL)?;
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
                DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => DeleteRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation deleteRbacAuthorizationV1alpha1CollectionClusterRoleBinding

#[derive(Debug)]
pub enum DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::apimachinery::pkg::apis::meta::v1::Status),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// delete collection of ClusterRoleBinding
    pub fn delete_rbac_authorization_v1alpha1_collection_cluster_role_binding<C>(
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
    ) -> Result<DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings")).map_err(::Error::URL)?;
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
                DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::Unauthorized(response),
            other => DeleteRbacAuthorizationV1alpha1CollectionClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation listRbacAuthorizationV1alpha1ClusterRoleBinding

#[derive(Debug)]
pub enum ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBindingList),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// list or watch objects of kind ClusterRoleBinding
    pub fn list_rbac_authorization_v1alpha1_cluster_role_binding<C>(
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
    ) -> Result<ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings")).map_err(::Error::URL)?;
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
                ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => ListRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation patchRbacAuthorizationV1alpha1ClusterRoleBinding

#[derive(Debug)]
pub enum PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// partially update the specified ClusterRoleBinding
    pub fn patch_rbac_authorization_v1alpha1_cluster_role_binding<C>(
        __client: &C,
        // name of the ClusterRoleBinding
        name: &str,
        body: &::v1_7::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}", name = name)).map_err(::Error::URL)?;
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
                PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => PatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation readRbacAuthorizationV1alpha1ClusterRoleBinding

#[derive(Debug)]
pub enum ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// read the specified ClusterRoleBinding
    pub fn read_rbac_authorization_v1alpha1_cluster_role_binding<C>(
        __client: &C,
        // name of the ClusterRoleBinding
        name: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}", name = name)).map_err(::Error::URL)?;
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
                ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => ReadRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceRbacAuthorizationV1alpha1ClusterRoleBinding

#[derive(Debug)]
pub enum ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBinding),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// replace the specified ClusterRoleBinding
    pub fn replace_rbac_authorization_v1alpha1_cluster_role_binding<C>(
        __client: &C,
        // name of the ClusterRoleBinding
        name: &str,
        body: &::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::ClusterRoleBinding,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/clusterrolebindings/{name}", name = name)).map_err(::Error::URL)?;
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
                ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => ReplaceRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRoleBinding

pub enum WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// watch changes to an object of kind ClusterRoleBinding
    pub fn watch_rbac_authorization_v1alpha1_cluster_role_binding<C>(
        __client: &C,
        // name of the ClusterRoleBinding
        name: &str,
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
    ) -> Result<WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings/{name}", name = name)).map_err(::Error::URL)?;
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
                WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Unauthorized(response),
            other => WatchRbacAuthorizationV1alpha1ClusterRoleBindingResponse::Other(other, response),
        })
    }
}

// Generated from operation watchRbacAuthorizationV1alpha1ClusterRoleBindingList

pub enum WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse<R> where R: ::std::io::Read {
    Ok(::serde_json::StreamDeserializer<'static, ::serde_json::de::IoRead<R>, ::v1_7::apimachinery::pkg::apis::meta::v1::WatchEvent>),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl ClusterRoleBinding {
    /// watch individual changes to a list of ClusterRoleBinding
    pub fn watch_rbac_authorization_v1alpha1_cluster_role_binding_list<C>(
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
    ) -> Result<WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/rbac.authorization.k8s.io/v1alpha1/watch/clusterrolebindings")).map_err(::Error::URL)?;
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
                WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse::Unauthorized(response),
            other => WatchRbacAuthorizationV1alpha1ClusterRoleBindingListResponse::Other(other, response),
        })
    }
}

// End rbac.authorization.k8s.io/v1alpha1/ClusterRoleBinding

impl<'de> ::serde::Deserialize<'de> for ClusterRoleBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_role_ref,
            Key_subjects,
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
                            "roleRef" => Field::Key_role_ref,
                            "subjects" => Field::Key_subjects,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = ClusterRoleBinding;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct ClusterRoleBinding")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_role_ref: Option<::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::RoleRef> = None;
                let mut value_subjects: Option<Vec<::v1_7::kubernetes::pkg::apis::rbac::v1alpha1::Subject>> = None;

                while let Some(key) = ::serde::de::MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => value_api_version = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_kind => value_kind = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_metadata => value_metadata = ::serde::de::MapAccess::next_value(&mut map)?,
                        Field::Key_role_ref => value_role_ref = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Key_subjects => value_subjects = Some(::serde::de::MapAccess::next_value(&mut map)?),
                        Field::Other => { let _: ::serde::de::IgnoredAny = ::serde::de::MapAccess::next_value(&mut map)?; },
                    }
                }

                Ok(ClusterRoleBinding {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    role_ref: value_role_ref.ok_or_else(|| ::serde::de::Error::missing_field("roleRef"))?,
                    subjects: value_subjects.ok_or_else(|| ::serde::de::Error::missing_field("subjects"))?,
                })
            }
        }

        deserializer.deserialize_struct(
            "ClusterRoleBinding",
            &[
                "apiVersion",
                "kind",
                "metadata",
                "roleRef",
                "subjects",
            ],
            Visitor,
        )
    }
}

impl ::serde::Serialize for ClusterRoleBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "ClusterRoleBinding",
            0 +
            self.api_version.as_ref().map_or(0, |_| 1) +
            self.kind.as_ref().map_or(0, |_| 1) +
            self.metadata.as_ref().map_or(0, |_| 1) +
            1 +
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
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "roleRef", &self.role_ref)?;
        ::serde::ser::SerializeStruct::serialize_field(&mut state, "subjects", &self.subjects)?;
        ::serde::ser::SerializeStruct::end(state)
    }
}
