#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SavingsplansSavingsPlanData_ {
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
    commitment: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    purchase_time: std::option::Option<terrars::PrimField<std::string::String>>,
    savings_plan_offering_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upfront_payment_amount: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SavingsplansSavingsPlan_Timeouts>,
}

struct SavingsplansSavingsPlan_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SavingsplansSavingsPlanData_>,
}

#[derive(Clone)]
pub struct SavingsplansSavingsPlan(std::rc::Rc<SavingsplansSavingsPlan_>);

impl SavingsplansSavingsPlan {
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
        "Set the field `purchase_time`.\nThe time at which to purchase the Savings Plan, in UTC format (YYYY-MM-DDTHH:MM:SSZ)."]
    pub fn set_purchase_time(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().purchase_time = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `upfront_payment_amount`.\nThe up-front payment amount."]
    pub fn set_upfront_payment_amount(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().upfront_payment_amount = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SavingsplansSavingsPlan_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `savings_plan_id` after provisioning.\n"]
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SavingsplansSavingsPlan_TimeoutsRef {
        SavingsplansSavingsPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SavingsplansSavingsPlan {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SavingsplansSavingsPlan { }

impl terrars::ToListMappable for SavingsplansSavingsPlan {
    type O = terrars::ListRef<SavingsplansSavingsPlanRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SavingsplansSavingsPlan_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_savingsplans_savings_plan".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSavingsplansSavingsPlan {
    pub tf_id: std::string::String,
    #[doc = "The hourly commitment, in USD."]
    pub commitment: terrars::PrimField<std::string::String>,
    #[doc = "The unique ID of a Savings Plan offering."]
    pub savings_plan_offering_id: terrars::PrimField<std::string::String>,
}

impl BuildSavingsplansSavingsPlan {
    pub fn build(self, stack: &mut terrars::Stack) -> SavingsplansSavingsPlan {
        let out = SavingsplansSavingsPlan(std::rc::Rc::new(SavingsplansSavingsPlan_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SavingsplansSavingsPlanData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                commitment: self.commitment,
                purchase_time: core::default::Default::default(),
                savings_plan_offering_id: self.savings_plan_offering_id,
                tags: core::default::Default::default(),
                upfront_payment_amount: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SavingsplansSavingsPlanRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SavingsplansSavingsPlanRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SavingsplansSavingsPlanRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `savings_plan_id` after provisioning.\n"]
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SavingsplansSavingsPlan_TimeoutsRef {
        SavingsplansSavingsPlan_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SavingsplansSavingsPlan_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SavingsplansSavingsPlan_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SavingsplansSavingsPlan_Timeouts {
    type O = terrars::BlockAssignable<SavingsplansSavingsPlan_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSavingsplansSavingsPlan_Timeouts {}

impl BuildSavingsplansSavingsPlan_Timeouts {
    pub fn build(self) -> SavingsplansSavingsPlan_Timeouts {
        SavingsplansSavingsPlan_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SavingsplansSavingsPlan_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SavingsplansSavingsPlan_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SavingsplansSavingsPlan_TimeoutsRef {
        SavingsplansSavingsPlan_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SavingsplansSavingsPlan_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
