use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftScheduledActionData {
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
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_time: std::option::Option<terrars::PrimField<std::string::String>>,
    iam_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    schedule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_action: std::option::Option<std::vec::Vec<RedshiftScheduledActionTargetActionEl>>,
    dynamic: RedshiftScheduledActionDynamic,
}

struct RedshiftScheduledAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftScheduledActionData>,
}

#[derive(Clone)]
pub struct RedshiftScheduledAction(std::rc::Rc<RedshiftScheduledAction_>);

impl RedshiftScheduledAction {
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

    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `end_time`.\n"]
    pub fn set_end_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().end_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_action`.\n"]
    pub fn set_target_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledActionTargetActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().target_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.target_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_action` after provisioning.\n"]
    pub fn target_action(&self) -> terrars::ListRef<RedshiftScheduledActionTargetActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }
}

impl terrars::Referable for RedshiftScheduledAction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RedshiftScheduledAction { }

impl terrars::ToListMappable for RedshiftScheduledAction {
    type O = terrars::ListRef<RedshiftScheduledActionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RedshiftScheduledAction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_redshift_scheduled_action".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRedshiftScheduledAction {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub iam_role: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub schedule: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledAction {
    pub fn build(self, stack: &mut terrars::Stack) -> RedshiftScheduledAction {
        let out = RedshiftScheduledAction(std::rc::Rc::new(RedshiftScheduledAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RedshiftScheduledActionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enable: core::default::Default::default(),
                end_time: core::default::Default::default(),
                iam_role: self.iam_role,
                id: core::default::Default::default(),
                name: self.name,
                schedule: self.schedule,
                start_time: core::default::Default::default(),
                target_action: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RedshiftScheduledActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RedshiftScheduledActionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_time` after provisioning.\n"]
    pub fn end_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.schedule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_action` after provisioning.\n"]
    pub fn target_action(&self) -> terrars::ListRef<RedshiftScheduledActionTargetActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionElPauseClusterEl {
    cluster_identifier: terrars::PrimField<std::string::String>,
}

impl RedshiftScheduledActionTargetActionElPauseClusterEl { }

impl terrars::ToListMappable for RedshiftScheduledActionTargetActionElPauseClusterEl {
    type O = terrars::BlockAssignable<RedshiftScheduledActionTargetActionElPauseClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionElPauseClusterEl {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledActionTargetActionElPauseClusterEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionElPauseClusterEl {
        RedshiftScheduledActionTargetActionElPauseClusterEl { cluster_identifier: self.cluster_identifier }
    }
}

pub struct RedshiftScheduledActionTargetActionElPauseClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledActionTargetActionElPauseClusterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftScheduledActionTargetActionElPauseClusterElRef {
        RedshiftScheduledActionTargetActionElPauseClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElPauseClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionElResizeClusterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classic: std::option::Option<terrars::PrimField<bool>>,
    cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cluster_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    node_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_nodes: std::option::Option<terrars::PrimField<f64>>,
}

impl RedshiftScheduledActionTargetActionElResizeClusterEl {
    #[doc = "Set the field `classic`.\n"]
    pub fn set_classic(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.classic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cluster_type`.\n"]
    pub fn set_cluster_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cluster_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `node_type`.\n"]
    pub fn set_node_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.node_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_nodes`.\n"]
    pub fn set_number_of_nodes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_nodes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RedshiftScheduledActionTargetActionElResizeClusterEl {
    type O = terrars::BlockAssignable<RedshiftScheduledActionTargetActionElResizeClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionElResizeClusterEl {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledActionTargetActionElResizeClusterEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionElResizeClusterEl {
        RedshiftScheduledActionTargetActionElResizeClusterEl {
            classic: core::default::Default::default(),
            cluster_identifier: self.cluster_identifier,
            cluster_type: core::default::Default::default(),
            node_type: core::default::Default::default(),
            number_of_nodes: core::default::Default::default(),
        }
    }
}

pub struct RedshiftScheduledActionTargetActionElResizeClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledActionTargetActionElResizeClusterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftScheduledActionTargetActionElResizeClusterElRef {
        RedshiftScheduledActionTargetActionElResizeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElResizeClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classic` after provisioning.\n"]
    pub fn classic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classic", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `cluster_type` after provisioning.\n"]
    pub fn cluster_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_type", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_nodes` after provisioning.\n"]
    pub fn number_of_nodes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_nodes", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionElResumeClusterEl {
    cluster_identifier: terrars::PrimField<std::string::String>,
}

impl RedshiftScheduledActionTargetActionElResumeClusterEl { }

impl terrars::ToListMappable for RedshiftScheduledActionTargetActionElResumeClusterEl {
    type O = terrars::BlockAssignable<RedshiftScheduledActionTargetActionElResumeClusterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionElResumeClusterEl {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledActionTargetActionElResumeClusterEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionElResumeClusterEl {
        RedshiftScheduledActionTargetActionElResumeClusterEl { cluster_identifier: self.cluster_identifier }
    }
}

pub struct RedshiftScheduledActionTargetActionElResumeClusterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledActionTargetActionElResumeClusterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftScheduledActionTargetActionElResumeClusterElRef {
        RedshiftScheduledActionTargetActionElResumeClusterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElResumeClusterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftScheduledActionTargetActionElDynamic {
    pause_cluster: std::option::Option<terrars::DynamicBlock<RedshiftScheduledActionTargetActionElPauseClusterEl>>,
    resize_cluster: std::option::Option<terrars::DynamicBlock<RedshiftScheduledActionTargetActionElResizeClusterEl>>,
    resume_cluster: std
    ::option
    ::Option<terrars::DynamicBlock<RedshiftScheduledActionTargetActionElResumeClusterEl>>,
}

#[derive(Serialize)]
pub struct RedshiftScheduledActionTargetActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pause_cluster: std::option::Option<std::vec::Vec<RedshiftScheduledActionTargetActionElPauseClusterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resize_cluster: std::option::Option<std::vec::Vec<RedshiftScheduledActionTargetActionElResizeClusterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resume_cluster: std::option::Option<std::vec::Vec<RedshiftScheduledActionTargetActionElResumeClusterEl>>,
    dynamic: RedshiftScheduledActionTargetActionElDynamic,
}

impl RedshiftScheduledActionTargetActionEl {
    #[doc = "Set the field `pause_cluster`.\n"]
    pub fn set_pause_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledActionTargetActionElPauseClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pause_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pause_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resize_cluster`.\n"]
    pub fn set_resize_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledActionTargetActionElResizeClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resize_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resize_cluster = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resume_cluster`.\n"]
    pub fn set_resume_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledActionTargetActionElResumeClusterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resume_cluster = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resume_cluster = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RedshiftScheduledActionTargetActionEl {
    type O = terrars::BlockAssignable<RedshiftScheduledActionTargetActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledActionTargetActionEl {}

impl BuildRedshiftScheduledActionTargetActionEl {
    pub fn build(self) -> RedshiftScheduledActionTargetActionEl {
        RedshiftScheduledActionTargetActionEl {
            pause_cluster: core::default::Default::default(),
            resize_cluster: core::default::Default::default(),
            resume_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RedshiftScheduledActionTargetActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledActionTargetActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftScheduledActionTargetActionElRef {
        RedshiftScheduledActionTargetActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledActionTargetActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pause_cluster` after provisioning.\n"]
    pub fn pause_cluster(&self) -> terrars::ListRef<RedshiftScheduledActionTargetActionElPauseClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pause_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `resize_cluster` after provisioning.\n"]
    pub fn resize_cluster(&self) -> terrars::ListRef<RedshiftScheduledActionTargetActionElResizeClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resize_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `resume_cluster` after provisioning.\n"]
    pub fn resume_cluster(&self) -> terrars::ListRef<RedshiftScheduledActionTargetActionElResumeClusterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resume_cluster", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftScheduledActionDynamic {
    target_action: std::option::Option<terrars::DynamicBlock<RedshiftScheduledActionTargetActionEl>>,
}
