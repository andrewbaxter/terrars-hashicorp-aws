#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataexchangeRevisionAssetsData_ {
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
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    data_set_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finalized: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset: std::option::Option<std::vec::Vec<DataexchangeRevisionAssets_Asset>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataexchangeRevisionAssets_Timeouts>,
    dynamic: DataexchangeRevisionAssetsDynamic,
}

struct DataexchangeRevisionAssets_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataexchangeRevisionAssetsData_>,
}

#[derive(Clone)]
pub struct DataexchangeRevisionAssets(std::rc::Rc<DataexchangeRevisionAssets_>);

impl DataexchangeRevisionAssets {
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

    #[doc = "Set the field `comment`.\n"]
    pub fn set_comment(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `finalized`.\n"]
    pub fn set_finalized(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().finalized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_destroy`.\n"]
    pub fn set_force_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_destroy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `asset`.\n"]
    pub fn set_asset(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataexchangeRevisionAssets_Asset>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().asset = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.asset = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataexchangeRevisionAssets_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `finalized` after provisioning.\n"]
    pub fn finalized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finalized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataexchangeRevisionAssets_TimeoutsRef {
        DataexchangeRevisionAssets_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataexchangeRevisionAssets {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DataexchangeRevisionAssets { }

impl terrars::ToListMappable for DataexchangeRevisionAssets {
    type O = terrars::ListRef<DataexchangeRevisionAssetsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DataexchangeRevisionAssets_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dataexchange_revision_assets".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataexchangeRevisionAssets {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub data_set_id: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeRevisionAssets {
    pub fn build(self, stack: &mut terrars::Stack) -> DataexchangeRevisionAssets {
        let out = DataexchangeRevisionAssets(std::rc::Rc::new(DataexchangeRevisionAssets_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataexchangeRevisionAssetsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                comment: core::default::Default::default(),
                data_set_id: self.data_set_id,
                finalized: core::default::Default::default(),
                force_destroy: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                asset: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DataexchangeRevisionAssetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataexchangeRevisionAssetsRef {
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

    #[doc = "Get a reference to the value of field `comment` after provisioning.\n"]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_set_id` after provisioning.\n"]
    pub fn data_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `finalized` after provisioning.\n"]
    pub fn finalized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.finalized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_destroy` after provisioning.\n"]
    pub fn force_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_destroy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataexchangeRevisionAssets_TimeoutsRef {
        DataexchangeRevisionAssets_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct DataexchangeRevisionAssetsDynamic {
    asset: std::option::Option<terrars::DynamicBlock<DataexchangeRevisionAssets_Asset>>,
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant {
    kms_key_arn: terrars::PrimField<std::string::String>,
}

impl DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant { }

impl terrars
::ToListMappable for DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant {
    type O =
        terrars
        ::BlockAssignable<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant {
    #[doc = ""]
    pub kms_key_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant {
        DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant {
            kms_key_arn: self.kms_key_arn,
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrantRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrantRef {
        DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrantRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrantRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceDynamic {
    kms_keys_to_grant: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant>,
    >,
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
    bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keys: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_keys_to_grant: std
    ::option
    ::Option<
        std::vec::Vec<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant>,
    >,
    dynamic: DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceDynamic,
}

impl DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
    #[doc = "Set the field `key_prefixes`.\n"]
    pub fn set_key_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.key_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keys`.\n"]
    pub fn set_keys(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.keys = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_keys_to_grant`.\n"]
    pub fn set_kms_keys_to_grant(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrant,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.kms_keys_to_grant = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.kms_keys_to_grant = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
        DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource {
            bucket: self.bucket,
            key_prefixes: core::default::Default::default(),
            keys: core::default::Default::default(),
            kms_keys_to_grant: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceRef {
        DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key_prefixes` after provisioning.\n"]
    pub fn key_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.key_prefixes", self.base))
    }

    #[doc = "Get a reference to the value of field `keys` after provisioning.\n"]
    pub fn keys(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.keys", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_keys_to_grant` after provisioning.\n"]
    pub fn kms_keys_to_grant(
        &self,
    ) -> terrars
    ::ListRef<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource_KmsKeysToGrantRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kms_keys_to_grant", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketDynamic {
    asset_source: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource>>,
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_source: std
    ::option
    ::Option<std::vec::Vec<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource>>,
    dynamic: DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketDynamic,
}

impl DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {
    #[doc = "Set the field `asset_source`.\n"]
    pub fn set_asset_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSource>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asset_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asset_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {}

impl BuildDataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {
        DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket {
            asset_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketRef {
        DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_alias` after provisioning.\n"]
    pub fn access_point_alias(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_alias", self.base))
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `asset_source` after provisioning.\n"]
    pub fn asset_source(
        &self,
    ) -> terrars::ListRef<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket_AssetSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource {
    bucket: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
}

impl DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource { }

impl terrars::ToListMappable for DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource {
    #[doc = ""]
    pub bucket: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource {
        DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource {
            bucket: self.bucket,
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSourceRef {
        DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket` after provisioning.\n"]
    pub fn bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Dynamic {
    asset_source: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource>>,
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asset_source: std::option::Option<std::vec::Vec<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource>>,
    dynamic: DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Dynamic,
}

impl DataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {
    #[doc = "Set the field `asset_source`.\n"]
    pub fn set_asset_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asset_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asset_source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {}

impl BuildDataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {
        DataexchangeRevisionAssets_Asset_ImportAssetsFromS3 {
            asset_source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Ref {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Ref {
        DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asset_source` after provisioning.\n"]
    pub fn asset_source(&self) -> terrars::ListRef<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3_AssetSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asset_source", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl {
    filename: terrars::PrimField<std::string::String>,
}

impl DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl { }

impl terrars::ToListMappable for DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl {
    #[doc = ""]
    pub filename: terrars::PrimField<std::string::String>,
}

impl BuildDataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl {
        DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl { filename: self.filename }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrlRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrlRef {
        DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrlRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrlRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.filename", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataexchangeRevisionAssets_AssetDynamic {
    create_s3_data_access_from_s3_bucket: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket>>,
    import_assets_from_s3: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3>>,
    import_assets_from_signed_url: std
    ::option
    ::Option<terrars::DynamicBlock<DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl>>,
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Asset {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_s3_data_access_from_s3_bucket: std
    ::option
    ::Option<std::vec::Vec<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_assets_from_s3: std::option::Option<std::vec::Vec<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    import_assets_from_signed_url: std
    ::option
    ::Option<std::vec::Vec<DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl>>,
    dynamic: DataexchangeRevisionAssets_AssetDynamic,
}

impl DataexchangeRevisionAssets_Asset {
    #[doc = "Set the field `create_s3_data_access_from_s3_bucket`.\n"]
    pub fn set_create_s3_data_access_from_s3_bucket(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3Bucket>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_s3_data_access_from_s3_bucket = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_s3_data_access_from_s3_bucket = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `import_assets_from_s3`.\n"]
    pub fn set_import_assets_from_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.import_assets_from_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.import_assets_from_s3 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `import_assets_from_signed_url`.\n"]
    pub fn set_import_assets_from_signed_url(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.import_assets_from_signed_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.import_assets_from_signed_url = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataexchangeRevisionAssets_Asset {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Asset>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Asset {}

impl BuildDataexchangeRevisionAssets_Asset {
    pub fn build(self) -> DataexchangeRevisionAssets_Asset {
        DataexchangeRevisionAssets_Asset {
            create_s3_data_access_from_s3_bucket: core::default::Default::default(),
            import_assets_from_s3: core::default::Default::default(),
            import_assets_from_signed_url: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_AssetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_AssetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataexchangeRevisionAssets_AssetRef {
        DataexchangeRevisionAssets_AssetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_AssetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }

    #[doc = "Get a reference to the value of field `create_s3_data_access_from_s3_bucket` after provisioning.\n"]
    pub fn create_s3_data_access_from_s3_bucket(
        &self,
    ) -> terrars::ListRef<DataexchangeRevisionAssets_Asset_CreateS3DataAccessFromS3BucketRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_s3_data_access_from_s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `import_assets_from_s3` after provisioning.\n"]
    pub fn import_assets_from_s3(&self) -> terrars::ListRef<DataexchangeRevisionAssets_Asset_ImportAssetsFromS3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.import_assets_from_s3", self.base))
    }

    #[doc = "Get a reference to the value of field `import_assets_from_signed_url` after provisioning.\n"]
    pub fn import_assets_from_signed_url(
        &self,
    ) -> terrars::ListRef<DataexchangeRevisionAssets_Asset_ImportAssetsFromSignedUrlRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.import_assets_from_signed_url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataexchangeRevisionAssets_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataexchangeRevisionAssets_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataexchangeRevisionAssets_Timeouts {
    type O = terrars::BlockAssignable<DataexchangeRevisionAssets_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataexchangeRevisionAssets_Timeouts {}

impl BuildDataexchangeRevisionAssets_Timeouts {
    pub fn build(self) -> DataexchangeRevisionAssets_Timeouts {
        DataexchangeRevisionAssets_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataexchangeRevisionAssets_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataexchangeRevisionAssets_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataexchangeRevisionAssets_TimeoutsRef {
        DataexchangeRevisionAssets_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataexchangeRevisionAssets_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
