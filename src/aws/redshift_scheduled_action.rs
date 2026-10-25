#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RedshiftScheduledActionData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    schedule: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_action: std::option::Option<std::vec::Vec<RedshiftScheduledAction_TargetAction>>,
    dynamic: RedshiftScheduledActionDynamic,
}

struct RedshiftScheduledAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RedshiftScheduledActionData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledAction_TargetAction>>,
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn target_action(&self) -> terrars::ListRef<RedshiftScheduledAction_TargetActionRef> {
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
            data: std::cell::RefCell::new(RedshiftScheduledActionData_ {
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
                region: core::default::Default::default(),
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn target_action(&self) -> terrars::ListRef<RedshiftScheduledAction_TargetActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_action", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RedshiftScheduledActionDynamic {
    target_action: std::option::Option<terrars::DynamicBlock<RedshiftScheduledAction_TargetAction>>,
}

#[derive(Serialize)]
pub struct RedshiftScheduledAction_TargetAction_PauseCluster {
    cluster_identifier: terrars::PrimField<std::string::String>,
}

impl RedshiftScheduledAction_TargetAction_PauseCluster { }

impl terrars::ToListMappable for RedshiftScheduledAction_TargetAction_PauseCluster {
    type O = terrars::BlockAssignable<RedshiftScheduledAction_TargetAction_PauseCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledAction_TargetAction_PauseCluster {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledAction_TargetAction_PauseCluster {
    pub fn build(self) -> RedshiftScheduledAction_TargetAction_PauseCluster {
        RedshiftScheduledAction_TargetAction_PauseCluster { cluster_identifier: self.cluster_identifier }
    }
}

#[allow(dead_code)]
pub struct RedshiftScheduledAction_TargetAction_PauseClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledAction_TargetAction_PauseClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftScheduledAction_TargetAction_PauseClusterRef {
        RedshiftScheduledAction_TargetAction_PauseClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledAction_TargetAction_PauseClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct RedshiftScheduledAction_TargetAction_ResizeCluster {
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

impl RedshiftScheduledAction_TargetAction_ResizeCluster {
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

impl terrars::ToListMappable for RedshiftScheduledAction_TargetAction_ResizeCluster {
    type O = terrars::BlockAssignable<RedshiftScheduledAction_TargetAction_ResizeCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledAction_TargetAction_ResizeCluster {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledAction_TargetAction_ResizeCluster {
    pub fn build(self) -> RedshiftScheduledAction_TargetAction_ResizeCluster {
        RedshiftScheduledAction_TargetAction_ResizeCluster {
            classic: core::default::Default::default(),
            cluster_identifier: self.cluster_identifier,
            cluster_type: core::default::Default::default(),
            node_type: core::default::Default::default(),
            number_of_nodes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftScheduledAction_TargetAction_ResizeClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledAction_TargetAction_ResizeClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftScheduledAction_TargetAction_ResizeClusterRef {
        RedshiftScheduledAction_TargetAction_ResizeClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledAction_TargetAction_ResizeClusterRef {
    #[allow(dead_code)]
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
pub struct RedshiftScheduledAction_TargetAction_ResumeCluster {
    cluster_identifier: terrars::PrimField<std::string::String>,
}

impl RedshiftScheduledAction_TargetAction_ResumeCluster { }

impl terrars::ToListMappable for RedshiftScheduledAction_TargetAction_ResumeCluster {
    type O = terrars::BlockAssignable<RedshiftScheduledAction_TargetAction_ResumeCluster>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledAction_TargetAction_ResumeCluster {
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
}

impl BuildRedshiftScheduledAction_TargetAction_ResumeCluster {
    pub fn build(self) -> RedshiftScheduledAction_TargetAction_ResumeCluster {
        RedshiftScheduledAction_TargetAction_ResumeCluster { cluster_identifier: self.cluster_identifier }
    }
}

#[allow(dead_code)]
pub struct RedshiftScheduledAction_TargetAction_ResumeClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledAction_TargetAction_ResumeClusterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RedshiftScheduledAction_TargetAction_ResumeClusterRef {
        RedshiftScheduledAction_TargetAction_ResumeClusterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledAction_TargetAction_ResumeClusterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct RedshiftScheduledAction_TargetActionDynamic {
    pause_cluster: std::option::Option<terrars::DynamicBlock<RedshiftScheduledAction_TargetAction_PauseCluster>>,
    resize_cluster: std::option::Option<terrars::DynamicBlock<RedshiftScheduledAction_TargetAction_ResizeCluster>>,
    resume_cluster: std::option::Option<terrars::DynamicBlock<RedshiftScheduledAction_TargetAction_ResumeCluster>>,
}

#[derive(Serialize)]
pub struct RedshiftScheduledAction_TargetAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pause_cluster: std::option::Option<std::vec::Vec<RedshiftScheduledAction_TargetAction_PauseCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resize_cluster: std::option::Option<std::vec::Vec<RedshiftScheduledAction_TargetAction_ResizeCluster>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resume_cluster: std::option::Option<std::vec::Vec<RedshiftScheduledAction_TargetAction_ResumeCluster>>,
    dynamic: RedshiftScheduledAction_TargetActionDynamic,
}

impl RedshiftScheduledAction_TargetAction {
    #[doc = "Set the field `pause_cluster`.\n"]
    pub fn set_pause_cluster(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledAction_TargetAction_PauseCluster>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledAction_TargetAction_ResizeCluster>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<RedshiftScheduledAction_TargetAction_ResumeCluster>>,
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

impl terrars::ToListMappable for RedshiftScheduledAction_TargetAction {
    type O = terrars::BlockAssignable<RedshiftScheduledAction_TargetAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRedshiftScheduledAction_TargetAction {}

impl BuildRedshiftScheduledAction_TargetAction {
    pub fn build(self) -> RedshiftScheduledAction_TargetAction {
        RedshiftScheduledAction_TargetAction {
            pause_cluster: core::default::Default::default(),
            resize_cluster: core::default::Default::default(),
            resume_cluster: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RedshiftScheduledAction_TargetActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RedshiftScheduledAction_TargetActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RedshiftScheduledAction_TargetActionRef {
        RedshiftScheduledAction_TargetActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RedshiftScheduledAction_TargetActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `pause_cluster` after provisioning.\n"]
    pub fn pause_cluster(&self) -> terrars::ListRef<RedshiftScheduledAction_TargetAction_PauseClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.pause_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `resize_cluster` after provisioning.\n"]
    pub fn resize_cluster(&self) -> terrars::ListRef<RedshiftScheduledAction_TargetAction_ResizeClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resize_cluster", self.base))
    }

    #[doc = "Get a reference to the value of field `resume_cluster` after provisioning.\n"]
    pub fn resume_cluster(&self) -> terrars::ListRef<RedshiftScheduledAction_TargetAction_ResumeClusterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resume_cluster", self.base))
    }
}
