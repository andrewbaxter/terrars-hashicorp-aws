#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubAutomationRuleData_ {
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
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_terminal: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_name: terrars::PrimField<std::string::String>,
    rule_order: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Actions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criteria: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria>>,
    dynamic: SecurityhubAutomationRuleDynamic,
}

struct SecurityhubAutomationRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityhubAutomationRuleData_>,
}

#[derive(Clone)]
pub struct SecurityhubAutomationRule(std::rc::Rc<SecurityhubAutomationRule_>);

impl SecurityhubAutomationRule {
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

    #[doc = "Set the field `is_terminal`.\n"]
    pub fn set_is_terminal(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().is_terminal = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_status`.\n"]
    pub fn set_rule_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `actions`.\n"]
    pub fn set_actions(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Actions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().actions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.actions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `criteria`.\n"]
    pub fn set_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_terminal` after provisioning.\n"]
    pub fn is_terminal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_terminal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_status` after provisioning.\n"]
    pub fn rule_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> terrars::ListRef<SecurityhubAutomationRule_CriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criteria", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityhubAutomationRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityhubAutomationRule { }

impl terrars::ToListMappable for SecurityhubAutomationRule {
    type O = terrars::ListRef<SecurityhubAutomationRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityhubAutomationRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securityhub_automation_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubAutomationRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_order: terrars::PrimField<f64>,
}

impl BuildSecurityhubAutomationRule {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityhubAutomationRule {
        let out = SecurityhubAutomationRule(std::rc::Rc::new(SecurityhubAutomationRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityhubAutomationRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                is_terminal: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                rule_order: self.rule_order,
                rule_status: core::default::Default::default(),
                tags: core::default::Default::default(),
                actions: core::default::Default::default(),
                criteria: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubAutomationRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubAutomationRuleRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `is_terminal` after provisioning.\n"]
    pub fn is_terminal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_terminal", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_status` after provisioning.\n"]
    pub fn rule_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> terrars::ListRef<SecurityhubAutomationRule_CriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criteria", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleDynamic {
    actions: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Actions>>,
    criteria: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note {
    text: terrars::PrimField<std::string::String>,
    updated_by: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note {
    #[doc = ""]
    pub text: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub updated_by: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note {
    pub fn build(self) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note {
            text: self.text,
            updated_by: self.updated_by,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_NoteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_NoteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_NoteRef {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_NoteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_NoteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `text` after provisioning.\n"]
    pub fn text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.text", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_by` after provisioning.\n"]
    pub fn updated_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_by", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings {
    id: terrars::PrimField<std::string::String>,
    product_arn: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub product_arn: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings {
    pub fn build(self) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings {
            id: self.id,
            product_arn: self.product_arn,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindingsRef {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `product_arn` after provisioning.\n"]
    pub fn product_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product: std::option::Option<terrars::PrimField<f64>>,
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {
    #[doc = "Set the field `label`.\n"]
    pub fn set_label(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product`.\n"]
    pub fn set_product(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.product = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {}

impl BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {
    pub fn build(self) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity {
            label: core::default::Default::default(),
            product: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_SeverityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_SeverityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_SeverityRef {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_SeverityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_SeverityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `product` after provisioning.\n"]
    pub fn product(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow {
    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow {}

impl BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow {
    pub fn build(self) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow { status: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate_WorkflowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Actions_FindingFieldsUpdate_WorkflowRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate_WorkflowRef {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate_WorkflowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate_WorkflowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_Actions_FindingFieldsUpdateDynamic {
    note: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note>>,
    related_findings: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings>>,
    severity: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity>>,
    workflow: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criticality: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_fields: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow>>,
    dynamic: SecurityhubAutomationRule_Actions_FindingFieldsUpdateDynamic,
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdate {
    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.confidence = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.criticality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `types`.\n"]
    pub fn set_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.user_defined_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.verification_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `note`.\n"]
    pub fn set_note(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Note>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings`.\n"]
    pub fn set_related_findings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_RelatedFindings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Severity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.severity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.severity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow`.\n"]
    pub fn set_workflow(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_Workflow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workflow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Actions_FindingFieldsUpdate {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate {}

impl BuildSecurityhubAutomationRule_Actions_FindingFieldsUpdate {
    pub fn build(self) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdate {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdate {
            confidence: core::default::Default::default(),
            criticality: core::default::Default::default(),
            types: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            note: core::default::Default::default(),
            related_findings: core::default::Default::default(),
            severity: core::default::Default::default(),
            workflow: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Actions_FindingFieldsUpdateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Actions_FindingFieldsUpdateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Actions_FindingFieldsUpdateRef {
        SecurityhubAutomationRule_Actions_FindingFieldsUpdateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Actions_FindingFieldsUpdateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `confidence` after provisioning.\n"]
    pub fn confidence(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.confidence", self.base))
    }

    #[doc = "Get a reference to the value of field `criticality` after provisioning.\n"]
    pub fn criticality(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.criticality", self.base))
    }

    #[doc = "Get a reference to the value of field `types` after provisioning.\n"]
    pub fn types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.types", self.base))
    }

    #[doc = "Get a reference to the value of field `user_defined_fields` after provisioning.\n"]
    pub fn user_defined_fields(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.user_defined_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `verification_state` after provisioning.\n"]
    pub fn verification_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_state", self.base))
    }

    #[doc = "Get a reference to the value of field `note` after provisioning.\n"]
    pub fn note(&self) -> terrars::ListRef<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_NoteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.note", self.base))
    }

    #[doc = "Get a reference to the value of field `severity` after provisioning.\n"]
    pub fn severity(&self) -> terrars::ListRef<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_SeverityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.severity", self.base))
    }

    #[doc = "Get a reference to the value of field `workflow` after provisioning.\n"]
    pub fn workflow(&self) -> terrars::ListRef<SecurityhubAutomationRule_Actions_FindingFieldsUpdate_WorkflowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.workflow", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_ActionsDynamic {
    finding_fields_update: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Actions_FindingFieldsUpdate>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Actions {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_fields_update: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Actions_FindingFieldsUpdate>>,
    dynamic: SecurityhubAutomationRule_ActionsDynamic,
}

impl SecurityhubAutomationRule_Actions {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finding_fields_update`.\n"]
    pub fn set_finding_fields_update(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Actions_FindingFieldsUpdate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_fields_update = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_fields_update = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Actions {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Actions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Actions {}

impl BuildSecurityhubAutomationRule_Actions {
    pub fn build(self) -> SecurityhubAutomationRule_Actions {
        SecurityhubAutomationRule_Actions {
            type_: core::default::Default::default(),
            finding_fields_update: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_ActionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_ActionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRule_ActionsRef {
        SecurityhubAutomationRule_ActionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_ActionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `finding_fields_update` after provisioning.\n"]
    pub fn finding_fields_update(&self) -> terrars::ListRef<SecurityhubAutomationRule_Actions_FindingFieldsUpdateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.finding_fields_update", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_AwsAccountId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_AwsAccountId { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_AwsAccountId {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_AwsAccountId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_AwsAccountId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_AwsAccountId {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_AwsAccountId {
        SecurityhubAutomationRule_Criteria_AwsAccountId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_AwsAccountIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_AwsAccountIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_AwsAccountIdRef {
        SecurityhubAutomationRule_Criteria_AwsAccountIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_AwsAccountIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_AwsAccountName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_AwsAccountName { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_AwsAccountName {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_AwsAccountName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_AwsAccountName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_AwsAccountName {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_AwsAccountName {
        SecurityhubAutomationRule_Criteria_AwsAccountName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_AwsAccountNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_AwsAccountNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_AwsAccountNameRef {
        SecurityhubAutomationRule_Criteria_AwsAccountNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_AwsAccountNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_CompanyName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_CompanyName { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_CompanyName {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CompanyName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_CompanyName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_CompanyName {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_CompanyName {
        SecurityhubAutomationRule_Criteria_CompanyName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_CompanyNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_CompanyNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_CompanyNameRef {
        SecurityhubAutomationRule_Criteria_CompanyNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_CompanyNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId {
        SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsIdRef {
        SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ComplianceSecurityControlId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ComplianceSecurityControlId {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId {
        SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ComplianceSecurityControlIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ComplianceSecurityControlIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ComplianceSecurityControlIdRef {
        SecurityhubAutomationRule_Criteria_ComplianceSecurityControlIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ComplianceSecurityControlIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ComplianceStatus {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ComplianceStatus { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ComplianceStatus {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ComplianceStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ComplianceStatus {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ComplianceStatus {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ComplianceStatus {
        SecurityhubAutomationRule_Criteria_ComplianceStatus {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ComplianceStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ComplianceStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ComplianceStatusRef {
        SecurityhubAutomationRule_Criteria_ComplianceStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ComplianceStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_Confidence {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gt: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lt: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<f64>>,
}

impl SecurityhubAutomationRule_Criteria_Confidence {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gt`.\n"]
    pub fn set_gt(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lt`.\n"]
    pub fn set_lt(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_Confidence {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Confidence>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_Confidence {}

impl BuildSecurityhubAutomationRule_Criteria_Confidence {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_Confidence {
        SecurityhubAutomationRule_Criteria_Confidence {
            eq: core::default::Default::default(),
            gt: core::default::Default::default(),
            gte: core::default::Default::default(),
            lt: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ConfidenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ConfidenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ConfidenceRef {
        SecurityhubAutomationRule_Criteria_ConfidenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ConfidenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gt` after provisioning.\n"]
    pub fn gt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gt", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lt` after provisioning.\n"]
    pub fn lt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lt", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_CreatedAt_DateRange {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubAutomationRule_Criteria_CreatedAt_DateRange { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_CreatedAt_DateRange {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_CreatedAt_DateRange {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubAutomationRule_Criteria_CreatedAt_DateRange {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_CreatedAt_DateRange {
        SecurityhubAutomationRule_Criteria_CreatedAt_DateRange {
            unit: self.unit,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef {
        SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_Criteria_CreatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_CreatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    dynamic: SecurityhubAutomationRule_Criteria_CreatedAtDynamic,
}

impl SecurityhubAutomationRule_Criteria_CreatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_CreatedAt {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_CreatedAt {}

impl BuildSecurityhubAutomationRule_Criteria_CreatedAt {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_CreatedAt {
        SecurityhubAutomationRule_Criteria_CreatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_CreatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_CreatedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_CreatedAtRef {
        SecurityhubAutomationRule_Criteria_CreatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_CreatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_Criticality {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gt: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lt: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<f64>>,
}

impl SecurityhubAutomationRule_Criteria_Criticality {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gt`.\n"]
    pub fn set_gt(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lt`.\n"]
    pub fn set_lt(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lt = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_Criticality {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Criticality>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_Criticality {}

impl BuildSecurityhubAutomationRule_Criteria_Criticality {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_Criticality {
        SecurityhubAutomationRule_Criteria_Criticality {
            eq: core::default::Default::default(),
            gt: core::default::Default::default(),
            gte: core::default::Default::default(),
            lt: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_CriticalityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_CriticalityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_CriticalityRef {
        SecurityhubAutomationRule_Criteria_CriticalityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_CriticalityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gt` after provisioning.\n"]
    pub fn gt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gt", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lt` after provisioning.\n"]
    pub fn lt(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lt", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_Description {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_Description { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_Description {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Description>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_Description {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_Description {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_Description {
        SecurityhubAutomationRule_Criteria_Description {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_DescriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_DescriptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_DescriptionRef {
        SecurityhubAutomationRule_Criteria_DescriptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_DescriptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_Criteria_FirstObservedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_FirstObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    dynamic: SecurityhubAutomationRule_Criteria_FirstObservedAtDynamic,
}

impl SecurityhubAutomationRule_Criteria_FirstObservedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_FirstObservedAt {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_FirstObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_FirstObservedAt {}

impl BuildSecurityhubAutomationRule_Criteria_FirstObservedAt {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_FirstObservedAt {
        SecurityhubAutomationRule_Criteria_FirstObservedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_FirstObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_FirstObservedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_FirstObservedAtRef {
        SecurityhubAutomationRule_Criteria_FirstObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_FirstObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_GeneratorId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_GeneratorId { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_GeneratorId {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_GeneratorId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_GeneratorId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_GeneratorId {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_GeneratorId {
        SecurityhubAutomationRule_Criteria_GeneratorId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_GeneratorIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_GeneratorIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_GeneratorIdRef {
        SecurityhubAutomationRule_Criteria_GeneratorIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_GeneratorIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_Id {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_Id { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_Id {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Id>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_Id {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_Id {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_Id {
        SecurityhubAutomationRule_Criteria_Id {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_IdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_IdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRule_Criteria_IdRef {
        SecurityhubAutomationRule_Criteria_IdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_IdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_Criteria_LastObservedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_LastObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    dynamic: SecurityhubAutomationRule_Criteria_LastObservedAtDynamic,
}

impl SecurityhubAutomationRule_Criteria_LastObservedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_LastObservedAt {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_LastObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_LastObservedAt {}

impl BuildSecurityhubAutomationRule_Criteria_LastObservedAt {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_LastObservedAt {
        SecurityhubAutomationRule_Criteria_LastObservedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_LastObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_LastObservedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_LastObservedAtRef {
        SecurityhubAutomationRule_Criteria_LastObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_LastObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_NoteText {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_NoteText { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_NoteText {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_NoteText>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_NoteText {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_NoteText {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_NoteText {
        SecurityhubAutomationRule_Criteria_NoteText {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_NoteTextRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_NoteTextRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_NoteTextRef {
        SecurityhubAutomationRule_Criteria_NoteTextRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_NoteTextRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_Criteria_NoteUpdatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_NoteUpdatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    dynamic: SecurityhubAutomationRule_Criteria_NoteUpdatedAtDynamic,
}

impl SecurityhubAutomationRule_Criteria_NoteUpdatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_NoteUpdatedAt {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_NoteUpdatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_NoteUpdatedAt {}

impl BuildSecurityhubAutomationRule_Criteria_NoteUpdatedAt {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_NoteUpdatedAt {
        SecurityhubAutomationRule_Criteria_NoteUpdatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_NoteUpdatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_NoteUpdatedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_NoteUpdatedAtRef {
        SecurityhubAutomationRule_Criteria_NoteUpdatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_NoteUpdatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_NoteUpdatedBy {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_NoteUpdatedBy { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_NoteUpdatedBy {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_NoteUpdatedBy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_NoteUpdatedBy {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_NoteUpdatedBy {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_NoteUpdatedBy {
        SecurityhubAutomationRule_Criteria_NoteUpdatedBy {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_NoteUpdatedByRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_NoteUpdatedByRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_NoteUpdatedByRef {
        SecurityhubAutomationRule_Criteria_NoteUpdatedByRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_NoteUpdatedByRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ProductArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ProductArn { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ProductArn {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ProductArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ProductArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ProductArn {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ProductArn {
        SecurityhubAutomationRule_Criteria_ProductArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ProductArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ProductArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ProductArnRef {
        SecurityhubAutomationRule_Criteria_ProductArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ProductArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ProductName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ProductName { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ProductName {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ProductName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ProductName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ProductName {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ProductName {
        SecurityhubAutomationRule_Criteria_ProductName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ProductNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ProductNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ProductNameRef {
        SecurityhubAutomationRule_Criteria_ProductNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ProductNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_RecordState {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_RecordState { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_RecordState {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_RecordState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_RecordState {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_RecordState {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_RecordState {
        SecurityhubAutomationRule_Criteria_RecordState {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_RecordStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_RecordStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_RecordStateRef {
        SecurityhubAutomationRule_Criteria_RecordStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_RecordStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_RelatedFindingsId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_RelatedFindingsId { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_RelatedFindingsId {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_RelatedFindingsId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_RelatedFindingsId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_RelatedFindingsId {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_RelatedFindingsId {
        SecurityhubAutomationRule_Criteria_RelatedFindingsId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_RelatedFindingsIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_RelatedFindingsIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_RelatedFindingsIdRef {
        SecurityhubAutomationRule_Criteria_RelatedFindingsIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_RelatedFindingsIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_RelatedFindingsProductArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_RelatedFindingsProductArn {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn {
        SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_RelatedFindingsProductArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_RelatedFindingsProductArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_RelatedFindingsProductArnRef {
        SecurityhubAutomationRule_Criteria_RelatedFindingsProductArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_RelatedFindingsProductArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceApplicationArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceApplicationArn { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceApplicationArn {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceApplicationArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceApplicationArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceApplicationArn {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceApplicationArn {
        SecurityhubAutomationRule_Criteria_ResourceApplicationArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceApplicationArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceApplicationArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceApplicationArnRef {
        SecurityhubAutomationRule_Criteria_ResourceApplicationArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceApplicationArnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceApplicationName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceApplicationName { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceApplicationName {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceApplicationName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceApplicationName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceApplicationName {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceApplicationName {
        SecurityhubAutomationRule_Criteria_ResourceApplicationName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceApplicationNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceApplicationNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceApplicationNameRef {
        SecurityhubAutomationRule_Criteria_ResourceApplicationNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceApplicationNameRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceDetailsOther {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceDetailsOther { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceDetailsOther {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceDetailsOther>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceDetailsOther {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceDetailsOther {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceDetailsOther {
        SecurityhubAutomationRule_Criteria_ResourceDetailsOther {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceDetailsOtherRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceDetailsOtherRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceDetailsOtherRef {
        SecurityhubAutomationRule_Criteria_ResourceDetailsOtherRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceDetailsOtherRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceId { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceId {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceId {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceId {
        SecurityhubAutomationRule_Criteria_ResourceId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceIdRef {
        SecurityhubAutomationRule_Criteria_ResourceIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceIdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourcePartition {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourcePartition { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourcePartition {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourcePartition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourcePartition {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourcePartition {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourcePartition {
        SecurityhubAutomationRule_Criteria_ResourcePartition {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourcePartitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourcePartitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourcePartitionRef {
        SecurityhubAutomationRule_Criteria_ResourcePartitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourcePartitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceRegion {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceRegion { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceRegion {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceRegion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceRegion {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceRegion {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceRegion {
        SecurityhubAutomationRule_Criteria_ResourceRegion {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceRegionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceRegionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceRegionRef {
        SecurityhubAutomationRule_Criteria_ResourceRegionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceRegionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceTags {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceTags { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceTags {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceTags {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceTags {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceTags {
        SecurityhubAutomationRule_Criteria_ResourceTags {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceTagsRef {
        SecurityhubAutomationRule_Criteria_ResourceTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceTagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_ResourceType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_ResourceType { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_ResourceType {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_ResourceType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_ResourceType {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_ResourceType {
        SecurityhubAutomationRule_Criteria_ResourceType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_ResourceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_ResourceTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_ResourceTypeRef {
        SecurityhubAutomationRule_Criteria_ResourceTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_ResourceTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_SeverityLabel {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_SeverityLabel { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_SeverityLabel {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_SeverityLabel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_SeverityLabel {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_SeverityLabel {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_SeverityLabel {
        SecurityhubAutomationRule_Criteria_SeverityLabel {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_SeverityLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_SeverityLabelRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_SeverityLabelRef {
        SecurityhubAutomationRule_Criteria_SeverityLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_SeverityLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_SourceUrl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_SourceUrl { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_SourceUrl {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_SourceUrl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_SourceUrl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_SourceUrl {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_SourceUrl {
        SecurityhubAutomationRule_Criteria_SourceUrl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_SourceUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_SourceUrlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_SourceUrlRef {
        SecurityhubAutomationRule_Criteria_SourceUrlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_SourceUrlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_Title {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_Title { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_Title {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Title>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_Title {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_Title {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_Title {
        SecurityhubAutomationRule_Criteria_Title {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_TitleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_TitleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRule_Criteria_TitleRef {
        SecurityhubAutomationRule_Criteria_TitleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_TitleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_Type {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_Type { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_Type {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Type>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_Type {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_Type {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_Type {
        SecurityhubAutomationRule_Criteria_Type {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_TypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_TypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRule_Criteria_TypeRef {
        SecurityhubAutomationRule_Criteria_TypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_TypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_Criteria_UpdatedAtDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_UpdatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    dynamic: SecurityhubAutomationRule_Criteria_UpdatedAtDynamic,
}

impl SecurityhubAutomationRule_Criteria_UpdatedAt {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt_DateRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_UpdatedAt {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_UpdatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_UpdatedAt {}

impl BuildSecurityhubAutomationRule_Criteria_UpdatedAt {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_UpdatedAt {
        SecurityhubAutomationRule_Criteria_UpdatedAt {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_UpdatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_UpdatedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_UpdatedAtRef {
        SecurityhubAutomationRule_Criteria_UpdatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_UpdatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubAutomationRule_Criteria_CreatedAt_DateRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_UserDefinedFields {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_UserDefinedFields { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_UserDefinedFields {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_UserDefinedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_UserDefinedFields {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_UserDefinedFields {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_UserDefinedFields {
        SecurityhubAutomationRule_Criteria_UserDefinedFields {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_UserDefinedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_UserDefinedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_UserDefinedFieldsRef {
        SecurityhubAutomationRule_Criteria_UserDefinedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_UserDefinedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_VerificationState {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_VerificationState { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_VerificationState {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_VerificationState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_VerificationState {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_VerificationState {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_VerificationState {
        SecurityhubAutomationRule_Criteria_VerificationState {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_VerificationStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_VerificationStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_VerificationStateRef {
        SecurityhubAutomationRule_Criteria_VerificationStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_VerificationStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria_WorkflowStatus {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRule_Criteria_WorkflowStatus { }

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria_WorkflowStatus {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_WorkflowStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria_WorkflowStatus {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRule_Criteria_WorkflowStatus {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria_WorkflowStatus {
        SecurityhubAutomationRule_Criteria_WorkflowStatus {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_Criteria_WorkflowStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_Criteria_WorkflowStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRule_Criteria_WorkflowStatusRef {
        SecurityhubAutomationRule_Criteria_WorkflowStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_Criteria_WorkflowStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRule_CriteriaDynamic {
    aws_account_id: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_AwsAccountId>>,
    aws_account_name: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_AwsAccountName>>,
    company_name: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CompanyName>>,
    compliance_associated_standards_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId>>,
    compliance_security_control_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId>>,
    compliance_status: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ComplianceStatus>>,
    confidence: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_Confidence>>,
    created_at: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_CreatedAt>>,
    criticality: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_Criticality>>,
    description: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_Description>>,
    first_observed_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_FirstObservedAt>>,
    generator_id: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_GeneratorId>>,
    id: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_Id>>,
    last_observed_at: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_LastObservedAt>>,
    note_text: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_NoteText>>,
    note_updated_at: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_NoteUpdatedAt>>,
    note_updated_by: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_NoteUpdatedBy>>,
    product_arn: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ProductArn>>,
    product_name: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ProductName>>,
    record_state: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_RecordState>>,
    related_findings_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_RelatedFindingsId>>,
    related_findings_product_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn>>,
    resource_application_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceApplicationArn>>,
    resource_application_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceApplicationName>>,
    resource_details_other: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceDetailsOther>>,
    resource_id: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceId>>,
    resource_partition: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourcePartition>>,
    resource_region: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceRegion>>,
    resource_tags: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceTags>>,
    resource_type: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_ResourceType>>,
    severity_label: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_SeverityLabel>>,
    source_url: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_SourceUrl>>,
    title: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_Title>>,
    type_: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_Type>>,
    updated_at: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_UpdatedAt>>,
    user_defined_fields: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_UserDefinedFields>>,
    verification_state: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_VerificationState>>,
    workflow_status: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRule_Criteria_WorkflowStatus>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRule_Criteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_AwsAccountId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_name: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_AwsAccountName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CompanyName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_associated_standards_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_security_control_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_status: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ComplianceStatus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_Confidence>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_CreatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criticality: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_Criticality>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_Description>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_observed_at: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_FirstObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generator_id: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_GeneratorId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_Id>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_observed_at: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_LastObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_text: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_NoteText>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_updated_at: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_NoteUpdatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_updated_by: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_NoteUpdatedBy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_arn: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ProductArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_name: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ProductName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_state: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_RecordState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings_id: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_RelatedFindingsId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings_product_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_application_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceApplicationArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_application_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceApplicationName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_details_other: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceDetailsOther>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_partition: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourcePartition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_region: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceRegion>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_ResourceType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_label: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_SeverityLabel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_url: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_SourceUrl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_Title>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_Type>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_UpdatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_fields: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_UserDefinedFields>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_state: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_VerificationState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_status: std::option::Option<std::vec::Vec<SecurityhubAutomationRule_Criteria_WorkflowStatus>>,
    dynamic: SecurityhubAutomationRule_CriteriaDynamic,
}

impl SecurityhubAutomationRule_Criteria {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_AwsAccountId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_account_name`.\n"]
    pub fn set_aws_account_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_AwsAccountName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `company_name`.\n"]
    pub fn set_company_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CompanyName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.company_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.company_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_associated_standards_id`.\n"]
    pub fn set_compliance_associated_standards_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ComplianceAssociatedStandardsId>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_associated_standards_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_associated_standards_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_security_control_id`.\n"]
    pub fn set_compliance_security_control_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ComplianceSecurityControlId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_security_control_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_security_control_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ComplianceStatus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Confidence>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_CreatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.created_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.created_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Criticality>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.criticality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Description>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.description = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.description = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_FirstObservedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.first_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.first_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generator_id`.\n"]
    pub fn set_generator_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_GeneratorId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generator_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generator_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Id>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_LastObservedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_text`.\n"]
    pub fn set_note_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_NoteText>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_updated_at`.\n"]
    pub fn set_note_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_NoteUpdatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_updated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_updated_by`.\n"]
    pub fn set_note_updated_by(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_NoteUpdatedBy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_updated_by = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_by = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_arn`.\n"]
    pub fn set_product_arn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ProductArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ProductName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_state`.\n"]
    pub fn set_record_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_RecordState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings_id`.\n"]
    pub fn set_related_findings_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_RelatedFindingsId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings_product_arn`.\n"]
    pub fn set_related_findings_product_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_RelatedFindingsProductArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings_product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_application_arn`.\n"]
    pub fn set_resource_application_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceApplicationArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_application_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_application_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_application_name`.\n"]
    pub fn set_resource_application_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceApplicationName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_application_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_application_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_details_other`.\n"]
    pub fn set_resource_details_other(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceDetailsOther>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_details_other = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_details_other = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_partition`.\n"]
    pub fn set_resource_partition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourcePartition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_partition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_partition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_region`.\n"]
    pub fn set_resource_region(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceRegion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_region = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_region = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceTags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_ResourceType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `severity_label`.\n"]
    pub fn set_severity_label(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_SeverityLabel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.severity_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_url`.\n"]
    pub fn set_source_url(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_SourceUrl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Title>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.title = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.title = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_Type>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.type_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_UpdatedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.updated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.updated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_defined_fields`.\n"]
    pub fn set_user_defined_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_UserDefinedFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_defined_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_VerificationState>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verification_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verification_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow_status`.\n"]
    pub fn set_workflow_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRule_Criteria_WorkflowStatus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workflow_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow_status = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRule_Criteria {
    type O = terrars::BlockAssignable<SecurityhubAutomationRule_Criteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRule_Criteria {}

impl BuildSecurityhubAutomationRule_Criteria {
    pub fn build(self) -> SecurityhubAutomationRule_Criteria {
        SecurityhubAutomationRule_Criteria {
            aws_account_id: core::default::Default::default(),
            aws_account_name: core::default::Default::default(),
            company_name: core::default::Default::default(),
            compliance_associated_standards_id: core::default::Default::default(),
            compliance_security_control_id: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            confidence: core::default::Default::default(),
            created_at: core::default::Default::default(),
            criticality: core::default::Default::default(),
            description: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            generator_id: core::default::Default::default(),
            id: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            note_text: core::default::Default::default(),
            note_updated_at: core::default::Default::default(),
            note_updated_by: core::default::Default::default(),
            product_arn: core::default::Default::default(),
            product_name: core::default::Default::default(),
            record_state: core::default::Default::default(),
            related_findings_id: core::default::Default::default(),
            related_findings_product_arn: core::default::Default::default(),
            resource_application_arn: core::default::Default::default(),
            resource_application_name: core::default::Default::default(),
            resource_details_other: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_partition: core::default::Default::default(),
            resource_region: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity_label: core::default::Default::default(),
            source_url: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_defined_fields: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            workflow_status: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRule_CriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRule_CriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRule_CriteriaRef {
        SecurityhubAutomationRule_CriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRule_CriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}
