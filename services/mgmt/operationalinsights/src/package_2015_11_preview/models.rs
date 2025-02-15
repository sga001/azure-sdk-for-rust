#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Datasources under OMS Workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "JSON object"]
    pub properties: Object,
    #[doc = "The ETag of the data source."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[doc = "The kind of the DataSource."]
    pub kind: DataSourceKind,
}
impl DataSource {
    pub fn new(properties: Object, kind: DataSourceKind) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            e_tag: None,
            kind,
        }
    }
}
#[doc = "DataSource filter. Right now, only filter by kind is supported."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceFilter {
    #[doc = "The kind of the DataSource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<DataSourceKind>,
}
impl DataSourceFilter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The kind of the DataSource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DataSourceKind {
    AzureActivityLog,
    ChangeTrackingPath,
    ChangeTrackingDefaultPath,
    ChangeTrackingDefaultRegistry,
    ChangeTrackingCustomRegistry,
    CustomLog,
    CustomLogCollection,
    GenericDataSource,
    #[serde(rename = "IISLogs")]
    IisLogs,
    LinuxPerformanceObject,
    LinuxPerformanceCollection,
    LinuxSyslog,
    LinuxSyslogCollection,
    WindowsEvent,
    WindowsPerformanceCounter,
}
#[doc = "The list data source by workspace operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourceListResult {
    #[doc = "A list of datasources."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DataSource>,
    #[doc = "The link (url) to the next page of datasources."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DataSourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Intelligence Pack containing a string name and boolean indicating if it's enabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IntelligencePack {
    #[doc = "The name of the intelligence pack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The enabled boolean for the intelligence pack."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The display name of the intelligence pack."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl IntelligencePack {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Linked service resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedService {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Linked service properties."]
    pub properties: LinkedServiceProperties,
}
impl LinkedService {
    pub fn new(properties: LinkedServiceProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[doc = "The list linked service operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinkedServiceListResult {
    #[doc = "Gets or sets a list of linked service instances."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<LinkedService>,
}
impl LinkedServiceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Linked service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkedServiceProperties {
    #[doc = "The resource id of the resource that will be linked to the workspace."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}
impl LinkedServiceProperties {
    pub fn new(resource_id: String) -> Self {
        Self { resource_id }
    }
}
#[doc = "A management group that is connected to a workspace"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroup {
    #[doc = "Management group properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ManagementGroupProperties>,
}
impl ManagementGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Management group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagementGroupProperties {
    #[doc = "The number of servers connected to the management group."]
    #[serde(rename = "serverCount", default, skip_serializing_if = "Option::is_none")]
    pub server_count: Option<i32>,
    #[doc = "Gets or sets a value indicating whether the management group is a gateway."]
    #[serde(rename = "isGateway", default, skip_serializing_if = "Option::is_none")]
    pub is_gateway: Option<bool>,
    #[doc = "The name of the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unique ID of the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The datetime that the management group was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[doc = "The last datetime that the management group received data."]
    #[serde(rename = "dataReceived", default, skip_serializing_if = "Option::is_none")]
    pub data_received: Option<String>,
    #[doc = "The version of System Center that is managing the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The SKU of System Center that is managing the management group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
}
impl ManagementGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The name of a metric."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[doc = "The system name of the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The localized name of the metric."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "JSON object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Object {}
impl Object {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Supported operation of OperationalInsights resource provider."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display metadata associated with the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Display metadata associated with the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "Service provider: Microsoft OperationsManagement."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource on which the operation is performed etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Type of operation: get, read, delete, etc."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Result of the request to list solution operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of solution operations supported by the OperationsManagement resource provider."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The status of operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationStatus {
    #[doc = "The operation Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The operation name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The status of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Describes the format of Error response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl OperationStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common properties of proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[doc = "Resource ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The shared keys for a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedKeys {
    #[doc = "The primary shared key of a workspace."]
    #[serde(rename = "primarySharedKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_shared_key: Option<String>,
    #[doc = "The secondary shared key of a workspace."]
    #[serde(rename = "secondarySharedKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_shared_key: Option<String>,
}
impl SharedKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The SKU (tier) of a workspace."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU."]
    pub name: sku::Name,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
        Self { name }
    }
}
pub mod sku {
    use super::*;
    #[doc = "The name of the SKU."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Free,
        Standard,
        Premium,
        PerNode,
        #[serde(rename = "PerGB2018")]
        PerGb2018,
        Standalone,
        CapacityReservation,
    }
}
#[doc = "A metric describing the usage of a resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageMetric {
    #[doc = "The name of a metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[doc = "The units used for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The current value of the metric."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[doc = "The quota limit for the metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[doc = "The time that the metric's value will reset."]
    #[serde(rename = "nextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[doc = "The quota period that determines the length of time between value resets."]
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
}
impl UsageMetric {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The top level Workspace resource container."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Workspace properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
    #[doc = "The ETag of the workspace."]
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl Workspace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list workspace management groups operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListManagementGroupsResult {
    #[doc = "Gets or sets a list of management groups attached to the workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagementGroup>,
}
impl WorkspaceListManagementGroupsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list workspaces operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListResult {
    #[doc = "A list of workspaces."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
}
impl WorkspaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The list workspace usages operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceListUsagesResult {
    #[doc = "Gets or sets a list of usage metrics for a workspace."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageMetric>,
}
impl WorkspaceListUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Workspace properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkspaceProperties {
    #[doc = "The provisioning state of the workspace."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<workspace_properties::ProvisioningState>,
    #[doc = "This is a read-only legacy property. It is always set to 'Azure' by the service. Kept here for backward compatibility."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "This is a read-only property. Represents the ID associated with the workspace."]
    #[serde(rename = "customerId", default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[doc = "This is a legacy property and is not used anymore. Kept here for backward compatibility."]
    #[serde(rename = "portalUrl", default, skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    #[doc = "The SKU (tier) of a workspace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The workspace data retention in days. -1 means Unlimited retention for the Unlimited Sku. 730 days is the maximum allowed for all other Skus. "]
    #[serde(rename = "retentionInDays", default, skip_serializing_if = "Option::is_none")]
    pub retention_in_days: Option<i32>,
}
impl WorkspaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod workspace_properties {
    use super::*;
    #[doc = "The provisioning state of the workspace."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Succeeded,
        Failed,
        Canceled,
        Deleting,
        ProvisioningAccount,
    }
}
