#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct S3filesAccessPointData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_user: std::option::Option<std::vec::Vec<S3filesAccessPoint_PosixUser>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<std::vec::Vec<S3filesAccessPoint_RootDirectory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<S3filesAccessPoint_Timeouts>,
    dynamic: S3filesAccessPointDynamic,
}

struct S3filesAccessPoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<S3filesAccessPointData_>,
}

#[derive(Clone)]
pub struct S3filesAccessPoint(std::rc::Rc<S3filesAccessPoint_>);

impl S3filesAccessPoint {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `posix_user`.\n"]
    pub fn set_posix_user(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3filesAccessPoint_PosixUser>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<S3filesAccessPoint_RootDirectory>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<S3filesAccessPoint_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<S3filesAccessPoint_PosixUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<S3filesAccessPoint_RootDirectoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3filesAccessPoint_TimeoutsRef {
        S3filesAccessPoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for S3filesAccessPoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for S3filesAccessPoint { }

impl terrars::ToListMappable for S3filesAccessPoint {
    type O = terrars::ListRef<S3filesAccessPointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for S3filesAccessPoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_s3files_access_point".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildS3filesAccessPoint {
    pub tf_id: std::string::String,
    #[doc = "File system ID"]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildS3filesAccessPoint {
    pub fn build(self, stack: &mut terrars::Stack) -> S3filesAccessPoint {
        let out = S3filesAccessPoint(std::rc::Rc::new(S3filesAccessPoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(S3filesAccessPointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                file_system_id: self.file_system_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                posix_user: core::default::Default::default(),
                root_directory: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct S3filesAccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesAccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl S3filesAccessPointRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\nFile system ID"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(&self) -> terrars::ListRef<S3filesAccessPoint_PosixUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::ListRef<S3filesAccessPoint_RootDirectoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_directory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> S3filesAccessPoint_TimeoutsRef {
        S3filesAccessPoint_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct S3filesAccessPointDynamic {
    posix_user: std::option::Option<terrars::DynamicBlock<S3filesAccessPoint_PosixUser>>,
    root_directory: std::option::Option<terrars::DynamicBlock<S3filesAccessPoint_RootDirectory>>,
}

#[derive(Serialize)]
pub struct S3filesAccessPoint_PosixUser {
    gid: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_gids: std::option::Option<terrars::SetField<terrars::PrimField<f64>>>,
    uid: terrars::PrimField<f64>,
}

impl S3filesAccessPoint_PosixUser {
    #[doc = "Set the field `secondary_gids`.\nSecondary POSIX group IDs"]
    pub fn set_secondary_gids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.secondary_gids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3filesAccessPoint_PosixUser {
    type O = terrars::BlockAssignable<S3filesAccessPoint_PosixUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3filesAccessPoint_PosixUser {
    #[doc = "POSIX group ID"]
    pub gid: terrars::PrimField<f64>,
    #[doc = "POSIX user ID"]
    pub uid: terrars::PrimField<f64>,
}

impl BuildS3filesAccessPoint_PosixUser {
    pub fn build(self) -> S3filesAccessPoint_PosixUser {
        S3filesAccessPoint_PosixUser {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}

#[allow(dead_code)]
pub struct S3filesAccessPoint_PosixUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesAccessPoint_PosixUserRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3filesAccessPoint_PosixUserRef {
        S3filesAccessPoint_PosixUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3filesAccessPoint_PosixUserRef {
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
pub struct S3filesAccessPoint_RootDirectory_CreationPermissions {
    owner_gid: terrars::PrimField<f64>,
    owner_uid: terrars::PrimField<f64>,
    permissions: terrars::PrimField<std::string::String>,
}

impl S3filesAccessPoint_RootDirectory_CreationPermissions { }

impl terrars::ToListMappable for S3filesAccessPoint_RootDirectory_CreationPermissions {
    type O = terrars::BlockAssignable<S3filesAccessPoint_RootDirectory_CreationPermissions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3filesAccessPoint_RootDirectory_CreationPermissions {
    #[doc = "Owner group ID"]
    pub owner_gid: terrars::PrimField<f64>,
    #[doc = "Owner user ID"]
    pub owner_uid: terrars::PrimField<f64>,
    #[doc = "POSIX permissions"]
    pub permissions: terrars::PrimField<std::string::String>,
}

impl BuildS3filesAccessPoint_RootDirectory_CreationPermissions {
    pub fn build(self) -> S3filesAccessPoint_RootDirectory_CreationPermissions {
        S3filesAccessPoint_RootDirectory_CreationPermissions {
            owner_gid: self.owner_gid,
            owner_uid: self.owner_uid,
            permissions: self.permissions,
        }
    }
}

#[allow(dead_code)]
pub struct S3filesAccessPoint_RootDirectory_CreationPermissionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesAccessPoint_RootDirectory_CreationPermissionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> S3filesAccessPoint_RootDirectory_CreationPermissionsRef {
        S3filesAccessPoint_RootDirectory_CreationPermissionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3filesAccessPoint_RootDirectory_CreationPermissionsRef {
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
struct S3filesAccessPoint_RootDirectoryDynamic {
    creation_permissions: std
    ::option
    ::Option<terrars::DynamicBlock<S3filesAccessPoint_RootDirectory_CreationPermissions>>,
}

#[derive(Serialize)]
pub struct S3filesAccessPoint_RootDirectory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    creation_permissions: std::option::Option<std::vec::Vec<S3filesAccessPoint_RootDirectory_CreationPermissions>>,
    dynamic: S3filesAccessPoint_RootDirectoryDynamic,
}

impl S3filesAccessPoint_RootDirectory {
    #[doc = "Set the field `path`.\nRoot directory path"]
    pub fn set_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `creation_permissions`.\n"]
    pub fn set_creation_permissions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<S3filesAccessPoint_RootDirectory_CreationPermissions>>,
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

impl terrars::ToListMappable for S3filesAccessPoint_RootDirectory {
    type O = terrars::BlockAssignable<S3filesAccessPoint_RootDirectory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3filesAccessPoint_RootDirectory {}

impl BuildS3filesAccessPoint_RootDirectory {
    pub fn build(self) -> S3filesAccessPoint_RootDirectory {
        S3filesAccessPoint_RootDirectory {
            path: core::default::Default::default(),
            creation_permissions: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3filesAccessPoint_RootDirectoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesAccessPoint_RootDirectoryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3filesAccessPoint_RootDirectoryRef {
        S3filesAccessPoint_RootDirectoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3filesAccessPoint_RootDirectoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\nRoot directory path"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `creation_permissions` after provisioning.\n"]
    pub fn creation_permissions(&self) -> terrars::ListRef<S3filesAccessPoint_RootDirectory_CreationPermissionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.creation_permissions", self.base))
    }
}

#[derive(Serialize)]
pub struct S3filesAccessPoint_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl S3filesAccessPoint_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for S3filesAccessPoint_Timeouts {
    type O = terrars::BlockAssignable<S3filesAccessPoint_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildS3filesAccessPoint_Timeouts {}

impl BuildS3filesAccessPoint_Timeouts {
    pub fn build(self) -> S3filesAccessPoint_Timeouts {
        S3filesAccessPoint_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct S3filesAccessPoint_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for S3filesAccessPoint_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> S3filesAccessPoint_TimeoutsRef {
        S3filesAccessPoint_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl S3filesAccessPoint_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
