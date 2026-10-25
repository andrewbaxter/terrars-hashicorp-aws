#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VpclatticeListenerRuleData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    listener_identifier: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<VpclatticeListenerRule_Timeouts>,
    dynamic: VpclatticeListenerRuleDynamic,
}

struct VpclatticeListenerRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VpclatticeListenerRuleData_>,
}

#[derive(Clone)]
pub struct VpclatticeListenerRule(std::rc::Rc<VpclatticeListenerRule_>);

impl VpclatticeListenerRule {
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(self, v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Match>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<VpclatticeListenerRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_identifier` after provisioning.\n"]
    pub fn listener_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<VpclatticeListenerRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<VpclatticeListenerRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeListenerRule_TimeoutsRef {
        VpclatticeListenerRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for VpclatticeListenerRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VpclatticeListenerRule { }

impl terrars::ToListMappable for VpclatticeListenerRule {
    type O = terrars::ListRef<VpclatticeListenerRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VpclatticeListenerRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_vpclattice_listener_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVpclatticeListenerRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub listener_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub service_identifier: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeListenerRule {
    pub fn build(self, stack: &mut terrars::Stack) -> VpclatticeListenerRule {
        let out = VpclatticeListenerRule(std::rc::Rc::new(VpclatticeListenerRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VpclatticeListenerRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                listener_identifier: self.listener_identifier,
                name: self.name,
                priority: self.priority,
                region: core::default::Default::default(),
                service_identifier: self.service_identifier,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action: core::default::Default::default(),
                match_: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VpclatticeListenerRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VpclatticeListenerRuleRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `listener_identifier` after provisioning.\n"]
    pub fn listener_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.listener_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_identifier` after provisioning.\n"]
    pub fn service_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<VpclatticeListenerRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<VpclatticeListenerRule_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> VpclatticeListenerRule_TimeoutsRef {
        VpclatticeListenerRule_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Action>>,
    match_: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Match>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Action_FixedResponse {
    status_code: terrars::PrimField<f64>,
}

impl VpclatticeListenerRule_Action_FixedResponse { }

impl terrars::ToListMappable for VpclatticeListenerRule_Action_FixedResponse {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Action_FixedResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Action_FixedResponse {
    #[doc = ""]
    pub status_code: terrars::PrimField<f64>,
}

impl BuildVpclatticeListenerRule_Action_FixedResponse {
    pub fn build(self) -> VpclatticeListenerRule_Action_FixedResponse {
        VpclatticeListenerRule_Action_FixedResponse { status_code: self.status_code }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Action_FixedResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Action_FixedResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeListenerRule_Action_FixedResponseRef {
        VpclatticeListenerRule_Action_FixedResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Action_FixedResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Action_Forward_TargetGroups {
    target_group_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl VpclatticeListenerRule_Action_Forward_TargetGroups {
    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Action_Forward_TargetGroups {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Action_Forward_TargetGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Action_Forward_TargetGroups {
    #[doc = ""]
    pub target_group_identifier: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeListenerRule_Action_Forward_TargetGroups {
    pub fn build(self) -> VpclatticeListenerRule_Action_Forward_TargetGroups {
        VpclatticeListenerRule_Action_Forward_TargetGroups {
            target_group_identifier: self.target_group_identifier,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Action_Forward_TargetGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Action_Forward_TargetGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeListenerRule_Action_Forward_TargetGroupsRef {
        VpclatticeListenerRule_Action_Forward_TargetGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Action_Forward_TargetGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_group_identifier` after provisioning.\n"]
    pub fn target_group_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRule_Action_ForwardDynamic {
    target_groups: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Action_Forward_TargetGroups>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Action_Forward {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_groups: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Action_Forward_TargetGroups>>,
    dynamic: VpclatticeListenerRule_Action_ForwardDynamic,
}

impl VpclatticeListenerRule_Action_Forward {
    #[doc = "Set the field `target_groups`.\n"]
    pub fn set_target_groups(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Action_Forward_TargetGroups>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_groups = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_groups = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Action_Forward {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Action_Forward>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Action_Forward {}

impl BuildVpclatticeListenerRule_Action_Forward {
    pub fn build(self) -> VpclatticeListenerRule_Action_Forward {
        VpclatticeListenerRule_Action_Forward {
            target_groups: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Action_ForwardRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Action_ForwardRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpclatticeListenerRule_Action_ForwardRef {
        VpclatticeListenerRule_Action_ForwardRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Action_ForwardRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_groups` after provisioning.\n"]
    pub fn target_groups(&self) -> terrars::ListRef<VpclatticeListenerRule_Action_Forward_TargetGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_groups", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRule_ActionDynamic {
    fixed_response: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Action_FixedResponse>>,
    forward: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Action_Forward>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fixed_response: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Action_FixedResponse>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forward: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Action_Forward>>,
    dynamic: VpclatticeListenerRule_ActionDynamic,
}

impl VpclatticeListenerRule_Action {
    #[doc = "Set the field `fixed_response`.\n"]
    pub fn set_fixed_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Action_FixedResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fixed_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fixed_response = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forward`.\n"]
    pub fn set_forward(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Action_Forward>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forward = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forward = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Action {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Action {}

impl BuildVpclatticeListenerRule_Action {
    pub fn build(self) -> VpclatticeListenerRule_Action {
        VpclatticeListenerRule_Action {
            fixed_response: core::default::Default::default(),
            forward: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpclatticeListenerRule_ActionRef {
        VpclatticeListenerRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fixed_response` after provisioning.\n"]
    pub fn fixed_response(&self) -> terrars::ListRef<VpclatticeListenerRule_Action_FixedResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fixed_response", self.base))
    }

    #[doc = "Get a reference to the value of field `forward` after provisioning.\n"]
    pub fn forward(&self) -> terrars::ListRef<VpclatticeListenerRule_Action_ForwardRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    contains: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {
    #[doc = "Set the field `contains`.\n"]
    pub fn set_contains(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {}

impl BuildVpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {
    pub fn build(self) -> VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {
        VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match {
            contains: core::default::Default::default(),
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_MatchRef {
        VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `contains` after provisioning.\n"]
    pub fn contains(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.contains", self.base))
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesDynamic {
    match_: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_sensitive: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match>>,
    dynamic: VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesDynamic,
}

impl VpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
    #[doc = "Set the field `case_sensitive`.\n"]
    pub fn set_case_sensitive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.case_sensitive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildVpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
    pub fn build(self) -> VpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
        VpclatticeListenerRule_Match_HttpMatch_HeaderMatches {
            case_sensitive: core::default::Default::default(),
            name: self.name,
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesRef {
        VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {}

impl BuildVpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {
    pub fn build(self) -> VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {
        VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Match_HttpMatch_PathMatch_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Match_HttpMatch_PathMatch_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeListenerRule_Match_HttpMatch_PathMatch_MatchRef {
        VpclatticeListenerRule_Match_HttpMatch_PathMatch_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Match_HttpMatch_PathMatch_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRule_Match_HttpMatch_PathMatchDynamic {
    match_: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Match_HttpMatch_PathMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_sensitive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match>>,
    dynamic: VpclatticeListenerRule_Match_HttpMatch_PathMatchDynamic,
}

impl VpclatticeListenerRule_Match_HttpMatch_PathMatch {
    #[doc = "Set the field `case_sensitive`.\n"]
    pub fn set_case_sensitive(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.case_sensitive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_PathMatch_Match>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_ = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Match_HttpMatch_PathMatch {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_PathMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Match_HttpMatch_PathMatch {}

impl BuildVpclatticeListenerRule_Match_HttpMatch_PathMatch {
    pub fn build(self) -> VpclatticeListenerRule_Match_HttpMatch_PathMatch {
        VpclatticeListenerRule_Match_HttpMatch_PathMatch {
            case_sensitive: core::default::Default::default(),
            match_: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Match_HttpMatch_PathMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Match_HttpMatch_PathMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VpclatticeListenerRule_Match_HttpMatch_PathMatchRef {
        VpclatticeListenerRule_Match_HttpMatch_PathMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Match_HttpMatch_PathMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `case_sensitive` after provisioning.\n"]
    pub fn case_sensitive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitive", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<VpclatticeListenerRule_Match_HttpMatch_PathMatch_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRule_Match_HttpMatchDynamic {
    header_matches: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches>>,
    path_match: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Match_HttpMatch_PathMatch>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Match_HttpMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_matches: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path_match: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Match_HttpMatch_PathMatch>>,
    dynamic: VpclatticeListenerRule_Match_HttpMatchDynamic,
}

impl VpclatticeListenerRule_Match_HttpMatch {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `header_matches`.\n"]
    pub fn set_header_matches(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_HeaderMatches>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_matches = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_matches = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `path_match`.\n"]
    pub fn set_path_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch_PathMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.path_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.path_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Match_HttpMatch {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Match_HttpMatch {}

impl BuildVpclatticeListenerRule_Match_HttpMatch {
    pub fn build(self) -> VpclatticeListenerRule_Match_HttpMatch {
        VpclatticeListenerRule_Match_HttpMatch {
            method: core::default::Default::default(),
            header_matches: core::default::Default::default(),
            path_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_Match_HttpMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_Match_HttpMatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpclatticeListenerRule_Match_HttpMatchRef {
        VpclatticeListenerRule_Match_HttpMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_Match_HttpMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `header_matches` after provisioning.\n"]
    pub fn header_matches(&self) -> terrars::ListRef<VpclatticeListenerRule_Match_HttpMatch_HeaderMatchesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_matches", self.base))
    }

    #[doc = "Get a reference to the value of field `path_match` after provisioning.\n"]
    pub fn path_match(&self) -> terrars::ListRef<VpclatticeListenerRule_Match_HttpMatch_PathMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct VpclatticeListenerRule_MatchDynamic {
    http_match: std::option::Option<terrars::DynamicBlock<VpclatticeListenerRule_Match_HttpMatch>>,
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_match: std::option::Option<std::vec::Vec<VpclatticeListenerRule_Match_HttpMatch>>,
    dynamic: VpclatticeListenerRule_MatchDynamic,
}

impl VpclatticeListenerRule_Match {
    #[doc = "Set the field `http_match`.\n"]
    pub fn set_http_match(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VpclatticeListenerRule_Match_HttpMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VpclatticeListenerRule_Match {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Match {}

impl BuildVpclatticeListenerRule_Match {
    pub fn build(self) -> VpclatticeListenerRule_Match {
        VpclatticeListenerRule_Match {
            http_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpclatticeListenerRule_MatchRef {
        VpclatticeListenerRule_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_match` after provisioning.\n"]
    pub fn http_match(&self) -> terrars::ListRef<VpclatticeListenerRule_Match_HttpMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_match", self.base))
    }
}

#[derive(Serialize)]
pub struct VpclatticeListenerRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl VpclatticeListenerRule_Timeouts {
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

impl terrars::ToListMappable for VpclatticeListenerRule_Timeouts {
    type O = terrars::BlockAssignable<VpclatticeListenerRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVpclatticeListenerRule_Timeouts {}

impl BuildVpclatticeListenerRule_Timeouts {
    pub fn build(self) -> VpclatticeListenerRule_Timeouts {
        VpclatticeListenerRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VpclatticeListenerRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VpclatticeListenerRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VpclatticeListenerRule_TimeoutsRef {
        VpclatticeListenerRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VpclatticeListenerRule_TimeoutsRef {
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
