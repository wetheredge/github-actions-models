//! Workflow events.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize, ser::SerializeMap as _};

use crate::common::EnvValue;

/// "Bare" workflow event triggers.
///
/// These appear when a workflow is triggered with an event with no context,
/// e.g.:
///
/// ```yaml
/// on: push
/// ```
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum BareEvent {
    BranchProtectionRule,
    CheckRun,
    CheckSuite,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Discussion,
    DiscussionComment,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    MergeGroup,
    Milestone,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequest,
    PullRequestComment,
    PullRequestReview,
    PullRequestReviewComment,
    PullRequestTarget,
    Push,
    RegistryPackage,
    Release,
    RepositoryDispatch,
    // NOTE: `schedule` is omitted, since it's never bare.
    Status,
    Watch,
    WorkflowCall,
    WorkflowDispatch,
    WorkflowRun,
}

/// Workflow event triggers, with bodies.
///
/// Like [`BareEvent`], but with per-event properties.
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct Events {
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub branch_protection_rule: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub check_run: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub check_suite: OptionalBody<GenericEvent>,
    // NOTE: `create` and `delete` are omitted, since they are always bare.
    // NOTE: `deployment` and `deployment_status` are omitted, since they are always bare.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub discussion: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub discussion_comment: OptionalBody<GenericEvent>,
    // NOTE: `fork` and `gollum` are omitted, since they are always bare.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub issue_comment: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub issues: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub label: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub merge_group: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub milestone: OptionalBody<GenericEvent>,
    // NOTE: `page_build` is omitted, since it is always bare.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub project: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub project_card: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub project_column: OptionalBody<GenericEvent>,
    // NOTE: `public` is omitted, since it is always bare.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub pull_request: OptionalBody<PullRequest>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub pull_request_comment: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub pull_request_review: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub pull_request_review_comment: OptionalBody<GenericEvent>,
    // NOTE: `pull_request_target` appears to have the same trigger filters as `pull_request`.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub pull_request_target: OptionalBody<PullRequest>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub push: OptionalBody<Push>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub registry_package: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub release: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub repository_dispatch: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub schedule: OptionalBody<Vec<Cron>>,
    // NOTE: `status` is omitted, since it is always bare.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub watch: OptionalBody<GenericEvent>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub workflow_call: OptionalBody<WorkflowCall>,
    // TODO: Custom type.
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub workflow_dispatch: OptionalBody<WorkflowDispatch>,
    #[serde(skip_serializing_if = "OptionalBody::is_missing")]
    pub workflow_run: OptionalBody<WorkflowRun>,
}

impl Events {
    /// Count the number of present event triggers.
    ///
    /// **IMPORTANT**: This must be kept in sync with the number of fields in `Events`.
    pub fn count(&self) -> u32 {
        // This is a little goofy, but it's faster than reflecting over the struct
        // or doing a serde round-trip.
        let mut count = 0;

        macro_rules! count_if_present {
            ($($field:ident),*) => {
                $(
                    if !matches!(self.$field, OptionalBody::Missing) {
                        count += 1;
                    }
                )*
            };
        }

        count_if_present!(
            branch_protection_rule,
            check_run,
            check_suite,
            discussion,
            discussion_comment,
            issue_comment,
            issues,
            label,
            merge_group,
            milestone,
            project,
            project_card,
            project_column,
            pull_request,
            pull_request_comment,
            pull_request_review,
            pull_request_review_comment,
            pull_request_target,
            push,
            registry_package,
            release,
            repository_dispatch,
            schedule,
            watch,
            workflow_call,
            workflow_dispatch,
            workflow_run
        );

        count
    }
}

/// A generic container type for distinguishing between
/// a missing key, an explicitly null key, and an explicit value `T`.
///
/// This is needed for modeling `on:` triggers, since GitHub distinguishes
/// between the non-presence of an event (no trigger) and the presence
/// of an empty event body (e.g. `pull_request:`), which means "trigger
/// with the defaults for this event type."
#[derive(Debug, Default)]
pub enum OptionalBody<T> {
    Default,
    #[default]
    Missing,
    Body(T),
}

impl<T> OptionalBody<T> {
    /// Returns `true` if the optional body is [`Missing`].
    ///
    /// [`Missing`]: OptionalBody::Missing
    #[must_use]
    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::Missing)
    }
}

impl<'de, T> Deserialize<'de> for OptionalBody<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Into::into)
    }
}

impl<T> Serialize for OptionalBody<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            OptionalBody::Default => serializer.serialize_map(Some(0))?.end(),
            OptionalBody::Missing => Err(serde::ser::Error::custom(
                "OptionalBody::Missing cannot be serialized",
            )),
            OptionalBody::Body(body) => body.serialize(serializer),
        }
    }
}

impl<T> From<Option<T>> for OptionalBody<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => OptionalBody::Body(v),
            None => OptionalBody::Default,
        }
    }
}

/// A generic event trigger body.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct GenericEvent {
    #[serde(
        default,
        with = "crate::common::scalar_or_vector",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub types: Vec<String>,
}

/// The body of a `pull_request` event trigger.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct PullRequest {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,

    #[serde(flatten)]
    pub branch_filters: Option<BranchFilters>,

    #[serde(flatten)]
    pub path_filters: Option<PathFilters>,
}

/// The body of a `push` event trigger.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Push {
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub branch_filters: Option<BranchFilters>,

    #[serde(flatten)]
    pub path_filters: Option<PathFilters>,

    #[serde(flatten)]
    pub tag_filters: Option<TagFilters>,
}

/// The body of a `cron` event trigger.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Cron {
    pub cron: String,
}

/// The body of a `workflow_call` event trigger.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCall {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub inputs: IndexMap<String, WorkflowCallInput>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub outputs: IndexMap<String, WorkflowCallOutput>,
    #[serde(
        default,
        skip_serializing_if = "IndexMap::is_empty",
        serialize_with = "crate::common::serialize_map_without_nones"
    )]
    pub secrets: IndexMap<String, Option<WorkflowCallSecret>>,
}

/// A single input in a `workflow_call` event trigger body.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    // TODO: model `default`?
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub required: bool,
    pub r#type: String,
}

/// A single output in a `workflow_call` event trigger body.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: String,
}

/// A single secret in a `workflow_call` event trigger body.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowCallSecret {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub required: bool,
}

/// The body of a `workflow_dispatch` event trigger.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowDispatch {
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub inputs: IndexMap<String, WorkflowDispatchInput>, // TODO: WorkflowDispatchInput
}

/// A single input in a `workflow_dispatch` event trigger body.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowDispatchInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    // TODO: model `default`?
    #[serde(default, skip_serializing_if = "crate::common::is_false")]
    pub required: bool,
    // TODO: Model as boolean, choice, number, environment, string; default is string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    // Only present when `type` is `choice`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<EnvValue>,
}

/// The body of a `workflow_run` event trigger.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct WorkflowRun {
    pub workflows: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub types: Vec<String>,
    #[serde(flatten)]
    pub branch_filters: Option<BranchFilters>,
}

/// Branch filtering variants for event trigger bodies.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum BranchFilters {
    Branches(Vec<String>),
    BranchesIgnore(Vec<String>),
}

/// Tag filtering variants for event trigger bodies.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum TagFilters {
    Tags(Vec<String>),
    TagsIgnore(Vec<String>),
}

/// Path filtering variants for event trigger bodies.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum PathFilters {
    Paths(Vec<String>),
    PathsIgnore(Vec<String>),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_events_count() {
        let events = "
push:
pull_request:
workflow_dispatch:
issue_comment:";

        let events = serde_yaml::from_str::<super::Events>(events).unwrap();
        assert_eq!(events.count(), 4);
    }
}
