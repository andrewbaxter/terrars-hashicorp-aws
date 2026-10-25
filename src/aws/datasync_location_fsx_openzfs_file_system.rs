#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatasyncLocationFsxOpenzfsFileSystemData_ {
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
    fsx_filesystem_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    security_group_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subdirectory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<std::vec::Vec<DatasyncLocationFsxOpenzfsFileSystem_Protocol>>,
    dynamic: DatasyncLocationFsxOpenzfsFileSystemDynamic,
}

struct DatasyncLocationFsxOpenzfsFileSystem_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatasyncLocationFsxOpenzfsFileSystemData_>,
}

#[derive(Clone)]
pub struct DatasyncLocationFsxOpenzfsFileSystem(std::rc::Rc<DatasyncLocationFsxOpenzfsFileSystem_>);

impl DatasyncLocationFsxOpenzfsFileSystem {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subdirectory`.\n"]
    pub fn set_subdirectory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subdirectory = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatasyncLocationFsxOpenzfsFileSystem_Protocol>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().protocol = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.protocol = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fsx_filesystem_arn` after provisioning.\n"]
    pub fn fsx_filesystem_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fsx_filesystem_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_arns` after provisioning.\n"]
    pub fn security_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::ListRef<DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }
}

impl terrars::Referable for DatasyncLocationFsxOpenzfsFileSystem {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatasyncLocationFsxOpenzfsFileSystem { }

impl terrars::ToListMappable for DatasyncLocationFsxOpenzfsFileSystem {
    type O = terrars::ListRef<DatasyncLocationFsxOpenzfsFileSystemRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatasyncLocationFsxOpenzfsFileSystem_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datasync_location_fsx_openzfs_file_system".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystem {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub fsx_filesystem_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub security_group_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDatasyncLocationFsxOpenzfsFileSystem {
    pub fn build(self, stack: &mut terrars::Stack) -> DatasyncLocationFsxOpenzfsFileSystem {
        let out = DatasyncLocationFsxOpenzfsFileSystem(std::rc::Rc::new(DatasyncLocationFsxOpenzfsFileSystem_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatasyncLocationFsxOpenzfsFileSystemData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                fsx_filesystem_arn: self.fsx_filesystem_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_arns: self.security_group_arns,
                subdirectory: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                protocol: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatasyncLocationFsxOpenzfsFileSystemRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationFsxOpenzfsFileSystemRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystemRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fsx_filesystem_arn` after provisioning.\n"]
    pub fn fsx_filesystem_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fsx_filesystem_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_arns` after provisioning.\n"]
    pub fn security_group_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subdirectory` after provisioning.\n"]
    pub fn subdirectory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subdirectory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `uri` after provisioning.\n"]
    pub fn uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::ListRef<DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationFsxOpenzfsFileSystemDynamic {
    protocol: std::option::Option<terrars::DynamicBlock<DatasyncLocationFsxOpenzfsFileSystem_Protocol>>,
}

#[derive(Serialize)]
pub struct DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions {
    type O = terrars::BlockAssignable<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions {}

impl BuildDatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions {
    pub fn build(self) -> DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions {
        DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions { version: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptionsRef {
        DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsDynamic {
    mount_options: std
    ::option
    ::Option<terrars::DynamicBlock<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions>>,
}

#[derive(Serialize)]
pub struct DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mount_options: std::option::Option<std::vec::Vec<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions>>,
    dynamic: DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsDynamic,
}

impl DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {
    #[doc = "Set the field `mount_options`.\n"]
    pub fn set_mount_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mount_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mount_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {
    type O = terrars::BlockAssignable<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {}

impl BuildDatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {
    pub fn build(self) -> DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {
        DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs {
            mount_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsRef {
        DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `mount_options` after provisioning.\n"]
    pub fn mount_options(&self) -> terrars::ListRef<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs_MountOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mount_options", self.base))
    }
}

#[derive(Serialize, Default)]
struct DatasyncLocationFsxOpenzfsFileSystem_ProtocolDynamic {
    nfs: std::option::Option<terrars::DynamicBlock<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs>>,
}

#[derive(Serialize)]
pub struct DatasyncLocationFsxOpenzfsFileSystem_Protocol {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<std::vec::Vec<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs>>,
    dynamic: DatasyncLocationFsxOpenzfsFileSystem_ProtocolDynamic,
}

impl DatasyncLocationFsxOpenzfsFileSystem_Protocol {
    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatasyncLocationFsxOpenzfsFileSystem_Protocol_Nfs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.nfs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.nfs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatasyncLocationFsxOpenzfsFileSystem_Protocol {
    type O = terrars::BlockAssignable<DatasyncLocationFsxOpenzfsFileSystem_Protocol>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatasyncLocationFsxOpenzfsFileSystem_Protocol {}

impl BuildDatasyncLocationFsxOpenzfsFileSystem_Protocol {
    pub fn build(self) -> DatasyncLocationFsxOpenzfsFileSystem_Protocol {
        DatasyncLocationFsxOpenzfsFileSystem_Protocol {
            nfs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef {
        DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatasyncLocationFsxOpenzfsFileSystem_ProtocolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `nfs` after provisioning.\n"]
    pub fn nfs(&self) -> terrars::ListRef<DatasyncLocationFsxOpenzfsFileSystem_Protocol_NfsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs", self.base))
    }
}
