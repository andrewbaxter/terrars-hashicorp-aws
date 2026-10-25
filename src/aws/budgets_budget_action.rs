#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BudgetsBudgetActionData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    action_type: terrars::PrimField<std::string::String>,
    approval_model: terrars::PrimField<std::string::String>,
    budget_name: terrars::PrimField<std::string::String>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    notification_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_threshold: std::option::Option<std::vec::Vec<BudgetsBudgetAction_ActionThreshold>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std::option::Option<std::vec::Vec<BudgetsBudgetAction_Definition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber: std::option::Option<std::vec::Vec<BudgetsBudgetAction_Subscriber>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BudgetsBudgetAction_Timeouts>,
    dynamic: BudgetsBudgetActionDynamic,
}

struct BudgetsBudgetAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BudgetsBudgetActionData_>,
}

#[derive(Clone)]
pub struct BudgetsBudgetAction(std::rc::Rc<BudgetsBudgetAction_>);

impl BudgetsBudgetAction {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `action_threshold`.\n"]
    pub fn set_action_threshold(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAction_ActionThreshold>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action_threshold = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action_threshold = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAction_Definition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subscriber`.\n"]
    pub fn set_subscriber(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAction_Subscriber>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscriber = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscriber = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BudgetsBudgetAction_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_type` after provisioning.\n"]
    pub fn action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_model` after provisioning.\n"]
    pub fn approval_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_name` after provisioning.\n"]
    pub fn budget_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_threshold` after provisioning.\n"]
    pub fn action_threshold(&self) -> terrars::ListRef<BudgetsBudgetAction_ActionThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<BudgetsBudgetAction_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BudgetsBudgetAction_TimeoutsRef {
        BudgetsBudgetAction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BudgetsBudgetAction {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BudgetsBudgetAction { }

impl terrars::ToListMappable for BudgetsBudgetAction {
    type O = terrars::ListRef<BudgetsBudgetActionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BudgetsBudgetAction_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_budgets_budget_action".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBudgetsBudgetAction {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub action_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub approval_model: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub budget_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notification_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetAction {
    pub fn build(self, stack: &mut terrars::Stack) -> BudgetsBudgetAction {
        let out = BudgetsBudgetAction(std::rc::Rc::new(BudgetsBudgetAction_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BudgetsBudgetActionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                action_type: self.action_type,
                approval_model: self.approval_model,
                budget_name: self.budget_name,
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                notification_type: self.notification_type,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                action_threshold: core::default::Default::default(),
                definition: core::default::Default::default(),
                subscriber: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BudgetsBudgetActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BudgetsBudgetActionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_id` after provisioning.\n"]
    pub fn action_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_type` after provisioning.\n"]
    pub fn action_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_model` after provisioning.\n"]
    pub fn approval_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approval_model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_name` after provisioning.\n"]
    pub fn budget_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_threshold` after provisioning.\n"]
    pub fn action_threshold(&self) -> terrars::ListRef<BudgetsBudgetAction_ActionThresholdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<BudgetsBudgetAction_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BudgetsBudgetAction_TimeoutsRef {
        BudgetsBudgetAction_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudgetActionDynamic {
    action_threshold: std::option::Option<terrars::DynamicBlock<BudgetsBudgetAction_ActionThreshold>>,
    definition: std::option::Option<terrars::DynamicBlock<BudgetsBudgetAction_Definition>>,
    subscriber: std::option::Option<terrars::DynamicBlock<BudgetsBudgetAction_Subscriber>>,
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_ActionThreshold {
    action_threshold_type: terrars::PrimField<std::string::String>,
    action_threshold_value: terrars::PrimField<f64>,
}

impl BudgetsBudgetAction_ActionThreshold { }

impl terrars::ToListMappable for BudgetsBudgetAction_ActionThreshold {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_ActionThreshold>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_ActionThreshold {
    #[doc = ""]
    pub action_threshold_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub action_threshold_value: terrars::PrimField<f64>,
}

impl BuildBudgetsBudgetAction_ActionThreshold {
    pub fn build(self) -> BudgetsBudgetAction_ActionThreshold {
        BudgetsBudgetAction_ActionThreshold {
            action_threshold_type: self.action_threshold_type,
            action_threshold_value: self.action_threshold_value,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_ActionThresholdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_ActionThresholdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetAction_ActionThresholdRef {
        BudgetsBudgetAction_ActionThresholdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_ActionThresholdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_threshold_type` after provisioning.\n"]
    pub fn action_threshold_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_threshold_type", self.base))
    }

    #[doc = "Get a reference to the value of field `action_threshold_value` after provisioning.\n"]
    pub fn action_threshold_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_threshold_value", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_Definition_IamActionDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    policy_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    users: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BudgetsBudgetAction_Definition_IamActionDefinition {
    #[doc = "Set the field `groups`.\n"]
    pub fn set_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `roles`.\n"]
    pub fn set_roles(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.roles = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `users`.\n"]
    pub fn set_users(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.users = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BudgetsBudgetAction_Definition_IamActionDefinition {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_Definition_IamActionDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_Definition_IamActionDefinition {
    #[doc = ""]
    pub policy_arn: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetAction_Definition_IamActionDefinition {
    pub fn build(self) -> BudgetsBudgetAction_Definition_IamActionDefinition {
        BudgetsBudgetAction_Definition_IamActionDefinition {
            groups: core::default::Default::default(),
            policy_arn: self.policy_arn,
            roles: core::default::Default::default(),
            users: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_Definition_IamActionDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_Definition_IamActionDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetAction_Definition_IamActionDefinitionRef {
        BudgetsBudgetAction_Definition_IamActionDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_Definition_IamActionDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `groups` after provisioning.\n"]
    pub fn groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.groups", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_arn` after provisioning.\n"]
    pub fn policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.roles", self.base))
    }

    #[doc = "Get a reference to the value of field `users` after provisioning.\n"]
    pub fn users(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.users", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_Definition_ScpActionDefinition {
    policy_id: terrars::PrimField<std::string::String>,
    target_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BudgetsBudgetAction_Definition_ScpActionDefinition { }

impl terrars::ToListMappable for BudgetsBudgetAction_Definition_ScpActionDefinition {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_Definition_ScpActionDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_Definition_ScpActionDefinition {
    #[doc = ""]
    pub policy_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBudgetsBudgetAction_Definition_ScpActionDefinition {
    pub fn build(self) -> BudgetsBudgetAction_Definition_ScpActionDefinition {
        BudgetsBudgetAction_Definition_ScpActionDefinition {
            policy_id: self.policy_id,
            target_ids: self.target_ids,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_Definition_ScpActionDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_Definition_ScpActionDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetAction_Definition_ScpActionDefinitionRef {
        BudgetsBudgetAction_Definition_ScpActionDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_Definition_ScpActionDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.base))
    }

    #[doc = "Get a reference to the value of field `target_ids` after provisioning.\n"]
    pub fn target_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_Definition_SsmActionDefinition {
    action_sub_type: terrars::PrimField<std::string::String>,
    instance_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
}

impl BudgetsBudgetAction_Definition_SsmActionDefinition { }

impl terrars::ToListMappable for BudgetsBudgetAction_Definition_SsmActionDefinition {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_Definition_SsmActionDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_Definition_SsmActionDefinition {
    #[doc = ""]
    pub action_sub_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetAction_Definition_SsmActionDefinition {
    pub fn build(self) -> BudgetsBudgetAction_Definition_SsmActionDefinition {
        BudgetsBudgetAction_Definition_SsmActionDefinition {
            action_sub_type: self.action_sub_type,
            instance_ids: self.instance_ids,
            region: self.region,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_Definition_SsmActionDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_Definition_SsmActionDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetAction_Definition_SsmActionDefinitionRef {
        BudgetsBudgetAction_Definition_SsmActionDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_Definition_SsmActionDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_sub_type` after provisioning.\n"]
    pub fn action_sub_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_sub_type", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_ids` after provisioning.\n"]
    pub fn instance_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudgetAction_DefinitionDynamic {
    iam_action_definition: std
    ::option
    ::Option<terrars::DynamicBlock<BudgetsBudgetAction_Definition_IamActionDefinition>>,
    scp_action_definition: std
    ::option
    ::Option<terrars::DynamicBlock<BudgetsBudgetAction_Definition_ScpActionDefinition>>,
    ssm_action_definition: std
    ::option
    ::Option<terrars::DynamicBlock<BudgetsBudgetAction_Definition_SsmActionDefinition>>,
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_Definition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_action_definition: std::option::Option<std::vec::Vec<BudgetsBudgetAction_Definition_IamActionDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scp_action_definition: std::option::Option<std::vec::Vec<BudgetsBudgetAction_Definition_ScpActionDefinition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssm_action_definition: std::option::Option<std::vec::Vec<BudgetsBudgetAction_Definition_SsmActionDefinition>>,
    dynamic: BudgetsBudgetAction_DefinitionDynamic,
}

impl BudgetsBudgetAction_Definition {
    #[doc = "Set the field `iam_action_definition`.\n"]
    pub fn set_iam_action_definition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAction_Definition_IamActionDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.iam_action_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.iam_action_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scp_action_definition`.\n"]
    pub fn set_scp_action_definition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAction_Definition_ScpActionDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scp_action_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scp_action_definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ssm_action_definition`.\n"]
    pub fn set_ssm_action_definition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAction_Definition_SsmActionDefinition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssm_action_definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_action_definition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudgetAction_Definition {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_Definition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_Definition {}

impl BuildBudgetsBudgetAction_Definition {
    pub fn build(self) -> BudgetsBudgetAction_Definition {
        BudgetsBudgetAction_Definition {
            iam_action_definition: core::default::Default::default(),
            scp_action_definition: core::default::Default::default(),
            ssm_action_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_DefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_DefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetAction_DefinitionRef {
        BudgetsBudgetAction_DefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_DefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_action_definition` after provisioning.\n"]
    pub fn iam_action_definition(&self) -> terrars::ListRef<BudgetsBudgetAction_Definition_IamActionDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_action_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `scp_action_definition` after provisioning.\n"]
    pub fn scp_action_definition(&self) -> terrars::ListRef<BudgetsBudgetAction_Definition_ScpActionDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scp_action_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `ssm_action_definition` after provisioning.\n"]
    pub fn ssm_action_definition(&self) -> terrars::ListRef<BudgetsBudgetAction_Definition_SsmActionDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssm_action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_Subscriber {
    address: terrars::PrimField<std::string::String>,
    subscription_type: terrars::PrimField<std::string::String>,
}

impl BudgetsBudgetAction_Subscriber { }

impl terrars::ToListMappable for BudgetsBudgetAction_Subscriber {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_Subscriber>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_Subscriber {
    #[doc = ""]
    pub address: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subscription_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetAction_Subscriber {
    pub fn build(self) -> BudgetsBudgetAction_Subscriber {
        BudgetsBudgetAction_Subscriber {
            address: self.address,
            subscription_type: self.subscription_type,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_SubscriberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_SubscriberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetAction_SubscriberRef {
        BudgetsBudgetAction_SubscriberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_SubscriberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `subscription_type` after provisioning.\n"]
    pub fn subscription_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subscription_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetAction_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BudgetsBudgetAction_Timeouts {
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

impl terrars::ToListMappable for BudgetsBudgetAction_Timeouts {
    type O = terrars::BlockAssignable<BudgetsBudgetAction_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAction_Timeouts {}

impl BuildBudgetsBudgetAction_Timeouts {
    pub fn build(self) -> BudgetsBudgetAction_Timeouts {
        BudgetsBudgetAction_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudgetAction_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAction_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetAction_TimeoutsRef {
        BudgetsBudgetAction_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAction_TimeoutsRef {
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
