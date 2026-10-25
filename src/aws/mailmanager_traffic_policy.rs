#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MailmanagerTrafficPolicyData_ {
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
    default_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_message_size_bytes: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_statement: std::option::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement>>,
    dynamic: MailmanagerTrafficPolicyDynamic,
}

struct MailmanagerTrafficPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MailmanagerTrafficPolicyData_>,
}

#[derive(Clone)]
pub struct MailmanagerTrafficPolicy(std::rc::Rc<MailmanagerTrafficPolicy_>);

impl MailmanagerTrafficPolicy {
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

    #[doc = "Set the field `max_message_size_bytes`.\n"]
    pub fn set_max_message_size_bytes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().max_message_size_bytes = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `policy_statement`.\n"]
    pub fn set_policy_statement(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_timestamp` after provisioning.\n"]
    pub fn last_updated_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_message_size_bytes` after provisioning.\n"]
    pub fn max_message_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_message_size_bytes", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_statement` after provisioning.\n"]
    pub fn policy_statement(&self) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_statement", self.extract_ref()))
    }
}

impl terrars::Referable for MailmanagerTrafficPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MailmanagerTrafficPolicy { }

impl terrars::ToListMappable for MailmanagerTrafficPolicy {
    type O = terrars::ListRef<MailmanagerTrafficPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MailmanagerTrafficPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mailmanager_traffic_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMailmanagerTrafficPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub default_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> MailmanagerTrafficPolicy {
        let out = MailmanagerTrafficPolicy(std::rc::Rc::new(MailmanagerTrafficPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MailmanagerTrafficPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_action: self.default_action,
                max_message_size_bytes: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                policy_statement: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MailmanagerTrafficPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MailmanagerTrafficPolicyRef {
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

    #[doc = "Get a reference to the value of field `created_timestamp` after provisioning.\n"]
    pub fn created_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_timestamp` after provisioning.\n"]
    pub fn last_updated_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_timestamp", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `max_message_size_bytes` after provisioning.\n"]
    pub fn max_message_size_bytes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_message_size_bytes", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_statement` after provisioning.\n"]
    pub fn policy_statement(&self) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_statement", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicyDynamic {
    policy_statement: std::option::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis {
    analyzer: terrars::PrimField<std::string::String>,
    result_field: terrars::PrimField<std::string::String>,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis { }

impl terrars
::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis {
    type O =
        terrars
        ::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis {
    #[doc = ""]
    pub analyzer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub result_field: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis {
            analyzer: self.analyzer,
            result_field: self.result_field,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analyzer` after provisioning.\n"]
    pub fn analyzer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analyzer", self.base))
    }

    #[doc = "Get a reference to the value of field `result_field` after provisioning.\n"]
    pub fn result_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result_field", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList {
    address_lists: terrars::ListField<terrars::PrimField<std::string::String>>,
    attribute: terrars::PrimField<std::string::String>,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList { }

impl terrars
::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList {
    type O =
        terrars
        ::BlockAssignable<
            MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList {
    #[doc = ""]
    pub address_lists: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub attribute: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList {
    pub fn build(
        self,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList {
            address_lists: self.address_lists,
            attribute: self.attribute,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressListRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_lists` after provisioning.\n"]
    pub fn address_lists(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_lists", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateDynamic {
    analysis: std
    ::option
    ::Option<
        terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis>,
    >,
    is_in_address_list: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList>,
    >,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_in_address_list: std
    ::option
    ::Option<
        std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList>,
    >,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {
    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `is_in_address_list`.\n"]
    pub fn set_is_in_address_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressList,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.is_in_address_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.is_in_address_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate {
            analysis: core::default::Default::default(),
            is_in_address_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `is_in_address_list` after provisioning.\n"]
    pub fn is_in_address_list(
        &self,
    ) -> terrars
    ::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate_IsInAddressListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.is_in_address_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionDynamic {
    evaluate: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_Evaluate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression {
            operator: self.operator,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate {
    attribute: terrars::PrimField<std::string::String>,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate { }

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate {
    #[doc = ""]
    pub attribute: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate { attribute: self.attribute }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionDynamic {
    evaluate: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression {
            operator: self.operator,
            values: self.values,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionDynamic {
    evaluate: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression {
            operator: self.operator,
            values: self.values,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateDynamic {
    analysis: std
    ::option
    ::Option<
        terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis>,
    >,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_Analysis,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate {
            attribute: core::default::Default::default(),
            analysis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionDynamic {
    evaluate: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_Evaluate,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression {
            operator: self.operator,
            values: self.values,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionDynamic {
    evaluate: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
    operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_Evaluate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression {
            operator: self.operator,
            value: self.value,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionRef {
        MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatement_ConditionDynamic {
    boolean_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression>>,
    ip_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression>>,
    ipv6_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression>>,
    string_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression>>,
    tls_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_expression: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_expression: std::option::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_expression: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_expression: std
    ::option
    ::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_expression: std::option::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatement_ConditionDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement_Condition {
    #[doc = "Set the field `boolean_expression`.\n"]
    pub fn set_boolean_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpression>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boolean_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boolean_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_expression`.\n"]
    pub fn set_ip_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ipv6_expression`.\n"]
    pub fn set_ipv6_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6Expression>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ipv6_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ipv6_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_expression`.\n"]
    pub fn set_string_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpression>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tls_expression`.\n"]
    pub fn set_tls_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpression>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tls_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tls_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement_Condition {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement_Condition {}

impl BuildMailmanagerTrafficPolicy_PolicyStatement_Condition {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement_Condition {
        MailmanagerTrafficPolicy_PolicyStatement_Condition {
            boolean_expression: core::default::Default::default(),
            ip_expression: core::default::Default::default(),
            ipv6_expression: core::default::Default::default(),
            string_expression: core::default::Default::default(),
            tls_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatement_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatement_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerTrafficPolicy_PolicyStatement_ConditionRef {
        MailmanagerTrafficPolicy_PolicyStatement_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatement_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boolean_expression` after provisioning.\n"]
    pub fn boolean_expression(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_BooleanExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_expression` after provisioning.\n"]
    pub fn ip_expression(&self) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_IpExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_expression` after provisioning.\n"]
    pub fn ipv6_expression(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_Ipv6ExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `string_expression` after provisioning.\n"]
    pub fn string_expression(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_StringExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_expression` after provisioning.\n"]
    pub fn tls_expression(
        &self,
    ) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_Condition_TlsExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tls_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerTrafficPolicy_PolicyStatementDynamic {
    condition: std::option::Option<terrars::DynamicBlock<MailmanagerTrafficPolicy_PolicyStatement_Condition>>,
}

#[derive(Serialize)]
pub struct MailmanagerTrafficPolicy_PolicyStatement {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<MailmanagerTrafficPolicy_PolicyStatement_Condition>>,
    dynamic: MailmanagerTrafficPolicy_PolicyStatementDynamic,
}

impl MailmanagerTrafficPolicy_PolicyStatement {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerTrafficPolicy_PolicyStatement {
    type O = terrars::BlockAssignable<MailmanagerTrafficPolicy_PolicyStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerTrafficPolicy_PolicyStatement {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerTrafficPolicy_PolicyStatement {
    pub fn build(self) -> MailmanagerTrafficPolicy_PolicyStatement {
        MailmanagerTrafficPolicy_PolicyStatement {
            action: self.action,
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerTrafficPolicy_PolicyStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerTrafficPolicy_PolicyStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerTrafficPolicy_PolicyStatementRef {
        MailmanagerTrafficPolicy_PolicyStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerTrafficPolicy_PolicyStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<MailmanagerTrafficPolicy_PolicyStatement_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }
}
