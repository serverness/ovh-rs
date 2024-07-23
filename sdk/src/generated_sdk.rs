// The contents of this file are generated; do not modify them.

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    /// Datacenter
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "bhs1",
    ///    "bhs2",
    ///    "bhs3",
    ///    "bhs4",
    ///    "bhs5",
    ///    "bhs6",
    ///    "bhs7",
    ///    "bhs8",
    ///    "cch01",
    ///    "crx1",
    ///    "crx2",
    ///    "dc1",
    ///    "waw1",
    ///    "lim1"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum Datacenter {
        #[serde(rename = "bhs1")]
        Bhs1,
        #[serde(rename = "bhs2")]
        Bhs2,
        #[serde(rename = "bhs3")]
        Bhs3,
        #[serde(rename = "bhs4")]
        Bhs4,
        #[serde(rename = "bhs5")]
        Bhs5,
        #[serde(rename = "bhs6")]
        Bhs6,
        #[serde(rename = "bhs7")]
        Bhs7,
        #[serde(rename = "bhs8")]
        Bhs8,
        #[serde(rename = "cch01")]
        Cch01,
        #[serde(rename = "crx1")]
        Crx1,
        #[serde(rename = "crx2")]
        Crx2,
        #[serde(rename = "dc1")]
        Dc1,
        #[serde(rename = "waw1")]
        Waw1,
        #[serde(rename = "lim1")]
        Lim1,
    }

    impl From<&Datacenter> for Datacenter {
        fn from(value: &Datacenter) -> Self {
            value.clone()
        }
    }

    impl ToString for Datacenter {
        fn to_string(&self) -> String {
            match *self {
                Self::Bhs1 => "bhs1".to_string(),
                Self::Bhs2 => "bhs2".to_string(),
                Self::Bhs3 => "bhs3".to_string(),
                Self::Bhs4 => "bhs4".to_string(),
                Self::Bhs5 => "bhs5".to_string(),
                Self::Bhs6 => "bhs6".to_string(),
                Self::Bhs7 => "bhs7".to_string(),
                Self::Bhs8 => "bhs8".to_string(),
                Self::Cch01 => "cch01".to_string(),
                Self::Crx1 => "crx1".to_string(),
                Self::Crx2 => "crx2".to_string(),
                Self::Dc1 => "dc1".to_string(),
                Self::Waw1 => "waw1".to_string(),
                Self::Lim1 => "lim1".to_string(),
            }
        }
    }

    impl std::str::FromStr for Datacenter {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "bhs1" => Ok(Self::Bhs1),
                "bhs2" => Ok(Self::Bhs2),
                "bhs3" => Ok(Self::Bhs3),
                "bhs4" => Ok(Self::Bhs4),
                "bhs5" => Ok(Self::Bhs5),
                "bhs6" => Ok(Self::Bhs6),
                "bhs7" => Ok(Self::Bhs7),
                "bhs8" => Ok(Self::Bhs8),
                "cch01" => Ok(Self::Cch01),
                "crx1" => Ok(Self::Crx1),
                "crx2" => Ok(Self::Crx2),
                "dc1" => Ok(Self::Dc1),
                "waw1" => Ok(Self::Waw1),
                "lim1" => Ok(Self::Lim1),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for Datacenter {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Datacenter {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Datacenter {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// DedicatedServer
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "availabilityZone",
    ///    "datacenter",
    ///    "ip",
    ///    "monitoring",
    ///    "name",
    ///    "newUpgradeSystem",
    ///    "noIntervention",
    ///    "os",
    ///    "powerState",
    ///    "professionalUse",
    ///    "rack",
    ///    "region",
    ///    "serverId",
    ///    "state",
    ///    "supportLevel"
    ///  ],
    ///  "properties": {
    ///    "availabilityZone": {
    ///      "type": "string"
    ///    },
    ///    "bootId": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "bootScript": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "commercialRange": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "datacenter": {
    ///      "$ref": "#/components/schemas/Datacenter"
    ///    },
    ///    "iam": {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/Iam"
    ///        }
    ///      ]
    ///    },
    ///    "ip": {
    ///      "type": "string"
    ///    },
    ///    "linkSpeed": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "monitoring": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "newUpgradeSystem": {
    ///      "type": "boolean"
    ///    },
    ///    "noIntervention": {
    ///      "type": "boolean"
    ///    },
    ///    "os": {
    ///      "type": "string"
    ///    },
    ///    "powerState": {
    ///      "$ref": "#/components/schemas/PowerState"
    ///    },
    ///    "professionalUse": {
    ///      "type": "boolean"
    ///    },
    ///    "rack": {
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "rescueMail": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rescueSshKey": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "reverse": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rootDevice": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "serverId": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "state": {
    ///      "$ref": "#/components/schemas/State"
    ///    },
    ///    "supportLevel": {
    ///      "$ref": "#/components/schemas/SupportLevel"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServer {
        #[serde(rename = "availabilityZone")]
        pub availability_zone: String,
        #[serde(rename = "bootId", default, skip_serializing_if = "Option::is_none")]
        pub boot_id: Option<u32>,
        #[serde(
            rename = "bootScript",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub boot_script: Option<String>,
        #[serde(
            rename = "commercialRange",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub commercial_range: Option<String>,
        pub datacenter: Datacenter,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iam: Option<Iam>,
        pub ip: String,
        #[serde(rename = "linkSpeed", default, skip_serializing_if = "Option::is_none")]
        pub link_speed: Option<u32>,
        pub monitoring: bool,
        pub name: String,
        #[serde(rename = "newUpgradeSystem")]
        pub new_upgrade_system: bool,
        #[serde(rename = "noIntervention")]
        pub no_intervention: bool,
        pub os: String,
        #[serde(rename = "powerState")]
        pub power_state: PowerState,
        #[serde(rename = "professionalUse")]
        pub professional_use: bool,
        pub rack: String,
        pub region: String,
        #[serde(
            rename = "rescueMail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rescue_mail: Option<String>,
        #[serde(
            rename = "rescueSshKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rescue_ssh_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reverse: Option<String>,
        #[serde(
            rename = "rootDevice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub root_device: Option<String>,
        #[serde(rename = "serverId")]
        pub server_id: u64,
        pub state: State,
        #[serde(rename = "supportLevel")]
        pub support_level: SupportLevel,
    }

    impl From<&DedicatedServer> for DedicatedServer {
        fn from(value: &DedicatedServer) -> Self {
            value.clone()
        }
    }

    impl DedicatedServer {
        pub fn builder() -> builder::DedicatedServer {
            Default::default()
        }
    }

    /// DedicatedServerInstallDetails
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "customHostname": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "diskGroupId": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "noRaid": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "postInstallationScriptLink": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "postInstallationScriptReturn": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "softRaidDevices": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "sshKeyName": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerInstallDetails {
        #[serde(
            rename = "customHostname",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub custom_hostname: Option<String>,
        #[serde(
            rename = "diskGroupId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub disk_group_id: Option<u32>,
        #[serde(rename = "noRaid", default, skip_serializing_if = "Option::is_none")]
        pub no_raid: Option<bool>,
        #[serde(
            rename = "postInstallationScriptLink",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub post_installation_script_link: Option<String>,
        #[serde(
            rename = "postInstallationScriptReturn",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub post_installation_script_return: Option<String>,
        #[serde(
            rename = "softRaidDevices",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub soft_raid_devices: Option<u32>,
        #[serde(
            rename = "sshKeyName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ssh_key_name: Option<String>,
    }

    impl From<&DedicatedServerInstallDetails> for DedicatedServerInstallDetails {
        fn from(value: &DedicatedServerInstallDetails) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerInstallDetails {
        pub fn builder() -> builder::DedicatedServerInstallDetails {
            Default::default()
        }
    }

    /// DedicatedServerInstallProgress
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "comment",
    ///    "status"
    ///  ],
    ///  "properties": {
    ///    "comment": {
    ///      "type": "string"
    ///    },
    ///    "error": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/DedicatedServerInstallProgressStatus"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerInstallProgress {
        pub comment: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        pub status: DedicatedServerInstallProgressStatus,
    }

    impl From<&DedicatedServerInstallProgress> for DedicatedServerInstallProgress {
        fn from(value: &DedicatedServerInstallProgress) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerInstallProgress {
        pub fn builder() -> builder::DedicatedServerInstallProgress {
            Default::default()
        }
    }

    /// DedicatedServerInstallProgressStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "doing",
    ///    "done",
    ///    "error",
    ///    "expired",
    ///    "idle",
    ///    "pending",
    ///    "stopping",
    ///    "todo"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum DedicatedServerInstallProgressStatus {
        #[serde(rename = "doing")]
        Doing,
        #[serde(rename = "done")]
        Done,
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "expired")]
        Expired,
        #[serde(rename = "idle")]
        Idle,
        #[serde(rename = "pending")]
        Pending,
        #[serde(rename = "stopping")]
        Stopping,
        #[serde(rename = "todo")]
        Todo,
    }

    impl From<&DedicatedServerInstallProgressStatus> for DedicatedServerInstallProgressStatus {
        fn from(value: &DedicatedServerInstallProgressStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for DedicatedServerInstallProgressStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Doing => "doing".to_string(),
                Self::Done => "done".to_string(),
                Self::Error => "error".to_string(),
                Self::Expired => "expired".to_string(),
                Self::Idle => "idle".to_string(),
                Self::Pending => "pending".to_string(),
                Self::Stopping => "stopping".to_string(),
                Self::Todo => "todo".to_string(),
            }
        }
    }

    impl std::str::FromStr for DedicatedServerInstallProgressStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "doing" => Ok(Self::Doing),
                "done" => Ok(Self::Done),
                "error" => Ok(Self::Error),
                "expired" => Ok(Self::Expired),
                "idle" => Ok(Self::Idle),
                "pending" => Ok(Self::Pending),
                "stopping" => Ok(Self::Stopping),
                "todo" => Ok(Self::Todo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for DedicatedServerInstallProgressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for DedicatedServerInstallProgressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for DedicatedServerInstallProgressStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// DedicatedServerInstallStart
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "details",
    ///    "partitionSchemeName",
    ///    "templateName",
    ///    "userMetadata"
    ///  ],
    ///  "properties": {
    ///    "details": {
    ///      "$ref": "#/components/schemas/DedicatedServerInstallDetails"
    ///    },
    ///    "partitionSchemeName": {
    ///      "type": "string"
    ///    },
    ///    "templateName": {
    ///      "type": "string"
    ///    },
    ///    "userMetadata": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DedicatedServerInstallTag"
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerInstallStart {
        pub details: DedicatedServerInstallDetails,
        #[serde(rename = "partitionSchemeName")]
        pub partition_scheme_name: String,
        #[serde(rename = "templateName")]
        pub template_name: String,
        #[serde(rename = "userMetadata")]
        pub user_metadata: Vec<DedicatedServerInstallTag>,
    }

    impl From<&DedicatedServerInstallStart> for DedicatedServerInstallStart {
        fn from(value: &DedicatedServerInstallStart) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerInstallStart {
        pub fn builder() -> builder::DedicatedServerInstallStart {
            Default::default()
        }
    }

    /// DedicatedServerInstallStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "elapsedTime",
    ///    "progress"
    ///  ],
    ///  "properties": {
    ///    "elapsedTime": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "progress": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/DedicatedServerInstallProgress"
    ///      }
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerInstallStatus {
        #[serde(rename = "elapsedTime")]
        pub elapsed_time: i32,
        pub progress: Vec<DedicatedServerInstallProgress>,
    }

    impl From<&DedicatedServerInstallStatus> for DedicatedServerInstallStatus {
        fn from(value: &DedicatedServerInstallStatus) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerInstallStatus {
        pub fn builder() -> builder::DedicatedServerInstallStatus {
            Default::default()
        }
    }

    /// DedicatedServerInstallTag
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "key",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "key": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerInstallTag {
        pub key: String,
        pub value: String,
    }

    impl From<&DedicatedServerInstallTag> for DedicatedServerInstallTag {
        fn from(value: &DedicatedServerInstallTag) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerInstallTag {
        pub fn builder() -> builder::DedicatedServerInstallTag {
            Default::default()
        }
    }

    /// DedicatedServerReboot
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "function",
    ///    "needSchedule",
    ///    "startDate",
    ///    "status",
    ///    "taskId"
    ///  ],
    ///  "properties": {
    ///    "comment": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "doneDate": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "function": {
    ///      "$ref": "#/components/schemas/RebootFunction"
    ///    },
    ///    "lastUpdate": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "needSchedule": {
    ///      "type": "boolean"
    ///    },
    ///    "note": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "plannedInterventionId": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "startDate": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/RebootStatus"
    ///    },
    ///    "tags": {
    ///      "type": [
    ///        "array",
    ///        "null"
    ///      ],
    ///      "items": {
    ///        "$ref": "#/components/schemas/RebootTag"
    ///      }
    ///    },
    ///    "taskId": {
    ///      "type": "integer",
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "ticketReference": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerReboot {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub comment: Option<String>,
        #[serde(rename = "doneDate", default, skip_serializing_if = "Option::is_none")]
        pub done_date: Option<String>,
        pub function: RebootFunction,
        #[serde(
            rename = "lastUpdate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_update: Option<String>,
        #[serde(rename = "needSchedule")]
        pub need_schedule: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub note: Option<String>,
        #[serde(
            rename = "plannedInterventionId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub planned_intervention_id: Option<u32>,
        #[serde(rename = "startDate")]
        pub start_date: String,
        pub status: RebootStatus,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<Vec<RebootTag>>,
        #[serde(rename = "taskId")]
        pub task_id: u32,
        #[serde(
            rename = "ticketReference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ticket_reference: Option<String>,
    }

    impl From<&DedicatedServerReboot> for DedicatedServerReboot {
        fn from(value: &DedicatedServerReboot) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerReboot {
        pub fn builder() -> builder::DedicatedServerReboot {
            Default::default()
        }
    }

    /// DedicatedServerUpdate
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "properties": {
    ///    "bootId": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint32",
    ///      "minimum": 0.0
    ///    },
    ///    "bootScript": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "monitoring": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "noIntervention": {
    ///      "type": [
    ///        "boolean",
    ///        "null"
    ///      ]
    ///    },
    ///    "rescueMail": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rescueSshKey": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rootDevice": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "state": {
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/State"
    ///        }
    ///      ]
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct DedicatedServerUpdate {
        #[serde(rename = "bootId", default, skip_serializing_if = "Option::is_none")]
        pub boot_id: Option<u32>,
        #[serde(
            rename = "bootScript",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub boot_script: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub monitoring: Option<bool>,
        #[serde(
            rename = "noIntervention",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_intervention: Option<bool>,
        #[serde(
            rename = "rescueMail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rescue_mail: Option<String>,
        #[serde(
            rename = "rescueSshKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub rescue_ssh_key: Option<String>,
        #[serde(
            rename = "rootDevice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub root_device: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<State>,
    }

    impl From<&DedicatedServerUpdate> for DedicatedServerUpdate {
        fn from(value: &DedicatedServerUpdate) -> Self {
            value.clone()
        }
    }

    impl DedicatedServerUpdate {
        pub fn builder() -> builder::DedicatedServerUpdate {
            Default::default()
        }
    }

    /// Error information from a response.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "description": "Error information from a response.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "request_id"
    ///  ],
    ///  "properties": {
    ///    "error_code": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "request_id": {
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        pub message: String,
        pub request_id: String,
    }

    impl From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    /// Iam
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "urn"
    ///  ],
    ///  "properties": {
    ///    "displayName": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "type": "string",
    ///      "format": "uuid"
    ///    },
    ///    "tags": {
    ///      "type": [
    ///        "object",
    ///        "null"
    ///      ],
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "urn": {
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Iam {
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        pub id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tags: Option<std::collections::HashMap<String, String>>,
        pub urn: String,
    }

    impl From<&Iam> for Iam {
        fn from(value: &Iam) -> Self {
            value.clone()
        }
    }

    impl Iam {
        pub fn builder() -> builder::Iam {
            Default::default()
        }
    }

    /// PowerState
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "poweroff",
    ///    "poweron"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum PowerState {
        #[serde(rename = "poweroff")]
        Poweroff,
        #[serde(rename = "poweron")]
        Poweron,
    }

    impl From<&PowerState> for PowerState {
        fn from(value: &PowerState) -> Self {
            value.clone()
        }
    }

    impl ToString for PowerState {
        fn to_string(&self) -> String {
            match *self {
                Self::Poweroff => "poweroff".to_string(),
                Self::Poweron => "poweron".to_string(),
            }
        }
    }

    impl std::str::FromStr for PowerState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "poweroff" => Ok(Self::Poweroff),
                "poweron" => Ok(Self::Poweron),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PowerState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PowerState {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PowerState {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// RebootFunction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "INFRA_002_VirtualNetworkInterface",
    ///    "INFRA_002_VirtualNetworkInterface_group",
    ///    "INFRA_002_VirtualNetworkInterface_ungroup",
    ///    "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_group",
    ///    "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_ungroup",
    ///    "INFRA_ONE_NETWORK_VirtualNetworkInterface_group",
    ///    "INFRA_ONE_NETWORK_VirtualNetworkInterface_ungroup",
    ///    "addVirtualMac",
    ///    "addWindowSplaFromExistingSerial",
    ///    "applyBackupFtpAcls",
    ///    "applyBackupFtpQuota",
    ///    "bypassAntiDDosGame",
    ///    "changePasswordBackupFTP",
    ///    "changeRipeOrg",
    ///    "checkAndReleaseIp",
    ///    "createBackupFTP",
    ///    "createOrUpdateRipeOrg",
    ///    "createPrivateNetwork",
    ///    "disableFirewall",
    ///    "enableFirewall",
    ///    "genericMoveFloatingIp",
    ///    "hardReboot",
    ///    "hardware_update",
    ///    "ipmi/configureSGX",
    ///    "migrateBackupFTP",
    ///    "moveFloatingIp",
    ///    "moveVirtualMac",
    ///    "reagregateBlock",
    ///    "rebootPower8To",
    ///    "reinstallServer",
    ///    "releaseIp",
    ///    "removeBackupFTP",
    ///    "removeVirtualMac",
    ///    "requestAccessIPMI",
    ///    "resetIPMI",
    ///    "resetIPMISession",
    ///    "testIPMIhttp",
    ///    "testIPMIpassword",
    ///    "testIPMIping",
    ///    "virtualMacAdd",
    ///    "virtualMacDelete"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum RebootFunction {
        #[serde(rename = "INFRA_002_VirtualNetworkInterface")]
        Infra002VirtualNetworkInterface,
        #[serde(rename = "INFRA_002_VirtualNetworkInterface_group")]
        Infra002VirtualNetworkInterfaceGroup,
        #[serde(rename = "INFRA_002_VirtualNetworkInterface_ungroup")]
        Infra002VirtualNetworkInterfaceUngroup,
        #[serde(rename = "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_group")]
        InfraOneNetworkOneapiVirtualNetworkInterfaceGroup,
        #[serde(rename = "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_ungroup")]
        InfraOneNetworkOneapiVirtualNetworkInterfaceUngroup,
        #[serde(rename = "INFRA_ONE_NETWORK_VirtualNetworkInterface_group")]
        InfraOneNetworkVirtualNetworkInterfaceGroup,
        #[serde(rename = "INFRA_ONE_NETWORK_VirtualNetworkInterface_ungroup")]
        InfraOneNetworkVirtualNetworkInterfaceUngroup,
        #[serde(rename = "addVirtualMac")]
        AddVirtualMac,
        #[serde(rename = "addWindowSplaFromExistingSerial")]
        AddWindowSplaFromExistingSerial,
        #[serde(rename = "applyBackupFtpAcls")]
        ApplyBackupFtpAcls,
        #[serde(rename = "applyBackupFtpQuota")]
        ApplyBackupFtpQuota,
        #[serde(rename = "bypassAntiDDosGame")]
        BypassAntiDDosGame,
        #[serde(rename = "changePasswordBackupFTP")]
        ChangePasswordBackupFtp,
        #[serde(rename = "changeRipeOrg")]
        ChangeRipeOrg,
        #[serde(rename = "checkAndReleaseIp")]
        CheckAndReleaseIp,
        #[serde(rename = "createBackupFTP")]
        CreateBackupFtp,
        #[serde(rename = "createOrUpdateRipeOrg")]
        CreateOrUpdateRipeOrg,
        #[serde(rename = "createPrivateNetwork")]
        CreatePrivateNetwork,
        #[serde(rename = "disableFirewall")]
        DisableFirewall,
        #[serde(rename = "enableFirewall")]
        EnableFirewall,
        #[serde(rename = "genericMoveFloatingIp")]
        GenericMoveFloatingIp,
        #[serde(rename = "hardReboot")]
        HardReboot,
        #[serde(rename = "hardware_update")]
        HardwareUpdate,
        #[serde(rename = "ipmi/configureSGX")]
        IpmiConfigureSgx,
        #[serde(rename = "migrateBackupFTP")]
        MigrateBackupFtp,
        #[serde(rename = "moveFloatingIp")]
        MoveFloatingIp,
        #[serde(rename = "moveVirtualMac")]
        MoveVirtualMac,
        #[serde(rename = "reagregateBlock")]
        ReagregateBlock,
        #[serde(rename = "rebootPower8To")]
        RebootPower8To,
        #[serde(rename = "reinstallServer")]
        ReinstallServer,
        #[serde(rename = "releaseIp")]
        ReleaseIp,
        #[serde(rename = "removeBackupFTP")]
        RemoveBackupFtp,
        #[serde(rename = "removeVirtualMac")]
        RemoveVirtualMac,
        #[serde(rename = "requestAccessIPMI")]
        RequestAccessIpmi,
        #[serde(rename = "resetIPMI")]
        ResetIpmi,
        #[serde(rename = "resetIPMISession")]
        ResetIpmiSession,
        #[serde(rename = "testIPMIhttp")]
        TestIpmIhttp,
        #[serde(rename = "testIPMIpassword")]
        TestIpmIpassword,
        #[serde(rename = "testIPMIping")]
        TestIpmIping,
        #[serde(rename = "virtualMacAdd")]
        VirtualMacAdd,
        #[serde(rename = "virtualMacDelete")]
        VirtualMacDelete,
    }

    impl From<&RebootFunction> for RebootFunction {
        fn from(value: &RebootFunction) -> Self {
            value.clone()
        }
    }

    impl ToString for RebootFunction {
        fn to_string(&self) -> String {
            match *self {
                Self::Infra002VirtualNetworkInterface => {
                    "INFRA_002_VirtualNetworkInterface".to_string()
                }
                Self::Infra002VirtualNetworkInterfaceGroup => {
                    "INFRA_002_VirtualNetworkInterface_group".to_string()
                }
                Self::Infra002VirtualNetworkInterfaceUngroup => {
                    "INFRA_002_VirtualNetworkInterface_ungroup".to_string()
                }
                Self::InfraOneNetworkOneapiVirtualNetworkInterfaceGroup => {
                    "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_group".to_string()
                }
                Self::InfraOneNetworkOneapiVirtualNetworkInterfaceUngroup => {
                    "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_ungroup".to_string()
                }
                Self::InfraOneNetworkVirtualNetworkInterfaceGroup => {
                    "INFRA_ONE_NETWORK_VirtualNetworkInterface_group".to_string()
                }
                Self::InfraOneNetworkVirtualNetworkInterfaceUngroup => {
                    "INFRA_ONE_NETWORK_VirtualNetworkInterface_ungroup".to_string()
                }
                Self::AddVirtualMac => "addVirtualMac".to_string(),
                Self::AddWindowSplaFromExistingSerial => {
                    "addWindowSplaFromExistingSerial".to_string()
                }
                Self::ApplyBackupFtpAcls => "applyBackupFtpAcls".to_string(),
                Self::ApplyBackupFtpQuota => "applyBackupFtpQuota".to_string(),
                Self::BypassAntiDDosGame => "bypassAntiDDosGame".to_string(),
                Self::ChangePasswordBackupFtp => "changePasswordBackupFTP".to_string(),
                Self::ChangeRipeOrg => "changeRipeOrg".to_string(),
                Self::CheckAndReleaseIp => "checkAndReleaseIp".to_string(),
                Self::CreateBackupFtp => "createBackupFTP".to_string(),
                Self::CreateOrUpdateRipeOrg => "createOrUpdateRipeOrg".to_string(),
                Self::CreatePrivateNetwork => "createPrivateNetwork".to_string(),
                Self::DisableFirewall => "disableFirewall".to_string(),
                Self::EnableFirewall => "enableFirewall".to_string(),
                Self::GenericMoveFloatingIp => "genericMoveFloatingIp".to_string(),
                Self::HardReboot => "hardReboot".to_string(),
                Self::HardwareUpdate => "hardware_update".to_string(),
                Self::IpmiConfigureSgx => "ipmi/configureSGX".to_string(),
                Self::MigrateBackupFtp => "migrateBackupFTP".to_string(),
                Self::MoveFloatingIp => "moveFloatingIp".to_string(),
                Self::MoveVirtualMac => "moveVirtualMac".to_string(),
                Self::ReagregateBlock => "reagregateBlock".to_string(),
                Self::RebootPower8To => "rebootPower8To".to_string(),
                Self::ReinstallServer => "reinstallServer".to_string(),
                Self::ReleaseIp => "releaseIp".to_string(),
                Self::RemoveBackupFtp => "removeBackupFTP".to_string(),
                Self::RemoveVirtualMac => "removeVirtualMac".to_string(),
                Self::RequestAccessIpmi => "requestAccessIPMI".to_string(),
                Self::ResetIpmi => "resetIPMI".to_string(),
                Self::ResetIpmiSession => "resetIPMISession".to_string(),
                Self::TestIpmIhttp => "testIPMIhttp".to_string(),
                Self::TestIpmIpassword => "testIPMIpassword".to_string(),
                Self::TestIpmIping => "testIPMIping".to_string(),
                Self::VirtualMacAdd => "virtualMacAdd".to_string(),
                Self::VirtualMacDelete => "virtualMacDelete".to_string(),
            }
        }
    }

    impl std::str::FromStr for RebootFunction {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "INFRA_002_VirtualNetworkInterface" => Ok(Self::Infra002VirtualNetworkInterface),
                "INFRA_002_VirtualNetworkInterface_group" => {
                    Ok(Self::Infra002VirtualNetworkInterfaceGroup)
                }
                "INFRA_002_VirtualNetworkInterface_ungroup" => {
                    Ok(Self::Infra002VirtualNetworkInterfaceUngroup)
                }
                "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_group" => {
                    Ok(Self::InfraOneNetworkOneapiVirtualNetworkInterfaceGroup)
                }
                "INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_ungroup" => {
                    Ok(Self::InfraOneNetworkOneapiVirtualNetworkInterfaceUngroup)
                }
                "INFRA_ONE_NETWORK_VirtualNetworkInterface_group" => {
                    Ok(Self::InfraOneNetworkVirtualNetworkInterfaceGroup)
                }
                "INFRA_ONE_NETWORK_VirtualNetworkInterface_ungroup" => {
                    Ok(Self::InfraOneNetworkVirtualNetworkInterfaceUngroup)
                }
                "addVirtualMac" => Ok(Self::AddVirtualMac),
                "addWindowSplaFromExistingSerial" => Ok(Self::AddWindowSplaFromExistingSerial),
                "applyBackupFtpAcls" => Ok(Self::ApplyBackupFtpAcls),
                "applyBackupFtpQuota" => Ok(Self::ApplyBackupFtpQuota),
                "bypassAntiDDosGame" => Ok(Self::BypassAntiDDosGame),
                "changePasswordBackupFTP" => Ok(Self::ChangePasswordBackupFtp),
                "changeRipeOrg" => Ok(Self::ChangeRipeOrg),
                "checkAndReleaseIp" => Ok(Self::CheckAndReleaseIp),
                "createBackupFTP" => Ok(Self::CreateBackupFtp),
                "createOrUpdateRipeOrg" => Ok(Self::CreateOrUpdateRipeOrg),
                "createPrivateNetwork" => Ok(Self::CreatePrivateNetwork),
                "disableFirewall" => Ok(Self::DisableFirewall),
                "enableFirewall" => Ok(Self::EnableFirewall),
                "genericMoveFloatingIp" => Ok(Self::GenericMoveFloatingIp),
                "hardReboot" => Ok(Self::HardReboot),
                "hardware_update" => Ok(Self::HardwareUpdate),
                "ipmi/configureSGX" => Ok(Self::IpmiConfigureSgx),
                "migrateBackupFTP" => Ok(Self::MigrateBackupFtp),
                "moveFloatingIp" => Ok(Self::MoveFloatingIp),
                "moveVirtualMac" => Ok(Self::MoveVirtualMac),
                "reagregateBlock" => Ok(Self::ReagregateBlock),
                "rebootPower8To" => Ok(Self::RebootPower8To),
                "reinstallServer" => Ok(Self::ReinstallServer),
                "releaseIp" => Ok(Self::ReleaseIp),
                "removeBackupFTP" => Ok(Self::RemoveBackupFtp),
                "removeVirtualMac" => Ok(Self::RemoveVirtualMac),
                "requestAccessIPMI" => Ok(Self::RequestAccessIpmi),
                "resetIPMI" => Ok(Self::ResetIpmi),
                "resetIPMISession" => Ok(Self::ResetIpmiSession),
                "testIPMIhttp" => Ok(Self::TestIpmIhttp),
                "testIPMIpassword" => Ok(Self::TestIpmIpassword),
                "testIPMIping" => Ok(Self::TestIpmIping),
                "virtualMacAdd" => Ok(Self::VirtualMacAdd),
                "virtualMacDelete" => Ok(Self::VirtualMacDelete),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for RebootFunction {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for RebootFunction {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for RebootFunction {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// RebootStatus
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "cancelled",
    ///    "customerError",
    ///    "doing",
    ///    "done",
    ///    "init",
    ///    "ovhError",
    ///    "todo"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum RebootStatus {
        #[serde(rename = "cancelled")]
        Cancelled,
        #[serde(rename = "customerError")]
        CustomerError,
        #[serde(rename = "doing")]
        Doing,
        #[serde(rename = "done")]
        Done,
        #[serde(rename = "init")]
        Init,
        #[serde(rename = "ovhError")]
        OvhError,
        #[serde(rename = "todo")]
        Todo,
    }

    impl From<&RebootStatus> for RebootStatus {
        fn from(value: &RebootStatus) -> Self {
            value.clone()
        }
    }

    impl ToString for RebootStatus {
        fn to_string(&self) -> String {
            match *self {
                Self::Cancelled => "cancelled".to_string(),
                Self::CustomerError => "customerError".to_string(),
                Self::Doing => "doing".to_string(),
                Self::Done => "done".to_string(),
                Self::Init => "init".to_string(),
                Self::OvhError => "ovhError".to_string(),
                Self::Todo => "todo".to_string(),
            }
        }
    }

    impl std::str::FromStr for RebootStatus {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "cancelled" => Ok(Self::Cancelled),
                "customerError" => Ok(Self::CustomerError),
                "doing" => Ok(Self::Doing),
                "done" => Ok(Self::Done),
                "init" => Ok(Self::Init),
                "ovhError" => Ok(Self::OvhError),
                "todo" => Ok(Self::Todo),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for RebootStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for RebootStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for RebootStatus {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// RebootTag
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "key",
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "key": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    /// }
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct RebootTag {
        pub key: String,
        pub value: String,
    }

    impl From<&RebootTag> for RebootTag {
        fn from(value: &RebootTag) -> Self {
            value.clone()
        }
    }

    impl RebootTag {
        pub fn builder() -> builder::RebootTag {
            Default::default()
        }
    }

    /// State
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "error",
    ///    "hacked",
    ///    "hackedBlocked",
    ///    "ok"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum State {
        #[serde(rename = "error")]
        Error,
        #[serde(rename = "hacked")]
        Hacked,
        #[serde(rename = "hackedBlocked")]
        HackedBlocked,
        #[serde(rename = "ok")]
        Ok,
    }

    impl From<&State> for State {
        fn from(value: &State) -> Self {
            value.clone()
        }
    }

    impl ToString for State {
        fn to_string(&self) -> String {
            match *self {
                Self::Error => "error".to_string(),
                Self::Hacked => "hacked".to_string(),
                Self::HackedBlocked => "hackedBlocked".to_string(),
                Self::Ok => "ok".to_string(),
            }
        }
    }

    impl std::str::FromStr for State {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "error" => Ok(Self::Error),
                "hacked" => Ok(Self::Hacked),
                "hackedBlocked" => Ok(Self::HackedBlocked),
                "ok" => Ok(Self::Ok),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for State {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for State {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for State {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// SupportLevel
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "critical",
    ///    "fastpath",
    ///    "gs",
    ///    "pro"
    ///  ]
    /// }
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum SupportLevel {
        #[serde(rename = "critical")]
        Critical,
        #[serde(rename = "fastpath")]
        Fastpath,
        #[serde(rename = "gs")]
        Gs,
        #[serde(rename = "pro")]
        Pro,
    }

    impl From<&SupportLevel> for SupportLevel {
        fn from(value: &SupportLevel) -> Self {
            value.clone()
        }
    }

    impl ToString for SupportLevel {
        fn to_string(&self) -> String {
            match *self {
                Self::Critical => "critical".to_string(),
                Self::Fastpath => "fastpath".to_string(),
                Self::Gs => "gs".to_string(),
                Self::Pro => "pro".to_string(),
            }
        }
    }

    impl std::str::FromStr for SupportLevel {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "critical" => Ok(Self::Critical),
                "fastpath" => Ok(Self::Fastpath),
                "gs" => Ok(Self::Gs),
                "pro" => Ok(Self::Pro),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for SupportLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for SupportLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for SupportLevel {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct DedicatedServer {
            availability_zone: Result<String, String>,
            boot_id: Result<Option<u32>, String>,
            boot_script: Result<Option<String>, String>,
            commercial_range: Result<Option<String>, String>,
            datacenter: Result<super::Datacenter, String>,
            iam: Result<Option<super::Iam>, String>,
            ip: Result<String, String>,
            link_speed: Result<Option<u32>, String>,
            monitoring: Result<bool, String>,
            name: Result<String, String>,
            new_upgrade_system: Result<bool, String>,
            no_intervention: Result<bool, String>,
            os: Result<String, String>,
            power_state: Result<super::PowerState, String>,
            professional_use: Result<bool, String>,
            rack: Result<String, String>,
            region: Result<String, String>,
            rescue_mail: Result<Option<String>, String>,
            rescue_ssh_key: Result<Option<String>, String>,
            reverse: Result<Option<String>, String>,
            root_device: Result<Option<String>, String>,
            server_id: Result<u64, String>,
            state: Result<super::State, String>,
            support_level: Result<super::SupportLevel, String>,
        }

        impl Default for DedicatedServer {
            fn default() -> Self {
                Self {
                    availability_zone: Err("no value supplied for availability_zone".to_string()),
                    boot_id: Ok(Default::default()),
                    boot_script: Ok(Default::default()),
                    commercial_range: Ok(Default::default()),
                    datacenter: Err("no value supplied for datacenter".to_string()),
                    iam: Ok(Default::default()),
                    ip: Err("no value supplied for ip".to_string()),
                    link_speed: Ok(Default::default()),
                    monitoring: Err("no value supplied for monitoring".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    new_upgrade_system: Err("no value supplied for new_upgrade_system".to_string()),
                    no_intervention: Err("no value supplied for no_intervention".to_string()),
                    os: Err("no value supplied for os".to_string()),
                    power_state: Err("no value supplied for power_state".to_string()),
                    professional_use: Err("no value supplied for professional_use".to_string()),
                    rack: Err("no value supplied for rack".to_string()),
                    region: Err("no value supplied for region".to_string()),
                    rescue_mail: Ok(Default::default()),
                    rescue_ssh_key: Ok(Default::default()),
                    reverse: Ok(Default::default()),
                    root_device: Ok(Default::default()),
                    server_id: Err("no value supplied for server_id".to_string()),
                    state: Err("no value supplied for state".to_string()),
                    support_level: Err("no value supplied for support_level".to_string()),
                }
            }
        }

        impl DedicatedServer {
            pub fn availability_zone<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.availability_zone = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for availability_zone: {}",
                        e
                    )
                });
                self
            }
            pub fn boot_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u32>>,
                T::Error: std::fmt::Display,
            {
                self.boot_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for boot_id: {}", e));
                self
            }
            pub fn boot_script<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.boot_script = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for boot_script: {}", e));
                self
            }
            pub fn commercial_range<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.commercial_range = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for commercial_range: {}",
                        e
                    )
                });
                self
            }
            pub fn datacenter<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Datacenter>,
                T::Error: std::fmt::Display,
            {
                self.datacenter = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for datacenter: {}", e));
                self
            }
            pub fn iam<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::Iam>>,
                T::Error: std::fmt::Display,
            {
                self.iam = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for iam: {}", e));
                self
            }
            pub fn ip<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.ip = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ip: {}", e));
                self
            }
            pub fn link_speed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u32>>,
                T::Error: std::fmt::Display,
            {
                self.link_speed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link_speed: {}", e));
                self
            }
            pub fn monitoring<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.monitoring = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for monitoring: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn new_upgrade_system<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.new_upgrade_system = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for new_upgrade_system: {}",
                        e
                    )
                });
                self
            }
            pub fn no_intervention<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.no_intervention = value.try_into().map_err(|e| {
                    format!("error converting supplied value for no_intervention: {}", e)
                });
                self
            }
            pub fn os<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.os = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for os: {}", e));
                self
            }
            pub fn power_state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PowerState>,
                T::Error: std::fmt::Display,
            {
                self.power_state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for power_state: {}", e));
                self
            }
            pub fn professional_use<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.professional_use = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for professional_use: {}",
                        e
                    )
                });
                self
            }
            pub fn rack<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.rack = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rack: {}", e));
                self
            }
            pub fn region<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.region = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for region: {}", e));
                self
            }
            pub fn rescue_mail<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.rescue_mail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rescue_mail: {}", e));
                self
            }
            pub fn rescue_ssh_key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.rescue_ssh_key = value.try_into().map_err(|e| {
                    format!("error converting supplied value for rescue_ssh_key: {}", e)
                });
                self
            }
            pub fn reverse<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.reverse = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for reverse: {}", e));
                self
            }
            pub fn root_device<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.root_device = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for root_device: {}", e));
                self
            }
            pub fn server_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self.server_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for server_id: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::State>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn support_level<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::SupportLevel>,
                T::Error: std::fmt::Display,
            {
                self.support_level = value.try_into().map_err(|e| {
                    format!("error converting supplied value for support_level: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServer> for super::DedicatedServer {
            type Error = super::error::ConversionError;
            fn try_from(value: DedicatedServer) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    availability_zone: value.availability_zone?,
                    boot_id: value.boot_id?,
                    boot_script: value.boot_script?,
                    commercial_range: value.commercial_range?,
                    datacenter: value.datacenter?,
                    iam: value.iam?,
                    ip: value.ip?,
                    link_speed: value.link_speed?,
                    monitoring: value.monitoring?,
                    name: value.name?,
                    new_upgrade_system: value.new_upgrade_system?,
                    no_intervention: value.no_intervention?,
                    os: value.os?,
                    power_state: value.power_state?,
                    professional_use: value.professional_use?,
                    rack: value.rack?,
                    region: value.region?,
                    rescue_mail: value.rescue_mail?,
                    rescue_ssh_key: value.rescue_ssh_key?,
                    reverse: value.reverse?,
                    root_device: value.root_device?,
                    server_id: value.server_id?,
                    state: value.state?,
                    support_level: value.support_level?,
                })
            }
        }

        impl From<super::DedicatedServer> for DedicatedServer {
            fn from(value: super::DedicatedServer) -> Self {
                Self {
                    availability_zone: Ok(value.availability_zone),
                    boot_id: Ok(value.boot_id),
                    boot_script: Ok(value.boot_script),
                    commercial_range: Ok(value.commercial_range),
                    datacenter: Ok(value.datacenter),
                    iam: Ok(value.iam),
                    ip: Ok(value.ip),
                    link_speed: Ok(value.link_speed),
                    monitoring: Ok(value.monitoring),
                    name: Ok(value.name),
                    new_upgrade_system: Ok(value.new_upgrade_system),
                    no_intervention: Ok(value.no_intervention),
                    os: Ok(value.os),
                    power_state: Ok(value.power_state),
                    professional_use: Ok(value.professional_use),
                    rack: Ok(value.rack),
                    region: Ok(value.region),
                    rescue_mail: Ok(value.rescue_mail),
                    rescue_ssh_key: Ok(value.rescue_ssh_key),
                    reverse: Ok(value.reverse),
                    root_device: Ok(value.root_device),
                    server_id: Ok(value.server_id),
                    state: Ok(value.state),
                    support_level: Ok(value.support_level),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerInstallDetails {
            custom_hostname: Result<Option<String>, String>,
            disk_group_id: Result<Option<u32>, String>,
            no_raid: Result<Option<bool>, String>,
            post_installation_script_link: Result<Option<String>, String>,
            post_installation_script_return: Result<Option<String>, String>,
            soft_raid_devices: Result<Option<u32>, String>,
            ssh_key_name: Result<Option<String>, String>,
        }

        impl Default for DedicatedServerInstallDetails {
            fn default() -> Self {
                Self {
                    custom_hostname: Ok(Default::default()),
                    disk_group_id: Ok(Default::default()),
                    no_raid: Ok(Default::default()),
                    post_installation_script_link: Ok(Default::default()),
                    post_installation_script_return: Ok(Default::default()),
                    soft_raid_devices: Ok(Default::default()),
                    ssh_key_name: Ok(Default::default()),
                }
            }
        }

        impl DedicatedServerInstallDetails {
            pub fn custom_hostname<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.custom_hostname = value.try_into().map_err(|e| {
                    format!("error converting supplied value for custom_hostname: {}", e)
                });
                self
            }
            pub fn disk_group_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u32>>,
                T::Error: std::fmt::Display,
            {
                self.disk_group_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for disk_group_id: {}", e)
                });
                self
            }
            pub fn no_raid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.no_raid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for no_raid: {}", e));
                self
            }
            pub fn post_installation_script_link<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.post_installation_script_link = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for post_installation_script_link: {}",
                        e
                    )
                });
                self
            }
            pub fn post_installation_script_return<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.post_installation_script_return = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for post_installation_script_return: {}",
                        e
                    )
                });
                self
            }
            pub fn soft_raid_devices<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u32>>,
                T::Error: std::fmt::Display,
            {
                self.soft_raid_devices = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for soft_raid_devices: {}",
                        e
                    )
                });
                self
            }
            pub fn ssh_key_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.ssh_key_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for ssh_key_name: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerInstallDetails> for super::DedicatedServerInstallDetails {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerInstallDetails,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    custom_hostname: value.custom_hostname?,
                    disk_group_id: value.disk_group_id?,
                    no_raid: value.no_raid?,
                    post_installation_script_link: value.post_installation_script_link?,
                    post_installation_script_return: value.post_installation_script_return?,
                    soft_raid_devices: value.soft_raid_devices?,
                    ssh_key_name: value.ssh_key_name?,
                })
            }
        }

        impl From<super::DedicatedServerInstallDetails> for DedicatedServerInstallDetails {
            fn from(value: super::DedicatedServerInstallDetails) -> Self {
                Self {
                    custom_hostname: Ok(value.custom_hostname),
                    disk_group_id: Ok(value.disk_group_id),
                    no_raid: Ok(value.no_raid),
                    post_installation_script_link: Ok(value.post_installation_script_link),
                    post_installation_script_return: Ok(value.post_installation_script_return),
                    soft_raid_devices: Ok(value.soft_raid_devices),
                    ssh_key_name: Ok(value.ssh_key_name),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerInstallProgress {
            comment: Result<String, String>,
            error: Result<Option<String>, String>,
            status: Result<super::DedicatedServerInstallProgressStatus, String>,
        }

        impl Default for DedicatedServerInstallProgress {
            fn default() -> Self {
                Self {
                    comment: Err("no value supplied for comment".to_string()),
                    error: Ok(Default::default()),
                    status: Err("no value supplied for status".to_string()),
                }
            }
        }

        impl DedicatedServerInstallProgress {
            pub fn comment<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.comment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for comment: {}", e));
                self
            }
            pub fn error<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::DedicatedServerInstallProgressStatus>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerInstallProgress>
            for super::DedicatedServerInstallProgress
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerInstallProgress,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    comment: value.comment?,
                    error: value.error?,
                    status: value.status?,
                })
            }
        }

        impl From<super::DedicatedServerInstallProgress> for DedicatedServerInstallProgress {
            fn from(value: super::DedicatedServerInstallProgress) -> Self {
                Self {
                    comment: Ok(value.comment),
                    error: Ok(value.error),
                    status: Ok(value.status),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerInstallStart {
            details: Result<super::DedicatedServerInstallDetails, String>,
            partition_scheme_name: Result<String, String>,
            template_name: Result<String, String>,
            user_metadata: Result<Vec<super::DedicatedServerInstallTag>, String>,
        }

        impl Default for DedicatedServerInstallStart {
            fn default() -> Self {
                Self {
                    details: Err("no value supplied for details".to_string()),
                    partition_scheme_name: Err(
                        "no value supplied for partition_scheme_name".to_string()
                    ),
                    template_name: Err("no value supplied for template_name".to_string()),
                    user_metadata: Err("no value supplied for user_metadata".to_string()),
                }
            }
        }

        impl DedicatedServerInstallStart {
            pub fn details<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::DedicatedServerInstallDetails>,
                T::Error: std::fmt::Display,
            {
                self.details = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for details: {}", e));
                self
            }
            pub fn partition_scheme_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.partition_scheme_name = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for partition_scheme_name: {}",
                        e
                    )
                });
                self
            }
            pub fn template_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.template_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for template_name: {}", e)
                });
                self
            }
            pub fn user_metadata<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::DedicatedServerInstallTag>>,
                T::Error: std::fmt::Display,
            {
                self.user_metadata = value.try_into().map_err(|e| {
                    format!("error converting supplied value for user_metadata: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerInstallStart> for super::DedicatedServerInstallStart {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerInstallStart,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    details: value.details?,
                    partition_scheme_name: value.partition_scheme_name?,
                    template_name: value.template_name?,
                    user_metadata: value.user_metadata?,
                })
            }
        }

        impl From<super::DedicatedServerInstallStart> for DedicatedServerInstallStart {
            fn from(value: super::DedicatedServerInstallStart) -> Self {
                Self {
                    details: Ok(value.details),
                    partition_scheme_name: Ok(value.partition_scheme_name),
                    template_name: Ok(value.template_name),
                    user_metadata: Ok(value.user_metadata),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerInstallStatus {
            elapsed_time: Result<i32, String>,
            progress: Result<Vec<super::DedicatedServerInstallProgress>, String>,
        }

        impl Default for DedicatedServerInstallStatus {
            fn default() -> Self {
                Self {
                    elapsed_time: Err("no value supplied for elapsed_time".to_string()),
                    progress: Err("no value supplied for progress".to_string()),
                }
            }
        }

        impl DedicatedServerInstallStatus {
            pub fn elapsed_time<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
                T::Error: std::fmt::Display,
            {
                self.elapsed_time = value.try_into().map_err(|e| {
                    format!("error converting supplied value for elapsed_time: {}", e)
                });
                self
            }
            pub fn progress<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::DedicatedServerInstallProgress>>,
                T::Error: std::fmt::Display,
            {
                self.progress = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for progress: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerInstallStatus> for super::DedicatedServerInstallStatus {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerInstallStatus,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    elapsed_time: value.elapsed_time?,
                    progress: value.progress?,
                })
            }
        }

        impl From<super::DedicatedServerInstallStatus> for DedicatedServerInstallStatus {
            fn from(value: super::DedicatedServerInstallStatus) -> Self {
                Self {
                    elapsed_time: Ok(value.elapsed_time),
                    progress: Ok(value.progress),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerInstallTag {
            key: Result<String, String>,
            value: Result<String, String>,
        }

        impl Default for DedicatedServerInstallTag {
            fn default() -> Self {
                Self {
                    key: Err("no value supplied for key".to_string()),
                    value: Err("no value supplied for value".to_string()),
                }
            }
        }

        impl DedicatedServerInstallTag {
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerInstallTag> for super::DedicatedServerInstallTag {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerInstallTag,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    key: value.key?,
                    value: value.value?,
                })
            }
        }

        impl From<super::DedicatedServerInstallTag> for DedicatedServerInstallTag {
            fn from(value: super::DedicatedServerInstallTag) -> Self {
                Self {
                    key: Ok(value.key),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerReboot {
            comment: Result<Option<String>, String>,
            done_date: Result<Option<String>, String>,
            function: Result<super::RebootFunction, String>,
            last_update: Result<Option<String>, String>,
            need_schedule: Result<bool, String>,
            note: Result<Option<String>, String>,
            planned_intervention_id: Result<Option<u32>, String>,
            start_date: Result<String, String>,
            status: Result<super::RebootStatus, String>,
            tags: Result<Option<Vec<super::RebootTag>>, String>,
            task_id: Result<u32, String>,
            ticket_reference: Result<Option<String>, String>,
        }

        impl Default for DedicatedServerReboot {
            fn default() -> Self {
                Self {
                    comment: Ok(Default::default()),
                    done_date: Ok(Default::default()),
                    function: Err("no value supplied for function".to_string()),
                    last_update: Ok(Default::default()),
                    need_schedule: Err("no value supplied for need_schedule".to_string()),
                    note: Ok(Default::default()),
                    planned_intervention_id: Ok(Default::default()),
                    start_date: Err("no value supplied for start_date".to_string()),
                    status: Err("no value supplied for status".to_string()),
                    tags: Ok(Default::default()),
                    task_id: Err("no value supplied for task_id".to_string()),
                    ticket_reference: Ok(Default::default()),
                }
            }
        }

        impl DedicatedServerReboot {
            pub fn comment<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.comment = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for comment: {}", e));
                self
            }
            pub fn done_date<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.done_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for done_date: {}", e));
                self
            }
            pub fn function<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::RebootFunction>,
                T::Error: std::fmt::Display,
            {
                self.function = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for function: {}", e));
                self
            }
            pub fn last_update<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.last_update = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for last_update: {}", e));
                self
            }
            pub fn need_schedule<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<bool>,
                T::Error: std::fmt::Display,
            {
                self.need_schedule = value.try_into().map_err(|e| {
                    format!("error converting supplied value for need_schedule: {}", e)
                });
                self
            }
            pub fn note<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.note = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for note: {}", e));
                self
            }
            pub fn planned_intervention_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u32>>,
                T::Error: std::fmt::Display,
            {
                self.planned_intervention_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for planned_intervention_id: {}",
                        e
                    )
                });
                self
            }
            pub fn start_date<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.start_date = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for start_date: {}", e));
                self
            }
            pub fn status<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::RebootStatus>,
                T::Error: std::fmt::Display,
            {
                self.status = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for status: {}", e));
                self
            }
            pub fn tags<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<Vec<super::RebootTag>>>,
                T::Error: std::fmt::Display,
            {
                self.tags = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tags: {}", e));
                self
            }
            pub fn task_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u32>,
                T::Error: std::fmt::Display,
            {
                self.task_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for task_id: {}", e));
                self
            }
            pub fn ticket_reference<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.ticket_reference = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for ticket_reference: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerReboot> for super::DedicatedServerReboot {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerReboot,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    comment: value.comment?,
                    done_date: value.done_date?,
                    function: value.function?,
                    last_update: value.last_update?,
                    need_schedule: value.need_schedule?,
                    note: value.note?,
                    planned_intervention_id: value.planned_intervention_id?,
                    start_date: value.start_date?,
                    status: value.status?,
                    tags: value.tags?,
                    task_id: value.task_id?,
                    ticket_reference: value.ticket_reference?,
                })
            }
        }

        impl From<super::DedicatedServerReboot> for DedicatedServerReboot {
            fn from(value: super::DedicatedServerReboot) -> Self {
                Self {
                    comment: Ok(value.comment),
                    done_date: Ok(value.done_date),
                    function: Ok(value.function),
                    last_update: Ok(value.last_update),
                    need_schedule: Ok(value.need_schedule),
                    note: Ok(value.note),
                    planned_intervention_id: Ok(value.planned_intervention_id),
                    start_date: Ok(value.start_date),
                    status: Ok(value.status),
                    tags: Ok(value.tags),
                    task_id: Ok(value.task_id),
                    ticket_reference: Ok(value.ticket_reference),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct DedicatedServerUpdate {
            boot_id: Result<Option<u32>, String>,
            boot_script: Result<Option<String>, String>,
            monitoring: Result<Option<bool>, String>,
            no_intervention: Result<Option<bool>, String>,
            rescue_mail: Result<Option<String>, String>,
            rescue_ssh_key: Result<Option<String>, String>,
            root_device: Result<Option<String>, String>,
            state: Result<Option<super::State>, String>,
        }

        impl Default for DedicatedServerUpdate {
            fn default() -> Self {
                Self {
                    boot_id: Ok(Default::default()),
                    boot_script: Ok(Default::default()),
                    monitoring: Ok(Default::default()),
                    no_intervention: Ok(Default::default()),
                    rescue_mail: Ok(Default::default()),
                    rescue_ssh_key: Ok(Default::default()),
                    root_device: Ok(Default::default()),
                    state: Ok(Default::default()),
                }
            }
        }

        impl DedicatedServerUpdate {
            pub fn boot_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<u32>>,
                T::Error: std::fmt::Display,
            {
                self.boot_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for boot_id: {}", e));
                self
            }
            pub fn boot_script<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.boot_script = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for boot_script: {}", e));
                self
            }
            pub fn monitoring<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.monitoring = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for monitoring: {}", e));
                self
            }
            pub fn no_intervention<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<bool>>,
                T::Error: std::fmt::Display,
            {
                self.no_intervention = value.try_into().map_err(|e| {
                    format!("error converting supplied value for no_intervention: {}", e)
                });
                self
            }
            pub fn rescue_mail<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.rescue_mail = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rescue_mail: {}", e));
                self
            }
            pub fn rescue_ssh_key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.rescue_ssh_key = value.try_into().map_err(|e| {
                    format!("error converting supplied value for rescue_ssh_key: {}", e)
                });
                self
            }
            pub fn root_device<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.root_device = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for root_device: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<super::State>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<DedicatedServerUpdate> for super::DedicatedServerUpdate {
            type Error = super::error::ConversionError;
            fn try_from(
                value: DedicatedServerUpdate,
            ) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    boot_id: value.boot_id?,
                    boot_script: value.boot_script?,
                    monitoring: value.monitoring?,
                    no_intervention: value.no_intervention?,
                    rescue_mail: value.rescue_mail?,
                    rescue_ssh_key: value.rescue_ssh_key?,
                    root_device: value.root_device?,
                    state: value.state?,
                })
            }
        }

        impl From<super::DedicatedServerUpdate> for DedicatedServerUpdate {
            fn from(value: super::DedicatedServerUpdate) -> Self {
                Self {
                    boot_id: Ok(value.boot_id),
                    boot_script: Ok(value.boot_script),
                    monitoring: Ok(value.monitoring),
                    no_intervention: Ok(value.no_intervention),
                    rescue_mail: Ok(value.rescue_mail),
                    rescue_ssh_key: Ok(value.rescue_ssh_key),
                    root_device: Ok(value.root_device),
                    state: Ok(value.state),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            error_code: Result<Option<String>, String>,
            message: Result<String, String>,
            request_id: Result<String, String>,
        }

        impl Default for Error {
            fn default() -> Self {
                Self {
                    error_code: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                    request_id: Err("no value supplied for request_id".to_string()),
                }
            }
        }

        impl Error {
            pub fn error_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error_code: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(value: Error) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    error_code: value.error_code?,
                    message: value.message?,
                    request_id: value.request_id?,
                })
            }
        }

        impl From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    error_code: Ok(value.error_code),
                    message: Ok(value.message),
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Iam {
            display_name: Result<Option<String>, String>,
            id: Result<uuid::Uuid, String>,
            tags: Result<Option<std::collections::HashMap<String, String>>, String>,
            urn: Result<String, String>,
        }

        impl Default for Iam {
            fn default() -> Self {
                Self {
                    display_name: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    tags: Ok(Default::default()),
                    urn: Err("no value supplied for urn".to_string()),
                }
            }
        }

        impl Iam {
            pub fn display_name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.display_name = value.try_into().map_err(|e| {
                    format!("error converting supplied value for display_name: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn tags<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<std::collections::HashMap<String, String>>>,
                T::Error: std::fmt::Display,
            {
                self.tags = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for tags: {}", e));
                self
            }
            pub fn urn<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.urn = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for urn: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Iam> for super::Iam {
            type Error = super::error::ConversionError;
            fn try_from(value: Iam) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    display_name: value.display_name?,
                    id: value.id?,
                    tags: value.tags?,
                    urn: value.urn?,
                })
            }
        }

        impl From<super::Iam> for Iam {
            fn from(value: super::Iam) -> Self {
                Self {
                    display_name: Ok(value.display_name),
                    id: Ok(value.id),
                    tags: Ok(value.tags),
                    urn: Ok(value.urn),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RebootTag {
            key: Result<String, String>,
            value: Result<String, String>,
        }

        impl Default for RebootTag {
            fn default() -> Self {
                Self {
                    key: Err("no value supplied for key".to_string()),
                    value: Err("no value supplied for value".to_string()),
                }
            }
        }

        impl RebootTag {
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<RebootTag> for super::RebootTag {
            type Error = super::error::ConversionError;
            fn try_from(value: RebootTag) -> Result<Self, super::error::ConversionError> {
                Ok(Self {
                    key: value.key?,
                    value: value.value?,
                })
            }
        }

        impl From<super::RebootTag> for RebootTag {
            fn from(value: super::RebootTag) -> Self {
                Self {
                    key: Ok(value.key),
                    value: Ok(value.value),
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
/// Client for OVH API
///
/// Version: v1
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "v1"
    }
}

pub trait ClientDedicatedServerExt {
    /// Sends a `GET` request to `/dedicated/server/{serviceName}`
    ///
    /// ```ignore
    /// let response = client.dedicated_server_get()
    ///    .service_name(service_name)
    ///    .send()
    ///    .await;
    /// ```
    fn dedicated_server_get(&self) -> builder::DedicatedServerGet;
    /// Sends a `PUT` request to `/dedicated/server/{serviceName}`
    ///
    /// ```ignore
    /// let response = client.dedicated_server_update()
    ///    .service_name(service_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn dedicated_server_update(&self) -> builder::DedicatedServerUpdate;
    /// Sends a `POST` request to `/dedicated/server/{serviceName}/reboot`
    ///
    /// ```ignore
    /// let response = client.dedicated_server_reboot()
    ///    .service_name(service_name)
    ///    .send()
    ///    .await;
    /// ```
    fn dedicated_server_reboot(&self) -> builder::DedicatedServerReboot;
}

impl ClientDedicatedServerExt for Client {
    fn dedicated_server_get(&self) -> builder::DedicatedServerGet {
        builder::DedicatedServerGet::new(self)
    }

    fn dedicated_server_update(&self) -> builder::DedicatedServerUpdate {
        builder::DedicatedServerUpdate::new(self)
    }

    fn dedicated_server_reboot(&self) -> builder::DedicatedServerReboot {
        builder::DedicatedServerReboot::new(self)
    }
}

pub trait ClientDedicatedServerInstallStartExt {
    /// Sends a `POST` request to
    /// `/dedicated/server/{serviceName}/install/start`
    ///
    /// ```ignore
    /// let response = client.dedicated_server_install_start()
    ///    .service_name(service_name)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn dedicated_server_install_start(&self) -> builder::DedicatedServerInstallStart;
}

impl ClientDedicatedServerInstallStartExt for Client {
    fn dedicated_server_install_start(&self) -> builder::DedicatedServerInstallStart {
        builder::DedicatedServerInstallStart::new(self)
    }
}

pub trait ClientDedicatedServerInstallStatusExt {
    /// Sends a `GET` request to
    /// `/dedicated/server/{serviceName}/install/status`
    ///
    /// ```ignore
    /// let response = client.dedicated_server_install_status()
    ///    .service_name(service_name)
    ///    .send()
    ///    .await;
    /// ```
    fn dedicated_server_install_status(&self) -> builder::DedicatedServerInstallStatus;
}

impl ClientDedicatedServerInstallStatusExt for Client {
    fn dedicated_server_install_status(&self) -> builder::DedicatedServerInstallStatus {
        builder::DedicatedServerInstallStatus::new(self)
    }
}

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /// Builder for [`ClientDedicatedServerExt::dedicated_server_get`]
    ///
    /// [`ClientDedicatedServerExt::dedicated_server_get`]: super::ClientDedicatedServerExt::dedicated_server_get
    #[derive(Debug, Clone)]
    pub struct DedicatedServerGet<'a> {
        client: &'a super::Client,
        service_name: Result<String, String>,
    }

    impl<'a> DedicatedServerGet<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                service_name: Err("service_name was not initialized".to_string()),
            }
        }

        pub fn service_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.service_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for service_name failed".to_string());
            self
        }

        /// Sends a `GET` request to `/dedicated/server/{serviceName}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DedicatedServer>, Error<types::Error>> {
            let Self {
                client,
                service_name,
            } = self;
            let service_name = service_name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/dedicated/server/{}",
                client.baseurl,
                encode_path(&service_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`ClientDedicatedServerExt::dedicated_server_update`]
    ///
    /// [`ClientDedicatedServerExt::dedicated_server_update`]: super::ClientDedicatedServerExt::dedicated_server_update
    #[derive(Debug, Clone)]
    pub struct DedicatedServerUpdate<'a> {
        client: &'a super::Client,
        service_name: Result<String, String>,
        body: Result<types::builder::DedicatedServerUpdate, String>,
    }

    impl<'a> DedicatedServerUpdate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                service_name: Err("service_name was not initialized".to_string()),
                body: Ok(types::builder::DedicatedServerUpdate::default()),
            }
        }

        pub fn service_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.service_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for service_name failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DedicatedServerUpdate>,
            <V as std::convert::TryInto<types::DedicatedServerUpdate>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `DedicatedServerUpdate` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::DedicatedServerUpdate,
            ) -> types::builder::DedicatedServerUpdate,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `PUT` request to `/dedicated/server/{serviceName}`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                service_name,
                body,
            } = self;
            let service_name = service_name.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::DedicatedServerUpdate::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/dedicated/server/{}",
                client.baseurl,
                encode_path(&service_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for
    /// [`ClientDedicatedServerInstallStartExt::dedicated_server_install_start`]
    ///
    /// [`ClientDedicatedServerInstallStartExt::dedicated_server_install_start`]: super::ClientDedicatedServerInstallStartExt::dedicated_server_install_start
    #[derive(Debug, Clone)]
    pub struct DedicatedServerInstallStart<'a> {
        client: &'a super::Client,
        service_name: Result<String, String>,
        body: Result<types::builder::DedicatedServerInstallStart, String>,
    }

    impl<'a> DedicatedServerInstallStart<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                service_name: Err("service_name was not initialized".to_string()),
                body: Ok(types::builder::DedicatedServerInstallStart::default()),
            }
        }

        pub fn service_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.service_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for service_name failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::DedicatedServerInstallStart>,
            <V as std::convert::TryInto<types::DedicatedServerInstallStart>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `DedicatedServerInstallStart` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::DedicatedServerInstallStart,
            ) -> types::builder::DedicatedServerInstallStart,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to
        /// `/dedicated/server/{serviceName}/install/start`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DedicatedServerReboot>, Error<types::Error>> {
            let Self {
                client,
                service_name,
                body,
            } = self;
            let service_name = service_name.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::DedicatedServerInstallStart::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/dedicated/server/{}/install/start",
                client.baseurl,
                encode_path(&service_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for
    /// [`ClientDedicatedServerInstallStatusExt::dedicated_server_install_status`]
    ///
    /// [`ClientDedicatedServerInstallStatusExt::dedicated_server_install_status`]: super::ClientDedicatedServerInstallStatusExt::dedicated_server_install_status
    #[derive(Debug, Clone)]
    pub struct DedicatedServerInstallStatus<'a> {
        client: &'a super::Client,
        service_name: Result<String, String>,
    }

    impl<'a> DedicatedServerInstallStatus<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                service_name: Err("service_name was not initialized".to_string()),
            }
        }

        pub fn service_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.service_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for service_name failed".to_string());
            self
        }

        /// Sends a `GET` request to
        /// `/dedicated/server/{serviceName}/install/status`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DedicatedServerInstallStatus>, Error<types::Error>>
        {
            let Self {
                client,
                service_name,
            } = self;
            let service_name = service_name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/dedicated/server/{}/install/status",
                client.baseurl,
                encode_path(&service_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`ClientDedicatedServerExt::dedicated_server_reboot`]
    ///
    /// [`ClientDedicatedServerExt::dedicated_server_reboot`]: super::ClientDedicatedServerExt::dedicated_server_reboot
    #[derive(Debug, Clone)]
    pub struct DedicatedServerReboot<'a> {
        client: &'a super::Client,
        service_name: Result<String, String>,
    }

    impl<'a> DedicatedServerReboot<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                service_name: Err("service_name was not initialized".to_string()),
            }
        }

        pub fn service_name<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.service_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for service_name failed".to_string());
            self
        }

        /// Sends a `POST` request to `/dedicated/server/{serviceName}/reboot`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::DedicatedServerReboot>, Error<types::Error>> {
            let Self {
                client,
                service_name,
            } = self;
            let service_name = service_name.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/dedicated/server/{}/reboot",
                client.baseurl,
                encode_path(&service_name.to_string()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
    pub use super::ClientDedicatedServerExt;
    pub use super::ClientDedicatedServerInstallStartExt;
    pub use super::ClientDedicatedServerInstallStatusExt;
}
