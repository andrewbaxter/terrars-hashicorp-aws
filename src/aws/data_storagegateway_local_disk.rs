use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataStoragegatewayLocalDiskData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_node: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk_path: std::option::Option<terrars::PrimField<std::string::String>>,
    gateway_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataStoragegatewayLocalDisk_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataStoragegatewayLocalDiskData>,
}

#[derive(Clone)]
pub struct DataStoragegatewayLocalDisk(std::rc::Rc<DataStoragegatewayLocalDisk_>);

impl DataStoragegatewayLocalDisk {
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

    #[doc = "Set the field `disk_node`.\n"]
    pub fn set_disk_node(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().disk_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disk_path`.\n"]
    pub fn set_disk_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().disk_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `disk_id` after provisioning.\n"]
    pub fn disk_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_node` after provisioning.\n"]
    pub fn disk_node(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_path` after provisioning.\n"]
    pub fn disk_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for DataStoragegatewayLocalDisk {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataStoragegatewayLocalDisk { }

impl terrars::ToListMappable for DataStoragegatewayLocalDisk {
    type O = terrars::ListRef<DataStoragegatewayLocalDiskRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataStoragegatewayLocalDisk_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_storagegateway_local_disk".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataStoragegatewayLocalDisk {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub gateway_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataStoragegatewayLocalDisk {
    pub fn build(self, stack: &mut terrars::Stack) -> DataStoragegatewayLocalDisk {
        let out = DataStoragegatewayLocalDisk(std::rc::Rc::new(DataStoragegatewayLocalDisk_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataStoragegatewayLocalDiskData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                disk_node: core::default::Default::default(),
                disk_path: core::default::Default::default(),
                gateway_arn: self.gateway_arn,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataStoragegatewayLocalDiskRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataStoragegatewayLocalDiskRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataStoragegatewayLocalDiskRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `disk_id` after provisioning.\n"]
    pub fn disk_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_node` after provisioning.\n"]
    pub fn disk_node(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_node", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disk_path` after provisioning.\n"]
    pub fn disk_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}
