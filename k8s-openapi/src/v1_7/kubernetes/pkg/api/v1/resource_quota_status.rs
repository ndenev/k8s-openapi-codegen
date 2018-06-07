// Generated from definition io.k8s.kubernetes.pkg.api.v1.ResourceQuotaStatus

/// ResourceQuotaStatus defines the enforced hard limits and observed use.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceQuotaStatus {
    /// Hard is the set of enforced hard limits for each named resource. More info: https://git.k8s.io/community/contributors/design-proposals/admission_control_resource_quota.md
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hard: Option<::std::collections::BTreeMap<String, ::v1_7::apimachinery::pkg::api::resource::Quantity>>,

    /// Used is the current observed total usage of the resource in the namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<::std::collections::BTreeMap<String, ::v1_7::apimachinery::pkg::api::resource::Quantity>>,
}