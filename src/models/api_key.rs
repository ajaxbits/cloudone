/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ApiKey : Details of an API key. Use API keys to authenticate requests with Deep Security Manager.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    /// Display name of the APIKey. Searchable as String.
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    /// Description of the APIKey. Searchable as String.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Country and language for the APIKey.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<Locale>,
    /// ID of the role assigned to the APIKey. Searchable as Numeric.
    #[serde(rename = "roleID", skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,
    /// Display name of the APIKey's time zone, e.g. America/New_York. Searchable as String.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// If true, the APIKey can be used to authenticate. If false, the APIKey is locked out. Searchable as Boolean.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Timestamp of the APIKey's creation, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// Timestamp of the APIKey's last successful authentication, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "lastSignIn", skip_serializing_if = "Option::is_none")]
    pub last_sign_in: Option<i64>,
    /// Timestamp of when a locked out APIKey will be unlocked, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "unlockTime", skip_serializing_if = "Option::is_none")]
    pub unlock_time: Option<i64>,
    /// Number of unsuccessful authentication attempts made since the last successful authentication. Searchable as Numeric.
    #[serde(rename = "unsuccessfulSignInAttempts", skip_serializing_if = "Option::is_none")]
    pub unsuccessful_sign_in_attempts: Option<i32>,
    /// Timestamp of the APIKey's expiry date, in milliseconds since epoch. Searchable as Date.
    #[serde(rename = "expiryDate", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<i64>,
    /// Secret key used to authenticate API requests. Only returned when creating a new APIKey or regenerating the secret key.
    #[serde(rename = "secretKey", skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    /// If true, the APIKey was created by the primary tenant (T0) to authenticate API calls against other tenants' databases. Searchable as Boolean.
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<bool>,
    /// ID of the APIKey. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl ApiKey {
    /// Details of an API key. Use API keys to authenticate requests with Deep Security Manager.
    pub fn new() -> ApiKey {
        ApiKey {
            key_name: None,
            description: None,
            locale: None,
            role_id: None,
            time_zone: None,
            active: None,
            created: None,
            last_sign_in: None,
            unlock_time: None,
            unsuccessful_sign_in_attempts: None,
            expiry_date: None,
            secret_key: None,
            service_account: None,
            ID: None,
        }
    }
}

/// Country and language for the APIKey.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "en-US")]
    EnUS,
    #[serde(rename = "ja-JP")]
    JaJP,
}

