use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataRedshiftClusterCredentialsData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_create: std::option::Option<terrars::PrimField<bool>>,
    cluster_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    db_name: std::option::Option<terrars::PrimField<std::string::String>>,
    db_user: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    duration_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataRedshiftClusterCredentials_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataRedshiftClusterCredentialsData>,
}

#[derive(Clone)]
pub struct DataRedshiftClusterCredentials(std::rc::Rc<DataRedshiftClusterCredentials_>);

impl DataRedshiftClusterCredentials {
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

    #[doc = "Set the field `auto_create`.\n"]
    pub fn set_auto_create(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().auto_create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_groups`.\n"]
    pub fn set_db_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().db_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `db_name`.\n"]
    pub fn set_db_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().db_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `duration_seconds`.\n"]
    pub fn set_duration_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().duration_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `auto_create` after provisioning.\n"]
    pub fn auto_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_groups` after provisioning.\n"]
    pub fn db_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_password` after provisioning.\n"]
    pub fn db_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `duration_seconds` after provisioning.\n"]
    pub fn duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataRedshiftClusterCredentials {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataRedshiftClusterCredentials { }

impl terrars::ToListMappable for DataRedshiftClusterCredentials {
    type O = terrars::ListRef<DataRedshiftClusterCredentialsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataRedshiftClusterCredentials_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_redshift_cluster_credentials".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataRedshiftClusterCredentials {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub db_user: terrars::PrimField<std::string::String>,
}

impl BuildDataRedshiftClusterCredentials {
    pub fn build(self, stack: &mut terrars::Stack) -> DataRedshiftClusterCredentials {
        let out = DataRedshiftClusterCredentials(std::rc::Rc::new(DataRedshiftClusterCredentials_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataRedshiftClusterCredentialsData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                auto_create: core::default::Default::default(),
                cluster_identifier: self.cluster_identifier,
                db_groups: core::default::Default::default(),
                db_name: core::default::Default::default(),
                db_user: self.db_user,
                duration_seconds: core::default::Default::default(),
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataRedshiftClusterCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataRedshiftClusterCredentialsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataRedshiftClusterCredentialsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `auto_create` after provisioning.\n"]
    pub fn auto_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_identifier` after provisioning.\n"]
    pub fn cluster_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_identifier", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_groups` after provisioning.\n"]
    pub fn db_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.db_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_name` after provisioning.\n"]
    pub fn db_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_password` after provisioning.\n"]
    pub fn db_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `db_user` after provisioning.\n"]
    pub fn db_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.db_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `duration_seconds` after provisioning.\n"]
    pub fn duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `expiration` after provisioning.\n"]
    pub fn expiration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
