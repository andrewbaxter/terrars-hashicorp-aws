#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreGatewayRuleData_ {
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
    gateway_identifier: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreGatewayRule_Timeouts>,
    dynamic: BedrockagentcoreGatewayRuleDynamic,
}

struct BedrockagentcoreGatewayRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreGatewayRuleData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreGatewayRule(std::rc::Rc<BedrockagentcoreGatewayRule_>);

impl BedrockagentcoreGatewayRule {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action>>,
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

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreGatewayRule_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_identifier` after provisioning.\n"]
    pub fn gateway_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_SystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayRule_TimeoutsRef {
        BedrockagentcoreGatewayRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreGatewayRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreGatewayRule { }

impl terrars::ToListMappable for BedrockagentcoreGatewayRule {
    type O = terrars::ListRef<BedrockagentcoreGatewayRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreGatewayRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_gateway_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreGatewayRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub gateway_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreGatewayRule {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreGatewayRule {
        let out = BedrockagentcoreGatewayRule(std::rc::Rc::new(BedrockagentcoreGatewayRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreGatewayRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                gateway_identifier: self.gateway_identifier,
                priority: self.priority,
                region: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreGatewayRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreGatewayRuleRef {
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

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_identifier` after provisioning.\n"]
    pub fn gateway_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_identifier", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `system` after provisioning.\n"]
    pub fn system(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_SystemRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreGatewayRule_TimeoutsRef {
        BedrockagentcoreGatewayRule_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action>>,
    condition: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Condition>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_System {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_by: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayRule_System {
    #[doc = "Set the field `managed_by`.\n"]
    pub fn set_managed_by(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.managed_by = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_System {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_System>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_System {}

impl BuildBedrockagentcoreGatewayRule_System {
    pub fn build(self) -> BedrockagentcoreGatewayRule_System {
        BedrockagentcoreGatewayRule_System { managed_by: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_SystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_SystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreGatewayRule_SystemRef {
        BedrockagentcoreGatewayRule_SystemRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_SystemRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `managed_by` after provisioning.\n"]
    pub fn managed_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_by", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride {
    bundle_arn: terrars::PrimField<std::string::String>,
    bundle_version: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride { }

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride {
    #[doc = ""]
    pub bundle_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bundle_version: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride {
            bundle_arn: self.bundle_arn,
            bundle_version: self.bundle_version,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverrideRef {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundle_arn` after provisioning.\n"]
    pub fn bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `bundle_version` after provisioning.\n"]
    pub fn bundle_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_version", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle {
    bundle_arn: terrars::PrimField<std::string::String>,
    bundle_version: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle { }

impl terrars
::ToListMappable for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle {
    #[doc = ""]
    pub bundle_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bundle_version: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle {
    pub fn build(
        self,
    ) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle {
            bundle_arn: self.bundle_arn,
            bundle_version: self.bundle_version,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundleRef {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bundle_arn` after provisioning.\n"]
    pub fn bundle_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `bundle_version` after provisioning.\n"]
    pub fn bundle_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_version", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitDynamic {
    configuration_bundle: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_bundle: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle,
        >,
    >,
    dynamic: BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitDynamic,
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_bundle`.\n"]
    pub fn set_configuration_bundle(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundle,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configuration_bundle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration_bundle = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
    type O =
        terrars
        ::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit {
            description: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: self.name,
            weight: self.weight,
            configuration_bundle: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitRef {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_bundle` after provisioning.\n"]
    pub fn configuration_bundle(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit_ConfigurationBundleRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_bundle", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideDynamic {
    traffic_split: std
    ::option
    ::Option<
        terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_split: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit>>,
    dynamic: BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideDynamic,
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {
    #[doc = "Set the field `traffic_split`.\n"]
    pub fn set_traffic_split(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplit,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.traffic_split = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.traffic_split = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {}

impl BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride {
            traffic_split: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideRef {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `traffic_split` after provisioning.\n"]
    pub fn traffic_split(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride_TrafficSplitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_split", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Action_ConfigurationBundleDynamic {
    static_override: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride>>,
    weighted_override: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundle {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_override: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_override: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride>>,
    dynamic: BedrockagentcoreGatewayRule_Action_ConfigurationBundleDynamic,
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundle {
    #[doc = "Set the field `static_override`.\n"]
    pub fn set_static_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weighted_override`.\n"]
    pub fn set_weighted_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weighted_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_ConfigurationBundle {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle {}

impl BuildBedrockagentcoreGatewayRule_Action_ConfigurationBundle {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundle {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundle {
            static_override: core::default::Default::default(),
            weighted_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_ConfigurationBundleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_ConfigurationBundleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_ConfigurationBundleRef {
        BedrockagentcoreGatewayRule_Action_ConfigurationBundleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_ConfigurationBundleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_override` after provisioning.\n"]
    pub fn static_override(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_StaticOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_override", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_override` after provisioning.\n"]
    pub fn weighted_override(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_ConfigurationBundle_WeightedOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_override", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute {
    target_name: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute { }

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute {
    #[doc = ""]
    pub target_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute {
        BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute { target_name: self.target_name }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRouteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRouteRef {
        BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_name` after provisioning.\n"]
    pub fn target_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    target_name: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
        BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit {
            description: core::default::Default::default(),
            metadata: core::default::Default::default(),
            name: self.name,
            target_name: self.target_name,
            weight: self.weight,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplitRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplitRef {
        BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_name` after provisioning.\n"]
    pub fn target_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_name", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteDynamic {
    traffic_split: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_split: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit>>,
    dynamic: BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteDynamic,
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {
    #[doc = "Set the field `traffic_split`.\n"]
    pub fn set_traffic_split(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplit>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.traffic_split = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.traffic_split = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {}

impl BuildBedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {
        BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute {
            traffic_split: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteRef {
        BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `traffic_split` after provisioning.\n"]
    pub fn traffic_split(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute_TrafficSplitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_split", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Action_RouteToTargetDynamic {
    static_route: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute>>,
    weighted_route: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_route: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_route: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute>>,
    dynamic: BedrockagentcoreGatewayRule_Action_RouteToTargetDynamic,
}

impl BedrockagentcoreGatewayRule_Action_RouteToTarget {
    #[doc = "Set the field `static_route`.\n"]
    pub fn set_static_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRoute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_route = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `weighted_route`.\n"]
    pub fn set_weighted_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRoute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.weighted_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.weighted_route = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action_RouteToTarget {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action_RouteToTarget {}

impl BuildBedrockagentcoreGatewayRule_Action_RouteToTarget {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action_RouteToTarget {
        BedrockagentcoreGatewayRule_Action_RouteToTarget {
            static_route: core::default::Default::default(),
            weighted_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Action_RouteToTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Action_RouteToTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Action_RouteToTargetRef {
        BedrockagentcoreGatewayRule_Action_RouteToTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Action_RouteToTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_route` after provisioning.\n"]
    pub fn static_route(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_RouteToTarget_StaticRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_route", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_route` after provisioning.\n"]
    pub fn weighted_route(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_RouteToTarget_WeightedRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.weighted_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_ActionDynamic {
    configuration_bundle: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_ConfigurationBundle>>,
    route_to_target: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Action_RouteToTarget>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_bundle: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_ConfigurationBundle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_to_target: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Action_RouteToTarget>>,
    dynamic: BedrockagentcoreGatewayRule_ActionDynamic,
}

impl BedrockagentcoreGatewayRule_Action {
    #[doc = "Set the field `configuration_bundle`.\n"]
    pub fn set_configuration_bundle(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_ConfigurationBundle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configuration_bundle = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configuration_bundle = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `route_to_target`.\n"]
    pub fn set_route_to_target(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action_RouteToTarget>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.route_to_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.route_to_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Action {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Action {}

impl BuildBedrockagentcoreGatewayRule_Action {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Action {
        BedrockagentcoreGatewayRule_Action {
            configuration_bundle: core::default::Default::default(),
            route_to_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreGatewayRule_ActionRef {
        BedrockagentcoreGatewayRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration_bundle` after provisioning.\n"]
    pub fn configuration_bundle(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_ConfigurationBundleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.configuration_bundle", self.base))
    }

    #[doc = "Get a reference to the value of field `route_to_target` after provisioning.\n"]
    pub fn route_to_target(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Action_RouteToTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_to_target", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPaths {
    any_of: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayRule_Condition_MatchPaths { }

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Condition_MatchPaths {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Condition_MatchPaths {
    #[doc = ""]
    pub any_of: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreGatewayRule_Condition_MatchPaths {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Condition_MatchPaths {
        BedrockagentcoreGatewayRule_Condition_MatchPaths { any_of: self.any_of }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Condition_MatchPathsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Condition_MatchPathsRef {
        BedrockagentcoreGatewayRule_Condition_MatchPathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Condition_MatchPathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operator: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
    #[doc = "Set the field `operator`.\n"]
    pub fn set_operator(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operator = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
        BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal {
            arn: self.arn,
            operator: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipalRef {
        BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfDynamic {
    iam_principal: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_principal: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal>>,
    dynamic: BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfDynamic,
}

impl BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {
    #[doc = "Set the field `iam_principal`.\n"]
    pub fn set_iam_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipal>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iam_principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_principal = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {}

impl BuildBedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {
        BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf {
            iam_principal: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfRef {
        BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_principal` after provisioning.\n"]
    pub fn iam_principal(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf_IamPrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_principal", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_Condition_MatchPrincipalsDynamic {
    any_of: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPrincipals {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf>>,
    dynamic: BedrockagentcoreGatewayRule_Condition_MatchPrincipalsDynamic,
}

impl BedrockagentcoreGatewayRule_Condition_MatchPrincipals {
    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOf>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.any_of = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.any_of = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Condition_MatchPrincipals {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPrincipals>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Condition_MatchPrincipals {}

impl BuildBedrockagentcoreGatewayRule_Condition_MatchPrincipals {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Condition_MatchPrincipals {
        BedrockagentcoreGatewayRule_Condition_MatchPrincipals {
            any_of: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_Condition_MatchPrincipalsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_Condition_MatchPrincipalsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreGatewayRule_Condition_MatchPrincipalsRef {
        BedrockagentcoreGatewayRule_Condition_MatchPrincipalsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_Condition_MatchPrincipalsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Condition_MatchPrincipals_AnyOfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreGatewayRule_ConditionDynamic {
    match_paths: std::option::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Condition_MatchPaths>>,
    match_principals: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreGatewayRule_Condition_MatchPrincipals>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_paths: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Condition_MatchPaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_principals: std::option::Option<std::vec::Vec<BedrockagentcoreGatewayRule_Condition_MatchPrincipals>>,
    dynamic: BedrockagentcoreGatewayRule_ConditionDynamic,
}

impl BedrockagentcoreGatewayRule_Condition {
    #[doc = "Set the field `match_paths`.\n"]
    pub fn set_match_paths(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPaths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_paths = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_principals`.\n"]
    pub fn set_match_principals(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition_MatchPrincipals>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_principals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_principals = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Condition {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Condition {}

impl BuildBedrockagentcoreGatewayRule_Condition {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Condition {
        BedrockagentcoreGatewayRule_Condition {
            match_paths: core::default::Default::default(),
            match_principals: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreGatewayRule_ConditionRef {
        BedrockagentcoreGatewayRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_paths` after provisioning.\n"]
    pub fn match_paths(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Condition_MatchPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `match_principals` after provisioning.\n"]
    pub fn match_principals(&self) -> terrars::ListRef<BedrockagentcoreGatewayRule_Condition_MatchPrincipalsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_principals", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreGatewayRule_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreGatewayRule_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreGatewayRule_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreGatewayRule_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreGatewayRule_Timeouts {}

impl BuildBedrockagentcoreGatewayRule_Timeouts {
    pub fn build(self) -> BedrockagentcoreGatewayRule_Timeouts {
        BedrockagentcoreGatewayRule_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreGatewayRule_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreGatewayRule_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreGatewayRule_TimeoutsRef {
        BedrockagentcoreGatewayRule_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreGatewayRule_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
