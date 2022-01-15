/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CertificateDetails : Certificate details



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateDetails {
    /// The certificate issuer.
    #[serde(rename = "issuerDN", skip_serializing_if = "Option::is_none")]
    pub issuer_dn: Option<String>,
    /// The certificate subject (owner).
    #[serde(rename = "subjectDN", skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    /// The date on which the certificate validity period begins.
    #[serde(rename = "notBefore", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<i64>,
    /// The date on which the certificate validity period ends.
    #[serde(rename = "notAfter", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<i64>,
    /// A number that uniquely identifies the certificate and is issued by the certification authority.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The sha1 fingerprint of the certificate.
    #[serde(rename = "sha1Fingerprint", skip_serializing_if = "Option::is_none")]
    pub sha1_fingerprint: Option<String>,
    /// The sha256 fingerprint of the certificate.
    #[serde(rename = "sha256Fingerprint", skip_serializing_if = "Option::is_none")]
    pub sha256_fingerprint: Option<String>,
}

impl CertificateDetails {
    /// Certificate details
    pub fn new() -> CertificateDetails {
        CertificateDetails {
            issuer_dn: None,
            subject_dn: None,
            not_before: None,
            not_after: None,
            serial_number: None,
            sha1_fingerprint: None,
            sha256_fingerprint: None,
        }
    }
}


