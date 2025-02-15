#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[doc = "The error's code."]
    pub code: String,
    #[doc = "A human readable error message."]
    pub message: String,
    #[doc = "Indicates which property in the request is responsible for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Additional error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[doc = "Contains details when the response code indicates an error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}
impl ErrorResponse {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[doc = "The identity type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The identity's principal id."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The identity's tenant id."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Hybrid Compute Machine properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
}
impl Machine {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Describes a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtension {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Describes Machine Extension Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl MachineExtension {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Describes the Machine Extension Instance View."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionInstanceView {
    #[doc = "The machine extension name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Instance view status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_extension_instance_view::Status>,
}
impl MachineExtensionInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_extension_instance_view {
    use super::*;
    #[doc = "Instance view status."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[doc = "The status code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "The level code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<status::Level>,
        #[doc = "The short localizable label for the status."]
        #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
        pub display_status: Option<String>,
        #[doc = "The detailed status message, including for alerts and error messages."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[doc = "The time of the status."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<String>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[doc = "The level code."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Info,
            Warning,
            Error,
        }
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The machine extension instance view."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<serde_json::Value>,
}
impl MachineExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes a Machine Extension Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Describes Machine Extension Update Properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl MachineExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Machine Extension."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdateProperties {
    #[doc = "How the extension handler should be forced to update even if the extension configuration has not changed."]
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[doc = "The name of the extension handler publisher."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Specifies the type of the extension; an example is \"CustomScriptExtension\"."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Specifies the version of the script handler."]
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[doc = "Indicates whether the extension should use a newer minor version if one is available at deployment time. Once deployed, however, the extension will not upgrade minor versions unless redeployed, even with this property set to true."]
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[doc = "Json formatted public settings for the extension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[doc = "The extension can contain either protectedSettings or protectedSettingsFromKeyVault or no protected settings at all."]
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
}
impl MachineExtensionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the Machine Extensions List Result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionsListResult {
    #[doc = "The list of extensions"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MachineExtension>,
    #[doc = "The uri to fetch the next page of machine extensions. Call ListNext() with this to fetch the next page of extensions."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MachineExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List hybrid machine operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListResult {
    #[doc = "The list of hybrid machines."]
    pub value: Vec<Machine>,
    #[doc = "The URI to fetch the next page of Machines. Call ListNext() with this URI to fetch the next page of hybrid machines."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MachineListResult {
    pub fn new(value: Vec<Machine>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Describes the properties of a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[doc = "Metadata pertaining to the geographic location of the resource."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[doc = "Specifies the operating system settings for the hybrid machine."]
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<serde_json::Value>,
    #[doc = "The provisioning state, which only appears in the response."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "The status of the hybrid machine agent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_properties::Status>,
    #[doc = "The time of the last status change."]
    #[serde(rename = "lastStatusChange", default, skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<String>,
    #[doc = "Details about the error state."]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetail>,
    #[doc = "The hybrid machine agent full version."]
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[doc = "Specifies the hybrid machine unique ID."]
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[doc = "Specifies the hybrid machine display name."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Specifies the hybrid machine FQDN."]
    #[serde(rename = "machineFqdn", default, skip_serializing_if = "Option::is_none")]
    pub machine_fqdn: Option<String>,
    #[doc = "Public Key that the client provides to be used during initial resource onboarding"]
    #[serde(rename = "clientPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[doc = "The Operating System running on the hybrid machine."]
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[doc = "The version of Operating System running on the hybrid machine."]
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[doc = "Specifies the Arc Machine's unique SMBIOS ID"]
    #[serde(rename = "vmUuid", default, skip_serializing_if = "Option::is_none")]
    pub vm_uuid: Option<String>,
    #[doc = "Machine Extensions information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<MachineExtensionInstanceView>,
    #[doc = "Specifies the Operating System product SKU."]
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<String>,
    #[doc = "Specifies the Windows domain name."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Specifies the AD fully qualified display name."]
    #[serde(rename = "adFqdn", default, skip_serializing_if = "Option::is_none")]
    pub ad_fqdn: Option<String>,
    #[doc = "Specifies the DNS fully qualified display name."]
    #[serde(rename = "dnsFqdn", default, skip_serializing_if = "Option::is_none")]
    pub dns_fqdn: Option<String>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_properties {
    use super::*;
    #[doc = "The status of the hybrid machine agent."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Connected,
        Disconnected,
        Error,
    }
}
#[doc = "Describes a hybrid machine Update."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[doc = "Hybrid Compute Machine Managed Identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
    #[doc = "Hybrid Compute Machine properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl MachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the ARM updatable properties of a hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdateProperties {
    #[doc = "Metadata pertaining to the geographic location of the resource."]
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
}
impl MachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specifies the operating system settings for the hybrid machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[doc = "Specifies the host OS name of the hybrid machine."]
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The List Compute Operation operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of compute operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationValue>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Compute Operation value."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValue {
    #[doc = "The origin of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Display properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<serde_json::Value>,
}
impl OperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the properties of a Hybrid Compute Operation Value Display."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValueDisplay {
    #[doc = "The display name of the compute operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "The display name of the resource the operation applies to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "The description of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource provider for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "The Update Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateResource {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to the geographic location of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationData {
    #[doc = "A canonical name for the geographic or physical location."]
    pub name: String,
    #[doc = "The city or locality where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The district, state, or province where the resource is located."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[doc = "The country or region where the resource is located"]
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
impl LocationData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            city: None,
            district: None,
            country_or_region: None,
        }
    }
}
