use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsDelegatedServicesData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    account_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOrganizationsDelegatedServices_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOrganizationsDelegatedServicesData>,
}

#[derive(Clone)]
pub struct DataOrganizationsDelegatedServices(std::rc::Rc<DataOrganizationsDelegatedServices_>);

impl DataOrganizationsDelegatedServices {
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

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delegated_services` after provisioning.\n"]
    pub fn delegated_services(&self) -> terrars::SetRef<DataOrganizationsDelegatedServicesDelegatedServicesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delegated_services", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataOrganizationsDelegatedServices {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOrganizationsDelegatedServices { }

impl terrars::ToListMappable for DataOrganizationsDelegatedServices {
    type O = terrars::ListRef<DataOrganizationsDelegatedServicesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOrganizationsDelegatedServices_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_organizations_delegated_services".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsDelegatedServices {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub account_id: terrars::PrimField<std::string::String>,
}

impl BuildDataOrganizationsDelegatedServices {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOrganizationsDelegatedServices {
        let out = DataOrganizationsDelegatedServices(std::rc::Rc::new(DataOrganizationsDelegatedServices_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOrganizationsDelegatedServicesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                account_id: self.account_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsDelegatedServicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsDelegatedServicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationsDelegatedServicesRef {
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

    #[doc = "Get a reference to the value of field `delegated_services` after provisioning.\n"]
    pub fn delegated_services(&self) -> terrars::SetRef<DataOrganizationsDelegatedServicesDelegatedServicesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delegated_services", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsDelegatedServicesDelegatedServicesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delegation_enabled_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsDelegatedServicesDelegatedServicesEl {
    #[doc = "Set the field `delegation_enabled_date`.\n"]
    pub fn set_delegation_enabled_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.delegation_enabled_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_principal`.\n"]
    pub fn set_service_principal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_principal = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsDelegatedServicesDelegatedServicesEl {
    type O = terrars::BlockAssignable<DataOrganizationsDelegatedServicesDelegatedServicesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsDelegatedServicesDelegatedServicesEl {}

impl BuildDataOrganizationsDelegatedServicesDelegatedServicesEl {
    pub fn build(self) -> DataOrganizationsDelegatedServicesDelegatedServicesEl {
        DataOrganizationsDelegatedServicesDelegatedServicesEl {
            delegation_enabled_date: core::default::Default::default(),
            service_principal: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsDelegatedServicesDelegatedServicesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsDelegatedServicesDelegatedServicesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsDelegatedServicesDelegatedServicesElRef {
        DataOrganizationsDelegatedServicesDelegatedServicesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsDelegatedServicesDelegatedServicesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delegation_enabled_date` after provisioning.\n"]
    pub fn delegation_enabled_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegation_enabled_date", self.base))
    }

    #[doc = "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_principal", self.base))
    }
}
