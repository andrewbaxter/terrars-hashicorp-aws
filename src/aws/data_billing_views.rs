#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBillingViewsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_view_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

struct DataBillingViews_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBillingViewsData_>,
}

#[derive(Clone)]
pub struct DataBillingViews(std::rc::Rc<DataBillingViews_>);

impl DataBillingViews {
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

    #[doc = "Set the field `billing_view_types`.\n"]
    pub fn set_billing_view_types(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().billing_view_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `billing_view` after provisioning.\n"]
    pub fn billing_view(&self) -> terrars::ListRef<DataBillingViews_BillingViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_types` after provisioning.\n"]
    pub fn billing_view_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_view_types", self.extract_ref()))
    }
}

impl terrars::Referable for DataBillingViews {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBillingViews { }

impl terrars::ToListMappable for DataBillingViews {
    type O = terrars::ListRef<DataBillingViewsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBillingViews_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_billing_views".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBillingViews {
    pub tf_id: std::string::String,
}

impl BuildDataBillingViews {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBillingViews {
        let out = DataBillingViews(std::rc::Rc::new(DataBillingViews_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBillingViewsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                billing_view_types: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBillingViewsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBillingViewsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBillingViewsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `billing_view` after provisioning.\n"]
    pub fn billing_view(&self) -> terrars::ListRef<DataBillingViews_BillingViewRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_view", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_types` after provisioning.\n"]
    pub fn billing_view_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.billing_view_types", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBillingViews_BillingView {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    billing_view_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_account_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBillingViews_BillingView {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `billing_view_type`.\n"]
    pub fn set_billing_view_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.billing_view_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_account_id`.\n"]
    pub fn set_owner_account_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner_account_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBillingViews_BillingView {
    type O = terrars::BlockAssignable<DataBillingViews_BillingView>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBillingViews_BillingView {}

impl BuildDataBillingViews_BillingView {
    pub fn build(self) -> DataBillingViews_BillingView {
        DataBillingViews_BillingView {
            arn: core::default::Default::default(),
            billing_view_type: core::default::Default::default(),
            description: core::default::Default::default(),
            name: core::default::Default::default(),
            owner_account_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBillingViews_BillingViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBillingViews_BillingViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBillingViews_BillingViewRef {
        DataBillingViews_BillingViewRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBillingViews_BillingViewRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `billing_view_type` after provisioning.\n"]
    pub fn billing_view_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_type", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.base))
    }
}
