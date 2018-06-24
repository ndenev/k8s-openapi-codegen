pub mod api;

pub mod apiextensions_apiserver;

pub mod apimachinery;

pub mod kube_aggregator;

// Generated from operation getAPIVersions

/// get available API versions
pub fn get_api_versions(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAPIVersionsResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroupList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAPIVersionsResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAPIVersionsResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAPIVersionsResponse::Unauthorized,
            _ => GetAPIVersionsResponse::Other,
        })
    }
}

// Generated from operation getAdmissionregistrationAPIGroup

/// get information of a group
pub fn get_admissionregistration_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/admissionregistration.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAdmissionregistrationAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAdmissionregistrationAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAdmissionregistrationAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAdmissionregistrationAPIGroupResponse::Unauthorized,
            _ => GetAdmissionregistrationAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getAdmissionregistrationV1alpha1APIResources

/// get available resources
pub fn get_admissionregistration_v1alpha1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/admissionregistration.k8s.io/v1alpha1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAdmissionregistrationV1alpha1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAdmissionregistrationV1alpha1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAdmissionregistrationV1alpha1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAdmissionregistrationV1alpha1APIResourcesResponse::Unauthorized,
            _ => GetAdmissionregistrationV1alpha1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAdmissionregistrationV1beta1APIResources

/// get available resources
pub fn get_admissionregistration_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/admissionregistration.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAdmissionregistrationV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAdmissionregistrationV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAdmissionregistrationV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAdmissionregistrationV1beta1APIResourcesResponse::Unauthorized,
            _ => GetAdmissionregistrationV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getApiextensionsAPIGroup

/// get information of a group
pub fn get_apiextensions_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apiextensions.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetApiextensionsAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetApiextensionsAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetApiextensionsAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetApiextensionsAPIGroupResponse::Unauthorized,
            _ => GetApiextensionsAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getApiextensionsV1beta1APIResources

/// get available resources
pub fn get_apiextensions_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apiextensions.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetApiextensionsV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetApiextensionsV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetApiextensionsV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetApiextensionsV1beta1APIResourcesResponse::Unauthorized,
            _ => GetApiextensionsV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getApiregistrationAPIGroup

/// get information of a group
pub fn get_apiregistration_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apiregistration.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetApiregistrationAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetApiregistrationAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetApiregistrationAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetApiregistrationAPIGroupResponse::Unauthorized,
            _ => GetApiregistrationAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getApiregistrationV1APIResources

/// get available resources
pub fn get_apiregistration_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apiregistration.k8s.io/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetApiregistrationV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetApiregistrationV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetApiregistrationV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetApiregistrationV1APIResourcesResponse::Unauthorized,
            _ => GetApiregistrationV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getApiregistrationV1beta1APIResources

/// get available resources
pub fn get_apiregistration_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apiregistration.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetApiregistrationV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetApiregistrationV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetApiregistrationV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetApiregistrationV1beta1APIResourcesResponse::Unauthorized,
            _ => GetApiregistrationV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAppsAPIGroup

/// get information of a group
pub fn get_apps_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apps/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAppsAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAppsAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAppsAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAppsAPIGroupResponse::Unauthorized,
            _ => GetAppsAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getAppsV1APIResources

/// get available resources
pub fn get_apps_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apps/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAppsV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAppsV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAppsV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAppsV1APIResourcesResponse::Unauthorized,
            _ => GetAppsV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAppsV1beta1APIResources

/// get available resources
pub fn get_apps_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apps/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAppsV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAppsV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAppsV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAppsV1beta1APIResourcesResponse::Unauthorized,
            _ => GetAppsV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAppsV1beta2APIResources

/// get available resources
pub fn get_apps_v1beta2_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/apps/v1beta2/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAppsV1beta2APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAppsV1beta2APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAppsV1beta2APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAppsV1beta2APIResourcesResponse::Unauthorized,
            _ => GetAppsV1beta2APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAuthenticationAPIGroup

/// get information of a group
pub fn get_authentication_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/authentication.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAuthenticationAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAuthenticationAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAuthenticationAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAuthenticationAPIGroupResponse::Unauthorized,
            _ => GetAuthenticationAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getAuthenticationV1APIResources

/// get available resources
pub fn get_authentication_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/authentication.k8s.io/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAuthenticationV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAuthenticationV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAuthenticationV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAuthenticationV1APIResourcesResponse::Unauthorized,
            _ => GetAuthenticationV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAuthenticationV1beta1APIResources

/// get available resources
pub fn get_authentication_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/authentication.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAuthenticationV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAuthenticationV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAuthenticationV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAuthenticationV1beta1APIResourcesResponse::Unauthorized,
            _ => GetAuthenticationV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAuthorizationAPIGroup

/// get information of a group
pub fn get_authorization_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/authorization.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAuthorizationAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAuthorizationAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAuthorizationAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAuthorizationAPIGroupResponse::Unauthorized,
            _ => GetAuthorizationAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getAuthorizationV1APIResources

/// get available resources
pub fn get_authorization_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/authorization.k8s.io/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAuthorizationV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAuthorizationV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAuthorizationV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAuthorizationV1APIResourcesResponse::Unauthorized,
            _ => GetAuthorizationV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAuthorizationV1beta1APIResources

/// get available resources
pub fn get_authorization_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/authorization.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAuthorizationV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAuthorizationV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAuthorizationV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAuthorizationV1beta1APIResourcesResponse::Unauthorized,
            _ => GetAuthorizationV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAutoscalingAPIGroup

/// get information of a group
pub fn get_autoscaling_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/autoscaling/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAutoscalingAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAutoscalingAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAutoscalingAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAutoscalingAPIGroupResponse::Unauthorized,
            _ => GetAutoscalingAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getAutoscalingV1APIResources

/// get available resources
pub fn get_autoscaling_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/autoscaling/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAutoscalingV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAutoscalingV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAutoscalingV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAutoscalingV1APIResourcesResponse::Unauthorized,
            _ => GetAutoscalingV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getAutoscalingV2beta1APIResources

/// get available resources
pub fn get_autoscaling_v2beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/autoscaling/v2beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetAutoscalingV2beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetAutoscalingV2beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetAutoscalingV2beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetAutoscalingV2beta1APIResourcesResponse::Unauthorized,
            _ => GetAutoscalingV2beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getBatchAPIGroup

/// get information of a group
pub fn get_batch_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/batch/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetBatchAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetBatchAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetBatchAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetBatchAPIGroupResponse::Unauthorized,
            _ => GetBatchAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getBatchV1APIResources

/// get available resources
pub fn get_batch_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/batch/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetBatchV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetBatchV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetBatchV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetBatchV1APIResourcesResponse::Unauthorized,
            _ => GetBatchV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getBatchV1beta1APIResources

/// get available resources
pub fn get_batch_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/batch/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetBatchV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetBatchV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetBatchV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetBatchV1beta1APIResourcesResponse::Unauthorized,
            _ => GetBatchV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getBatchV2alpha1APIResources

/// get available resources
pub fn get_batch_v2alpha1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/batch/v2alpha1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetBatchV2alpha1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetBatchV2alpha1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetBatchV2alpha1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetBatchV2alpha1APIResourcesResponse::Unauthorized,
            _ => GetBatchV2alpha1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getCertificatesAPIGroup

/// get information of a group
pub fn get_certificates_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/certificates.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetCertificatesAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetCertificatesAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetCertificatesAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetCertificatesAPIGroupResponse::Unauthorized,
            _ => GetCertificatesAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getCertificatesV1beta1APIResources

/// get available resources
pub fn get_certificates_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/certificates.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetCertificatesV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetCertificatesV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetCertificatesV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetCertificatesV1beta1APIResourcesResponse::Unauthorized,
            _ => GetCertificatesV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getCodeVersion

/// get the code version
pub fn get_code_version(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/version/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetCodeVersionResponse {
    Ok(::v1_10::apimachinery::pkg::version::Info),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetCodeVersionResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetCodeVersionResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetCodeVersionResponse::Unauthorized,
            _ => GetCodeVersionResponse::Other,
        })
    }
}

// Generated from operation getCoreAPIVersions

/// get available API versions
pub fn get_core_api_versions(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/api/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetCoreAPIVersionsResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIVersions),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetCoreAPIVersionsResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetCoreAPIVersionsResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetCoreAPIVersionsResponse::Unauthorized,
            _ => GetCoreAPIVersionsResponse::Other,
        })
    }
}

// Generated from operation getCoreV1APIResources

/// get available resources
pub fn get_core_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/api/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetCoreV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetCoreV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetCoreV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetCoreV1APIResourcesResponse::Unauthorized,
            _ => GetCoreV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getEventsAPIGroup

/// get information of a group
pub fn get_events_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/events.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetEventsAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetEventsAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetEventsAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetEventsAPIGroupResponse::Unauthorized,
            _ => GetEventsAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getEventsV1beta1APIResources

/// get available resources
pub fn get_events_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/events.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetEventsV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetEventsV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetEventsV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetEventsV1beta1APIResourcesResponse::Unauthorized,
            _ => GetEventsV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getExtensionsAPIGroup

/// get information of a group
pub fn get_extensions_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/extensions/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetExtensionsAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetExtensionsAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetExtensionsAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetExtensionsAPIGroupResponse::Unauthorized,
            _ => GetExtensionsAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getExtensionsV1beta1APIResources

/// get available resources
pub fn get_extensions_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/extensions/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetExtensionsV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetExtensionsV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetExtensionsV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetExtensionsV1beta1APIResourcesResponse::Unauthorized,
            _ => GetExtensionsV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getNetworkingAPIGroup

/// get information of a group
pub fn get_networking_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/networking.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetNetworkingAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetNetworkingAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetNetworkingAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetNetworkingAPIGroupResponse::Unauthorized,
            _ => GetNetworkingAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getNetworkingV1APIResources

/// get available resources
pub fn get_networking_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/networking.k8s.io/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetNetworkingV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetNetworkingV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetNetworkingV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetNetworkingV1APIResourcesResponse::Unauthorized,
            _ => GetNetworkingV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getPolicyAPIGroup

/// get information of a group
pub fn get_policy_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/policy/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetPolicyAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetPolicyAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetPolicyAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetPolicyAPIGroupResponse::Unauthorized,
            _ => GetPolicyAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getPolicyV1beta1APIResources

/// get available resources
pub fn get_policy_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/policy/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetPolicyV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetPolicyV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetPolicyV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetPolicyV1beta1APIResourcesResponse::Unauthorized,
            _ => GetPolicyV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getRbacAuthorizationAPIGroup

/// get information of a group
pub fn get_rbac_authorization_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetRbacAuthorizationAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetRbacAuthorizationAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetRbacAuthorizationAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationAPIGroupResponse::Unauthorized,
            _ => GetRbacAuthorizationAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getRbacAuthorizationV1APIResources

/// get available resources
pub fn get_rbac_authorization_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetRbacAuthorizationV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetRbacAuthorizationV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetRbacAuthorizationV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationV1APIResourcesResponse::Unauthorized,
            _ => GetRbacAuthorizationV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getRbacAuthorizationV1alpha1APIResources

/// get available resources
pub fn get_rbac_authorization_v1alpha1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/v1alpha1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetRbacAuthorizationV1alpha1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetRbacAuthorizationV1alpha1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetRbacAuthorizationV1alpha1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationV1alpha1APIResourcesResponse::Unauthorized,
            _ => GetRbacAuthorizationV1alpha1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getRbacAuthorizationV1beta1APIResources

/// get available resources
pub fn get_rbac_authorization_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/rbac.authorization.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetRbacAuthorizationV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetRbacAuthorizationV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetRbacAuthorizationV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetRbacAuthorizationV1beta1APIResourcesResponse::Unauthorized,
            _ => GetRbacAuthorizationV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getSchedulingAPIGroup

/// get information of a group
pub fn get_scheduling_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/scheduling.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetSchedulingAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetSchedulingAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetSchedulingAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetSchedulingAPIGroupResponse::Unauthorized,
            _ => GetSchedulingAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getSchedulingV1alpha1APIResources

/// get available resources
pub fn get_scheduling_v1alpha1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/scheduling.k8s.io/v1alpha1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetSchedulingV1alpha1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetSchedulingV1alpha1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetSchedulingV1alpha1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetSchedulingV1alpha1APIResourcesResponse::Unauthorized,
            _ => GetSchedulingV1alpha1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getSettingsAPIGroup

/// get information of a group
pub fn get_settings_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/settings.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetSettingsAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetSettingsAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetSettingsAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetSettingsAPIGroupResponse::Unauthorized,
            _ => GetSettingsAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getSettingsV1alpha1APIResources

/// get available resources
pub fn get_settings_v1alpha1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/settings.k8s.io/v1alpha1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetSettingsV1alpha1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetSettingsV1alpha1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetSettingsV1alpha1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetSettingsV1alpha1APIResourcesResponse::Unauthorized,
            _ => GetSettingsV1alpha1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getStorageAPIGroup

/// get information of a group
pub fn get_storage_api_group(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/storage.k8s.io/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetStorageAPIGroupResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIGroup),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetStorageAPIGroupResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetStorageAPIGroupResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetStorageAPIGroupResponse::Unauthorized,
            _ => GetStorageAPIGroupResponse::Other,
        })
    }
}

// Generated from operation getStorageV1APIResources

/// get available resources
pub fn get_storage_v1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/storage.k8s.io/v1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetStorageV1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetStorageV1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetStorageV1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetStorageV1APIResourcesResponse::Unauthorized,
            _ => GetStorageV1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getStorageV1alpha1APIResources

/// get available resources
pub fn get_storage_v1alpha1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/storage.k8s.io/v1alpha1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetStorageV1alpha1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetStorageV1alpha1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetStorageV1alpha1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetStorageV1alpha1APIResourcesResponse::Unauthorized,
            _ => GetStorageV1alpha1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation getStorageV1beta1APIResources

/// get available resources
pub fn get_storage_v1beta1_api_resources(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/apis/storage.k8s.io/v1beta1/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum GetStorageV1beta1APIResourcesResponse {
    Ok(::v1_10::apimachinery::pkg::apis::meta::v1::APIResourceList),
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for GetStorageV1beta1APIResourcesResponse {
    fn try_from_slice(status_code: ::http::StatusCode, buf: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::OK => {
                let result = match ::serde_json::from_slice(buf) {
                    Ok(value) => value,
                    Err(ref err) if err.is_eof() => return Err(::ResponseError::NeedMoreData),
                    Err(err) => return Err(::ResponseError::Json(err)),
                };
                GetStorageV1beta1APIResourcesResponse::Ok(result)
            },
            ::http::StatusCode::UNAUTHORIZED => GetStorageV1beta1APIResourcesResponse::Unauthorized,
            _ => GetStorageV1beta1APIResourcesResponse::Other,
        })
    }
}

// Generated from operation logFileHandler

pub fn log_file_handler(
    // path to the log
    logpath: &str,
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/logs/{logpath}", logpath = logpath);

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum LogFileHandlerResponse {
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for LogFileHandlerResponse {
    fn try_from_slice(status_code: ::http::StatusCode, _: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::UNAUTHORIZED => LogFileHandlerResponse::Unauthorized,
            _ => LogFileHandlerResponse::Other,
        })
    }
}

// Generated from operation logFileListHandler

pub fn log_file_list_handler(
) -> Result<::http::Request<Vec<u8>>, ::RequestError> {
    let __url = format!("/logs/");

    let mut __request = ::http::Request::get(__url);
    let __body = vec![];
    __request.body(__body).map_err(::RequestError::Http)
}

#[derive(Debug)]
pub enum LogFileListHandlerResponse {
    Unauthorized,
    Other,
}

impl<'a> ::Response<'a> for LogFileListHandlerResponse {
    fn try_from_slice(status_code: ::http::StatusCode, _: &'a [u8]) -> Result<Self, ::ResponseError> {
        Ok(match status_code {
            ::http::StatusCode::UNAUTHORIZED => LogFileListHandlerResponse::Unauthorized,
            _ => LogFileListHandlerResponse::Other,
        })
    }
}
