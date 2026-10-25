#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAccountPrimaryContactData_ {
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
}

struct DataAccountPrimaryContact_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountPrimaryContactData_>,
}

#[derive(Clone)]
pub struct DataAccountPrimaryContact(std::rc::Rc<DataAccountPrimaryContact_>);

impl DataAccountPrimaryContact {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_line_3` after provisioning.\n"]
    pub fn address_line_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_3", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_name` after provisioning.\n"]
    pub fn company_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `district_or_county` after provisioning.\n"]
    pub fn district_or_county(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.district_or_county", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_or_region` after provisioning.\n"]
    pub fn state_or_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_or_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_url` after provisioning.\n"]
    pub fn website_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.website_url", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountPrimaryContact {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountPrimaryContact { }

impl terrars::ToListMappable for DataAccountPrimaryContact {
    type O = terrars::ListRef<DataAccountPrimaryContactRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountPrimaryContact_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_account_primary_contact".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountPrimaryContact {
    pub tf_id: std::string::String,
}

impl BuildDataAccountPrimaryContact {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountPrimaryContact {
        let out = DataAccountPrimaryContact(std::rc::Rc::new(DataAccountPrimaryContact_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountPrimaryContactData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountPrimaryContactRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountPrimaryContactRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountPrimaryContactRef {
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

    #[doc = "Get a reference to the value of field `address_line_1` after provisioning.\n"]
    pub fn address_line_1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_1", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_line_2` after provisioning.\n"]
    pub fn address_line_2(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_2", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `address_line_3` after provisioning.\n"]
    pub fn address_line_3(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_line_3", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `city` after provisioning.\n"]
    pub fn city(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.city", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `company_name` after provisioning.\n"]
    pub fn company_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.company_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `country_code` after provisioning.\n"]
    pub fn country_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `district_or_county` after provisioning.\n"]
    pub fn district_or_county(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.district_or_county", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `full_name` after provisioning.\n"]
    pub fn full_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.full_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phone_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `postal_code` after provisioning.\n"]
    pub fn postal_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.postal_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_or_region` after provisioning.\n"]
    pub fn state_or_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_or_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `website_url` after provisioning.\n"]
    pub fn website_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.website_url", self.extract_ref()))
    }
}
