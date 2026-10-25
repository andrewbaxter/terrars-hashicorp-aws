#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSavingsplansSavingsPlanData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    savings_plan_id: terrars::PrimField<std::string::String>,
}

struct DataSavingsplansSavingsPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSavingsplansSavingsPlanData_>,
}

#[derive(Clone)]
pub struct DataSavingsplansSavingsPlan(std::rc::Rc<DataSavingsplansSavingsPlan_>);

impl DataSavingsplansSavingsPlan {
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

    #[doc = "Get a reference to the value of field `commitment` after provisioning.\nThe hourly commitment, in USD."]
    pub fn commitment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\nThe currency of the Savings Plan."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ec2_instance_family` after provisioning.\nThe EC2 instance family for the Savings Plan."]
    pub fn ec2_instance_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\nThe end time of the Savings Plan."]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offering_id` after provisioning.\nThe ID of the offering."]
    pub fn offering_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offering_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_option` after provisioning.\nThe payment option for the Savings Plan."]
    pub fn payment_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_option", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_types` after provisioning.\nThe product types."]
    pub fn product_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.product_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purchase_time` after provisioning.\nThe time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ)."]
    pub fn purchase_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purchase_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_payment_amount` after provisioning.\nThe recurring payment amount."]
    pub fn recurring_payment_amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_payment_amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe AWS Region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `returnable_until` after provisioning.\nThe recurring payment amount."]
    pub fn returnable_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.returnable_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_plan_arn` after provisioning.\n"]
    pub fn savings_plan_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_plan_id` after provisioning.\nThe ID of the Savings Plan."]
    pub fn savings_plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `savings_plan_offering_id` after provisioning.\nThe unique ID of a Savings Plan offering."]
    pub fn savings_plan_offering_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_offering_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_plan_type` after provisioning.\nThe type of Savings Plan."]
    pub fn savings_plan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\nThe start time of the Savings Plan."]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the Savings Plan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `term_duration_in_seconds` after provisioning.\nThe duration of the term, in seconds."]
    pub fn term_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.term_duration_in_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upfront_payment_amount` after provisioning.\nThe up-front payment amount."]
    pub fn upfront_payment_amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upfront_payment_amount", self.extract_ref()))
    }
}

impl terrars::Referable for DataSavingsplansSavingsPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSavingsplansSavingsPlan { }

impl terrars::ToListMappable for DataSavingsplansSavingsPlan {
    type O = terrars::ListRef<DataSavingsplansSavingsPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSavingsplansSavingsPlan_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_savingsplans_savings_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSavingsplansSavingsPlan {
    pub tf_id: std::string::String,
    #[doc = "The ID of the Savings Plan."]
    pub savings_plan_id: terrars::PrimField<std::string::String>,
}

impl BuildDataSavingsplansSavingsPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSavingsplansSavingsPlan {
        let out = DataSavingsplansSavingsPlan(std::rc::Rc::new(DataSavingsplansSavingsPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSavingsplansSavingsPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                savings_plan_id: self.savings_plan_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSavingsplansSavingsPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSavingsplansSavingsPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSavingsplansSavingsPlanRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `commitment` after provisioning.\nThe hourly commitment, in USD."]
    pub fn commitment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\nThe currency of the Savings Plan."]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\nThe description."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `ec2_instance_family` after provisioning.\nThe EC2 instance family for the Savings Plan."]
    pub fn ec2_instance_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\nThe end time of the Savings Plan."]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offering_id` after provisioning.\nThe ID of the offering."]
    pub fn offering_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offering_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `payment_option` after provisioning.\nThe payment option for the Savings Plan."]
    pub fn payment_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_option", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_types` after provisioning.\nThe product types."]
    pub fn product_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.product_types", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `purchase_time` after provisioning.\nThe time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ)."]
    pub fn purchase_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.purchase_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `recurring_payment_amount` after provisioning.\nThe recurring payment amount."]
    pub fn recurring_payment_amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recurring_payment_amount", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\nThe AWS Region."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `returnable_until` after provisioning.\nThe recurring payment amount."]
    pub fn returnable_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.returnable_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_plan_arn` after provisioning.\n"]
    pub fn savings_plan_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_plan_id` after provisioning.\nThe ID of the Savings Plan."]
    pub fn savings_plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `savings_plan_offering_id` after provisioning.\nThe unique ID of a Savings Plan offering."]
    pub fn savings_plan_offering_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_offering_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `savings_plan_type` after provisioning.\nThe type of Savings Plan."]
    pub fn savings_plan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.savings_plan_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\nThe start time of the Savings Plan."]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\nThe current state of the Savings Plan."]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `term_duration_in_seconds` after provisioning.\nThe duration of the term, in seconds."]
    pub fn term_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.term_duration_in_seconds", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `upfront_payment_amount` after provisioning.\nThe up-front payment amount."]
    pub fn upfront_payment_amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upfront_payment_amount", self.extract_ref()))
    }
}
