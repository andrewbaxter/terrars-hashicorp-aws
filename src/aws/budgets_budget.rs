use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BudgetsBudgetData {
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
    budget_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_filters: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_amount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    limit_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_period_end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_period_start: std::option::Option<terrars::PrimField<std::string::String>>,
    time_unit: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_adjust_data: std::option::Option<std::vec::Vec<BudgetsBudgetAutoAdjustDataEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_filter: std::option::Option<std::vec::Vec<BudgetsBudgetCostFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_types: std::option::Option<std::vec::Vec<BudgetsBudgetCostTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification: std::option::Option<std::vec::Vec<BudgetsBudgetNotificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    planned_limit: std::option::Option<std::vec::Vec<BudgetsBudgetPlannedLimitEl>>,
    dynamic: BudgetsBudgetDynamic,
}

struct BudgetsBudget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BudgetsBudgetData>,
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

    #[doc = "Set the field `cost_filters`.\n"]
    pub fn set_cost_filters(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().cost_filters = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAutoAdjustDataEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetCostFilterEl>>,
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
    pub fn set_cost_types(self, v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetCostTypesEl>>) -> Self {
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

    #[doc = "Set the field `notification`.\n"]
    pub fn set_notification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetNotificationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetPlannedLimitEl>>,
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

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_filters` after provisioning.\n"]
    pub fn cost_filters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.cost_filters", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
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
    pub fn auto_adjust_data(&self) -> terrars::ListRef<BudgetsBudgetAutoAdjustDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> terrars::ListRef<BudgetsBudgetCostTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
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
            data: std::cell::RefCell::new(BudgetsBudgetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                budget_type: self.budget_type,
                cost_filters: core::default::Default::default(),
                id: core::default::Default::default(),
                limit_amount: core::default::Default::default(),
                limit_unit: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                time_period_end: core::default::Default::default(),
                time_period_start: core::default::Default::default(),
                time_unit: self.time_unit,
                auto_adjust_data: core::default::Default::default(),
                cost_filter: core::default::Default::default(),
                cost_types: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_filters` after provisioning.\n"]
    pub fn cost_filters(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.cost_filters", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
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
    pub fn auto_adjust_data(&self) -> terrars::ListRef<BudgetsBudgetAutoAdjustDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> terrars::ListRef<BudgetsBudgetCostTypesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    budget_adjustment_period: terrars::PrimField<f64>,
}

impl BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl { }

impl terrars::ToListMappable for BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    type O = terrars::BlockAssignable<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    #[doc = ""]
    pub budget_adjustment_period: terrars::PrimField<f64>,
}

impl BuildBudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
    pub fn build(self) -> BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl {
        BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl { budget_adjustment_period: self.budget_adjustment_period }
    }
}

pub struct BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
        BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef {
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
struct BudgetsBudgetAutoAdjustDataElDynamic {
    historical_options: std::option::Option<terrars::DynamicBlock<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
}

#[derive(Serialize)]
pub struct BudgetsBudgetAutoAdjustDataEl {
    auto_adjust_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    historical_options: std::option::Option<std::vec::Vec<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
    dynamic: BudgetsBudgetAutoAdjustDataElDynamic,
}

impl BudgetsBudgetAutoAdjustDataEl {
    #[doc = "Set the field `historical_options`.\n"]
    pub fn set_historical_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BudgetsBudgetAutoAdjustDataElHistoricalOptionsEl>>,
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

impl terrars::ToListMappable for BudgetsBudgetAutoAdjustDataEl {
    type O = terrars::BlockAssignable<BudgetsBudgetAutoAdjustDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetAutoAdjustDataEl {
    #[doc = ""]
    pub auto_adjust_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetAutoAdjustDataEl {
    pub fn build(self) -> BudgetsBudgetAutoAdjustDataEl {
        BudgetsBudgetAutoAdjustDataEl {
            auto_adjust_type: self.auto_adjust_type,
            historical_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BudgetsBudgetAutoAdjustDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetAutoAdjustDataElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetAutoAdjustDataElRef {
        BudgetsBudgetAutoAdjustDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetAutoAdjustDataElRef {
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
    pub fn historical_options(&self) -> terrars::ListRef<BudgetsBudgetAutoAdjustDataElHistoricalOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.historical_options", self.base))
    }
}

#[derive(Serialize)]
pub struct BudgetsBudgetCostFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BudgetsBudgetCostFilterEl { }

impl terrars::ToListMappable for BudgetsBudgetCostFilterEl {
    type O = terrars::BlockAssignable<BudgetsBudgetCostFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetCostFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBudgetsBudgetCostFilterEl {
    pub fn build(self) -> BudgetsBudgetCostFilterEl {
        BudgetsBudgetCostFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct BudgetsBudgetCostFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetCostFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetCostFilterElRef {
        BudgetsBudgetCostFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetCostFilterElRef {
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
pub struct BudgetsBudgetCostTypesEl {
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

impl BudgetsBudgetCostTypesEl {
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

impl terrars::ToListMappable for BudgetsBudgetCostTypesEl {
    type O = terrars::BlockAssignable<BudgetsBudgetCostTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetCostTypesEl {}

impl BuildBudgetsBudgetCostTypesEl {
    pub fn build(self) -> BudgetsBudgetCostTypesEl {
        BudgetsBudgetCostTypesEl {
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

pub struct BudgetsBudgetCostTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetCostTypesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetCostTypesElRef {
        BudgetsBudgetCostTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetCostTypesElRef {
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
pub struct BudgetsBudgetNotificationEl {
    comparison_operator: terrars::PrimField<std::string::String>,
    notification_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_email_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_sns_topic_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    threshold: terrars::PrimField<f64>,
    threshold_type: terrars::PrimField<std::string::String>,
}

impl BudgetsBudgetNotificationEl {
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

impl terrars::ToListMappable for BudgetsBudgetNotificationEl {
    type O = terrars::BlockAssignable<BudgetsBudgetNotificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetNotificationEl {
    #[doc = ""]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub notification_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub threshold: terrars::PrimField<f64>,
    #[doc = ""]
    pub threshold_type: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetNotificationEl {
    pub fn build(self) -> BudgetsBudgetNotificationEl {
        BudgetsBudgetNotificationEl {
            comparison_operator: self.comparison_operator,
            notification_type: self.notification_type,
            subscriber_email_addresses: core::default::Default::default(),
            subscriber_sns_topic_arns: core::default::Default::default(),
            threshold: self.threshold,
            threshold_type: self.threshold_type,
        }
    }
}

pub struct BudgetsBudgetNotificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetNotificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetNotificationElRef {
        BudgetsBudgetNotificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetNotificationElRef {
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
pub struct BudgetsBudgetPlannedLimitEl {
    amount: terrars::PrimField<std::string::String>,
    start_time: terrars::PrimField<std::string::String>,
    unit: terrars::PrimField<std::string::String>,
}

impl BudgetsBudgetPlannedLimitEl { }

impl terrars::ToListMappable for BudgetsBudgetPlannedLimitEl {
    type O = terrars::BlockAssignable<BudgetsBudgetPlannedLimitEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBudgetsBudgetPlannedLimitEl {
    #[doc = ""]
    pub amount: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub start_time: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
}

impl BuildBudgetsBudgetPlannedLimitEl {
    pub fn build(self) -> BudgetsBudgetPlannedLimitEl {
        BudgetsBudgetPlannedLimitEl {
            amount: self.amount,
            start_time: self.start_time,
            unit: self.unit,
        }
    }
}

pub struct BudgetsBudgetPlannedLimitElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BudgetsBudgetPlannedLimitElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BudgetsBudgetPlannedLimitElRef {
        BudgetsBudgetPlannedLimitElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BudgetsBudgetPlannedLimitElRef {
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

#[derive(Serialize, Default)]
struct BudgetsBudgetDynamic {
    auto_adjust_data: std::option::Option<terrars::DynamicBlock<BudgetsBudgetAutoAdjustDataEl>>,
    cost_filter: std::option::Option<terrars::DynamicBlock<BudgetsBudgetCostFilterEl>>,
    cost_types: std::option::Option<terrars::DynamicBlock<BudgetsBudgetCostTypesEl>>,
    notification: std::option::Option<terrars::DynamicBlock<BudgetsBudgetNotificationEl>>,
    planned_limit: std::option::Option<terrars::DynamicBlock<BudgetsBudgetPlannedLimitEl>>,
}
