/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// IdentifiedFileRights : Identified file right details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentifiedFileRights {
    /// Right to delete identified files.
    #[serde(rename = "canDeleteIdentifiedFiles", skip_serializing_if = "Option::is_none")]
    pub can_delete_identified_files: Option<bool>,
    /// Right to download identified files.
    #[serde(rename = "canDownloadIdentifiedFiles", skip_serializing_if = "Option::is_none")]
    pub can_download_identified_files: Option<bool>,
    /// Right to restore identified files.
    #[serde(rename = "canRestoreIdentifiedFiles", skip_serializing_if = "Option::is_none")]
    pub can_restore_identified_files: Option<bool>,
    /// Right to submit identified files to Deep Discovery Analyzer.
    #[serde(rename = "canSubmitIdentifiedFiles", skip_serializing_if = "Option::is_none")]
    pub can_submit_identified_files: Option<bool>,
}

impl IdentifiedFileRights {
    /// Identified file right details.
    pub fn new() -> IdentifiedFileRights {
        IdentifiedFileRights {
            can_delete_identified_files: None,
            can_download_identified_files: None,
            can_restore_identified_files: None,
            can_submit_identified_files: None,
        }
    }
}


