#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
pub type MetadataDocument = String;
#[doc = "Operation definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Operation name."]
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
        #[doc = "Resource provider name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "Resource name on which the operation is performed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "Operation name."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "Operation description."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "List of available operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResults {
    #[doc = "OData entity count; represents the number of operations returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "List of available operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl OperationsListResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy assignment summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentSummary {
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Non-compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[doc = "Policy definitions summary."]
    #[serde(rename = "policyDefinitions", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_definitions: Vec<PolicyDefinitionSummary>,
}
impl PolicyAssignmentSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy definition summary."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionSummary {
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Policy definition reference ID."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "Policy effect, i.e. policy definition action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[doc = "Non-compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
}
impl PolicyDefinitionSummary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy event record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEvent {
    #[doc = "OData entity ID; always set to null since policy event records do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Timestamp for the policy event record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Resource ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Effective parameters for the policy assignment."]
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[doc = "Flag which states whether the resource is compliant against the policy assignment it was evaluated against."]
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[doc = "Subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "List of resource tags."]
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[doc = "Policy assignment name."]
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[doc = "Policy assignment owner."]
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[doc = "Policy assignment parameters."]
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[doc = "Policy assignment scope."]
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[doc = "Policy definition name."]
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[doc = "Policy definition action, i.e. effect."]
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[doc = "Policy definition category."]
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Policy set definition name, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[doc = "Policy set definition owner, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[doc = "Policy set definition category, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[doc = "Policy set definition parameters, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[doc = "Comma separated list of management group IDs, which represent the hierarchy of the management groups the resource is under."]
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[doc = "Reference ID for the policy definition inside the policy set, if the policy assignment is for a policy set."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
    #[doc = "Tenant ID for the policy event record."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Principal object ID for the user who initiated the resource operation that triggered the policy event."]
    #[serde(rename = "principalOid", default, skip_serializing_if = "Option::is_none")]
    pub principal_oid: Option<String>,
}
impl PolicyEvent {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyEventsQueryResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of policy event records returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Query results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyEvent>,
}
impl PolicyEventsQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Policy state record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyState {
    #[doc = "OData entity ID; always set to null since policy state records do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Timestamp for the policy state record."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Resource ID."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Policy assignment ID."]
    #[serde(rename = "policyAssignmentId", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_id: Option<String>,
    #[doc = "Policy definition ID."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "Effective parameters for the policy assignment."]
    #[serde(rename = "effectiveParameters", default, skip_serializing_if = "Option::is_none")]
    pub effective_parameters: Option<String>,
    #[doc = "Flag which states whether the resource is compliant against the policy assignment it was evaluated against."]
    #[serde(rename = "isCompliant", default, skip_serializing_if = "Option::is_none")]
    pub is_compliant: Option<bool>,
    #[doc = "Subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Resource type."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "Resource location."]
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[doc = "Resource group name."]
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[doc = "List of resource tags."]
    #[serde(rename = "resourceTags", default, skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<String>,
    #[doc = "Policy assignment name."]
    #[serde(rename = "policyAssignmentName", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_name: Option<String>,
    #[doc = "Policy assignment owner."]
    #[serde(rename = "policyAssignmentOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_owner: Option<String>,
    #[doc = "Policy assignment parameters."]
    #[serde(rename = "policyAssignmentParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_parameters: Option<String>,
    #[doc = "Policy assignment scope."]
    #[serde(rename = "policyAssignmentScope", default, skip_serializing_if = "Option::is_none")]
    pub policy_assignment_scope: Option<String>,
    #[doc = "Policy definition name."]
    #[serde(rename = "policyDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_name: Option<String>,
    #[doc = "Policy definition action, i.e. effect."]
    #[serde(rename = "policyDefinitionAction", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_action: Option<String>,
    #[doc = "Policy definition category."]
    #[serde(rename = "policyDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_category: Option<String>,
    #[doc = "Policy set definition ID, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_id: Option<String>,
    #[doc = "Policy set definition name, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_name: Option<String>,
    #[doc = "Policy set definition owner, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionOwner", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_owner: Option<String>,
    #[doc = "Policy set definition category, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionCategory", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_category: Option<String>,
    #[doc = "Policy set definition parameters, if the policy assignment is for a policy set."]
    #[serde(rename = "policySetDefinitionParameters", default, skip_serializing_if = "Option::is_none")]
    pub policy_set_definition_parameters: Option<String>,
    #[doc = "Comma separated list of management group IDs, which represent the hierarchy of the management groups the resource is under."]
    #[serde(rename = "managementGroupIds", default, skip_serializing_if = "Option::is_none")]
    pub management_group_ids: Option<String>,
    #[doc = "Reference ID for the policy definition inside the policy set, if the policy assignment is for a policy set."]
    #[serde(rename = "policyDefinitionReferenceId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_reference_id: Option<String>,
}
impl PolicyState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Query results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyStatesQueryResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of policy state records returned."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Query results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyState>,
}
impl PolicyStatesQueryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryFailure {
    #[doc = "Error definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<query_failure::Error>,
}
impl QueryFailure {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod query_failure {
    use super::*;
    #[doc = "Error definition."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Service specific error code which serves as the substatus for the HTTP error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Description of the error."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Summarize action results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummarizeResults {
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "OData entity count; represents the number of summaries returned; always set to 1."]
    #[serde(rename = "@odata.count", default, skip_serializing_if = "Option::is_none")]
    pub odata_count: Option<i32>,
    #[doc = "Summarize action results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Summary>,
}
impl SummarizeResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Summary results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Summary {
    #[doc = "OData entity ID; always set to null since summaries do not have an entity ID."]
    #[serde(rename = "@odata.id", default, skip_serializing_if = "Option::is_none")]
    pub odata_id: Option<String>,
    #[doc = "OData context string; used by OData clients to resolve type information based on metadata."]
    #[serde(rename = "@odata.context", default, skip_serializing_if = "Option::is_none")]
    pub odata_context: Option<String>,
    #[doc = "Non-compliance summary on a particular summary level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<SummaryResults>,
    #[doc = "Policy assignments summary."]
    #[serde(rename = "policyAssignments", default, skip_serializing_if = "Vec::is_empty")]
    pub policy_assignments: Vec<PolicyAssignmentSummary>,
}
impl Summary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Non-compliance summary on a particular summary level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SummaryResults {
    #[doc = "HTTP POST URI for queryResults action on Microsoft.PolicyInsights to retrieve raw results for the non-compliance summary."]
    #[serde(rename = "queryResultsUri", default, skip_serializing_if = "Option::is_none")]
    pub query_results_uri: Option<String>,
    #[doc = "Number of non-compliant resources."]
    #[serde(rename = "nonCompliantResources", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_resources: Option<i32>,
    #[doc = "Number of non-compliant policies."]
    #[serde(rename = "nonCompliantPolicies", default, skip_serializing_if = "Option::is_none")]
    pub non_compliant_policies: Option<i32>,
}
impl SummaryResults {
    pub fn new() -> Self {
        Self::default()
    }
}
