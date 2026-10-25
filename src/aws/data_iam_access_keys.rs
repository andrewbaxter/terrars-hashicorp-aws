#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataIamAccessKeysData_ {
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
    user: terrars::PrimField<std::string::String>,
}

struct DataIamAccessKeys_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataIamAccessKeysData_>,
}

#[derive(Clone)]
pub struct DataIamAccessKeys(std::rc::Rc<DataIamAccessKeys_>);

impl DataIamAccessKeys {
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

    #[doc = "Get a reference to the value of field `access_keys` after provisioning.\n"]
    pub fn access_keys(&self) -> terrars::SetRef<DataIamAccessKeys_AccessKeysRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

impl terrars::Referable for DataIamAccessKeys {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataIamAccessKeys { }

impl terrars::ToListMappable for DataIamAccessKeys {
    type O = terrars::ListRef<DataIamAccessKeysRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataIamAccessKeys_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_iam_access_keys".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataIamAccessKeys {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub user: terrars::PrimField<std::string::String>,
}

impl BuildDataIamAccessKeys {
    pub fn build(self, stack: &mut terrars::Stack) -> DataIamAccessKeys {
        let out = DataIamAccessKeys(std::rc::Rc::new(DataIamAccessKeys_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataIamAccessKeysData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                user: self.user,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataIamAccessKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamAccessKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataIamAccessKeysRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `access_keys` after provisioning.\n"]
    pub fn access_keys(&self) -> terrars::SetRef<DataIamAccessKeys_AccessKeysRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.access_keys", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user` after provisioning.\n"]
    pub fn user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataIamAccessKeys_AccessKeys {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataIamAccessKeys_AccessKeys {
    #[doc = "Set the field `access_key_id`.\n"]
    pub fn set_access_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_date`.\n"]
    pub fn set_create_date(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataIamAccessKeys_AccessKeys {
    type O = terrars::BlockAssignable<DataIamAccessKeys_AccessKeys>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataIamAccessKeys_AccessKeys {}

impl BuildDataIamAccessKeys_AccessKeys {
    pub fn build(self) -> DataIamAccessKeys_AccessKeys {
        DataIamAccessKeys_AccessKeys {
            access_key_id: core::default::Default::default(),
            create_date: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataIamAccessKeys_AccessKeysRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataIamAccessKeys_AccessKeysRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataIamAccessKeys_AccessKeysRef {
        DataIamAccessKeys_AccessKeysRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataIamAccessKeys_AccessKeysRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `create_date` after provisioning.\n"]
    pub fn create_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_date", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}
