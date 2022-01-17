/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// CommonObjectImportTask : Describes the common object import task.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommonObjectImportTask {
    /// Common object content zipped and base64-encoded, write only.
    #[serde(rename = "commonObjectContent")]
    pub common_object_content: String,
    /// Type of the common objects to import. Searchable as Choice.
    #[serde(rename = "type")]
    pub _type: Type,
    /// Status of the task.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Timestamp of the created time, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Timestamp of the last updated time, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastUpdated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<i64>,
    /// Common object mapping between Deep Security Manager and Workload Security.
    #[serde(
        rename = "commonObjectMappings",
        skip_serializing_if = "Option::is_none"
    )]
    pub common_object_mappings: Option<Vec<crate::models::CommonObjectMapping>>,
    /// Type of failed import.
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// ID of the CommonObjectImportTask.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
    /// GUID of the task. Searchable as String.
    #[serde(rename = "taskGUID", skip_serializing_if = "Option::is_none")]
    pub task_guid: Option<String>,
}

impl CommonObjectImportTask {
    /// Describes the common object import task.
    pub fn new(common_object_content: String, _type: Type) -> CommonObjectImportTask {
        CommonObjectImportTask {
            common_object_content,
            _type,
            status: None,
            created: None,
            last_updated: None,
            common_object_mappings: None,
            error_code: None,
            ID: None,
            task_guid: None,
        }
    }
}

/// Type of the common objects to import. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "directory-lists")]
    DirectoryLists,
    #[serde(rename = "file-extension-lists")]
    FileExtensionLists,
    #[serde(rename = "file-lists")]
    FileLists,
    #[serde(rename = "ip-lists")]
    IpLists,
    #[serde(rename = "mac-lists")]
    MacLists,
    #[serde(rename = "port-lists")]
    PortLists,
    #[serde(rename = "contexts")]
    Contexts,
    #[serde(rename = "stateful-configurations")]
    StatefulConfigurations,
    #[serde(rename = "schedules")]
    Schedules,
}
/// Status of the task.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "failed")]
    Failed,
}
