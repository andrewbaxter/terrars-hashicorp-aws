#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxS3AccessPointAttachmentData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    openzfs_configuration: std::option::Option<std::vec::Vec<FsxS3AccessPointAttachment_OpenzfsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_access_point: std::option::Option<std::vec::Vec<FsxS3AccessPointAttachment_S3AccessPoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxS3AccessPointAttachment_Timeouts>,
    dynamic: FsxS3AccessPointAttachmentDynamic,
}

struct FsxS3AccessPointAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxS3AccessPointAttachmentData_>,
}

#[derive(Clone)]
pub struct FsxS3AccessPointAttachment(std::rc::Rc<FsxS3AccessPointAttachment_>);

impl FsxS3AccessPointAttachment {
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

    #[doc = "Set the field `openzfs_configuration`.\n"]
    pub fn set_openzfs_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxS3AccessPointAttachment_OpenzfsConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().openzfs_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.openzfs_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_access_point`.\n"]
    pub fn set_s3_access_point(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxS3AccessPointAttachment_S3AccessPoint>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_access_point = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_access_point = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxS3AccessPointAttachment_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_access_point_alias` after provisioning.\n"]
    pub fn s3_access_point_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_point_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_access_point_arn` after provisioning.\n"]
    pub fn s3_access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_point_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openzfs_configuration` after provisioning.\n"]
    pub fn openzfs_configuration(&self) -> terrars::ListRef<FsxS3AccessPointAttachment_OpenzfsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openzfs_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_access_point` after provisioning.\n"]
    pub fn s3_access_point(&self) -> terrars::ListRef<FsxS3AccessPointAttachment_S3AccessPointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_access_point", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxS3AccessPointAttachment_TimeoutsRef {
        FsxS3AccessPointAttachment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for FsxS3AccessPointAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxS3AccessPointAttachment { }

impl terrars::ToListMappable for FsxS3AccessPointAttachment {
    type O = terrars::ListRef<FsxS3AccessPointAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxS3AccessPointAttachment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_s3_access_point_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxS3AccessPointAttachment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFsxS3AccessPointAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxS3AccessPointAttachment {
        let out = FsxS3AccessPointAttachment(std::rc::Rc::new(FsxS3AccessPointAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxS3AccessPointAttachmentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                type_: self.type_,
                openzfs_configuration: core::default::Default::default(),
                s3_access_point: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxS3AccessPointAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxS3AccessPointAttachmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_access_point_alias` after provisioning.\n"]
    pub fn s3_access_point_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_point_alias", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_access_point_arn` after provisioning.\n"]
    pub fn s3_access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_access_point_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `openzfs_configuration` after provisioning.\n"]
    pub fn openzfs_configuration(&self) -> terrars::ListRef<FsxS3AccessPointAttachment_OpenzfsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.openzfs_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3_access_point` after provisioning.\n"]
    pub fn s3_access_point(&self) -> terrars::ListRef<FsxS3AccessPointAttachment_S3AccessPointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_access_point", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxS3AccessPointAttachment_TimeoutsRef {
        FsxS3AccessPointAttachment_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachmentDynamic {
    openzfs_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxS3AccessPointAttachment_OpenzfsConfiguration>>,
    s3_access_point: std::option::Option<terrars::DynamicBlock<FsxS3AccessPointAttachment_S3AccessPoint>>,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
    gid: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_gids: std::option::Option<terrars::ListField<terrars::PrimField<f64>>>,
    uid: terrars::PrimField<f64>,
}

impl FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
    #[doc = "Set the field `secondary_gids`.\n"]
    pub fn set_secondary_gids(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<f64>>>,
    ) -> Self {
        self.secondary_gids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
    type O = terrars::BlockAssignable<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
    #[doc = ""]
    pub gid: terrars::PrimField<f64>,
    #[doc = ""]
    pub uid: terrars::PrimField<f64>,
}

impl BuildFsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
    pub fn build(self) -> FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
        FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser {
            gid: self.gid,
            secondary_gids: core::default::Default::default(),
            uid: self.uid,
        }
    }
}

#[allow(dead_code)]
pub struct FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUserRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUserRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUserRef {
        FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUserRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUserRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `gid` after provisioning.\n"]
    pub fn gid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gid", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_gids` after provisioning.\n"]
    pub fn secondary_gids(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_gids", self.base))
    }

    #[doc = "Get a reference to the value of field `uid` after provisioning.\n"]
    pub fn uid(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uid", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityDynamic {
    posix_user: std
    ::option
    ::Option<terrars::DynamicBlock<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser>>,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    posix_user: std
    ::option
    ::Option<std::vec::Vec<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser>>,
    dynamic: FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityDynamic,
}

impl FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
    #[doc = "Set the field `posix_user`.\n"]
    pub fn set_posix_user(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUser,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.posix_user = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.posix_user = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
    type O = terrars::BlockAssignable<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildFsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
    pub fn build(self) -> FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
        FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity {
            type_: self.type_,
            posix_user: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityRef {
        FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `posix_user` after provisioning.\n"]
    pub fn posix_user(
        &self,
    ) -> terrars::ListRef<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity_PosixUserRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.posix_user", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachment_OpenzfsConfigurationDynamic {
    file_system_identity: std
    ::option
    ::Option<terrars::DynamicBlock<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity>>,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachment_OpenzfsConfiguration {
    volume_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_identity: std
    ::option
    ::Option<std::vec::Vec<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity>>,
    dynamic: FsxS3AccessPointAttachment_OpenzfsConfigurationDynamic,
}

impl FsxS3AccessPointAttachment_OpenzfsConfiguration {
    #[doc = "Set the field `file_system_identity`.\n"]
    pub fn set_file_system_identity(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentity>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_system_identity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_system_identity = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxS3AccessPointAttachment_OpenzfsConfiguration {
    type O = terrars::BlockAssignable<FsxS3AccessPointAttachment_OpenzfsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachment_OpenzfsConfiguration {
    #[doc = ""]
    pub volume_id: terrars::PrimField<std::string::String>,
}

impl BuildFsxS3AccessPointAttachment_OpenzfsConfiguration {
    pub fn build(self) -> FsxS3AccessPointAttachment_OpenzfsConfiguration {
        FsxS3AccessPointAttachment_OpenzfsConfiguration {
            volume_id: self.volume_id,
            file_system_identity: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxS3AccessPointAttachment_OpenzfsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachment_OpenzfsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxS3AccessPointAttachment_OpenzfsConfigurationRef {
        FsxS3AccessPointAttachment_OpenzfsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachment_OpenzfsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_identity` after provisioning.\n"]
    pub fn file_system_identity(
        &self,
    ) -> terrars::ListRef<FsxS3AccessPointAttachment_OpenzfsConfiguration_FileSystemIdentityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_system_identity", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration {
    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration {
    type O = terrars::BlockAssignable<FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration {}

impl BuildFsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration {
    pub fn build(self) -> FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration {
        FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration { vpc_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FsxS3AccessPointAttachment_S3AccessPoint_VpcConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachment_S3AccessPoint_VpcConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxS3AccessPointAttachment_S3AccessPoint_VpcConfigurationRef {
        FsxS3AccessPointAttachment_S3AccessPoint_VpcConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachment_S3AccessPoint_VpcConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxS3AccessPointAttachment_S3AccessPointDynamic {
    vpc_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration>>,
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachment_S3AccessPoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_configuration: std::option::Option<std::vec::Vec<FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration>>,
    dynamic: FsxS3AccessPointAttachment_S3AccessPointDynamic,
}

impl FsxS3AccessPointAttachment_S3AccessPoint {
    #[doc = "Set the field `policy`.\n"]
    pub fn set_policy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_configuration`.\n"]
    pub fn set_vpc_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FsxS3AccessPointAttachment_S3AccessPoint_VpcConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vpc_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vpc_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxS3AccessPointAttachment_S3AccessPoint {
    type O = terrars::BlockAssignable<FsxS3AccessPointAttachment_S3AccessPoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachment_S3AccessPoint {}

impl BuildFsxS3AccessPointAttachment_S3AccessPoint {
    pub fn build(self) -> FsxS3AccessPointAttachment_S3AccessPoint {
        FsxS3AccessPointAttachment_S3AccessPoint {
            policy: core::default::Default::default(),
            vpc_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxS3AccessPointAttachment_S3AccessPointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachment_S3AccessPointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxS3AccessPointAttachment_S3AccessPointRef {
        FsxS3AccessPointAttachment_S3AccessPointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachment_S3AccessPointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy` after provisioning.\n"]
    pub fn policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_configuration` after provisioning.\n"]
    pub fn vpc_configuration(&self) -> terrars::ListRef<FsxS3AccessPointAttachment_S3AccessPoint_VpcConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxS3AccessPointAttachment_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxS3AccessPointAttachment_Timeouts {
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

impl terrars::ToListMappable for FsxS3AccessPointAttachment_Timeouts {
    type O = terrars::BlockAssignable<FsxS3AccessPointAttachment_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxS3AccessPointAttachment_Timeouts {}

impl BuildFsxS3AccessPointAttachment_Timeouts {
    pub fn build(self) -> FsxS3AccessPointAttachment_Timeouts {
        FsxS3AccessPointAttachment_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxS3AccessPointAttachment_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxS3AccessPointAttachment_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxS3AccessPointAttachment_TimeoutsRef {
        FsxS3AccessPointAttachment_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxS3AccessPointAttachment_TimeoutsRef {
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
