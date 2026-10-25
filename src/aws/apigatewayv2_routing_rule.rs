#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Apigatewayv2RoutingRuleData_ {
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
    domain_name: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<Apigatewayv2RoutingRule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<Apigatewayv2RoutingRule_Condition>>,
    dynamic: Apigatewayv2RoutingRuleDynamic,
}

struct Apigatewayv2RoutingRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Apigatewayv2RoutingRuleData_>,
}

#[derive(Clone)]
pub struct Apigatewayv2RoutingRule(std::rc::Rc<Apigatewayv2RoutingRule_>);

impl Apigatewayv2RoutingRule {
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RoutingRule_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition>>,
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

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `routing_rule_arn` after provisioning.\n"]
    pub fn routing_rule_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_rule_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_rule_id` after provisioning.\n"]
    pub fn routing_rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }
}

impl terrars::Referable for Apigatewayv2RoutingRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Apigatewayv2RoutingRule { }

impl terrars::ToListMappable for Apigatewayv2RoutingRule {
    type O = terrars::ListRef<Apigatewayv2RoutingRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Apigatewayv2RoutingRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_apigatewayv2_routing_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApigatewayv2RoutingRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildApigatewayv2RoutingRule {
    pub fn build(self, stack: &mut terrars::Stack) -> Apigatewayv2RoutingRule {
        let out = Apigatewayv2RoutingRule(std::rc::Rc::new(Apigatewayv2RoutingRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Apigatewayv2RoutingRuleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_name: self.domain_name,
                priority: self.priority,
                region: core::default::Default::default(),
                action: core::default::Default::default(),
                condition: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Apigatewayv2RoutingRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Apigatewayv2RoutingRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `routing_rule_arn` after provisioning.\n"]
    pub fn routing_rule_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_rule_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `routing_rule_id` after provisioning.\n"]
    pub fn routing_rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.routing_rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2RoutingRuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<Apigatewayv2RoutingRule_Action>>,
    condition: std::option::Option<terrars::DynamicBlock<Apigatewayv2RoutingRule_Condition>>,
}

#[derive(Serialize)]
pub struct Apigatewayv2RoutingRule_Action_InvokeApi {
    api_id: terrars::PrimField<std::string::String>,
    stage: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strip_base_path: std::option::Option<terrars::PrimField<bool>>,
}

impl Apigatewayv2RoutingRule_Action_InvokeApi {
    #[doc = "Set the field `strip_base_path`.\n"]
    pub fn set_strip_base_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.strip_base_path = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Apigatewayv2RoutingRule_Action_InvokeApi {
    type O = terrars::BlockAssignable<Apigatewayv2RoutingRule_Action_InvokeApi>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RoutingRule_Action_InvokeApi {
    #[doc = ""]
    pub api_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub stage: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2RoutingRule_Action_InvokeApi {
    pub fn build(self) -> Apigatewayv2RoutingRule_Action_InvokeApi {
        Apigatewayv2RoutingRule_Action_InvokeApi {
            api_id: self.api_id,
            stage: self.stage,
            strip_base_path: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2RoutingRule_Action_InvokeApiRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRule_Action_InvokeApiRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2RoutingRule_Action_InvokeApiRef {
        Apigatewayv2RoutingRule_Action_InvokeApiRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RoutingRule_Action_InvokeApiRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.base))
    }

    #[doc = "Get a reference to the value of field `stage` after provisioning.\n"]
    pub fn stage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stage", self.base))
    }

    #[doc = "Get a reference to the value of field `strip_base_path` after provisioning.\n"]
    pub fn strip_base_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strip_base_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2RoutingRule_ActionDynamic {
    invoke_api: std::option::Option<terrars::DynamicBlock<Apigatewayv2RoutingRule_Action_InvokeApi>>,
}

#[derive(Serialize)]
pub struct Apigatewayv2RoutingRule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoke_api: std::option::Option<std::vec::Vec<Apigatewayv2RoutingRule_Action_InvokeApi>>,
    dynamic: Apigatewayv2RoutingRule_ActionDynamic,
}

impl Apigatewayv2RoutingRule_Action {
    #[doc = "Set the field `invoke_api`.\n"]
    pub fn set_invoke_api(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RoutingRule_Action_InvokeApi>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invoke_api = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invoke_api = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Apigatewayv2RoutingRule_Action {
    type O = terrars::BlockAssignable<Apigatewayv2RoutingRule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RoutingRule_Action {}

impl BuildApigatewayv2RoutingRule_Action {
    pub fn build(self) -> Apigatewayv2RoutingRule_Action {
        Apigatewayv2RoutingRule_Action {
            invoke_api: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2RoutingRule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2RoutingRule_ActionRef {
        Apigatewayv2RoutingRule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RoutingRule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invoke_api` after provisioning.\n"]
    pub fn invoke_api(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_Action_InvokeApiRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invoke_api", self.base))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2RoutingRule_Condition_MatchBasePaths {
    any_of: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Apigatewayv2RoutingRule_Condition_MatchBasePaths { }

impl terrars::ToListMappable for Apigatewayv2RoutingRule_Condition_MatchBasePaths {
    type O = terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition_MatchBasePaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RoutingRule_Condition_MatchBasePaths {
    #[doc = ""]
    pub any_of: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildApigatewayv2RoutingRule_Condition_MatchBasePaths {
    pub fn build(self) -> Apigatewayv2RoutingRule_Condition_MatchBasePaths {
        Apigatewayv2RoutingRule_Condition_MatchBasePaths { any_of: self.any_of }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2RoutingRule_Condition_MatchBasePathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRule_Condition_MatchBasePathsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Apigatewayv2RoutingRule_Condition_MatchBasePathsRef {
        Apigatewayv2RoutingRule_Condition_MatchBasePathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RoutingRule_Condition_MatchBasePathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
}

#[derive(Serialize)]
pub struct Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf {
    header: terrars::PrimField<std::string::String>,
    value_glob: terrars::PrimField<std::string::String>,
}

impl Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf { }

impl terrars::ToListMappable for Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf {
    type O = terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf {
    #[doc = ""]
    pub header: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_glob: terrars::PrimField<std::string::String>,
}

impl BuildApigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf {
    pub fn build(self) -> Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf {
        Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf {
            header: self.header,
            value_glob: self.value_glob,
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOfRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOfRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOfRef {
        Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOfRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOfRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `value_glob` after provisioning.\n"]
    pub fn value_glob(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_glob", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2RoutingRule_Condition_MatchHeadersDynamic {
    any_of: std::option::Option<terrars::DynamicBlock<Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf>>,
}

#[derive(Serialize)]
pub struct Apigatewayv2RoutingRule_Condition_MatchHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    any_of: std::option::Option<std::vec::Vec<Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf>>,
    dynamic: Apigatewayv2RoutingRule_Condition_MatchHeadersDynamic,
}

impl Apigatewayv2RoutingRule_Condition_MatchHeaders {
    #[doc = "Set the field `any_of`.\n"]
    pub fn set_any_of(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOf>>,
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

impl terrars::ToListMappable for Apigatewayv2RoutingRule_Condition_MatchHeaders {
    type O = terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition_MatchHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RoutingRule_Condition_MatchHeaders {}

impl BuildApigatewayv2RoutingRule_Condition_MatchHeaders {
    pub fn build(self) -> Apigatewayv2RoutingRule_Condition_MatchHeaders {
        Apigatewayv2RoutingRule_Condition_MatchHeaders {
            any_of: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2RoutingRule_Condition_MatchHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRule_Condition_MatchHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Apigatewayv2RoutingRule_Condition_MatchHeadersRef {
        Apigatewayv2RoutingRule_Condition_MatchHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RoutingRule_Condition_MatchHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `any_of` after provisioning.\n"]
    pub fn any_of(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_Condition_MatchHeaders_AnyOfRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.any_of", self.base))
    }
}

#[derive(Serialize, Default)]
struct Apigatewayv2RoutingRule_ConditionDynamic {
    match_base_paths: std::option::Option<terrars::DynamicBlock<Apigatewayv2RoutingRule_Condition_MatchBasePaths>>,
    match_headers: std::option::Option<terrars::DynamicBlock<Apigatewayv2RoutingRule_Condition_MatchHeaders>>,
}

#[derive(Serialize)]
pub struct Apigatewayv2RoutingRule_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_base_paths: std::option::Option<std::vec::Vec<Apigatewayv2RoutingRule_Condition_MatchBasePaths>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_headers: std::option::Option<std::vec::Vec<Apigatewayv2RoutingRule_Condition_MatchHeaders>>,
    dynamic: Apigatewayv2RoutingRule_ConditionDynamic,
}

impl Apigatewayv2RoutingRule_Condition {
    #[doc = "Set the field `match_base_paths`.\n"]
    pub fn set_match_base_paths(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition_MatchBasePaths>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_base_paths = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_base_paths = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `match_headers`.\n"]
    pub fn set_match_headers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition_MatchHeaders>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Apigatewayv2RoutingRule_Condition {
    type O = terrars::BlockAssignable<Apigatewayv2RoutingRule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildApigatewayv2RoutingRule_Condition {}

impl BuildApigatewayv2RoutingRule_Condition {
    pub fn build(self) -> Apigatewayv2RoutingRule_Condition {
        Apigatewayv2RoutingRule_Condition {
            match_base_paths: core::default::Default::default(),
            match_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Apigatewayv2RoutingRule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Apigatewayv2RoutingRule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Apigatewayv2RoutingRule_ConditionRef {
        Apigatewayv2RoutingRule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Apigatewayv2RoutingRule_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_base_paths` after provisioning.\n"]
    pub fn match_base_paths(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_Condition_MatchBasePathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_base_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `match_headers` after provisioning.\n"]
    pub fn match_headers(&self) -> terrars::ListRef<Apigatewayv2RoutingRule_Condition_MatchHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_headers", self.base))
    }
}
