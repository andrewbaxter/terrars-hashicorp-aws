#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BudgetsBudgetData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_view_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    budget_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_amount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_period_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_period_start: std::option::Option<terrars::PrimField<std::string::String>>,
    time_unit: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_adjust_data: std::option::Option<std::vec::Vec<BudgetsBudget_AutoAdjustData>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_filter: std::option::Option<std::vec::Vec<BudgetsBudget_CostFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_types: std::option::Option<std::vec::Vec<BudgetsBudget_CostTypes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_expression: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification: std::option::Option<std::vec::Vec<BudgetsBudget_Notification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    planned_limit: std::option::Option<std::vec::Vec<BudgetsBudget_PlannedLimit>>,
    dynamic: BudgetsBudgetDynamic,
}

struct BudgetsBudget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BudgetsBudgetData_>,
}

#[derive(Clone)]
pub struct BudgetsBudget(std::rc::Rc<BudgetsBudget_>);

impl BudgetsBudget {
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

    #[doc = "Set the field `billing_view_arn`.\n"]
    pub fn set_billing_view_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().billing_view_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit_amount`.\n"]
    pub fn set_limit_amount(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().limit_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `limit_unit`.\n"]
    pub fn set_limit_unit(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().limit_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metrics`.\n"]
    pub fn set_metrics(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().metrics = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `time_period_end`.\n"]
    pub fn set_time_period_end(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().time_period_end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_period_start`.\n"]
    pub fn set_time_period_start(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().time_period_start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_adjust_data`.\n"]
    pub fn set_auto_adjust_data(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_AutoAdjustData>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_adjust_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_adjust_data = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_filter`.\n"]
    pub fn set_cost_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_CostFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cost_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cost_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_types`.\n"]
    pub fn set_cost_types(self, v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_CostTypes>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cost_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cost_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `filter_expression`.\n"]
    pub fn set_filter_expression(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `notification`.\n"]
    pub fn set_notification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_Notification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `planned_limit`.\n"]
    pub fn set_planned_limit(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_PlannedLimit>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().planned_limit = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.planned_limit = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limit_amount` after provisioning.\n"]
    pub fn limit_amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limit_unit` after provisioning.\n"]
    pub fn limit_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_end` after provisioning.\n"]
    pub fn time_period_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_period_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_start` after provisioning.\n"]
    pub fn time_period_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_period_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> terrars::ListRef<BudgetsBudget_AutoAdjustDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> terrars::ListRef<BudgetsBudget_CostTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_expression` after provisioning.\n"]
    pub fn filter_expression(&self) -> terrars::ListRef<BudgetsBudget_FilterExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_expression", self.extract_ref()))
    }
}

impl terrars::Referable for BudgetsBudget {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BudgetsBudget { }

impl terrars::ToListMappable for BudgetsBudget {
    type O = terrars::ListRef<BudgetsBudgetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BudgetsBudget_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_budgets_budget".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBudgetsBudget {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub budget_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time_unit: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudget {
    pub fn build(self, stack: &mut terrars::Stack) -> BudgetsBudget {
        let out = BudgetsBudget(std::rc::Rc::new(BudgetsBudget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BudgetsBudgetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                billing_view_arn: core::default::Default::default(),
                budget_type: self.budget_type,
                id: core::default::Default::default(),
                limit_amount: core::default::Default::default(),
                limit_unit: core::default::Default::default(),
                metrics: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                time_period_end: core::default::Default::default(),
                time_period_start: core::default::Default::default(),
                time_unit: self.time_unit,
                auto_adjust_data: core::default::Default::default(),
                cost_filter: core::default::Default::default(),
                cost_types: core::default::Default::default(),
                filter_expression: core::default::Default::default(),
                notification: core::default::Default::default(),
                planned_limit: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BudgetsBudgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BudgetsBudgetRef {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limit_amount` after provisioning.\n"]
    pub fn limit_amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `limit_unit` after provisioning.\n"]
    pub fn limit_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metrics` after provisioning.\n"]
    pub fn metrics(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_end` after provisioning.\n"]
    pub fn time_period_end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_period_end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_period_start` after provisioning.\n"]
    pub fn time_period_start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_period_start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `time_unit` after provisioning.\n"]
    pub fn time_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time_unit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> terrars::ListRef<BudgetsBudget_AutoAdjustDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> terrars::ListRef<BudgetsBudget_CostTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_expression` after provisioning.\n"]
    pub fn filter_expression(&self) -> terrars::ListRef<BudgetsBudget_FilterExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_expression", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudgetDynamic {
    auto_adjust_data: std::option::Option<terrars::DynamicBlock<BudgetsBudget_AutoAdjustData>>,
    cost_filter: std::option::Option<terrars::DynamicBlock<BudgetsBudget_CostFilter>>,
    cost_types: std::option::Option<terrars::DynamicBlock<BudgetsBudget_CostTypes>>,
    filter_expression: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression>>,
    notification: std::option::Option<terrars::DynamicBlock<BudgetsBudget_Notification>>,
    planned_limit: std::option::Option<terrars::DynamicBlock<BudgetsBudget_PlannedLimit>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_AutoAdjustData_HistoricalOptions {
    budget_adjustment_period: terrars::PrimField<f64>,
}

impl BudgetsBudget_AutoAdjustData_HistoricalOptions { }

impl terrars::ToListMappable for BudgetsBudget_AutoAdjustData_HistoricalOptions {
    type O = terrars::BlockAssignable<BudgetsBudget_AutoAdjustData_HistoricalOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_AutoAdjustData_HistoricalOptions {
    #[doc = ""]
    pub budget_adjustment_period: terrars::PrimField<f64>,
}

impl BuildBudgetsBudget_AutoAdjustData_HistoricalOptions {
    pub fn build(self) -> BudgetsBudget_AutoAdjustData_HistoricalOptions {
        BudgetsBudget_AutoAdjustData_HistoricalOptions { budget_adjustment_period: self.budget_adjustment_period }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
        BudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `budget_adjustment_period` after provisioning.\n"]
    pub fn budget_adjustment_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_adjustment_period", self.base))
    }

    #[doc = "Get a reference to the value of field `lookback_available_periods` after provisioning.\n"]
    pub fn lookback_available_periods(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lookback_available_periods", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_AutoAdjustDataDynamic {
    historical_options: std::option::Option<terrars::DynamicBlock<BudgetsBudget_AutoAdjustData_HistoricalOptions>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_AutoAdjustData {
    auto_adjust_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    historical_options: std::option::Option<std::vec::Vec<BudgetsBudget_AutoAdjustData_HistoricalOptions>>,
    dynamic: BudgetsBudget_AutoAdjustDataDynamic,
}

impl BudgetsBudget_AutoAdjustData {
    #[doc = "Set the field `historical_options`.\n"]
    pub fn set_historical_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_AutoAdjustData_HistoricalOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.historical_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.historical_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_AutoAdjustData {
    type O = terrars::BlockAssignable<BudgetsBudget_AutoAdjustData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_AutoAdjustData {
    #[doc = ""]
    pub auto_adjust_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudget_AutoAdjustData {
    pub fn build(self) -> BudgetsBudget_AutoAdjustData {
        BudgetsBudget_AutoAdjustData {
            auto_adjust_type: self.auto_adjust_type,
            historical_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_AutoAdjustDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_AutoAdjustDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_AutoAdjustDataRef {
        BudgetsBudget_AutoAdjustDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_AutoAdjustDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_adjust_type` after provisioning.\n"]
    pub fn auto_adjust_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_adjust_type", self.base))
    }

    #[doc = "Get a reference to the value of field `last_auto_adjust_time` after provisioning.\n"]
    pub fn last_auto_adjust_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_auto_adjust_time", self.base))
    }

    #[doc = "Get a reference to the value of field `historical_options` after provisioning.\n"]
    pub fn historical_options(&self) -> terrars::ListRef<BudgetsBudget_AutoAdjustData_HistoricalOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.historical_options", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_CostFilter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BudgetsBudget_CostFilter { }

impl terrars::ToListMappable for BudgetsBudget_CostFilter {
    type O = terrars::BlockAssignable<BudgetsBudget_CostFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_CostFilter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBudgetsBudget_CostFilter {
    pub fn build(self) -> BudgetsBudget_CostFilter {
        BudgetsBudget_CostFilter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_CostFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_CostFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_CostFilterRef {
        BudgetsBudget_CostFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_CostFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_CostTypes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_credit: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_discount: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_other_subscription: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_recurring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_refund: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_subscription: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_tax: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    include_upfront: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_amortized: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_blended: std::option::Option<terrars::PrimField<bool>>,
}

impl BudgetsBudget_CostTypes {
    #[doc = "Set the field `include_credit`.\n"]
    pub fn set_include_credit(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_credit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_discount`.\n"]
    pub fn set_include_discount(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_discount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_other_subscription`.\n"]
    pub fn set_include_other_subscription(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_other_subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_recurring`.\n"]
    pub fn set_include_recurring(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_recurring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_refund`.\n"]
    pub fn set_include_refund(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_refund = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_subscription`.\n"]
    pub fn set_include_subscription(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_subscription = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_support`.\n"]
    pub fn set_include_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_tax`.\n"]
    pub fn set_include_tax(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_tax = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `include_upfront`.\n"]
    pub fn set_include_upfront(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.include_upfront = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_amortized`.\n"]
    pub fn set_use_amortized(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_amortized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `use_blended`.\n"]
    pub fn set_use_blended(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_blended = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_CostTypes {
    type O = terrars::BlockAssignable<BudgetsBudget_CostTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_CostTypes {}

impl BuildBudgetsBudget_CostTypes {
    pub fn build(self) -> BudgetsBudget_CostTypes {
        BudgetsBudget_CostTypes {
            include_credit: core::default::Default::default(),
            include_discount: core::default::Default::default(),
            include_other_subscription: core::default::Default::default(),
            include_recurring: core::default::Default::default(),
            include_refund: core::default::Default::default(),
            include_subscription: core::default::Default::default(),
            include_support: core::default::Default::default(),
            include_tax: core::default::Default::default(),
            include_upfront: core::default::Default::default(),
            use_amortized: core::default::Default::default(),
            use_blended: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_CostTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_CostTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_CostTypesRef {
        BudgetsBudget_CostTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_CostTypesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `include_credit` after provisioning.\n"]
    pub fn include_credit(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_credit", self.base))
    }

    #[doc = "Get a reference to the value of field `include_discount` after provisioning.\n"]
    pub fn include_discount(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_discount", self.base))
    }

    #[doc = "Get a reference to the value of field `include_other_subscription` after provisioning.\n"]
    pub fn include_other_subscription(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_other_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `include_recurring` after provisioning.\n"]
    pub fn include_recurring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_recurring", self.base))
    }

    #[doc = "Get a reference to the value of field `include_refund` after provisioning.\n"]
    pub fn include_refund(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_refund", self.base))
    }

    #[doc = "Get a reference to the value of field `include_subscription` after provisioning.\n"]
    pub fn include_subscription(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_subscription", self.base))
    }

    #[doc = "Get a reference to the value of field `include_support` after provisioning.\n"]
    pub fn include_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_support", self.base))
    }

    #[doc = "Get a reference to the value of field `include_tax` after provisioning.\n"]
    pub fn include_tax(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_tax", self.base))
    }

    #[doc = "Get a reference to the value of field `include_upfront` after provisioning.\n"]
    pub fn include_upfront(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.include_upfront", self.base))
    }

    #[doc = "Get a reference to the value of field `use_amortized` after provisioning.\n"]
    pub fn use_amortized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_amortized", self.base))
    }

    #[doc = "Get a reference to the value of field `use_blended` after provisioning.\n"]
    pub fn use_blended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_blended", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_CostCategories {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BudgetsBudget_FilterExpression_CostCategories {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_CostCategories {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_CostCategories {}

impl BuildBudgetsBudget_FilterExpression_CostCategories {
    pub fn build(self) -> BudgetsBudget_FilterExpression_CostCategories {
        BudgetsBudget_FilterExpression_CostCategories {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_CostCategoriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_CostCategoriesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudget_FilterExpression_CostCategoriesRef {
        BudgetsBudget_FilterExpression_CostCategoriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_CostCategoriesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Dimensions {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BudgetsBudget_FilterExpression_Dimensions {
    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Dimensions {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Dimensions {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBudgetsBudget_FilterExpression_Dimensions {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Dimensions {
        BudgetsBudget_FilterExpression_Dimensions {
            key: self.key,
            match_options: core::default::Default::default(),
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_DimensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_DimensionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_DimensionsRef {
        BudgetsBudget_FilterExpression_DimensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_DimensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BudgetsBudget_FilterExpression_Tags {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Tags {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Tags {}

impl BuildBudgetsBudget_FilterExpression_Tags {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Tags {
        BudgetsBudget_FilterExpression_Tags {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_TagsRef {
        BudgetsBudget_FilterExpression_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpression_Or_AndDynamic {
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Or_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpression_Or_AndDynamic,
}

impl BudgetsBudget_FilterExpression_Or_And {
    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Or_And {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Or_And {}

impl BuildBudgetsBudget_FilterExpression_Or_And {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Or_And {
        BudgetsBudget_FilterExpression_Or_And {
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_Or_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_Or_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_Or_AndRef {
        BudgetsBudget_FilterExpression_Or_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_Or_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpression_Or_NotDynamic {
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Or_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpression_Or_NotDynamic,
}

impl BudgetsBudget_FilterExpression_Or_Not {
    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Or_Not {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Or_Not {}

impl BuildBudgetsBudget_FilterExpression_Or_Not {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Or_Not {
        BudgetsBudget_FilterExpression_Or_Not {
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_Or_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_Or_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_Or_NotRef {
        BudgetsBudget_FilterExpression_Or_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_Or_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpression_Or_OrDynamic {
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Or_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpression_Or_OrDynamic,
}

impl BudgetsBudget_FilterExpression_Or_Or {
    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Or_Or {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Or_Or {}

impl BuildBudgetsBudget_FilterExpression_Or_Or {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Or_Or {
        BudgetsBudget_FilterExpression_Or_Or {
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_Or_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_Or_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_Or_OrRef {
        BudgetsBudget_FilterExpression_Or_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_Or_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpression_AndDynamic {
    and: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_And>>,
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    not: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_Not>>,
    or: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpression_AndDynamic,
}

impl BudgetsBudget_FilterExpression_And {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Not>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Or>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_And {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_And {}

impl BuildBudgetsBudget_FilterExpression_And {
    pub fn build(self) -> BudgetsBudget_FilterExpression_And {
        BudgetsBudget_FilterExpression_And {
            and: core::default::Default::default(),
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_AndRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_AndRef {
        BudgetsBudget_FilterExpression_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_OrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpression_NotDynamic {
    and: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_And>>,
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    not: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_Not>>,
    or: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpression_NotDynamic,
}

impl BudgetsBudget_FilterExpression_Not {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Not>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Or>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Not {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Not {}

impl BuildBudgetsBudget_FilterExpression_Not {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Not {
        BudgetsBudget_FilterExpression_Not {
            and: core::default::Default::default(),
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_NotRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_NotRef {
        BudgetsBudget_FilterExpression_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_OrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpression_OrDynamic {
    and: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_And>>,
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    not: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_Not>>,
    or: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpression_OrDynamic,
}

impl BudgetsBudget_FilterExpression_Or {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Not>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or_Or>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression_Or {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression_Or {}

impl BuildBudgetsBudget_FilterExpression_Or {
    pub fn build(self) -> BudgetsBudget_FilterExpression_Or {
        BudgetsBudget_FilterExpression_Or {
            and: core::default::Default::default(),
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpression_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpression_OrRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpression_OrRef {
        BudgetsBudget_FilterExpression_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpression_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_Or_OrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct BudgetsBudget_FilterExpressionDynamic {
    and: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_And>>,
    cost_categories: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_CostCategories>>,
    dimensions: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Dimensions>>,
    not: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Not>>,
    or: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<BudgetsBudget_FilterExpression_Tags>>,
}

#[derive(Serialize)]
pub struct BudgetsBudget_FilterExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_categories: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_CostCategories>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BudgetsBudget_FilterExpression_Tags>>,
    dynamic: BudgetsBudget_FilterExpressionDynamic,
}

impl BudgetsBudget_FilterExpression {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_categories`.\n"]
    pub fn set_cost_categories(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_CostCategories>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_categories = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_categories = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Not>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Or>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudget_FilterExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_FilterExpression {
    type O = terrars::BlockAssignable<BudgetsBudget_FilterExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_FilterExpression {}

impl BuildBudgetsBudget_FilterExpression {
    pub fn build(self) -> BudgetsBudget_FilterExpression {
        BudgetsBudget_FilterExpression {
            and: core::default::Default::default(),
            cost_categories: core::default::Default::default(),
            dimensions: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_FilterExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_FilterExpressionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_FilterExpressionRef {
        BudgetsBudget_FilterExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_FilterExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and` after provisioning.\n"]
    pub fn and(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_AndRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and", self.base))
    }

    #[doc = "Get a reference to the value of field `cost_categories` after provisioning.\n"]
    pub fn cost_categories(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_CostCategoriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_categories", self.base))
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `or` after provisioning.\n"]
    pub fn or(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_OrRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BudgetsBudget_FilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_Notification {
    comparison_operator: terrars::PrimField<std::string::String>,
    notification_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_email_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_sns_topic_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    threshold: terrars::PrimField<f64>,
    threshold_type: terrars::PrimField<std::string::String>,
}

impl BudgetsBudget_Notification {
    #[doc = "Set the field `subscriber_email_addresses`.\n"]
    pub fn set_subscriber_email_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subscriber_email_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber_sns_topic_arns`.\n"]
    pub fn set_subscriber_sns_topic_arns(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subscriber_sns_topic_arns = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BudgetsBudget_Notification {
    type O = terrars::BlockAssignable<BudgetsBudget_Notification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_Notification {
    #[doc = ""]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notification_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub threshold: terrars::PrimField<f64>,
    #[doc = ""]
    pub threshold_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudget_Notification {
    pub fn build(self) -> BudgetsBudget_Notification {
        BudgetsBudget_Notification {
            comparison_operator: self.comparison_operator,
            notification_type: self.notification_type,
            subscriber_email_addresses: core::default::Default::default(),
            subscriber_sns_topic_arns: core::default::Default::default(),
            threshold: self.threshold,
            threshold_type: self.threshold_type,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_NotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_NotificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_NotificationRef {
        BudgetsBudget_NotificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_NotificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `notification_type` after provisioning.\n"]
    pub fn notification_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subscriber_email_addresses` after provisioning.\n"]
    pub fn subscriber_email_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subscriber_email_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `subscriber_sns_topic_arns` after provisioning.\n"]
    pub fn subscriber_sns_topic_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subscriber_sns_topic_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold_type` after provisioning.\n"]
    pub fn threshold_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold_type", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudget_PlannedLimit {
    amount: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
    unit: terrars::PrimField<std::string::String>,
}

impl BudgetsBudget_PlannedLimit { }

impl terrars::ToListMappable for BudgetsBudget_PlannedLimit {
    type O = terrars::BlockAssignable<BudgetsBudget_PlannedLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudget_PlannedLimit {
    #[doc = ""]
    pub amount: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudget_PlannedLimit {
    pub fn build(self) -> BudgetsBudget_PlannedLimit {
        BudgetsBudget_PlannedLimit {
            amount: self.amount,
            start_time: self.start_time,
            unit: self.unit,
        }
    }
}

#[allow(dead_code)]
pub struct BudgetsBudget_PlannedLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudget_PlannedLimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudget_PlannedLimitRef {
        BudgetsBudget_PlannedLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudget_PlannedLimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `start_time` after provisioning.\n"]
    pub fn start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_time", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}
