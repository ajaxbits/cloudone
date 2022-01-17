use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod account_details_api;
pub mod administrator_roles_api;
pub mod administrators_api;
pub mod agent_activation_information_api;
pub mod agent_deployment_scripts_api;
pub mod agent_version_control_profiles_api;
pub mod agent_version_controls_api;
pub mod anti_malware_configurations_api;
pub mod api_keys_api;
pub mod api_usage_api;
pub mod application_types_api;
pub mod aws_connector_settings_api;
pub mod aws_connectors_api;
pub mod azure_connectors_api;
pub mod certificates_api;
pub mod common_object_import_tasks_api;
pub mod computer_firewall_rule_assignments_api;
pub mod computer_firewall_rule_details_api;
pub mod computer_groups_api;
pub mod computer_integrity_monitoring_rule_assignments_recommendations_api;
pub mod computer_integrity_monitoring_rule_details_api;
pub mod computer_intrusion_prevention_application_type_details_api;
pub mod computer_intrusion_prevention_rule_assignments_recommendations_api;
pub mod computer_intrusion_prevention_rule_details_api;
pub mod computer_log_inspection_rule_assignments_recommendations_api;
pub mod computer_log_inspection_rule_details_api;
pub mod computers_api;
pub mod contacts_api;
pub mod contexts_api;
pub mod data_center_gateways_api;
pub mod directory_lists_api;
pub mod event_based_tasks_api;
pub mod file_extension_lists_api;
pub mod file_lists_api;
pub mod firewall_rules_api;
pub mod gcp_connector_actions_api;
pub mod gcp_connectors_api;
pub mod global_rules_api;
pub mod integrity_monitoring_rules_api;
pub mod interface_types_api;
pub mod intrusion_prevention_rules_api;
pub mod ip_lists_api;
pub mod log_inspection_rules_api;
pub mod mac_lists_api;
pub mod moved_computers_api;
pub mod policies_api;
pub mod policy_firewall_rule_assignments_api;
pub mod policy_firewall_rule_details_api;
pub mod policy_import_tasks_api;
pub mod policy_integrity_monitoring_rule_assignments_recommendations_api;
pub mod policy_integrity_monitoring_rule_details_api;
pub mod policy_intrusion_prevention_application_type_details_api;
pub mod policy_intrusion_prevention_rule_assignments_recommendations_api;
pub mod policy_intrusion_prevention_rule_details_api;
pub mod policy_log_inspection_rule_assignments_recommendations_api;
pub mod policy_log_inspection_rule_details_api;
pub mod port_lists_api;
pub mod report_templates_api;
pub mod rulesets_api;
pub mod sap_entitlement_api;
pub mod scheduled_tasks_api;
pub mod schedules_api;
pub mod software_changes_api;
pub mod software_inventories_api;
pub mod stateful_configurations_api;
pub mod system_settings_api;
pub mod trust_rules_api;
pub mod trust_rulesets_api;
pub mod v_center_connector_actions_api;
pub mod v_center_connectors_api;

pub mod configuration;
