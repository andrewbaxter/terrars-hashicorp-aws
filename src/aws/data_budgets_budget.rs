#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBudgetsBudgetData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataBudgetsBudget_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBudgetsBudgetData_>,
}

#[derive(Clone)]
pub struct DataBudgetsBudget(std::rc::Rc<DataBudgetsBudget_>);

impl DataBudgetsBudget {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> terrars::ListRef<DataBudgetsBudget_AutoAdjustDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_exceeded` after provisioning.\n"]
    pub fn budget_exceeded(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_exceeded", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_limit` after provisioning.\n"]
    pub fn budget_limit(&self) -> terrars::ListRef<DataBudgetsBudget_BudgetLimitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.budget_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `calculated_spend` after provisioning.\n"]
    pub fn calculated_spend(&self) -> terrars::ListRef<DataBudgetsBudget_CalculatedSpendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.calculated_spend", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_filter` after provisioning.\n"]
    pub fn cost_filter(&self) -> terrars::SetRef<DataBudgetsBudget_CostFilterRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cost_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> terrars::ListRef<DataBudgetsBudget_CostTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification` after provisioning.\n"]
    pub fn notification(&self) -> terrars::SetRef<DataBudgetsBudget_NotificationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.notification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `planned_limit` after provisioning.\n"]
    pub fn planned_limit(&self) -> terrars::SetRef<DataBudgetsBudget_PlannedLimitRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.planned_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
}

impl terrars::Referable for DataBudgetsBudget {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBudgetsBudget { }

impl terrars::ToListMappable for DataBudgetsBudget {
    type O = terrars::ListRef<DataBudgetsBudgetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBudgetsBudget_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_budgets_budget".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBudgetsBudget {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataBudgetsBudget {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBudgetsBudget {
        let out = DataBudgetsBudget(std::rc::Rc::new(DataBudgetsBudget_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBudgetsBudgetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                name_prefix: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBudgetsBudgetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudgetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBudgetsBudgetRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_adjust_data` after provisioning.\n"]
    pub fn auto_adjust_data(&self) -> terrars::ListRef<DataBudgetsBudget_AutoAdjustDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_adjust_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_arn` after provisioning.\n"]
    pub fn billing_view_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_exceeded` after provisioning.\n"]
    pub fn budget_exceeded(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_exceeded", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_limit` after provisioning.\n"]
    pub fn budget_limit(&self) -> terrars::ListRef<DataBudgetsBudget_BudgetLimitRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.budget_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `budget_type` after provisioning.\n"]
    pub fn budget_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.budget_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `calculated_spend` after provisioning.\n"]
    pub fn calculated_spend(&self) -> terrars::ListRef<DataBudgetsBudget_CalculatedSpendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.calculated_spend", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_filter` after provisioning.\n"]
    pub fn cost_filter(&self) -> terrars::SetRef<DataBudgetsBudget_CostFilterRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cost_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cost_types` after provisioning.\n"]
    pub fn cost_types(&self) -> terrars::ListRef<DataBudgetsBudget_CostTypesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification` after provisioning.\n"]
    pub fn notification(&self) -> terrars::SetRef<DataBudgetsBudget_NotificationRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.notification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `planned_limit` after provisioning.\n"]
    pub fn planned_limit(&self) -> terrars::SetRef<DataBudgetsBudget_PlannedLimitRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.planned_limit", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataBudgetsBudget_AutoAdjustData_HistoricalOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    budget_adjustment_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lookback_available_periods: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBudgetsBudget_AutoAdjustData_HistoricalOptions {
    #[doc = "Set the field `budget_adjustment_period`.\n"]
    pub fn set_budget_adjustment_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.budget_adjustment_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lookback_available_periods`.\n"]
    pub fn set_lookback_available_periods(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lookback_available_periods = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_AutoAdjustData_HistoricalOptions {
    type O = terrars::BlockAssignable<DataBudgetsBudget_AutoAdjustData_HistoricalOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_AutoAdjustData_HistoricalOptions {}

impl BuildDataBudgetsBudget_AutoAdjustData_HistoricalOptions {
    pub fn build(self) -> DataBudgetsBudget_AutoAdjustData_HistoricalOptions {
        DataBudgetsBudget_AutoAdjustData_HistoricalOptions {
            budget_adjustment_period: core::default::Default::default(),
            lookback_available_periods: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
        DataBudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_AutoAdjustData_HistoricalOptionsRef {
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

#[derive(Serialize)]
pub struct DataBudgetsBudget_AutoAdjustData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_adjust_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    historical_options: std::option::Option<terrars::ListField<DataBudgetsBudget_AutoAdjustData_HistoricalOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_auto_adjust_time: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBudgetsBudget_AutoAdjustData {
    #[doc = "Set the field `auto_adjust_type`.\n"]
    pub fn set_auto_adjust_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_adjust_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `historical_options`.\n"]
    pub fn set_historical_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBudgetsBudget_AutoAdjustData_HistoricalOptions>>,
    ) -> Self {
        self.historical_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `last_auto_adjust_time`.\n"]
    pub fn set_last_auto_adjust_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.last_auto_adjust_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_AutoAdjustData {
    type O = terrars::BlockAssignable<DataBudgetsBudget_AutoAdjustData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_AutoAdjustData {}

impl BuildDataBudgetsBudget_AutoAdjustData {
    pub fn build(self) -> DataBudgetsBudget_AutoAdjustData {
        DataBudgetsBudget_AutoAdjustData {
            auto_adjust_type: core::default::Default::default(),
            historical_options: core::default::Default::default(),
            last_auto_adjust_time: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_AutoAdjustDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_AutoAdjustDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_AutoAdjustDataRef {
        DataBudgetsBudget_AutoAdjustDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_AutoAdjustDataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_adjust_type` after provisioning.\n"]
    pub fn auto_adjust_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_adjust_type", self.base))
    }

    #[doc = "Get a reference to the value of field `historical_options` after provisioning.\n"]
    pub fn historical_options(&self) -> terrars::ListRef<DataBudgetsBudget_AutoAdjustData_HistoricalOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.historical_options", self.base))
    }

    #[doc = "Get a reference to the value of field `last_auto_adjust_time` after provisioning.\n"]
    pub fn last_auto_adjust_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_auto_adjust_time", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudget_BudgetLimit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBudgetsBudget_BudgetLimit {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_BudgetLimit {
    type O = terrars::BlockAssignable<DataBudgetsBudget_BudgetLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_BudgetLimit {}

impl BuildDataBudgetsBudget_BudgetLimit {
    pub fn build(self) -> DataBudgetsBudget_BudgetLimit {
        DataBudgetsBudget_BudgetLimit {
            amount: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_BudgetLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_BudgetLimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_BudgetLimitRef {
        DataBudgetsBudget_BudgetLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_BudgetLimitRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudget_CalculatedSpend_ActualSpend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBudgetsBudget_CalculatedSpend_ActualSpend {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_CalculatedSpend_ActualSpend {
    type O = terrars::BlockAssignable<DataBudgetsBudget_CalculatedSpend_ActualSpend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_CalculatedSpend_ActualSpend {}

impl BuildDataBudgetsBudget_CalculatedSpend_ActualSpend {
    pub fn build(self) -> DataBudgetsBudget_CalculatedSpend_ActualSpend {
        DataBudgetsBudget_CalculatedSpend_ActualSpend {
            amount: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_CalculatedSpend_ActualSpendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_CalculatedSpend_ActualSpendRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBudgetsBudget_CalculatedSpend_ActualSpendRef {
        DataBudgetsBudget_CalculatedSpend_ActualSpendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_CalculatedSpend_ActualSpendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudget_CalculatedSpend {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actual_spend: std::option::Option<terrars::ListField<DataBudgetsBudget_CalculatedSpend_ActualSpend>>,
}

impl DataBudgetsBudget_CalculatedSpend {
    #[doc = "Set the field `actual_spend`.\n"]
    pub fn set_actual_spend(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBudgetsBudget_CalculatedSpend_ActualSpend>>,
    ) -> Self {
        self.actual_spend = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_CalculatedSpend {
    type O = terrars::BlockAssignable<DataBudgetsBudget_CalculatedSpend>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_CalculatedSpend {}

impl BuildDataBudgetsBudget_CalculatedSpend {
    pub fn build(self) -> DataBudgetsBudget_CalculatedSpend {
        DataBudgetsBudget_CalculatedSpend { actual_spend: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_CalculatedSpendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_CalculatedSpendRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_CalculatedSpendRef {
        DataBudgetsBudget_CalculatedSpendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_CalculatedSpendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actual_spend` after provisioning.\n"]
    pub fn actual_spend(&self) -> terrars::ListRef<DataBudgetsBudget_CalculatedSpend_ActualSpendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actual_spend", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBudgetsBudget_CostFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataBudgetsBudget_CostFilter {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for DataBudgetsBudget_CostFilter {
    type O = terrars::BlockAssignable<DataBudgetsBudget_CostFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_CostFilter {}

impl BuildDataBudgetsBudget_CostFilter {
    pub fn build(self) -> DataBudgetsBudget_CostFilter {
        DataBudgetsBudget_CostFilter {
            name: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_CostFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_CostFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_CostFilterRef {
        DataBudgetsBudget_CostFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_CostFilterRef {
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
pub struct DataBudgetsBudget_CostTypes {
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

impl DataBudgetsBudget_CostTypes {
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

impl terrars::ToListMappable for DataBudgetsBudget_CostTypes {
    type O = terrars::BlockAssignable<DataBudgetsBudget_CostTypes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_CostTypes {}

impl BuildDataBudgetsBudget_CostTypes {
    pub fn build(self) -> DataBudgetsBudget_CostTypes {
        DataBudgetsBudget_CostTypes {
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
pub struct DataBudgetsBudget_CostTypesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_CostTypesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_CostTypesRef {
        DataBudgetsBudget_CostTypesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_CostTypesRef {
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
pub struct DataBudgetsBudget_Notification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comparison_operator: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_email_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber_sns_topic_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBudgetsBudget_Notification {
    #[doc = "Set the field `comparison_operator`.\n"]
    pub fn set_comparison_operator(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.comparison_operator = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_type`.\n"]
    pub fn set_notification_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.notification_type = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold_type`.\n"]
    pub fn set_threshold_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.threshold_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_Notification {
    type O = terrars::BlockAssignable<DataBudgetsBudget_Notification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_Notification {}

impl BuildDataBudgetsBudget_Notification {
    pub fn build(self) -> DataBudgetsBudget_Notification {
        DataBudgetsBudget_Notification {
            comparison_operator: core::default::Default::default(),
            notification_type: core::default::Default::default(),
            subscriber_email_addresses: core::default::Default::default(),
            subscriber_sns_topic_arns: core::default::Default::default(),
            threshold: core::default::Default::default(),
            threshold_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_NotificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_NotificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_NotificationRef {
        DataBudgetsBudget_NotificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_NotificationRef {
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
pub struct DataBudgetsBudget_PlannedLimit {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBudgetsBudget_PlannedLimit {
    #[doc = "Set the field `amount`.\n"]
    pub fn set_amount(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_time`.\n"]
    pub fn set_start_time(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBudgetsBudget_PlannedLimit {
    type O = terrars::BlockAssignable<DataBudgetsBudget_PlannedLimit>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBudgetsBudget_PlannedLimit {}

impl BuildDataBudgetsBudget_PlannedLimit {
    pub fn build(self) -> DataBudgetsBudget_PlannedLimit {
        DataBudgetsBudget_PlannedLimit {
            amount: core::default::Default::default(),
            start_time: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBudgetsBudget_PlannedLimitRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBudgetsBudget_PlannedLimitRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBudgetsBudget_PlannedLimitRef {
        DataBudgetsBudget_PlannedLimitRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBudgetsBudget_PlannedLimitRef {
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
