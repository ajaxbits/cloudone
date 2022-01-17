/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// AccountRights : Account right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountRights {
    /// Right to delete accounts.
    #[serde(rename = "canDeleteAccounts", skip_serializing_if = "Option::is_none")]
    pub can_delete_accounts: Option<bool>,
    /// Right to update accounts.
    #[serde(rename = "canUpdateAccounts", skip_serializing_if = "Option::is_none")]
    pub can_update_accounts: Option<bool>,
    /// Right to view account billing.
    #[serde(
        rename = "canViewAccountBilling",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_view_account_billing: Option<bool>,
}

impl AccountRights {
    /// Account right details.
    pub fn new() -> AccountRights {
        AccountRights {
            can_delete_accounts: None,
            can_update_accounts: None,
            can_view_account_billing: None,
        }
    }
}
