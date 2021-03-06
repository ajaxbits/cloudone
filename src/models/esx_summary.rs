/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// EsxSummary : VMware ESX Host Information.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EsxSummary {
    /// Manufacturer of the ESX. Searchable as String.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Model of the ESX. Searchable as String.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Quantity and processor speed of the ESX's processors. Searchable as Numeric.
    #[serde(rename = "processors", skip_serializing_if = "Option::is_none")]
    pub processors: Option<String>,
    /// Detailed information on the make and model of the ESX's processor. Searchable as String.
    #[serde(rename = "processorType", skip_serializing_if = "Option::is_none")]
    pub processor_type: Option<String>,
    /// Number of Network interface controllers the ESX has. Searchable as Numeric.
    #[serde(rename = "numberOfNICs", skip_serializing_if = "Option::is_none")]
    pub number_of_nics: Option<String>,
    /// State of the ESX.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Number of virtual machines hosted on the ESX.
    #[serde(rename = "virtualMachines", skip_serializing_if = "Option::is_none")]
    pub virtual_machines: Option<String>,
    /// Indicates whether vMotion is enabled on the ESX. The is value is true if enabled. Searchable as Boolean.
    #[serde(rename = "vMotionEnabled", skip_serializing_if = "Option::is_none")]
    pub v_motion_enabled: Option<String>,
    /// List of name/value pairs of ESX Custom Attributes
    #[serde(rename = "customAttributes", skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<crate::models::CustomAttribute>>,
    #[serde(rename = "TPMEnabled", skip_serializing_if = "Option::is_none")]
    pub tpm_enabled: Option<bool>,
    #[serde(rename = "TPMAlertsEnabled", skip_serializing_if = "Option::is_none")]
    pub tpm_alerts_enabled: Option<bool>,
    #[serde(rename = "TPMHasData", skip_serializing_if = "Option::is_none")]
    pub tpm_has_data: Option<bool>,
    #[serde(rename = "TPMLastChecked", skip_serializing_if = "Option::is_none")]
    pub tpm_last_checked: Option<i64>,
}

impl EsxSummary {
    /// VMware ESX Host Information.
    pub fn new() -> EsxSummary {
        EsxSummary {
            manufacturer: None,
            model: None,
            processors: None,
            processor_type: None,
            number_of_nics: None,
            state: None,
            virtual_machines: None,
            v_motion_enabled: None,
            custom_attributes: None,
            tpm_enabled: None,
            tpm_alerts_enabled: None,
            tpm_has_data: None,
            tpm_last_checked: None,
        }
    }
}
