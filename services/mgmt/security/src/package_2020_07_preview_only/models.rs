#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Baseline details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Baseline {
    #[doc = "Expected results."]
    #[serde(rename = "expectedResults", default, skip_serializing_if = "Vec::is_empty")]
    pub expected_results: Vec<Vec<String>>,
    #[doc = "Baseline update time (UTC)."]
    #[serde(rename = "updatedTime", default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}
impl Baseline {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule result adjusted with baseline."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaselineAdjustedResult {
    #[doc = "Baseline details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline: Option<Baseline>,
    #[doc = "The rule result status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    #[doc = "Results the are not in baseline."]
    #[serde(rename = "resultsNotInBaseline", default, skip_serializing_if = "Vec::is_empty")]
    pub results_not_in_baseline: Vec<Vec<String>>,
    #[doc = "Results the are in baseline."]
    #[serde(rename = "resultsOnlyInBaseline", default, skip_serializing_if = "Vec::is_empty")]
    pub results_only_in_baseline: Vec<Vec<String>>,
}
impl BaselineAdjustedResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The benchmark references."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BenchmarkReference {
    #[doc = "The benchmark name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    #[doc = "The benchmark reference."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}
impl BenchmarkReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule query details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryCheck {
    #[doc = "The rule query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[doc = "Expected result."]
    #[serde(rename = "expectedResult", default, skip_serializing_if = "Vec::is_empty")]
    pub expected_result: Vec<Vec<String>>,
    #[doc = "Column names of expected result."]
    #[serde(rename = "columnNames", default, skip_serializing_if = "Vec::is_empty")]
    pub column_names: Vec<String>,
}
impl QueryCheck {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Remediation details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Remediation {
    #[doc = "Remediation description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Remediation script."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scripts: Vec<String>,
    #[doc = "Is remediation automated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automated: Option<bool>,
    #[doc = "Optional link to remediate in Azure Portal."]
    #[serde(rename = "portalLink", default, skip_serializing_if = "Option::is_none")]
    pub portal_link: Option<String>,
}
impl Remediation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes an Azure resource."]
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
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleResults {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Rule results properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RuleResultsProperties>,
}
impl RuleResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule results input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleResultsInput {
    #[doc = "Take results from latest scan."]
    #[serde(rename = "latestScan", default, skip_serializing_if = "Option::is_none")]
    pub latest_scan: Option<bool>,
    #[doc = "Expected results to be inserted into the baseline.\r\nLeave this field empty it LatestScan == true."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<Vec<String>>,
}
impl RuleResultsInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rule results properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RuleResultsProperties {
    #[doc = "Expected results in the baseline."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub results: Vec<Vec<String>>,
}
impl RuleResultsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The rule severity."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RuleSeverity {
    High,
    Medium,
    Low,
    Informational,
    Obsolete,
}
#[doc = "The rule result status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RuleStatus {
    NonFinding,
    Finding,
    InternalError,
}
#[doc = "The rule type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum RuleType {
    Binary,
    BaselineExpected,
    PositiveList,
    NegativeList,
}
#[doc = "A list of rules results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesResults {
    #[doc = "List of rule results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RuleResults>,
}
impl RulesResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Rules results input."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RulesResultsInput {
    #[doc = "Take results from latest scan."]
    #[serde(rename = "latestScan", default, skip_serializing_if = "Option::is_none")]
    pub latest_scan: Option<bool>,
    #[doc = "Expected results to be inserted into the baseline.\r\nLeave this field empty it LatestScan == true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<serde_json::Value>,
}
impl RulesResultsInput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scan {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A vulnerability assessment scan record properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScanProperties>,
}
impl Scan {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan record properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanProperties {
    #[doc = "The scan trigger type."]
    #[serde(rename = "triggerType", default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<ScanTriggerType>,
    #[doc = "The scan status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ScanState>,
    #[doc = "The server name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[doc = "The database name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[doc = "The SQL version."]
    #[serde(rename = "sqlVersion", default, skip_serializing_if = "Option::is_none")]
    pub sql_version: Option<String>,
    #[doc = "The scan start time (UTC)."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "Scan results are valid until end time (UTC)."]
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The number of failed rules with high severity."]
    #[serde(rename = "highSeverityFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub high_severity_failed_rules_count: Option<i32>,
    #[doc = "The number of failed rules with medium severity."]
    #[serde(rename = "mediumSeverityFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub medium_severity_failed_rules_count: Option<i32>,
    #[doc = "The number of failed rules with low severity."]
    #[serde(rename = "lowSeverityFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub low_severity_failed_rules_count: Option<i32>,
    #[doc = "The number of total passed rules."]
    #[serde(rename = "totalPassedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub total_passed_rules_count: Option<i32>,
    #[doc = "The number of total failed rules."]
    #[serde(rename = "totalFailedRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub total_failed_rules_count: Option<i32>,
    #[doc = "The number of total rules assessed."]
    #[serde(rename = "totalRulesCount", default, skip_serializing_if = "Option::is_none")]
    pub total_rules_count: Option<i32>,
    #[doc = "Baseline created for this database, and has one or more rules."]
    #[serde(rename = "isBaselineApplied", default, skip_serializing_if = "Option::is_none")]
    pub is_baseline_applied: Option<bool>,
}
impl ScanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan result for a single rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A vulnerability assessment scan result properties for a single rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScanResultProperties>,
}
impl ScanResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A vulnerability assessment scan result properties for a single rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResultProperties {
    #[doc = "The rule Id."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "The rule result status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    #[doc = "Indicated whether the results specified here are trimmed."]
    #[serde(rename = "isTrimmed", default, skip_serializing_if = "Option::is_none")]
    pub is_trimmed: Option<bool>,
    #[doc = "The results of the query that was run."]
    #[serde(rename = "queryResults", default, skip_serializing_if = "Vec::is_empty")]
    pub query_results: Vec<Vec<String>>,
    #[doc = "Remediation details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[doc = "The rule result adjusted with baseline."]
    #[serde(rename = "baselineAdjustedResult", default, skip_serializing_if = "Option::is_none")]
    pub baseline_adjusted_result: Option<BaselineAdjustedResult>,
    #[doc = "vulnerability assessment rule metadata details."]
    #[serde(rename = "ruleMetadata", default, skip_serializing_if = "Option::is_none")]
    pub rule_metadata: Option<VaRule>,
}
impl ScanResultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of vulnerability assessment scan results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScanResults {
    #[doc = "List of vulnerability assessment scan results."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScanResult>,
}
impl ScanResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The scan status."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScanState {
    Failed,
    FailedToRun,
    InProgress,
    Passed,
}
#[doc = "The scan trigger type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ScanTriggerType {
    OnDemand,
    Recurring,
}
#[doc = "A list of vulnerability assessment scan records."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Scans {
    #[doc = "List of vulnerability assessment scan records."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Scan>,
}
impl Scans {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "vulnerability assessment rule metadata details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct VaRule {
    #[doc = "The rule Id."]
    #[serde(rename = "ruleId", default, skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[doc = "The rule severity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<RuleSeverity>,
    #[doc = "The rule category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The rule type."]
    #[serde(rename = "ruleType", default, skip_serializing_if = "Option::is_none")]
    pub rule_type: Option<RuleType>,
    #[doc = "The rule title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The rule description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The rule rationale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rationale: Option<String>,
    #[doc = "The rule query details."]
    #[serde(rename = "queryCheck", default, skip_serializing_if = "Option::is_none")]
    pub query_check: Option<QueryCheck>,
    #[doc = "The benchmark references."]
    #[serde(rename = "benchmarkReferences", default, skip_serializing_if = "Vec::is_empty")]
    pub benchmark_references: Vec<BenchmarkReference>,
}
impl VaRule {
    pub fn new() -> Self {
        Self::default()
    }
}
