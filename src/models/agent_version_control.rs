/*
 * Trend Micro Workload Security API
 *
 * Copyright 2018 - 2022 Trend Micro Incorporated.<br/>Get protected, stay secured, and keep informed with Trend Micro Workload Security's new RESTful API. Access system data and manage security configurations to automate your security workflows and integrate Workload Security into your CI/CD pipeline.  # Authentication  <!-- ReDoc-Inject: <security-definitions> -->
 *
 * The version of the OpenAPI document: 50.0.827
 *
 * Generated by: https://openapi-generator.tech
 */

/// AgentVersionControl : Agent version control details.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AgentVersionControl {
    /// OS of the agent version control. Searchable as Choice.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<Os>,
    /// Distro of the agent version control. Searchable as Choice.
    #[serde(rename = "distro", skip_serializing_if = "Option::is_none")]
    pub distro: Option<Distro>,
    /// Platform of the agent version control. Searchable as Choice.
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    /// Architecture of the agent version control. Searchable as Choice.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<Architecture>,
    /// Type of the agent version control. Searchable as Choice.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// Version of the agent version control. Searchable as String.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// List of available versions for the platform.
    #[serde(rename = "availableVersions", skip_serializing_if = "Option::is_none")]
    pub available_versions: Option<Vec<crate::models::AvailableVersion>>,
    /// ID of the agent version control. Searchable as ID.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub ID: Option<i32>,
}

impl AgentVersionControl {
    /// Agent version control details.
    pub fn new() -> AgentVersionControl {
        AgentVersionControl {
            os: None,
            distro: None,
            platform: None,
            architecture: None,
            _type: None,
            version: None,
            available_versions: None,
            ID: None,
        }
    }
}

/// OS of the agent version control. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Os {
    #[serde(rename = "Windows")]
    Windows,
    #[serde(rename = "Linux")]
    Linux,
    #[serde(rename = "Unix")]
    Unix,
    #[serde(rename = "macOS")]
    MacOS,
}
/// Distro of the agent version control. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Distro {
    #[serde(rename = "Red Hat")]
    RedHat,
    #[serde(rename = "CentOS")]
    CentOS,
    #[serde(rename = "SUSE")]
    SUSE,
    #[serde(rename = "Ubuntu")]
    Ubuntu,
    #[serde(rename = "Debian")]
    Debian,
    #[serde(rename = "Oracle Linux")]
    OracleLinux,
    #[serde(rename = "Amazon Linux")]
    AmazonLinux,
    #[serde(rename = "CloudLinux")]
    CloudLinux,
    #[serde(rename = "AlmaLinux")]
    AlmaLinux,
    #[serde(rename = "Rocky Linux")]
    RockyLinux,
    #[serde(rename = "Solaris")]
    Solaris,
    #[serde(rename = "AIX")]
    AIX,
}
/// Platform of the agent version control. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "Microsoft Windows 2000 Server")]
    MicrosoftWindows2000Server,
    #[serde(rename = "Microsoft Windows Server 2003")]
    MicrosoftWindowsServer2003,
    #[serde(rename = "Microsoft Windows Server 2003 R2")]
    MicrosoftWindowsServer2003R2,
    #[serde(rename = "Microsoft Windows Server 2008")]
    MicrosoftWindowsServer2008,
    #[serde(rename = "Microsoft Windows Server 2008 R2")]
    MicrosoftWindowsServer2008R2,
    #[serde(rename = "Microsoft Windows Server 2012")]
    MicrosoftWindowsServer2012,
    #[serde(rename = "Microsoft Windows Server 2012 R2")]
    MicrosoftWindowsServer2012R2,
    #[serde(rename = "Microsoft Windows Server 2016")]
    MicrosoftWindowsServer2016,
    #[serde(rename = "Microsoft Windows Server 2019")]
    MicrosoftWindowsServer2019,
    #[serde(rename = "Microsoft Windows Server 2022")]
    MicrosoftWindowsServer2022,
    #[serde(rename = "Microsoft Windows 2000 Professional")]
    MicrosoftWindows2000Professional,
    #[serde(rename = "Microsoft Windows XP")]
    MicrosoftWindowsXP,
    #[serde(rename = "Microsoft Windows 7")]
    MicrosoftWindows7,
    #[serde(rename = "Microsoft Windows 8")]
    MicrosoftWindows8,
    #[serde(rename = "Microsoft Windows 8.1")]
    MicrosoftWindows81,
    #[serde(rename = "Microsoft Windows 10")]
    MicrosoftWindows10,
    #[serde(rename = "Microsoft Windows 11")]
    MicrosoftWindows11,
    #[serde(rename = "Solaris 10 U4 to U6")]
    Solaris10U4ToU6,
    #[serde(rename = "Solaris 10 U7 to U11")]
    Solaris10U7ToU11,
    #[serde(rename = "Solaris 11.0 to 11.3")]
    Solaris110To113,
    #[serde(rename = "Solaris 11.4")]
    Solaris114,
    #[serde(rename = "AIX 5.3")]
    AIX53,
    #[serde(rename = "AIX 6.1")]
    AIX61,
    #[serde(rename = "AIX 7.1")]
    AIX71,
    #[serde(rename = "AIX 7.2")]
    AIX72,
    #[serde(rename = "Red Hat Enterprise 5")]
    RedHatEnterprise5,
    #[serde(rename = "Red Hat Enterprise 6")]
    RedHatEnterprise6,
    #[serde(rename = "Red Hat Enterprise 7")]
    RedHatEnterprise7,
    #[serde(rename = "Red Hat Enterprise 8")]
    RedHatEnterprise8,
    #[serde(rename = "Ubuntu Linux 10")]
    UbuntuLinux10,
    #[serde(rename = "Ubuntu Linux 12")]
    UbuntuLinux12,
    #[serde(rename = "Ubuntu Linux 14")]
    UbuntuLinux14,
    #[serde(rename = "Ubuntu Linux 16")]
    UbuntuLinux16,
    #[serde(rename = "Ubuntu Linux 18")]
    UbuntuLinux18,
    #[serde(rename = "Ubuntu Linux 20")]
    UbuntuLinux20,
    #[serde(rename = "CentOS 5")]
    CentOS5,
    #[serde(rename = "CentOS 6")]
    CentOS6,
    #[serde(rename = "CentOS 7")]
    CentOS7,
    #[serde(rename = "CentOS 8")]
    CentOS8,
    #[serde(rename = "Debian 6")]
    Debian6,
    #[serde(rename = "Debian 7")]
    Debian7,
    #[serde(rename = "Debian 8")]
    Debian8,
    #[serde(rename = "Debian 9")]
    Debian9,
    #[serde(rename = "Debian 10")]
    Debian10,
    #[serde(rename = "Amazon Linux")]
    AmazonLinux,
    #[serde(rename = "Amazon Linux 2")]
    AmazonLinux2,
    #[serde(rename = "Oracle Linux Release 5")]
    OracleLinuxRelease5,
    #[serde(rename = "Oracle Linux Release 6")]
    OracleLinuxRelease6,
    #[serde(rename = "Oracle Linux Release 7")]
    OracleLinuxRelease7,
    #[serde(rename = "Oracle Linux Release 8")]
    OracleLinuxRelease8,
    #[serde(rename = "SUSE Enterprise Server 10")]
    SUSEEnterpriseServer10,
    #[serde(rename = "SUSE Enterprise Server 11")]
    SUSEEnterpriseServer11,
    #[serde(rename = "SUSE Enterprise Server 12")]
    SUSEEnterpriseServer12,
    #[serde(rename = "SUSE Enterprise Server 15")]
    SUSEEnterpriseServer15,
    #[serde(rename = "CloudLinux 5")]
    CloudLinux5,
    #[serde(rename = "CloudLinux 6")]
    CloudLinux6,
    #[serde(rename = "CloudLinux 7")]
    CloudLinux7,
    #[serde(rename = "CloudLinux 8")]
    CloudLinux8,
    #[serde(rename = "AlmaLinux 8")]
    AlmaLinux8,
    #[serde(rename = "Rocky Linux 8")]
    RockyLinux8,
    #[serde(rename = "macOS 10.15 Catalina")]
    MacOS1015Catalina,
    #[serde(rename = "macOS 11 Big Sur")]
    MacOS11BigSur,
    #[serde(rename = "macOS 12 Monterey")]
    MacOS12Monterey,
}
/// Architecture of the agent version control. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Architecture {
    #[serde(rename = "32-bit")]
    _32Bit,
    #[serde(rename = "64-bit")]
    _64Bit,
    #[serde(rename = "sparc")]
    Sparc,
    #[serde(rename = "powerpc")]
    Powerpc,
    #[serde(rename = "64-bit Arm")]
    _64BitArm,
    #[serde(rename = "Universal")]
    Universal,
}
/// Type of the agent version control. Searchable as Choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "latest-lts")]
    LatestLts,
    #[serde(rename = "specified")]
    Specified,
    #[serde(rename = "unknown")]
    Unknown,
}
