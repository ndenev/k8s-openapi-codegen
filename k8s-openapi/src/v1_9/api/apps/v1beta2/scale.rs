// Generated from definition io.k8s.api.apps.v1beta2.Scale

/// Scale represents a scaling request for a resource.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Scale {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    pub kind: Option<String>,

    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata.
    pub metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status.
    pub spec: Option<::v1_9::api::apps::v1beta2::ScaleSpec>,

    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status. Read-only.
    pub status: Option<::v1_9::api::apps::v1beta2::ScaleStatus>,
}

// Begin apps/v1beta2/Scale

// Generated from operation patchAppsV1beta2NamespacedDeploymentScale

#[derive(Debug)]
pub enum PatchAppsV1beta2NamespacedDeploymentScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// partially update scale of the specified Deployment
    pub fn patch_apps_v1beta2_namespaced_deployment_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAppsV1beta2NamespacedDeploymentScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/deployments/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAppsV1beta2NamespacedDeploymentScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAppsV1beta2NamespacedDeploymentScaleResponse::Unauthorized(response),
            other => PatchAppsV1beta2NamespacedDeploymentScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation patchAppsV1beta2NamespacedReplicaSetScale

#[derive(Debug)]
pub enum PatchAppsV1beta2NamespacedReplicaSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// partially update scale of the specified ReplicaSet
    pub fn patch_apps_v1beta2_namespaced_replica_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAppsV1beta2NamespacedReplicaSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAppsV1beta2NamespacedReplicaSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAppsV1beta2NamespacedReplicaSetScaleResponse::Unauthorized(response),
            other => PatchAppsV1beta2NamespacedReplicaSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation patchAppsV1beta2NamespacedStatefulSetScale

#[derive(Debug)]
pub enum PatchAppsV1beta2NamespacedStatefulSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// partially update scale of the specified StatefulSet
    pub fn patch_apps_v1beta2_namespaced_stateful_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::apimachinery::pkg::apis::meta::v1::Patch,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<PatchAppsV1beta2NamespacedStatefulSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/statefulsets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                PatchAppsV1beta2NamespacedStatefulSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => PatchAppsV1beta2NamespacedStatefulSetScaleResponse::Unauthorized(response),
            other => PatchAppsV1beta2NamespacedStatefulSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation readAppsV1beta2NamespacedDeploymentScale

#[derive(Debug)]
pub enum ReadAppsV1beta2NamespacedDeploymentScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// read scale of the specified Deployment
    pub fn read_apps_v1beta2_namespaced_deployment_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAppsV1beta2NamespacedDeploymentScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/deployments/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadAppsV1beta2NamespacedDeploymentScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAppsV1beta2NamespacedDeploymentScaleResponse::Unauthorized(response),
            other => ReadAppsV1beta2NamespacedDeploymentScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation readAppsV1beta2NamespacedReplicaSetScale

#[derive(Debug)]
pub enum ReadAppsV1beta2NamespacedReplicaSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// read scale of the specified ReplicaSet
    pub fn read_apps_v1beta2_namespaced_replica_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAppsV1beta2NamespacedReplicaSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadAppsV1beta2NamespacedReplicaSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAppsV1beta2NamespacedReplicaSetScaleResponse::Unauthorized(response),
            other => ReadAppsV1beta2NamespacedReplicaSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation readAppsV1beta2NamespacedStatefulSetScale

#[derive(Debug)]
pub enum ReadAppsV1beta2NamespacedStatefulSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// read scale of the specified StatefulSet
    pub fn read_apps_v1beta2_namespaced_stateful_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReadAppsV1beta2NamespacedStatefulSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/statefulsets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReadAppsV1beta2NamespacedStatefulSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReadAppsV1beta2NamespacedStatefulSetScaleResponse::Unauthorized(response),
            other => ReadAppsV1beta2NamespacedStatefulSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceAppsV1beta2NamespacedDeploymentScale

#[derive(Debug)]
pub enum ReplaceAppsV1beta2NamespacedDeploymentScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Created(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// replace scale of the specified Deployment
    pub fn replace_apps_v1beta2_namespaced_deployment_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::apps::v1beta2::Scale,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAppsV1beta2NamespacedDeploymentScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/deployments/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Unauthorized(response),
            other => ReplaceAppsV1beta2NamespacedDeploymentScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceAppsV1beta2NamespacedReplicaSetScale

#[derive(Debug)]
pub enum ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Created(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// replace scale of the specified ReplicaSet
    pub fn replace_apps_v1beta2_namespaced_replica_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::apps::v1beta2::Scale,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/replicasets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Unauthorized(response),
            other => ReplaceAppsV1beta2NamespacedReplicaSetScaleResponse::Other(other, response),
        })
    }
}

// Generated from operation replaceAppsV1beta2NamespacedStatefulSetScale

#[derive(Debug)]
pub enum ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse<R> where R: ::std::io::Read {
    Ok(::v1_9::api::apps::v1beta2::Scale),
    Created(::v1_9::api::apps::v1beta2::Scale),
    Unauthorized(R),
    Other(::http::StatusCode, R),
}

impl Scale {
    /// replace scale of the specified StatefulSet
    pub fn replace_apps_v1beta2_namespaced_stateful_set_scale<C>(
        __client: &C,
        // name of the Scale
        name: &str,
        // object name and auth scope, such as for teams and projects
        namespace: &str,
        body: &::v1_9::api::apps::v1beta2::Scale,
        // If 'true', then the output is pretty printed.
        pretty: Option<&str>,
    ) -> Result<ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse<C::Response>, ::Error<C::Error>> where C: ::Client {
        let mut __url = __client.base_url().join(&format!("/apis/apps/v1beta2/namespaces/{namespace}/statefulsets/{name}/scale", name = name, namespace = namespace)).map_err(::Error::URL)?;
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
                ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Ok(result)
            },
            ::http::StatusCode::CREATED => {
                let result = ::serde_json::from_reader(response).map_err(::Error::JSON)?;
                ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Created(result)
            },
            ::http::StatusCode::UNAUTHORIZED => ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Unauthorized(response),
            other => ReplaceAppsV1beta2NamespacedStatefulSetScaleResponse::Other(other, response),
        })
    }
}

// End apps/v1beta2/Scale

impl<'de> ::serde::Deserialize<'de> for Scale {
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
            type Value = Scale;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "struct Scale")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: ::serde::de::MapAccess<'de> {
                let mut value_api_version: Option<String> = None;
                let mut value_kind: Option<String> = None;
                let mut value_metadata: Option<::v1_9::apimachinery::pkg::apis::meta::v1::ObjectMeta> = None;
                let mut value_spec: Option<::v1_9::api::apps::v1beta2::ScaleSpec> = None;
                let mut value_status: Option<::v1_9::api::apps::v1beta2::ScaleStatus> = None;

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

                Ok(Scale {
                    api_version: value_api_version,
                    kind: value_kind,
                    metadata: value_metadata,
                    spec: value_spec,
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            "Scale",
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

impl ::serde::Serialize for Scale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
        let mut state = serializer.serialize_struct(
            "Scale",
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
