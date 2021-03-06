/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// QueryTracebackRights : Query traceback right details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryTracebackRights {
    /// Right to view query tracebacks.
    #[serde(
        rename = "canViewQueryTracebacks",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_view_query_tracebacks: Option<bool>,
}

impl QueryTracebackRights {
    /// Query traceback right details.
    pub fn new() -> QueryTracebackRights {
        QueryTracebackRights {
            can_view_query_tracebacks: None,
        }
    }
}
