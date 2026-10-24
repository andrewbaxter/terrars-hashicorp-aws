use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BudgetsBudgetActionData {
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
    action_threshold: std::option::Option<std::vec::Vec<BudgetsBudgetActionActionThresholdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std::option::Option<std::vec::Vec<BudgetsBudgetActionDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber: std::option::Option<std::vec::Vec<BudgetsBudgetActionSubscriberEl>>,
    dynamic: BudgetsBudgetActionDynamic,
}

struct BudgetsBudgetAction_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BudgetsBudgetActionData>,
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

    #[doc = "Set the field `action_threshold`.\n"]
    pub fn set_action_threshold(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetActionActionThresholdEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetActionDefinitionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetActionSubscriberEl>>,
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

    #[doc = "Get a reference to the value of field `action_threshold` after provisioning.\n"]
    pub fn action_threshold(&self) -> terrars::ListRef<BudgetsBudgetActionActionThresholdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<BudgetsBudgetActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
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
            data: std::cell::RefCell::new(BudgetsBudgetActionData {
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
                action_threshold: core::default::Default::default(),
                definition: core::default::Default::default(),
                subscriber: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `action_threshold` after provisioning.\n"]
    pub fn action_threshold(&self) -> terrars::ListRef<BudgetsBudgetActionActionThresholdElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<BudgetsBudgetActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionActionThresholdEl {
    action_threshold_type: terrars::PrimField<std::string::String>,
    action_threshold_value: terrars::PrimField<f64>,
}

impl BudgetsBudgetActionActionThresholdEl { }

impl terrars::ToListMappable for BudgetsBudgetActionActionThresholdEl {
    type O = terrars::BlockAssignable<BudgetsBudgetActionActionThresholdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionActionThresholdEl {
    #[doc = ""]
    pub action_threshold_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub action_threshold_value: terrars::PrimField<f64>,
}

impl BuildBudgetsBudgetActionActionThresholdEl {
    pub fn build(self) -> BudgetsBudgetActionActionThresholdEl {
        BudgetsBudgetActionActionThresholdEl {
            action_threshold_type: self.action_threshold_type,
            action_threshold_value: self.action_threshold_value,
        }
    }
}

pub struct BudgetsBudgetActionActionThresholdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionActionThresholdElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetActionActionThresholdElRef {
        BudgetsBudgetActionActionThresholdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionActionThresholdElRef {
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
pub struct BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    policy_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    roles: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    users: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
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

impl terrars::ToListMappable for BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    type O = terrars::BlockAssignable<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    #[doc = ""]
    pub policy_arn: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetActionDefinitionElIamActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
        BudgetsBudgetActionDefinitionElIamActionDefinitionEl {
            groups: core::default::Default::default(),
            policy_arn: self.policy_arn,
            roles: core::default::Default::default(),
            users: core::default::Default::default(),
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElIamActionDefinitionElRef {
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
pub struct BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    policy_id: terrars::PrimField<std::string::String>,
    target_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BudgetsBudgetActionDefinitionElScpActionDefinitionEl { }

impl terrars::ToListMappable for BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    type O = terrars::BlockAssignable<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    #[doc = ""]
    pub policy_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBudgetsBudgetActionDefinitionElScpActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
        BudgetsBudgetActionDefinitionElScpActionDefinitionEl {
            policy_id: self.policy_id,
            target_ids: self.target_ids,
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElScpActionDefinitionElRef {
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
pub struct BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    action_sub_type: terrars::PrimField<std::string::String>,
    instance_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    region: terrars::PrimField<std::string::String>,
}

impl BudgetsBudgetActionDefinitionElSsmActionDefinitionEl { }

impl terrars::ToListMappable for BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    type O = terrars::BlockAssignable<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    #[doc = ""]
    pub action_sub_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub instance_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
        BudgetsBudgetActionDefinitionElSsmActionDefinitionEl {
            action_sub_type: self.action_sub_type,
            instance_ids: self.instance_ids,
            region: self.region,
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef {
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
struct BudgetsBudgetActionDefinitionElDynamic {
    iam_action_definition: std
    ::option
    ::Option<terrars::DynamicBlock<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>>,
    scp_action_definition: std
    ::option
    ::Option<terrars::DynamicBlock<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>>,
    ssm_action_definition: std
    ::option
    ::Option<terrars::DynamicBlock<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>>,
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_action_definition: std::option::Option<std::vec::Vec<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scp_action_definition: std::option::Option<std::vec::Vec<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssm_action_definition: std::option::Option<std::vec::Vec<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>>,
    dynamic: BudgetsBudgetActionDefinitionElDynamic,
}

impl BudgetsBudgetActionDefinitionEl {
    #[doc = "Set the field `iam_action_definition`.\n"]
    pub fn set_iam_action_definition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetActionDefinitionElIamActionDefinitionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetActionDefinitionElScpActionDefinitionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetActionDefinitionElSsmActionDefinitionEl>>,
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

impl terrars::ToListMappable for BudgetsBudgetActionDefinitionEl {
    type O = terrars::BlockAssignable<BudgetsBudgetActionDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionDefinitionEl {}

impl BuildBudgetsBudgetActionDefinitionEl {
    pub fn build(self) -> BudgetsBudgetActionDefinitionEl {
        BudgetsBudgetActionDefinitionEl {
            iam_action_definition: core::default::Default::default(),
            scp_action_definition: core::default::Default::default(),
            ssm_action_definition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BudgetsBudgetActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionDefinitionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetActionDefinitionElRef {
        BudgetsBudgetActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_action_definition` after provisioning.\n"]
    pub fn iam_action_definition(&self) -> terrars::ListRef<BudgetsBudgetActionDefinitionElIamActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_action_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `scp_action_definition` after provisioning.\n"]
    pub fn scp_action_definition(&self) -> terrars::ListRef<BudgetsBudgetActionDefinitionElScpActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scp_action_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `ssm_action_definition` after provisioning.\n"]
    pub fn ssm_action_definition(&self) -> terrars::ListRef<BudgetsBudgetActionDefinitionElSsmActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssm_action_definition", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetActionSubscriberEl {
    address: terrars::PrimField<std::string::String>,
    subscription_type: terrars::PrimField<std::string::String>,
}

impl BudgetsBudgetActionSubscriberEl { }

impl terrars::ToListMappable for BudgetsBudgetActionSubscriberEl {
    type O = terrars::BlockAssignable<BudgetsBudgetActionSubscriberEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetActionSubscriberEl {
    #[doc = ""]
    pub address: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub subscription_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetActionSubscriberEl {
    pub fn build(self) -> BudgetsBudgetActionSubscriberEl {
        BudgetsBudgetActionSubscriberEl {
            address: self.address,
            subscription_type: self.subscription_type,
        }
    }
}

pub struct BudgetsBudgetActionSubscriberElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetActionSubscriberElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetActionSubscriberElRef {
        BudgetsBudgetActionSubscriberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetActionSubscriberElRef {
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

#[derive(Serialize, Default)]
struct BudgetsBudgetActionDynamic {
    action_threshold: std::option::Option<terrars::DynamicBlock<BudgetsBudgetActionActionThresholdEl>>,
    definition: std::option::Option<terrars::DynamicBlock<BudgetsBudgetActionDefinitionEl>>,
    subscriber: std::option::Option<terrars::DynamicBlock<BudgetsBudgetActionSubscriberEl>>,
}
