/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// ReportTemplate : Details of a report template.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// Name of the report template.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the report template.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Flag indicating whether or not the report template supports a time filter. Searchable as Boolean.
    #[serde(
        rename = "timeFilterSupported",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_filter_supported: Option<bool>,
    /// Flag indicating whether or not the report template supports a computer filter. Searchable as Boolean.
    #[serde(
        rename = "computerFilterSupported",
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_filter_supported: Option<bool>,
    /// Flag indicating whether or not the report template supports a tag filter. Searchable as Boolean.
    #[serde(rename = "tagFilterSupported", skip_serializing_if = "Option::is_none")]
    pub tag_filter_supported: Option<bool>,
    /// List of supported report formats.
    #[serde(rename = "supportedFormats", skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<Vec<SupportedFormats>>,
    /// ID of the report template. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl ReportTemplate {
    /// Details of a report template.
    pub fn new() -> ReportTemplate {
        ReportTemplate {
            name: None,
            description: None,
            time_filter_supported: None,
            computer_filter_supported: None,
            tag_filter_supported: None,
            supported_formats: None,
            ID: None,
        }
    }
}

/// List of supported report formats.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedFormats {
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "csv")]
    Csv,
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "plaintext")]
    Plaintext,
    #[serde(rename = "rtf")]
    Rtf,
    #[serde(rename = "xls")]
    Xls,
    #[serde(rename = "xml")]
    Xml,
}
