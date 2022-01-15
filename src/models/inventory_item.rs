/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 * 
 * Generated by: https://openapi-generator.tech
 */

/// InventoryItem : Software inventory item details.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InventoryItem {
    /// Vendor name of the inventory item as reported by the package management system on the computer. Searchable as String.
    #[serde(rename = "vendorName", skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    /// Product name of the inventory item as reported by the package management system on the computer. Searchable as String.
    #[serde(rename = "productName", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// Product version of the inventory item as reported by the package management system on the computer. Searchable as String.
    #[serde(rename = "productVersion", skip_serializing_if = "Option::is_none")]
    pub product_version: Option<String>,
    /// File version of the inventory item as reported by the package management system on the computer. Searchable as String.
    #[serde(rename = "fileVersion", skip_serializing_if = "Option::is_none")]
    pub file_version: Option<String>,
    /// File description of the inventory item as reported by the package management system on the computer. Searchable as String.
    #[serde(rename = "fileDescription", skip_serializing_if = "Option::is_none")]
    pub file_description: Option<String>,
    /// File category of the inventory item as reported by the package management system on the computer. Searchable as String.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// SHA-256 hash of the inventory item. Searchable as String.
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    /// SHA-1 hash of the inventory item. Searchable as String.
    #[serde(rename = "sha1", skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    /// MD5 hash of the inventory item. Searchable as String.
    #[serde(rename = "md5", skip_serializing_if = "Option::is_none")]
    pub md5: Option<String>,
    /// File name of the inventory item. Searchable as String.
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// File path of the inventory item. Searchable as String.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// File size of the inventory item in bytes. Searchable as Numeric.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// ID of the inventory item. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i64>,
}

impl InventoryItem {
    /// Software inventory item details.
    pub fn new() -> InventoryItem {
        InventoryItem {
            vendor_name: None,
            product_name: None,
            product_version: None,
            file_version: None,
            file_description: None,
            category: None,
            sha256: None,
            sha1: None,
            md5: None,
            file_name: None,
            path: None,
            size: None,
            ID: None,
        }
    }
}


