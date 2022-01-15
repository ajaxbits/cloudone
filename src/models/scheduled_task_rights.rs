/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ScheduledTaskRights : Scheduled task right details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduledTaskRights {
    /// Right to create new scheduled tasks.
    #[serde(rename = "canCreateNewScheduledTasks", skip_serializing_if = "Option::is_none")]
    pub can_create_new_scheduled_tasks: Option<bool>,
    /// Right to delete scheduled tasks.
    #[serde(rename = "canDeleteScheduledTasks", skip_serializing_if = "Option::is_none")]
    pub can_delete_scheduled_tasks: Option<bool>,
    /// Right to edit scheduled task properties.
    #[serde(rename = "canEditScheduledTaskProperties", skip_serializing_if = "Option::is_none")]
    pub can_edit_scheduled_task_properties: Option<bool>,
    /// Right to execute scheduled tasks.
    #[serde(rename = "canExecuteScheduledTasks", skip_serializing_if = "Option::is_none")]
    pub can_execute_scheduled_tasks: Option<bool>,
    /// Right to view scheduled tasks.
    #[serde(rename = "canViewScheduledTasks", skip_serializing_if = "Option::is_none")]
    pub can_view_scheduled_tasks: Option<bool>,
}

impl ScheduledTaskRights {
    /// Scheduled task right details.
    pub fn new() -> ScheduledTaskRights {
        ScheduledTaskRights {
            can_create_new_scheduled_tasks: None,
            can_delete_scheduled_tasks: None,
            can_edit_scheduled_task_properties: None,
            can_execute_scheduled_tasks: None,
            can_view_scheduled_tasks: None,
        }
    }
}

