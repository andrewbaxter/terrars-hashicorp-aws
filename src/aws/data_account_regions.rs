#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAccountRegionsData_ {
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
    region_opt_status_contains: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

struct DataAccountRegions_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountRegionsData_>,
}

#[derive(Clone)]
pub struct DataAccountRegions(std::rc::Rc<DataAccountRegions_>);

impl DataAccountRegions {
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

    #[doc = "Set the field `region_opt_status_contains`.\n"]
    pub fn set_region_opt_status_contains(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().region_opt_status_contains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `region_opt_status_contains` after provisioning.\n"]
    pub fn region_opt_status_contains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_opt_status_contains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<DataAccountRegions_RegionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountRegions {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountRegions { }

impl terrars::ToListMappable for DataAccountRegions {
    type O = terrars::ListRef<DataAccountRegionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountRegions_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_account_regions".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountRegions {
    pub tf_id: std::string::String,
}

impl BuildDataAccountRegions {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountRegions {
        let out = DataAccountRegions(std::rc::Rc::new(DataAccountRegions_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountRegionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: core::default::Default::default(),
                region_opt_status_contains: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountRegionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRegionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountRegionsRef {
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

    #[doc = "Get a reference to the value of field `region_opt_status_contains` after provisioning.\n"]
    pub fn region_opt_status_contains(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.region_opt_status_contains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::ListRef<DataAccountRegions_RegionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountRegions_Regions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region_opt_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountRegions_Regions {
    #[doc = "Set the field `region_name`.\n"]
    pub fn set_region_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region_opt_status`.\n"]
    pub fn set_region_opt_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region_opt_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountRegions_Regions {
    type O = terrars::BlockAssignable<DataAccountRegions_Regions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountRegions_Regions {}

impl BuildDataAccountRegions_Regions {
    pub fn build(self) -> DataAccountRegions_Regions {
        DataAccountRegions_Regions {
            region_name: core::default::Default::default(),
            region_opt_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAccountRegions_RegionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountRegions_RegionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountRegions_RegionsRef {
        DataAccountRegions_RegionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountRegions_RegionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region_name` after provisioning.\n"]
    pub fn region_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_name", self.base))
    }

    #[doc = "Get a reference to the value of field `region_opt_status` after provisioning.\n"]
    pub fn region_opt_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region_opt_status", self.base))
    }
}
