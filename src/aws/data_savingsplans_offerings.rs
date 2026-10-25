#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSavingsplansOfferingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currencies: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    descriptions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    durations: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offering_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plan_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_codes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataSavingsplansOfferings_Filter>>,
    dynamic: DataSavingsplansOfferingsDynamic,
}

struct DataSavingsplansOfferings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSavingsplansOfferingsData_>,
}

#[derive(Clone)]
pub struct DataSavingsplansOfferings(std::rc::Rc<DataSavingsplansOfferings_>);

impl DataSavingsplansOfferings {
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

    #[doc = "Set the field `currencies`.\n"]
    pub fn set_currencies(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().currencies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `descriptions`.\n"]
    pub fn set_descriptions(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().descriptions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `durations`.\n"]
    pub fn set_durations(self, v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>) -> Self {
        self.0.data.borrow_mut().durations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offering_ids`.\n"]
    pub fn set_offering_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().offering_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operations`.\n"]
    pub fn set_operations(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().operations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_options`.\n"]
    pub fn set_payment_options(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().payment_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plan_types`.\n"]
    pub fn set_plan_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().plan_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_type`.\n"]
    pub fn set_product_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().product_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_codes`.\n"]
    pub fn set_service_codes(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().service_codes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_types`.\n"]
    pub fn set_usage_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().usage_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataSavingsplansOfferings_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `currencies` after provisioning.\n"]
    pub fn currencies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.currencies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `descriptions` after provisioning.\n"]
    pub fn descriptions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.descriptions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `durations` after provisioning.\n"]
    pub fn durations(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.durations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offering_ids` after provisioning.\n"]
    pub fn offering_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.offering_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offerings` after provisioning.\n"]
    pub fn offerings(&self) -> terrars::ListRef<DataSavingsplansOfferings_OfferingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offerings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payment_options` after provisioning.\n"]
    pub fn payment_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.payment_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_types` after provisioning.\n"]
    pub fn plan_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.plan_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_type` after provisioning.\n"]
    pub fn product_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_codes` after provisioning.\n"]
    pub fn service_codes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_codes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_types` after provisioning.\n"]
    pub fn usage_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.usage_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataSavingsplansOfferings_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

impl terrars::Referable for DataSavingsplansOfferings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSavingsplansOfferings { }

impl terrars::ToListMappable for DataSavingsplansOfferings {
    type O = terrars::ListRef<DataSavingsplansOfferingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSavingsplansOfferings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_savingsplans_offerings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSavingsplansOfferings {
    pub tf_id: std::string::String,
}

impl BuildDataSavingsplansOfferings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSavingsplansOfferings {
        let out = DataSavingsplansOfferings(std::rc::Rc::new(DataSavingsplansOfferings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSavingsplansOfferingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                currencies: core::default::Default::default(),
                descriptions: core::default::Default::default(),
                durations: core::default::Default::default(),
                offering_ids: core::default::Default::default(),
                operations: core::default::Default::default(),
                payment_options: core::default::Default::default(),
                plan_types: core::default::Default::default(),
                product_type: core::default::Default::default(),
                service_codes: core::default::Default::default(),
                usage_types: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSavingsplansOfferingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSavingsplansOfferingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSavingsplansOfferingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `currencies` after provisioning.\n"]
    pub fn currencies(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.currencies", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `descriptions` after provisioning.\n"]
    pub fn descriptions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.descriptions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `durations` after provisioning.\n"]
    pub fn durations(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.durations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offering_ids` after provisioning.\n"]
    pub fn offering_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.offering_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `offerings` after provisioning.\n"]
    pub fn offerings(&self) -> terrars::ListRef<DataSavingsplansOfferings_OfferingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.offerings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operations` after provisioning.\n"]
    pub fn operations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.operations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payment_options` after provisioning.\n"]
    pub fn payment_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.payment_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_types` after provisioning.\n"]
    pub fn plan_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.plan_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `product_type` after provisioning.\n"]
    pub fn product_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.product_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_codes` after provisioning.\n"]
    pub fn service_codes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.service_codes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_types` after provisioning.\n"]
    pub fn usage_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.usage_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<DataSavingsplansOfferings_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataSavingsplansOfferingsDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataSavingsplansOfferings_Filter>>,
}

#[derive(Serialize)]
pub struct DataSavingsplansOfferings_Offerings_Properties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSavingsplansOfferings_Offerings_Properties {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSavingsplansOfferings_Offerings_Properties {
    type O = terrars::BlockAssignable<DataSavingsplansOfferings_Offerings_Properties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSavingsplansOfferings_Offerings_Properties {}

impl BuildDataSavingsplansOfferings_Offerings_Properties {
    pub fn build(self) -> DataSavingsplansOfferings_Offerings_Properties {
        DataSavingsplansOfferings_Offerings_Properties {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSavingsplansOfferings_Offerings_PropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSavingsplansOfferings_Offerings_PropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSavingsplansOfferings_Offerings_PropertiesRef {
        DataSavingsplansOfferings_Offerings_PropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSavingsplansOfferings_Offerings_PropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSavingsplansOfferings_Offerings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    offering_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operation: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    plan_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::ListField<DataSavingsplansOfferings_Offerings_Properties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSavingsplansOfferings_Offerings {
    #[doc = "Set the field `currency`.\n"]
    pub fn set_currency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration_seconds`.\n"]
    pub fn set_duration_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.duration_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `offering_id`.\n"]
    pub fn set_offering_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.offering_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operation`.\n"]
    pub fn set_operation(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.operation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payment_option`.\n"]
    pub fn set_payment_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payment_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `plan_type`.\n"]
    pub fn set_plan_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.plan_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `product_types`.\n"]
    pub fn set_product_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.product_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataSavingsplansOfferings_Offerings_Properties>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_code`.\n"]
    pub fn set_service_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_type`.\n"]
    pub fn set_usage_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.usage_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSavingsplansOfferings_Offerings {
    type O = terrars::BlockAssignable<DataSavingsplansOfferings_Offerings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSavingsplansOfferings_Offerings {}

impl BuildDataSavingsplansOfferings_Offerings {
    pub fn build(self) -> DataSavingsplansOfferings_Offerings {
        DataSavingsplansOfferings_Offerings {
            currency: core::default::Default::default(),
            description: core::default::Default::default(),
            duration_seconds: core::default::Default::default(),
            offering_id: core::default::Default::default(),
            operation: core::default::Default::default(),
            payment_option: core::default::Default::default(),
            plan_type: core::default::Default::default(),
            product_types: core::default::Default::default(),
            properties: core::default::Default::default(),
            service_code: core::default::Default::default(),
            usage_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSavingsplansOfferings_OfferingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSavingsplansOfferings_OfferingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSavingsplansOfferings_OfferingsRef {
        DataSavingsplansOfferings_OfferingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSavingsplansOfferings_OfferingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\n"]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `duration_seconds` after provisioning.\n"]
    pub fn duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `offering_id` after provisioning.\n"]
    pub fn offering_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.offering_id", self.base))
    }

    #[doc = "Get a reference to the value of field `operation` after provisioning.\n"]
    pub fn operation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operation", self.base))
    }

    #[doc = "Get a reference to the value of field `payment_option` after provisioning.\n"]
    pub fn payment_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payment_option", self.base))
    }

    #[doc = "Get a reference to the value of field `plan_type` after provisioning.\n"]
    pub fn plan_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_type", self.base))
    }

    #[doc = "Get a reference to the value of field `product_types` after provisioning.\n"]
    pub fn product_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.product_types", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::ListRef<DataSavingsplansOfferings_Offerings_PropertiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_type` after provisioning.\n"]
    pub fn usage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSavingsplansOfferings_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataSavingsplansOfferings_Filter { }

impl terrars::ToListMappable for DataSavingsplansOfferings_Filter {
    type O = terrars::BlockAssignable<DataSavingsplansOfferings_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSavingsplansOfferings_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataSavingsplansOfferings_Filter {
    pub fn build(self) -> DataSavingsplansOfferings_Filter {
        DataSavingsplansOfferings_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataSavingsplansOfferings_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSavingsplansOfferings_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSavingsplansOfferings_FilterRef {
        DataSavingsplansOfferings_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSavingsplansOfferings_FilterRef {
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
