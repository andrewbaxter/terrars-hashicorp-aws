use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataServicequotasServiceQuotaData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    quota_name: std::option::Option<terrars::PrimField<std::string::String>>,
    service_code: terrars::PrimField<std::string::String>,
}

struct DataServicequotasServiceQuota_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataServicequotasServiceQuotaData>,
}

#[derive(Clone)]
pub struct DataServicequotasServiceQuota(std::rc::Rc<DataServicequotasServiceQuota_>);

impl DataServicequotasServiceQuota {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_code`.\n"]
    pub fn set_quota_code(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `quota_name`.\n"]
    pub fn set_quota_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().quota_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}

impl terrars::Referable for DataServicequotasServiceQuota {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataServicequotasServiceQuota { }

impl terrars::ToListMappable for DataServicequotasServiceQuota {
    type O = terrars::ListRef<DataServicequotasServiceQuotaRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataServicequotasServiceQuota_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_servicequotas_service_quota".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataServicequotasServiceQuota {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub service_code: terrars::PrimField<std::string::String>,
}

impl BuildDataServicequotasServiceQuota {
    pub fn build(self, stack: &mut terrars::Stack) -> DataServicequotasServiceQuota {
        let out = DataServicequotasServiceQuota(std::rc::Rc::new(DataServicequotasServiceQuota_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataServicequotasServiceQuotaData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                quota_code: core::default::Default::default(),
                quota_name: core::default::Default::default(),
                service_code: self.service_code,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataServicequotasServiceQuotaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataServicequotasServiceQuotaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataServicequotasServiceQuotaRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `adjustable` after provisioning.\n"]
    pub fn adjustable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.adjustable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_value` after provisioning.\n"]
    pub fn default_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_value", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_quota` after provisioning.\n"]
    pub fn global_quota(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_quota", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_code` after provisioning.\n"]
    pub fn quota_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `quota_name` after provisioning.\n"]
    pub fn quota_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.quota_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_code` after provisioning.\n"]
    pub fn service_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.extract_ref()))
    }
}
