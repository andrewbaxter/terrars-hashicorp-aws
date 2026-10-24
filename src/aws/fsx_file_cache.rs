use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct FsxFileCacheData {
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
    copy_tags_to_data_repository_associations: std::option::Option<terrars::PrimField<bool>>,
    file_cache_type: terrars::PrimField<std::string::String>,
    file_cache_type_version: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    storage_capacity: terrars::PrimField<f64>,
    subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_repository_association: std::option::Option<std::vec::Vec<FsxFileCacheDataRepositoryAssociationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lustre_configuration: std::option::Option<std::vec::Vec<FsxFileCacheLustreConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<FsxFileCacheTimeoutsEl>,
    dynamic: FsxFileCacheDynamic,
}

struct FsxFileCache_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<FsxFileCacheData>,
}

#[derive(Clone)]
pub struct FsxFileCache(std::rc::Rc<FsxFileCache_>);

impl FsxFileCache {
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

    #[doc = "Set the field `copy_tags_to_data_repository_associations`.\n"]
    pub fn set_copy_tags_to_data_repository_associations(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().copy_tags_to_data_repository_associations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_repository_association`.\n"]
    pub fn set_data_repository_association(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxFileCacheDataRepositoryAssociationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_repository_association = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_repository_association = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lustre_configuration`.\n"]
    pub fn set_lustre_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxFileCacheLustreConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lustre_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lustre_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<FsxFileCacheTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `copy_tags_to_data_repository_associations` after provisioning.\n"]
    pub fn copy_tags_to_data_repository_associations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_data_repository_associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_repository_association_ids` after provisioning.\n"]
    pub fn data_repository_association_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.data_repository_association_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_cache_id` after provisioning.\n"]
    pub fn file_cache_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_cache_type` after provisioning.\n"]
    pub fn file_cache_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_cache_type_version` after provisioning.\n"]
    pub fn file_cache_type_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_type_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxFileCacheTimeoutsElRef {
        FsxFileCacheTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for FsxFileCache {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for FsxFileCache { }

impl terrars::ToListMappable for FsxFileCache {
    type O = terrars::ListRef<FsxFileCacheRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for FsxFileCache_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_fsx_file_cache".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildFsxFileCache {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub file_cache_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_cache_type_version: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub storage_capacity: terrars::PrimField<f64>,
    #[doc = ""]
    pub subnet_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildFsxFileCache {
    pub fn build(self, stack: &mut terrars::Stack) -> FsxFileCache {
        let out = FsxFileCache(std::rc::Rc::new(FsxFileCache_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(FsxFileCacheData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                copy_tags_to_data_repository_associations: core::default::Default::default(),
                file_cache_type: self.file_cache_type,
                file_cache_type_version: self.file_cache_type_version,
                id: core::default::Default::default(),
                kms_key_id: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                storage_capacity: self.storage_capacity,
                subnet_ids: self.subnet_ids,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                data_repository_association: core::default::Default::default(),
                lustre_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct FsxFileCacheRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl FsxFileCacheRef {
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

    #[doc = "Get a reference to the value of field `copy_tags_to_data_repository_associations` after provisioning.\n"]
    pub fn copy_tags_to_data_repository_associations(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.copy_tags_to_data_repository_associations", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_repository_association_ids` after provisioning.\n"]
    pub fn data_repository_association_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.data_repository_association_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `dns_name` after provisioning.\n"]
    pub fn dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_cache_id` after provisioning.\n"]
    pub fn file_cache_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_cache_type` after provisioning.\n"]
    pub fn file_cache_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_cache_type_version` after provisioning.\n"]
    pub fn file_cache_type_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_type_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interface_ids` after provisioning.\n"]
    pub fn network_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_interface_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> FsxFileCacheTimeoutsElRef {
        FsxFileCacheTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct FsxFileCacheDataRepositoryAssociationElNfsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dns_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    version: terrars::PrimField<std::string::String>,
}

impl FsxFileCacheDataRepositoryAssociationElNfsEl {
    #[doc = "Set the field `dns_ips`.\n"]
    pub fn set_dns_ips(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.dns_ips = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxFileCacheDataRepositoryAssociationElNfsEl {
    type O = terrars::BlockAssignable<FsxFileCacheDataRepositoryAssociationElNfsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxFileCacheDataRepositoryAssociationElNfsEl {
    #[doc = ""]
    pub version: terrars::PrimField<std::string::String>,
}

impl BuildFsxFileCacheDataRepositoryAssociationElNfsEl {
    pub fn build(self) -> FsxFileCacheDataRepositoryAssociationElNfsEl {
        FsxFileCacheDataRepositoryAssociationElNfsEl {
            dns_ips: core::default::Default::default(),
            version: self.version,
        }
    }
}

pub struct FsxFileCacheDataRepositoryAssociationElNfsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheDataRepositoryAssociationElNfsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxFileCacheDataRepositoryAssociationElNfsElRef {
        FsxFileCacheDataRepositoryAssociationElNfsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxFileCacheDataRepositoryAssociationElNfsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dns_ips` after provisioning.\n"]
    pub fn dns_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dns_ips", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxFileCacheDataRepositoryAssociationElDynamic {
    nfs: std::option::Option<terrars::DynamicBlock<FsxFileCacheDataRepositoryAssociationElNfsEl>>,
}

#[derive(Serialize)]
pub struct FsxFileCacheDataRepositoryAssociationEl {
    data_repository_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_repository_subdirectories: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    file_cache_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs: std::option::Option<std::vec::Vec<FsxFileCacheDataRepositoryAssociationElNfsEl>>,
    dynamic: FsxFileCacheDataRepositoryAssociationElDynamic,
}

impl FsxFileCacheDataRepositoryAssociationEl {
    #[doc = "Set the field `data_repository_subdirectories`.\n"]
    pub fn set_data_repository_subdirectories(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.data_repository_subdirectories = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nfs`.\n"]
    pub fn set_nfs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<FsxFileCacheDataRepositoryAssociationElNfsEl>>,
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

impl terrars::ToListMappable for FsxFileCacheDataRepositoryAssociationEl {
    type O = terrars::BlockAssignable<FsxFileCacheDataRepositoryAssociationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxFileCacheDataRepositoryAssociationEl {
    #[doc = ""]
    pub data_repository_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub file_cache_path: terrars::PrimField<std::string::String>,
}

impl BuildFsxFileCacheDataRepositoryAssociationEl {
    pub fn build(self) -> FsxFileCacheDataRepositoryAssociationEl {
        FsxFileCacheDataRepositoryAssociationEl {
            data_repository_path: self.data_repository_path,
            data_repository_subdirectories: core::default::Default::default(),
            file_cache_path: self.file_cache_path,
            tags: core::default::Default::default(),
            nfs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FsxFileCacheDataRepositoryAssociationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheDataRepositoryAssociationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxFileCacheDataRepositoryAssociationElRef {
        FsxFileCacheDataRepositoryAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxFileCacheDataRepositoryAssociationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.base))
    }

    #[doc = "Get a reference to the value of field `data_repository_path` after provisioning.\n"]
    pub fn data_repository_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_repository_path", self.base))
    }

    #[doc = "Get a reference to the value of field `data_repository_subdirectories` after provisioning.\n"]
    pub fn data_repository_subdirectories(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.data_repository_subdirectories", self.base))
    }

    #[doc = "Get a reference to the value of field `file_cache_id` after provisioning.\n"]
    pub fn file_cache_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_cache_path` after provisioning.\n"]
    pub fn file_cache_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_cache_path", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_path` after provisioning.\n"]
    pub fn file_system_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_path", self.base))
    }

    #[doc = "Get a reference to the value of field `imported_file_chunk_size` after provisioning.\n"]
    pub fn imported_file_chunk_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.imported_file_chunk_size", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxFileCacheLustreConfigurationElLogConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    level: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxFileCacheLustreConfigurationElLogConfigurationEl {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.level = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for FsxFileCacheLustreConfigurationElLogConfigurationEl {
    type O = terrars::BlockAssignable<FsxFileCacheLustreConfigurationElLogConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxFileCacheLustreConfigurationElLogConfigurationEl {}

impl BuildFsxFileCacheLustreConfigurationElLogConfigurationEl {
    pub fn build(self) -> FsxFileCacheLustreConfigurationElLogConfigurationEl {
        FsxFileCacheLustreConfigurationElLogConfigurationEl {
            destination: core::default::Default::default(),
            level: core::default::Default::default(),
        }
    }
}

pub struct FsxFileCacheLustreConfigurationElLogConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheLustreConfigurationElLogConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxFileCacheLustreConfigurationElLogConfigurationElRef {
        FsxFileCacheLustreConfigurationElLogConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxFileCacheLustreConfigurationElLogConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    storage_capacity: terrars::PrimField<f64>,
}

impl FsxFileCacheLustreConfigurationElMetadataConfigurationEl { }

impl terrars::ToListMappable for FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    type O = terrars::BlockAssignable<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    #[doc = ""]
    pub storage_capacity: terrars::PrimField<f64>,
}

impl BuildFsxFileCacheLustreConfigurationElMetadataConfigurationEl {
    pub fn build(self) -> FsxFileCacheLustreConfigurationElMetadataConfigurationEl {
        FsxFileCacheLustreConfigurationElMetadataConfigurationEl { storage_capacity: self.storage_capacity }
    }
}

pub struct FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
        FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxFileCacheLustreConfigurationElMetadataConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_capacity` after provisioning.\n"]
    pub fn storage_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_capacity", self.base))
    }
}

#[derive(Serialize, Default)]
struct FsxFileCacheLustreConfigurationElDynamic {
    metadata_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>>,
}

#[derive(Serialize)]
pub struct FsxFileCacheLustreConfigurationEl {
    deployment_type: terrars::PrimField<std::string::String>,
    per_unit_storage_throughput: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weekly_maintenance_start_time: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_configuration: std::option::Option<std::vec::Vec<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>>,
    dynamic: FsxFileCacheLustreConfigurationElDynamic,
}

impl FsxFileCacheLustreConfigurationEl {
    #[doc = "Set the field `weekly_maintenance_start_time`.\n"]
    pub fn set_weekly_maintenance_start_time(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.weekly_maintenance_start_time = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `metadata_configuration`.\n"]
    pub fn set_metadata_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<FsxFileCacheLustreConfigurationElMetadataConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metadata_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metadata_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for FsxFileCacheLustreConfigurationEl {
    type O = terrars::BlockAssignable<FsxFileCacheLustreConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxFileCacheLustreConfigurationEl {
    #[doc = ""]
    pub deployment_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub per_unit_storage_throughput: terrars::PrimField<f64>,
}

impl BuildFsxFileCacheLustreConfigurationEl {
    pub fn build(self) -> FsxFileCacheLustreConfigurationEl {
        FsxFileCacheLustreConfigurationEl {
            deployment_type: self.deployment_type,
            per_unit_storage_throughput: self.per_unit_storage_throughput,
            weekly_maintenance_start_time: core::default::Default::default(),
            metadata_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct FsxFileCacheLustreConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheLustreConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxFileCacheLustreConfigurationElRef {
        FsxFileCacheLustreConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxFileCacheLustreConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.base))
    }

    #[doc = "Get a reference to the value of field `log_configuration` after provisioning.\n"]
    pub fn log_configuration(&self) -> terrars::SetRef<FsxFileCacheLustreConfigurationElLogConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `mount_name` after provisioning.\n"]
    pub fn mount_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mount_name", self.base))
    }

    #[doc = "Get a reference to the value of field `per_unit_storage_throughput` after provisioning.\n"]
    pub fn per_unit_storage_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.per_unit_storage_throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `weekly_maintenance_start_time` after provisioning.\n"]
    pub fn weekly_maintenance_start_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weekly_maintenance_start_time", self.base))
    }
}

#[derive(Serialize)]
pub struct FsxFileCacheTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl FsxFileCacheTimeoutsEl {
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

impl terrars::ToListMappable for FsxFileCacheTimeoutsEl {
    type O = terrars::BlockAssignable<FsxFileCacheTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildFsxFileCacheTimeoutsEl {}

impl BuildFsxFileCacheTimeoutsEl {
    pub fn build(self) -> FsxFileCacheTimeoutsEl {
        FsxFileCacheTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct FsxFileCacheTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for FsxFileCacheTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> FsxFileCacheTimeoutsElRef {
        FsxFileCacheTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl FsxFileCacheTimeoutsElRef {
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

#[derive(Serialize, Default)]
struct FsxFileCacheDynamic {
    data_repository_association: std::option::Option<terrars::DynamicBlock<FsxFileCacheDataRepositoryAssociationEl>>,
    lustre_configuration: std::option::Option<terrars::DynamicBlock<FsxFileCacheLustreConfigurationEl>>,
}
