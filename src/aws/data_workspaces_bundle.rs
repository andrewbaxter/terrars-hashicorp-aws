#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataWorkspacesBundleData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bundle_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataWorkspacesBundle_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataWorkspacesBundleData_>,
}

#[derive(Clone)]
pub struct DataWorkspacesBundle(std::rc::Rc<DataWorkspacesBundle_>);

impl DataWorkspacesBundle {
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

    #[doc = "Set the field `bundle_id`.\n"]
    pub fn set_bundle_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bundle_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner`.\n"]
    pub fn set_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().owner = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::ListRef<DataWorkspacesBundle_ComputeTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_storage` after provisioning.\n"]
    pub fn root_storage(&self) -> terrars::ListRef<DataWorkspacesBundle_RootStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_storage` after provisioning.\n"]
    pub fn user_storage(&self) -> terrars::ListRef<DataWorkspacesBundle_UserStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_storage", self.extract_ref()))
    }
}

impl terrars::Referable for DataWorkspacesBundle {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataWorkspacesBundle { }

impl terrars::ToListMappable for DataWorkspacesBundle {
    type O = terrars::ListRef<DataWorkspacesBundleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataWorkspacesBundle_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_workspaces_bundle".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataWorkspacesBundle {
    pub tf_id: std::string::String,
}

impl BuildDataWorkspacesBundle {
    pub fn build(self, stack: &mut terrars::Stack) -> DataWorkspacesBundle {
        let out = DataWorkspacesBundle(std::rc::Rc::new(DataWorkspacesBundle_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataWorkspacesBundleData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                bundle_id: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                owner: core::default::Default::default(),
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataWorkspacesBundleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesBundleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataWorkspacesBundleRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bundle_id` after provisioning.\n"]
    pub fn bundle_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bundle_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::ListRef<DataWorkspacesBundle_ComputeTypeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner` after provisioning.\n"]
    pub fn owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_storage` after provisioning.\n"]
    pub fn root_storage(&self) -> terrars::ListRef<DataWorkspacesBundle_RootStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_storage` after provisioning.\n"]
    pub fn user_storage(&self) -> terrars::ListRef<DataWorkspacesBundle_UserStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.user_storage", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesBundle_ComputeType {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesBundle_ComputeType {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesBundle_ComputeType {
    type O = terrars::BlockAssignable<DataWorkspacesBundle_ComputeType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesBundle_ComputeType {}

impl BuildDataWorkspacesBundle_ComputeType {
    pub fn build(self) -> DataWorkspacesBundle_ComputeType {
        DataWorkspacesBundle_ComputeType { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesBundle_ComputeTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesBundle_ComputeTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkspacesBundle_ComputeTypeRef {
        DataWorkspacesBundle_ComputeTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesBundle_ComputeTypeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesBundle_RootStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesBundle_RootStorage {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesBundle_RootStorage {
    type O = terrars::BlockAssignable<DataWorkspacesBundle_RootStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesBundle_RootStorage {}

impl BuildDataWorkspacesBundle_RootStorage {
    pub fn build(self) -> DataWorkspacesBundle_RootStorage {
        DataWorkspacesBundle_RootStorage { capacity: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesBundle_RootStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesBundle_RootStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkspacesBundle_RootStorageRef {
        DataWorkspacesBundle_RootStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesBundle_RootStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataWorkspacesBundle_UserStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataWorkspacesBundle_UserStorage {
    #[doc = "Set the field `capacity`.\n"]
    pub fn set_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataWorkspacesBundle_UserStorage {
    type O = terrars::BlockAssignable<DataWorkspacesBundle_UserStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataWorkspacesBundle_UserStorage {}

impl BuildDataWorkspacesBundle_UserStorage {
    pub fn build(self) -> DataWorkspacesBundle_UserStorage {
        DataWorkspacesBundle_UserStorage { capacity: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataWorkspacesBundle_UserStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataWorkspacesBundle_UserStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataWorkspacesBundle_UserStorageRef {
        DataWorkspacesBundle_UserStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataWorkspacesBundle_UserStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.base))
    }
}
