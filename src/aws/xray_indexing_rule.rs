#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct XrayIndexingRuleData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<XrayIndexingRule_Rule>>,
    dynamic: XrayIndexingRuleDynamic,
}

struct XrayIndexingRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<XrayIndexingRuleData_>,
}

#[derive(Clone)]
pub struct XrayIndexingRule(std::rc::Rc<XrayIndexingRule_>);

impl XrayIndexingRule {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<XrayIndexingRule_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<XrayIndexingRule_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for XrayIndexingRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for XrayIndexingRule { }

impl terrars::ToListMappable for XrayIndexingRule {
    type O = terrars::ListRef<XrayIndexingRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for XrayIndexingRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_xray_indexing_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildXrayIndexingRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildXrayIndexingRule {
    pub fn build(self, stack: &mut terrars::Stack) -> XrayIndexingRule {
        let out = XrayIndexingRule(std::rc::Rc::new(XrayIndexingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(XrayIndexingRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct XrayIndexingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for XrayIndexingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl XrayIndexingRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<XrayIndexingRule_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct XrayIndexingRuleDynamic {
    rule: std::option::Option<terrars::DynamicBlock<XrayIndexingRule_Rule>>,
}

#[derive(Serialize)]
pub struct XrayIndexingRule_Rule_Probabilistic {
    desired_sampling_percentage: terrars::PrimField<f64>,
}

impl XrayIndexingRule_Rule_Probabilistic { }

impl terrars::ToListMappable for XrayIndexingRule_Rule_Probabilistic {
    type O = terrars::BlockAssignable<XrayIndexingRule_Rule_Probabilistic>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildXrayIndexingRule_Rule_Probabilistic {
    #[doc = ""]
    pub desired_sampling_percentage: terrars::PrimField<f64>,
}

impl BuildXrayIndexingRule_Rule_Probabilistic {
    pub fn build(self) -> XrayIndexingRule_Rule_Probabilistic {
        XrayIndexingRule_Rule_Probabilistic { desired_sampling_percentage: self.desired_sampling_percentage }
    }
}

#[allow(dead_code)]
pub struct XrayIndexingRule_Rule_ProbabilisticRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for XrayIndexingRule_Rule_ProbabilisticRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> XrayIndexingRule_Rule_ProbabilisticRef {
        XrayIndexingRule_Rule_ProbabilisticRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl XrayIndexingRule_Rule_ProbabilisticRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actual_sampling_percentage` after provisioning.\n"]
    pub fn actual_sampling_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actual_sampling_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_sampling_percentage` after provisioning.\n"]
    pub fn desired_sampling_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_sampling_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct XrayIndexingRule_RuleDynamic {
    probabilistic: std::option::Option<terrars::DynamicBlock<XrayIndexingRule_Rule_Probabilistic>>,
}

#[derive(Serialize)]
pub struct XrayIndexingRule_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    probabilistic: std::option::Option<std::vec::Vec<XrayIndexingRule_Rule_Probabilistic>>,
    dynamic: XrayIndexingRule_RuleDynamic,
}

impl XrayIndexingRule_Rule {
    #[doc = "Set the field `probabilistic`.\n"]
    pub fn set_probabilistic(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<XrayIndexingRule_Rule_Probabilistic>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.probabilistic = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.probabilistic = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for XrayIndexingRule_Rule {
    type O = terrars::BlockAssignable<XrayIndexingRule_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildXrayIndexingRule_Rule {}

impl BuildXrayIndexingRule_Rule {
    pub fn build(self) -> XrayIndexingRule_Rule {
        XrayIndexingRule_Rule {
            probabilistic: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct XrayIndexingRule_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for XrayIndexingRule_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> XrayIndexingRule_RuleRef {
        XrayIndexingRule_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl XrayIndexingRule_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `probabilistic` after provisioning.\n"]
    pub fn probabilistic(&self) -> terrars::ListRef<XrayIndexingRule_Rule_ProbabilisticRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.probabilistic", self.base))
    }
}
