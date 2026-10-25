#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataS3filesAccessPointData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_user: std::option::Option<std::vec::Vec<DataS3filesAccessPoint_PosixUser>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<std::vec::Vec<DataS3filesAccessPoint_RootDirectory>>,
    dynamic: DataS3filesAccessPointDynamic,
}

struct DataS3filesAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataS3filesAccessPointData_>,
}

#[derive(Clone)]
pub struct DataS3filesAccessPoint(std::rc::Rc<DataS3filesAccessPoint_>);

impl DataS3filesAccessPoint {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `posix_user`.\n"]
    pub fn set_posix_user(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataS3filesAccessPoint_PosixUser>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().posix_user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.posix_user = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataS3filesAccessPoint_RootDirectory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_directory = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_directory = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\nFile system ID"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAccess point ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAccess point name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\nAWS account ID of the owner"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nAccess point status"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<DataS3filesAccessPoint_PosixUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<DataS3filesAccessPoint_RootDirectoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }
}

impl terrars::Referable for DataS3filesAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataS3filesAccessPoint { }

impl terrars::ToListMappable for DataS3filesAccessPoint {
    type O = terrars::ListRef<DataS3filesAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataS3filesAccessPoint_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_s3files_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataS3filesAccessPoint {
    pub tf_id: std::string::String,
    #[doc = "Access point ID"]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataS3filesAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> DataS3filesAccessPoint {
        let out = DataS3filesAccessPoint(std::rc::Rc::new(DataS3filesAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataS3filesAccessPointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
                posix_user: core::default::Default::default(),
                root_directory: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataS3filesAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3filesAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataS3filesAccessPointRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\nFile system ID"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\nAccess point ID"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nAccess point name"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\nAWS account ID of the owner"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\nAccess point status"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<DataS3filesAccessPoint_PosixUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<DataS3filesAccessPoint_RootDirectoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataS3filesAccessPointDynamic {
    posix_user: std::option::Option<terrars::DynamicBlock<DataS3filesAccessPoint_PosixUser>>,
    root_directory: std::option::Option<terrars::DynamicBlock<DataS3filesAccessPoint_RootDirectory>>,
}

#[derive(Serialize)]
pub struct DataS3filesAccessPoint_PosixUser {}

impl DataS3filesAccessPoint_PosixUser { }

impl terrars::ToListMappable for DataS3filesAccessPoint_PosixUser {
    type O = terrars::BlockAssignable<DataS3filesAccessPoint_PosixUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3filesAccessPoint_PosixUser {}

impl BuildDataS3filesAccessPoint_PosixUser {
    pub fn build(self) -> DataS3filesAccessPoint_PosixUser {
        DataS3filesAccessPoint_PosixUser {}
    }
}

#[allow(dead_code)]
pub struct DataS3filesAccessPoint_PosixUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3filesAccessPoint_PosixUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3filesAccessPoint_PosixUserRef {
        DataS3filesAccessPoint_PosixUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3filesAccessPoint_PosixUserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\nPOSIX group ID"]
    pub fn gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_gids` after provisioning.\nSecondary POSIX group IDs"]
    pub fn secondary_gids(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_gids", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\nPOSIX user ID"]
    pub fn uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize)]
pub struct DataS3filesAccessPoint_RootDirectory_CreationPermissions {}

impl DataS3filesAccessPoint_RootDirectory_CreationPermissions { }

impl terrars::ToListMappable for DataS3filesAccessPoint_RootDirectory_CreationPermissions {
    type O = terrars::BlockAssignable<DataS3filesAccessPoint_RootDirectory_CreationPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3filesAccessPoint_RootDirectory_CreationPermissions {}

impl BuildDataS3filesAccessPoint_RootDirectory_CreationPermissions {
    pub fn build(self) -> DataS3filesAccessPoint_RootDirectory_CreationPermissions {
        DataS3filesAccessPoint_RootDirectory_CreationPermissions {}
    }
}

#[allow(dead_code)]
pub struct DataS3filesAccessPoint_RootDirectory_CreationPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3filesAccessPoint_RootDirectory_CreationPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataS3filesAccessPoint_RootDirectory_CreationPermissionsRef {
        DataS3filesAccessPoint_RootDirectory_CreationPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3filesAccessPoint_RootDirectory_CreationPermissionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `owner_gid` after provisioning.\nOwner group ID"]
    pub fn owner_gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_gid", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_uid` after provisioning.\nOwner user ID"]
    pub fn owner_uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_uid", self.base))
    }

    #[doc = "Get a reference to the value of field `permissions` after provisioning.\nPOSIX permissions"]
    pub fn permissions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permissions", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataS3filesAccessPoint_RootDirectoryDynamic {
    creation_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<DataS3filesAccessPoint_RootDirectory_CreationPermissions>>,
}

#[derive(Serialize)]
pub struct DataS3filesAccessPoint_RootDirectory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_permissions: std::option::Option<std::vec::Vec<DataS3filesAccessPoint_RootDirectory_CreationPermissions>>,
    dynamic: DataS3filesAccessPoint_RootDirectoryDynamic,
}

impl DataS3filesAccessPoint_RootDirectory {
    #[doc = "Set the field `creation_permissions`.\n"]
    pub fn set_creation_permissions(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataS3filesAccessPoint_RootDirectory_CreationPermissions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.creation_permissions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.creation_permissions = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataS3filesAccessPoint_RootDirectory {
    type O = terrars::BlockAssignable<DataS3filesAccessPoint_RootDirectory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataS3filesAccessPoint_RootDirectory {}

impl BuildDataS3filesAccessPoint_RootDirectory {
    pub fn build(self) -> DataS3filesAccessPoint_RootDirectory {
        DataS3filesAccessPoint_RootDirectory {
            creation_permissions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataS3filesAccessPoint_RootDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataS3filesAccessPoint_RootDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataS3filesAccessPoint_RootDirectoryRef {
        DataS3filesAccessPoint_RootDirectoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataS3filesAccessPoint_RootDirectoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nRoot directory path"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_permissions` after provisioning.\n"]
    pub fn creation_permissions(&self) -> terrars::ListRef<DataS3filesAccessPoint_RootDirectory_CreationPermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_permissions", self.base))
    }
}
