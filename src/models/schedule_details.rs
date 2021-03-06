/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// ScheduleDetails : Controls when a scheduled task runs.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleDetails {
    /// The timezone used to interpret the scheduled task schedule.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Recurrence type.
    #[serde(rename = "recurrenceType", skip_serializing_if = "Option::is_none")]
    pub recurrence_type: Option<RecurrenceType>,
    /// Number of times the task should execute.
    #[serde(rename = "recurrenceCount", skip_serializing_if = "Option::is_none")]
    pub recurrence_count: Option<i32>,
    #[serde(
        rename = "hourlyScheduleParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub hourly_schedule_parameters: Option<Box<crate::models::HourlyScheduleParameters>>,
    #[serde(
        rename = "dailyScheduleParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub daily_schedule_parameters: Option<Box<crate::models::DailyScheduleParameters>>,
    #[serde(
        rename = "weeklyScheduleParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub weekly_schedule_parameters: Option<Box<crate::models::WeeklyScheduleParameters>>,
    #[serde(
        rename = "monthlyScheduleParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub monthly_schedule_parameters: Option<Box<crate::models::MonthlyScheduleParameters>>,
    #[serde(
        rename = "onceOnlyScheduleParameters",
        skip_serializing_if = "Option::is_none"
    )]
    pub once_only_schedule_parameters: Option<Box<crate::models::OnceOnlyScheduleParameters>>,
}

impl ScheduleDetails {
    /// Controls when a scheduled task runs.
    pub fn new() -> ScheduleDetails {
        ScheduleDetails {
            time_zone: None,
            recurrence_type: None,
            recurrence_count: None,
            hourly_schedule_parameters: None,
            daily_schedule_parameters: None,
            weekly_schedule_parameters: None,
            monthly_schedule_parameters: None,
            once_only_schedule_parameters: None,
        }
    }
}

/// Recurrence type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecurrenceType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "hourly")]
    Hourly,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
}
