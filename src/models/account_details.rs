/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// AccountDetails : Account details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountDetails {
    /// License mode of the current Cloud One Account.
    #[serde(rename = "licenseMode", skip_serializing_if = "Option::is_none")]
    pub license_mode: Option<String>,
    /// Account ID of the current Cloud One Account.
    #[serde(rename = "cloudOneAccountID", skip_serializing_if = "Option::is_none")]
    pub cloud_one_account_id: Option<String>,
    /// Account Name of the current Cloud One Account.
    #[serde(rename = "cloudOneAccountName", skip_serializing_if = "Option::is_none")]
    pub cloud_one_account_name: Option<String>,
}

impl AccountDetails {
    /// Account details.
    pub fn new() -> AccountDetails {
        AccountDetails {
            license_mode: None,
            cloud_one_account_id: None,
            cloud_one_account_name: None,
        }
    }
}


