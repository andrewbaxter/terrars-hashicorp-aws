#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxDataRepositoryAssociationData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    batch_import_meta_data_on_create: std::option::Option<terrars::PrimField<bool>>,
    data_repository_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_data_in_filesystem: std::option::Option<terrars::PrimField<bool>>,
    file_system_id: terrars::PrimField<std::string::String>,
    file_system_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    imported_file_chunk_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3: std::option::Option<std::vec::Vec<FsxDataRepositoryAssociation_S3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxDataRepositoryAssociation_Timeouts>,
    dynamic: FsxDataRepositoryAssociationDynamic,
}

struct FsxDataRepositoryAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxDataRepositoryAssociationData_>,
}

#[derive(Clone)]
pub struct FsxDataRepositoryAssociation(std::rc::Rc<FsxDataRepositoryAssociation_>);

impl FsxDataRepositoryAssociation {
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

    #[doc = "Set the field `batch_import_meta_data_on_create`.\n"]
    pub fn set_batch_import_meta_data_on_create(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().batch_import_meta_data_on_create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_data_in_filesystem`.\n"]
    pub fn set_delete_data_in_filesystem(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_data_in_filesystem = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `imported_file_chunk_size`.\n"]
    pub fn set_imported_file_chunk_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().imported_file_chunk_size = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3`.\n"]
    pub fn set_s3(self, v: impl std::convert::Into<terrars::BlockAssignable<FsxDataRepositoryAssociation_S3>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxDataRepositoryAssociation_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `batch_import_meta_data_on_create` after provisioning.\n"]
    pub fn batch_import_meta_data_on_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_import_meta_data_on_create", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_repository_path` after provisioning.\n"]
    pub fn data_repository_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_repository_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_data_in_filesystem` after provisioning.\n"]
    pub fn delete_data_in_filesystem(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_data_in_filesystem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<FsxDataRepositoryAssociation_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxDataRepositoryAssociation_TimeoutsRef {
        FsxDataRepositoryAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for FsxDataRepositoryAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxDataRepositoryAssociation { }

impl terrars::ToListMappable for FsxDataRepositoryAssociation {
    type O = terrars::ListRef<FsxDataRepositoryAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxDataRepositoryAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_data_repository_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxDataRepositoryAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_repository_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_system_path: terrars::PrimField<std::string::String>,
}

impl BuildFsxDataRepositoryAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxDataRepositoryAssociation {
        let out = FsxDataRepositoryAssociation(std::rc::Rc::new(FsxDataRepositoryAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxDataRepositoryAssociationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                batch_import_meta_data_on_create: core::default::Default::default(),
                data_repository_path: self.data_repository_path,
                delete_data_in_filesystem: core::default::Default::default(),
                file_system_id: self.file_system_id,
                file_system_path: self.file_system_path,
                id: core::default::Default::default(),
                imported_file_chunk_size: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                s3: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxDataRepositoryAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxDataRepositoryAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxDataRepositoryAssociationRef {
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

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `batch_import_meta_data_on_create` after provisioning.\n"]
    pub fn batch_import_meta_data_on_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.batch_import_meta_data_on_create", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_repository_path` after provisioning.\n"]
    pub fn data_repository_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_repository_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_data_in_filesystem` after provisioning.\n"]
    pub fn delete_data_in_filesystem(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_data_in_filesystem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `s3` after provisioning.\n"]
    pub fn s3(&self) -> terrars::ListRef<FsxDataRepositoryAssociation_S3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxDataRepositoryAssociation_TimeoutsRef {
        FsxDataRepositoryAssociation_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct FsxDataRepositoryAssociationDynamic {
    s3: std::option::Option<terrars::DynamicBlock<FsxDataRepositoryAssociation_S3>>,
}

#[derive(Serialize)]
pub struct FsxDataRepositoryAssociation_S3_AutoExportPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl FsxDataRepositoryAssociation_S3_AutoExportPolicy {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxDataRepositoryAssociation_S3_AutoExportPolicy {
    type O = terrars::BlockAssignable<FsxDataRepositoryAssociation_S3_AutoExportPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxDataRepositoryAssociation_S3_AutoExportPolicy {}

impl BuildFsxDataRepositoryAssociation_S3_AutoExportPolicy {
    pub fn build(self) -> FsxDataRepositoryAssociation_S3_AutoExportPolicy {
        FsxDataRepositoryAssociation_S3_AutoExportPolicy { events: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FsxDataRepositoryAssociation_S3_AutoExportPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxDataRepositoryAssociation_S3_AutoExportPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxDataRepositoryAssociation_S3_AutoExportPolicyRef {
        FsxDataRepositoryAssociation_S3_AutoExportPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxDataRepositoryAssociation_S3_AutoExportPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxDataRepositoryAssociation_S3_AutoImportPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl FsxDataRepositoryAssociation_S3_AutoImportPolicy {
    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxDataRepositoryAssociation_S3_AutoImportPolicy {
    type O = terrars::BlockAssignable<FsxDataRepositoryAssociation_S3_AutoImportPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxDataRepositoryAssociation_S3_AutoImportPolicy {}

impl BuildFsxDataRepositoryAssociation_S3_AutoImportPolicy {
    pub fn build(self) -> FsxDataRepositoryAssociation_S3_AutoImportPolicy {
        FsxDataRepositoryAssociation_S3_AutoImportPolicy { events: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct FsxDataRepositoryAssociation_S3_AutoImportPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxDataRepositoryAssociation_S3_AutoImportPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxDataRepositoryAssociation_S3_AutoImportPolicyRef {
        FsxDataRepositoryAssociation_S3_AutoImportPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxDataRepositoryAssociation_S3_AutoImportPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxDataRepositoryAssociation_S3Dynamic {
    auto_export_policy: std::option::Option<terrars::DynamicBlock<FsxDataRepositoryAssociation_S3_AutoExportPolicy>>,
    auto_import_policy: std
    ::option
    ::Option<terrars::DynamicBlock<FsxDataRepositoryAssociation_S3_AutoImportPolicy>>,
}

#[derive(Serialize)]
pub struct FsxDataRepositoryAssociation_S3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_export_policy: std::option::Option<std::vec::Vec<FsxDataRepositoryAssociation_S3_AutoExportPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_import_policy: std::option::Option<std::vec::Vec<FsxDataRepositoryAssociation_S3_AutoImportPolicy>>,
    dynamic: FsxDataRepositoryAssociation_S3Dynamic,
}

impl FsxDataRepositoryAssociation_S3 {
    #[doc = "Set the field `auto_export_policy`.\n"]
    pub fn set_auto_export_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxDataRepositoryAssociation_S3_AutoExportPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_export_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_export_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_import_policy`.\n"]
    pub fn set_auto_import_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxDataRepositoryAssociation_S3_AutoImportPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_import_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_import_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxDataRepositoryAssociation_S3 {
    type O = terrars::BlockAssignable<FsxDataRepositoryAssociation_S3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxDataRepositoryAssociation_S3 {}

impl BuildFsxDataRepositoryAssociation_S3 {
    pub fn build(self) -> FsxDataRepositoryAssociation_S3 {
        FsxDataRepositoryAssociation_S3 {
            auto_export_policy: core::default::Default::default(),
            auto_import_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxDataRepositoryAssociation_S3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxDataRepositoryAssociation_S3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxDataRepositoryAssociation_S3Ref {
        FsxDataRepositoryAssociation_S3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxDataRepositoryAssociation_S3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_export_policy` after provisioning.\n"]
    pub fn auto_export_policy(&self) -> terrars::ListRef<FsxDataRepositoryAssociation_S3_AutoExportPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_export_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_import_policy` after provisioning.\n"]
    pub fn auto_import_policy(&self) -> terrars::ListRef<FsxDataRepositoryAssociation_S3_AutoImportPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_import_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxDataRepositoryAssociation_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxDataRepositoryAssociation_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxDataRepositoryAssociation_Timeouts {
    type O = terrars::BlockAssignable<FsxDataRepositoryAssociation_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxDataRepositoryAssociation_Timeouts {}

impl BuildFsxDataRepositoryAssociation_Timeouts {
    pub fn build(self) -> FsxDataRepositoryAssociation_Timeouts {
        FsxDataRepositoryAssociation_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct FsxDataRepositoryAssociation_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxDataRepositoryAssociation_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxDataRepositoryAssociation_TimeoutsRef {
        FsxDataRepositoryAssociation_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxDataRepositoryAssociation_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
