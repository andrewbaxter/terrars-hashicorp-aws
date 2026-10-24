use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOrganizationsDelegatedAdministratorsData {
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
    service_principal: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOrganizationsDelegatedAdministrators_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOrganizationsDelegatedAdministratorsData>,
}

#[derive(Clone)]
pub struct DataOrganizationsDelegatedAdministrators(std::rc::Rc<DataOrganizationsDelegatedAdministrators_>);

impl DataOrganizationsDelegatedAdministrators {
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

    #[doc = "Set the field `service_principal`.\n"]
    pub fn set_service_principal(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_principal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `delegated_administrators` after provisioning.\n"]
    pub fn delegated_administrators(
        &self,
    ) -> terrars::SetRef<DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delegated_administrators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_principal", self.extract_ref()))
    }
}

impl terrars::Referable for DataOrganizationsDelegatedAdministrators {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOrganizationsDelegatedAdministrators { }

impl terrars::ToListMappable for DataOrganizationsDelegatedAdministrators {
    type O = terrars::ListRef<DataOrganizationsDelegatedAdministratorsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOrganizationsDelegatedAdministrators_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_organizations_delegated_administrators".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOrganizationsDelegatedAdministrators {
    pub tf_id: std::string::String,
}

impl BuildDataOrganizationsDelegatedAdministrators {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOrganizationsDelegatedAdministrators {
        let out =
            DataOrganizationsDelegatedAdministrators(std::rc::Rc::new(DataOrganizationsDelegatedAdministrators_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: std::cell::RefCell::new(DataOrganizationsDelegatedAdministratorsData {
                    depends_on: core::default::Default::default(),
                    provider: std::option::Option::None,
                    lifecycle: core::default::Default::default(),
                    for_each: std::option::Option::None,
                    id: core::default::Default::default(),
                    service_principal: core::default::Default::default(),
                }),
            }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOrganizationsDelegatedAdministratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsDelegatedAdministratorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOrganizationsDelegatedAdministratorsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `delegated_administrators` after provisioning.\n"]
    pub fn delegated_administrators(
        &self,
    ) -> terrars::SetRef<DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.delegated_administrators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_principal` after provisioning.\n"]
    pub fn service_principal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_principal", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delegation_enabled_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    joined_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    joined_timestamp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delegation_enabled_date`.\n"]
    pub fn set_delegation_enabled_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.delegation_enabled_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email`.\n"]
    pub fn set_email(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.email = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `joined_method`.\n"]
    pub fn set_joined_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.joined_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `joined_timestamp`.\n"]
    pub fn set_joined_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.joined_timestamp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    type O = terrars::BlockAssignable<DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {}

impl BuildDataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
    pub fn build(self) -> DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
        DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsEl {
            arn: core::default::Default::default(),
            delegation_enabled_date: core::default::Default::default(),
            email: core::default::Default::default(),
            id: core::default::Default::default(),
            joined_method: core::default::Default::default(),
            joined_timestamp: core::default::Default::default(),
            name: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

pub struct DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
        DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOrganizationsDelegatedAdministratorsDelegatedAdministratorsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `delegation_enabled_date` after provisioning.\n"]
    pub fn delegation_enabled_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delegation_enabled_date", self.base))
    }

    #[doc = "Get a reference to the value of field `email` after provisioning.\n"]
    pub fn email(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_method` after provisioning.\n"]
    pub fn joined_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.joined_method", self.base))
    }

    #[doc = "Get a reference to the value of field `joined_timestamp` after provisioning.\n"]
    pub fn joined_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.joined_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
