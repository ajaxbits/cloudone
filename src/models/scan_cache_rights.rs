/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScanCacheRights : Scan cache right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScanCacheRights {
    /// Right to manage scan cache configuration.
    #[serde(rename = "canManageScanCache", skip_serializing_if = "Option::is_none")]
    pub can_manage_scan_cache: Option<bool>,
}

impl ScanCacheRights {
    /// Scan cache right details.
    pub fn new() -> ScanCacheRights {
        ScanCacheRights {
            can_manage_scan_cache: None,
        }
    }
}
