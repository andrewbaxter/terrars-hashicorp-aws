#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EvidentlyLaunchData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    project: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    randomization_salt: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<std::vec::Vec<EvidentlyLaunch_Groups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_monitors: std::option::Option<std::vec::Vec<EvidentlyLaunch_MetricMonitors>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduled_splits_config: std::option::Option<std::vec::Vec<EvidentlyLaunch_ScheduledSplitsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EvidentlyLaunch_Timeouts>,
    dynamic: EvidentlyLaunchDynamic,
}

struct EvidentlyLaunch_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EvidentlyLaunchData_>,
}

#[derive(Clone)]
pub struct EvidentlyLaunch(std::rc::Rc<EvidentlyLaunch_>);

impl EvidentlyLaunch {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `randomization_salt`.\n"]
    pub fn set_randomization_salt(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().randomization_salt = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `groups`.\n"]
    pub fn set_groups(self, v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyLaunch_Groups>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.groups = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metric_monitors`.\n"]
    pub fn set_metric_monitors(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyLaunch_MetricMonitors>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metric_monitors = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metric_monitors = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduled_splits_config`.\n"]
    pub fn set_scheduled_splits_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyLaunch_ScheduledSplitsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scheduled_splits_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scheduled_splits_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EvidentlyLaunch_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution` after provisioning.\n"]
    pub fn execution(&self) -> terrars::ListRef<EvidentlyLaunch_ExecutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `randomization_salt` after provisioning.\n"]
    pub fn randomization_salt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.randomization_salt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::ListRef<EvidentlyLaunch_GroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_monitors` after provisioning.\n"]
    pub fn metric_monitors(&self) -> terrars::ListRef<EvidentlyLaunch_MetricMonitorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_monitors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduled_splits_config` after provisioning.\n"]
    pub fn scheduled_splits_config(&self) -> terrars::ListRef<EvidentlyLaunch_ScheduledSplitsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_splits_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyLaunch_TimeoutsRef {
        EvidentlyLaunch_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EvidentlyLaunch {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EvidentlyLaunch { }

impl terrars::ToListMappable for EvidentlyLaunch {
    type O = terrars::ListRef<EvidentlyLaunchRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EvidentlyLaunch_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_evidently_launch".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEvidentlyLaunch {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub project: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyLaunch {
    pub fn build(self, stack: &mut terrars::Stack) -> EvidentlyLaunch {
        let out = EvidentlyLaunch(std::rc::Rc::new(EvidentlyLaunch_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EvidentlyLaunchData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                project: self.project,
                randomization_salt: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                groups: core::default::Default::default(),
                metric_monitors: core::default::Default::default(),
                scheduled_splits_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EvidentlyLaunchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EvidentlyLaunchRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution` after provisioning.\n"]
    pub fn execution(&self) -> terrars::ListRef<EvidentlyLaunch_ExecutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_time` after provisioning.\n"]
    pub fn last_updated_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project` after provisioning.\n"]
    pub fn project(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `randomization_salt` after provisioning.\n"]
    pub fn randomization_salt(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.randomization_salt", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::ListRef<EvidentlyLaunch_GroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_monitors` after provisioning.\n"]
    pub fn metric_monitors(&self) -> terrars::ListRef<EvidentlyLaunch_MetricMonitorsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_monitors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduled_splits_config` after provisioning.\n"]
    pub fn scheduled_splits_config(&self) -> terrars::ListRef<EvidentlyLaunch_ScheduledSplitsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduled_splits_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EvidentlyLaunch_TimeoutsRef {
        EvidentlyLaunch_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunchDynamic {
    groups: std::option::Option<terrars::DynamicBlock<EvidentlyLaunch_Groups>>,
    metric_monitors: std::option::Option<terrars::DynamicBlock<EvidentlyLaunch_MetricMonitors>>,
    scheduled_splits_config: std::option::Option<terrars::DynamicBlock<EvidentlyLaunch_ScheduledSplitsConfig>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_Execution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ended_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    started_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyLaunch_Execution {
    #[doc = "Set the field `ended_time`.\n"]
    pub fn set_ended_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ended_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `started_time`.\n"]
    pub fn set_started_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.started_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_Execution {
    type O = terrars::BlockAssignable<EvidentlyLaunch_Execution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_Execution {}

impl BuildEvidentlyLaunch_Execution {
    pub fn build(self) -> EvidentlyLaunch_Execution {
        EvidentlyLaunch_Execution {
            ended_time: core::default::Default::default(),
            started_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_ExecutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_ExecutionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyLaunch_ExecutionRef {
        EvidentlyLaunch_ExecutionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_ExecutionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ended_time` after provisioning.\n"]
    pub fn ended_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ended_time", self.base))
    }

    #[doc = "Get a reference to the value of field `started_time` after provisioning.\n"]
    pub fn started_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_time", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_Groups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    feature: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    variation: terrars::PrimField<std::string::String>,
}

impl EvidentlyLaunch_Groups {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_Groups {
    type O = terrars::BlockAssignable<EvidentlyLaunch_Groups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_Groups {
    #[doc = ""]
    pub feature: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub variation: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyLaunch_Groups {
    pub fn build(self) -> EvidentlyLaunch_Groups {
        EvidentlyLaunch_Groups {
            description: core::default::Default::default(),
            feature: self.feature,
            name: self.name,
            variation: self.variation,
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_GroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_GroupsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyLaunch_GroupsRef {
        EvidentlyLaunch_GroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_GroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `feature` after provisioning.\n"]
    pub fn feature(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.feature", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `variation` after provisioning.\n"]
    pub fn variation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variation", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_MetricMonitors_MetricDefinition {
    entity_id_key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_pattern: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit_label: std::option::Option<terrars::PrimField<std::string::String>>,
    value_key: terrars::PrimField<std::string::String>,
}

impl EvidentlyLaunch_MetricMonitors_MetricDefinition {
    #[doc = "Set the field `event_pattern`.\n"]
    pub fn set_event_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event_pattern = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit_label`.\n"]
    pub fn set_unit_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit_label = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_MetricMonitors_MetricDefinition {
    type O = terrars::BlockAssignable<EvidentlyLaunch_MetricMonitors_MetricDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_MetricMonitors_MetricDefinition {
    #[doc = ""]
    pub entity_id_key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_key: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyLaunch_MetricMonitors_MetricDefinition {
    pub fn build(self) -> EvidentlyLaunch_MetricMonitors_MetricDefinition {
        EvidentlyLaunch_MetricMonitors_MetricDefinition {
            entity_id_key: self.entity_id_key,
            event_pattern: core::default::Default::default(),
            name: self.name,
            unit_label: core::default::Default::default(),
            value_key: self.value_key,
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_MetricMonitors_MetricDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_MetricMonitors_MetricDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EvidentlyLaunch_MetricMonitors_MetricDefinitionRef {
        EvidentlyLaunch_MetricMonitors_MetricDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_MetricMonitors_MetricDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id_key` after provisioning.\n"]
    pub fn entity_id_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id_key", self.base))
    }

    #[doc = "Get a reference to the value of field `event_pattern` after provisioning.\n"]
    pub fn event_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_label` after provisioning.\n"]
    pub fn unit_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_label", self.base))
    }

    #[doc = "Get a reference to the value of field `value_key` after provisioning.\n"]
    pub fn value_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_key", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunch_MetricMonitorsDynamic {
    metric_definition: std::option::Option<terrars::DynamicBlock<EvidentlyLaunch_MetricMonitors_MetricDefinition>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_MetricMonitors {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_definition: std::option::Option<std::vec::Vec<EvidentlyLaunch_MetricMonitors_MetricDefinition>>,
    dynamic: EvidentlyLaunch_MetricMonitorsDynamic,
}

impl EvidentlyLaunch_MetricMonitors {
    #[doc = "Set the field `metric_definition`.\n"]
    pub fn set_metric_definition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyLaunch_MetricMonitors_MetricDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metric_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metric_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_MetricMonitors {
    type O = terrars::BlockAssignable<EvidentlyLaunch_MetricMonitors>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_MetricMonitors {}

impl BuildEvidentlyLaunch_MetricMonitors {
    pub fn build(self) -> EvidentlyLaunch_MetricMonitors {
        EvidentlyLaunch_MetricMonitors {
            metric_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_MetricMonitorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_MetricMonitorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyLaunch_MetricMonitorsRef {
        EvidentlyLaunch_MetricMonitorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_MetricMonitorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_definition` after provisioning.\n"]
    pub fn metric_definition(&self) -> terrars::ListRef<EvidentlyLaunch_MetricMonitors_MetricDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metric_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides {
    evaluation_order: terrars::PrimField<f64>,
    segment: terrars::PrimField<std::string::String>,
    weights: terrars::RecField<terrars::PrimField<f64>>,
}

impl EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides { }

impl terrars::ToListMappable for EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides {
    type O = terrars::BlockAssignable<EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides {
    #[doc = ""]
    pub evaluation_order: terrars::PrimField<f64>,
    #[doc = ""]
    pub segment: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weights: terrars::RecField<terrars::PrimField<f64>>,
}

impl BuildEvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides {
    pub fn build(self) -> EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides {
        EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides {
            evaluation_order: self.evaluation_order,
            segment: self.segment,
            weights: self.weights,
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverridesRef {
        EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluation_order` after provisioning.\n"]
    pub fn evaluation_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_order", self.base))
    }

    #[doc = "Get a reference to the value of field `segment` after provisioning.\n"]
    pub fn segment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.segment", self.base))
    }

    #[doc = "Get a reference to the value of field `weights` after provisioning.\n"]
    pub fn weights(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.weights", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunch_ScheduledSplitsConfig_StepsDynamic {
    segment_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_ScheduledSplitsConfig_Steps {
    group_weights: terrars::RecField<terrars::PrimField<f64>>,
    start_time: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segment_overrides: std::option::Option<std::vec::Vec<EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides>>,
    dynamic: EvidentlyLaunch_ScheduledSplitsConfig_StepsDynamic,
}

impl EvidentlyLaunch_ScheduledSplitsConfig_Steps {
    #[doc = "Set the field `segment_overrides`.\n"]
    pub fn set_segment_overrides(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.segment_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.segment_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_ScheduledSplitsConfig_Steps {
    type O = terrars::BlockAssignable<EvidentlyLaunch_ScheduledSplitsConfig_Steps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_ScheduledSplitsConfig_Steps {
    #[doc = ""]
    pub group_weights: terrars::RecField<terrars::PrimField<f64>>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
}

impl BuildEvidentlyLaunch_ScheduledSplitsConfig_Steps {
    pub fn build(self) -> EvidentlyLaunch_ScheduledSplitsConfig_Steps {
        EvidentlyLaunch_ScheduledSplitsConfig_Steps {
            group_weights: self.group_weights,
            start_time: self.start_time,
            segment_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_ScheduledSplitsConfig_StepsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_ScheduledSplitsConfig_StepsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EvidentlyLaunch_ScheduledSplitsConfig_StepsRef {
        EvidentlyLaunch_ScheduledSplitsConfig_StepsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_ScheduledSplitsConfig_StepsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `group_weights` after provisioning.\n"]
    pub fn group_weights(&self) -> terrars::RecRef<terrars::PrimExpr<f64>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.group_weights", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `segment_overrides` after provisioning.\n"]
    pub fn segment_overrides(&self) -> terrars::ListRef<EvidentlyLaunch_ScheduledSplitsConfig_Steps_SegmentOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segment_overrides", self.base))
    }
}

#[derive(Serialize, Default)]
struct EvidentlyLaunch_ScheduledSplitsConfigDynamic {
    steps: std::option::Option<terrars::DynamicBlock<EvidentlyLaunch_ScheduledSplitsConfig_Steps>>,
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_ScheduledSplitsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    steps: std::option::Option<std::vec::Vec<EvidentlyLaunch_ScheduledSplitsConfig_Steps>>,
    dynamic: EvidentlyLaunch_ScheduledSplitsConfigDynamic,
}

impl EvidentlyLaunch_ScheduledSplitsConfig {
    #[doc = "Set the field `steps`.\n"]
    pub fn set_steps(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EvidentlyLaunch_ScheduledSplitsConfig_Steps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.steps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.steps = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_ScheduledSplitsConfig {
    type O = terrars::BlockAssignable<EvidentlyLaunch_ScheduledSplitsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_ScheduledSplitsConfig {}

impl BuildEvidentlyLaunch_ScheduledSplitsConfig {
    pub fn build(self) -> EvidentlyLaunch_ScheduledSplitsConfig {
        EvidentlyLaunch_ScheduledSplitsConfig {
            steps: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_ScheduledSplitsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_ScheduledSplitsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyLaunch_ScheduledSplitsConfigRef {
        EvidentlyLaunch_ScheduledSplitsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_ScheduledSplitsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `steps` after provisioning.\n"]
    pub fn steps(&self) -> terrars::ListRef<EvidentlyLaunch_ScheduledSplitsConfig_StepsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.steps", self.base))
    }
}

#[derive(Serialize)]
pub struct EvidentlyLaunch_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EvidentlyLaunch_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EvidentlyLaunch_Timeouts {
    type O = terrars::BlockAssignable<EvidentlyLaunch_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEvidentlyLaunch_Timeouts {}

impl BuildEvidentlyLaunch_Timeouts {
    pub fn build(self) -> EvidentlyLaunch_Timeouts {
        EvidentlyLaunch_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EvidentlyLaunch_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EvidentlyLaunch_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EvidentlyLaunch_TimeoutsRef {
        EvidentlyLaunch_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EvidentlyLaunch_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
