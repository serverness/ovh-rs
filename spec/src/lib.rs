use std::collections::HashMap;
use dropshot::{
    endpoint, ApiDescription, HttpError, HttpResponseOk, PaginationParams,
    Path, Query, RequestContext, ResultsPage, TypedBody,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

struct ApiCtx {}

pub fn generate() -> Result<Value, String> {
    // Build a description of the API.
    let mut api: ApiDescription<ApiCtx> = ApiDescription::new();
    api.register(dedicated_server_get).unwrap();
    api.register(dedicated_server_update).unwrap();
    api.register(dedicated_server_reboot).unwrap();
    api.register(dedicated_server_install_start).unwrap();
    api.register(dedicated_server_install_status).unwrap();

    match api.openapi("OVH API", "v1")
        .json() {
        Ok(contents) => {
            Ok(contents)
        }

        Err(e) => {
            Err(e.to_string())
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum Datacenter {
    Bhs1,
    Bhs2,
    Bhs3,
    Bhs4,
    Bhs5,
    Bhs6,
    Bhs7,
    Bhs8,
    Cch01,
    Crx1,
    Crx2,
    Dc1,
    Waw1,
    Lim1,
    // todo
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Iam {
    pub display_name: Option<String>,
    pub id: Uuid,
    pub urn: String,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum State {
    Error,
    Hacked,
    HackedBlocked,
    Ok,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum SupportLevel {
    Critical,
    Fastpath,
    Gs,
    Pro,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum PowerState {
    Poweroff,
    Poweron,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServer {
    pub availability_zone: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub boot_id: Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub boot_script: Option<String>,
    pub commercial_range: Option<String>,
    pub datacenter: Datacenter,
    pub iam: Option<Iam>,
    pub ip: String, // todo ipv4
    pub link_speed: Option<u32>,
    pub monitoring: bool,
    pub name: String,
    pub new_upgrade_system: bool,
    pub no_intervention: bool,
    pub os: String,
    pub power_state: PowerState,
    pub professional_use: bool,
    pub rack: String,
    pub region: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub rescue_mail: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub rescue_ssh_key: Option<String>,
    pub reverse: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub root_device: Option<String>,
    pub server_id: u64,
    pub state: State,
    pub support_level: SupportLevel,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct ServicePathParams {
    service_name: String,
}

#[endpoint {
method = GET,
path = "/dedicated/server/{serviceName}",
tags = [ "dedicated_server" ],
}]
async fn dedicated_server_get(
    _rqctx: RequestContext<ApiCtx>,
    path_params: Path<ServicePathParams>,
) -> Result<HttpResponseOk<DedicatedServer>, HttpError> {
    todo!()
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct DedicatedServerUpdate {
    #[serde(skip_serializing_if="Option::is_none")]
    pub boot_id: Option<u32>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub boot_script: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub monitoring: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub no_intervention: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub rescue_mail: Option<String>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub rescue_ssh_key: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub root_device: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub state: Option<State>,
}

#[endpoint {
method = PUT,
path = "/dedicated/server/{serviceName}",
tags = [ "dedicated_server" ],
}]
async fn dedicated_server_update(
    _rqctx: RequestContext<ApiCtx>,
    path_params: Path<ServicePathParams>,
    b: TypedBody<DedicatedServerUpdate>
) -> Result<HttpResponseOk<()>, HttpError> {
    todo!()
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RebootFunction {
    #[serde(rename="INFRA_002_VirtualNetworkInterface")]
    Infra002VirtualNetworkInterface,
    #[serde(rename="INFRA_002_VirtualNetworkInterface_group")]
    Infra002VirtualNetworkInterfaceGroup,
    #[serde(rename="INFRA_002_VirtualNetworkInterface_ungroup")]
    Infra002VirtualNetworkInterfaceUngroup,
    #[serde(rename="INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_group")]
    InfraOneNetworkOneapiVirtualNetworkInterfaceGroup,
    #[serde(rename="INFRA_ONE_NETWORK_ONEAPI_VirtualNetworkInterface_ungroup")]
    InfraOneNetworkOneapiVirtualNetworkInterfaceUngroup,
    #[serde(rename="INFRA_ONE_NETWORK_VirtualNetworkInterface_group")]
    InfraOneNetworkVirtualNetworkInterfaceGroup,
    #[serde(rename="INFRA_ONE_NETWORK_VirtualNetworkInterface_ungroup")]
    InfraOneNetworkVirtualNetworkInterfaceUngroup,
    #[serde(rename="addVirtualMac")]
    AddVirtualMac,
    #[serde(rename="addWindowSplaFromExistingSerial")]
    AddWindowSplaFromExistingSerial,
    #[serde(rename="applyBackupFtpAcls")]
    ApplyBackupFtpAcls,
    #[serde(rename="applyBackupFtpQuota")]
    ApplyBackupFtpQuota,
    #[serde(rename="bypassAntiDDosGame")]
    BypassAntiDDosGame,
    #[serde(rename="changePasswordBackupFTP")]
    ChangePasswordBackupFTP,
    #[serde(rename="changeRipeOrg")]
    ChangeRipeOrg,
    #[serde(rename="checkAndReleaseIp")]
    CheckAndReleaseIp,
    #[serde(rename="createBackupFTP")]
    CreateBackupFTP,
    #[serde(rename="createOrUpdateRipeOrg")]
    CreateOrUpdateRipeOrg,
    #[serde(rename="createPrivateNetwork")]
    CreatePrivateNetwork,
    #[serde(rename="disableFirewall")]
    DisableFirewall,
    #[serde(rename="enableFirewall")]
    EnableFirewall,
    #[serde(rename="genericMoveFloatingIp")]
    GenericMoveFloatingIp,
    #[serde(rename="hardReboot")]
    HardReboot,
    #[serde(rename="hardware_update")]
    HardwareUpdate,
    #[serde(rename="ipmi/configureSGX")]
    IpmiConfigureSgx,
    #[serde(rename="migrateBackupFTP")]
    MigrateBackupFTP,
    #[serde(rename="moveFloatingIp")]
    MoveFloatingIp,
    #[serde(rename="moveVirtualMac")]
    MoveVirtualMac,
    #[serde(rename="reagregateBlock")]
    ReagregateBlock,
    #[serde(rename="rebootPower8To")]
    RebootPower8To,
    #[serde(rename="reinstallServer")]
    ReinstallServer,
    #[serde(rename="releaseIp")]
    ReleaseIp,
    #[serde(rename="removeBackupFTP")]
    RemoveBackupFTP,
    #[serde(rename="removeVirtualMac")]
    RemoveVirtualMac,
    #[serde(rename="requestAccessIPMI")]
    RequestAccessIPMI,
    #[serde(rename="resetIPMI")]
    ResetIPMI,
    #[serde(rename="resetIPMISession")]
    ResetIPMISession,
    #[serde(rename="testIPMIhttp")]
    TestIPMIhttp,
    #[serde(rename="testIPMIpassword")]
    TestIPMIpassword,
    #[serde(rename="testIPMIping")]
    TestIPMIping,
    #[serde(rename="virtualMacAdd")]
    VirtualMacAdd,
    #[serde(rename="virtualMacDelete")]
    VirtualMacDelete,
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RebootStatus {
    Cancelled,
    CustomerError,
    Doing,
    Done,
    Init,
    OvhError,
    Todo,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RebootTag {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerReboot {
    pub comment: Option<String>,
    pub done_date: Option<String>,
    pub function: RebootFunction,
    pub last_update: Option<String>,
    pub need_schedule: bool,
    pub note: Option<String>,
    pub planned_intervention_id: Option<u32>,
    pub start_date: String,
    pub status: RebootStatus,
    pub tags: Option<Vec<RebootTag>>,
    pub task_id: u32,
    pub ticket_reference: Option<String>
}

#[endpoint {
method = POST,
path = "/dedicated/server/{serviceName}/reboot",
tags = [ "dedicated_server" ],
}]
async fn dedicated_server_reboot(
    _rqctx: RequestContext<ApiCtx>,
    path_params: Path<ServicePathParams>,
) -> Result<HttpResponseOk<DedicatedServerReboot>, HttpError> {
    todo!()
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerInstallTag {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerInstallDetails {
    pub custom_hostname: Option<String>,
    pub disk_group_id: Option<u32>,
    pub no_raid: Option<bool>,
    pub post_installation_script_link: Option<String>,
    pub post_installation_script_return: Option<String>,
    pub soft_raid_devices: Option<u32>,
    pub ssh_key_name: Option<String>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerInstallStart {
    pub details: DedicatedServerInstallDetails,
    pub partition_scheme_name: String,
    pub template_name: String,
    pub user_metadata: Vec<DedicatedServerInstallTag>
}

#[endpoint {
method = POST,
path = "/dedicated/server/{serviceName}/install/start",
tags = [ "dedicated_server_install_start" ],
}]
async fn dedicated_server_install_start(
    _rqctx: RequestContext<ApiCtx>,
    path_params: Path<ServicePathParams>,
    _body: TypedBody<DedicatedServerInstallStart>
) -> Result<HttpResponseOk<DedicatedServerReboot>, HttpError> {
    todo!()
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum DedicatedServerInstallProgressStatus {
    Doing,
    Done,
    Error,
    Expired,
    Idle,
    Pending,
    Stopping,
    Todo,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerInstallProgress {
    pub comment: String,
    pub error: Option<String>,
    pub status: DedicatedServerInstallProgressStatus
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DedicatedServerInstallStatus {
    pub elapsed_time: i32,
    pub progress: Vec<DedicatedServerInstallProgress>
}

#[endpoint {
method = GET,
path = "/dedicated/server/{serviceName}/install/status",
tags = [ "dedicated_server_install_status" ],
}]
async fn dedicated_server_install_status(
    _rqctx: RequestContext<ApiCtx>,
    _path_params: Path<ServicePathParams>,
) -> Result<HttpResponseOk<DedicatedServerInstallStatus>, HttpError> {
    todo!()
}