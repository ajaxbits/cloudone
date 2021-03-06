/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountLicenseSync : C1 License Sync information

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountLicenseSync {
    /// CloudOneBilling license state. Valid values are 'licensed' and 'unlicensed'
    #[serde(
        rename = "cloudOneBillingState",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_one_billing_state: Option<String>,
    /// The license expiration date.
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// customer Account ID of the tenant.
    #[serde(rename = "customerAccountID", skip_serializing_if = "Option::is_none")]
    pub customer_account_id: Option<String>,
}

impl AccountLicenseSync {
    /// C1 License Sync information
    pub fn new() -> AccountLicenseSync {
        AccountLicenseSync {
            cloud_one_billing_state: None,
            expires: None,
            customer_account_id: None,
        }
    }
}
