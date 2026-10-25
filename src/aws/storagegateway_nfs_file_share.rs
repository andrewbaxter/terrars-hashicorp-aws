#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct StoragegatewayNfsFileShareData_ {
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
    audit_destination_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_region: std::option::Option<terrars::PrimField<std::string::String>>,
    client_list: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_storage_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_share_name: std::option::Option<terrars::PrimField<std::string::String>>,
    gateway_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    guess_mime_type_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    location_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_acl: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_pays: std::option::Option<terrars::PrimField<bool>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    squash: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_attributes: std::option::Option<std::vec::Vec<StoragegatewayNfsFileShare_CacheAttributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nfs_file_share_defaults: std::option::Option<std::vec::Vec<StoragegatewayNfsFileShare_NfsFileShareDefaults>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StoragegatewayNfsFileShare_Timeouts>,
    dynamic: StoragegatewayNfsFileShareDynamic,
}

struct StoragegatewayNfsFileShare_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StoragegatewayNfsFileShareData_>,
}

#[derive(Clone)]
pub struct StoragegatewayNfsFileShare(std::rc::Rc<StoragegatewayNfsFileShare_>);

impl StoragegatewayNfsFileShare {
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

    #[doc = "Set the field `audit_destination_arn`.\n"]
    pub fn set_audit_destination_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().audit_destination_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_region`.\n"]
    pub fn set_bucket_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_storage_class`.\n"]
    pub fn set_default_storage_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_storage_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_share_name`.\n"]
    pub fn set_file_share_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().file_share_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `guess_mime_type_enabled`.\n"]
    pub fn set_guess_mime_type_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().guess_mime_type_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_encrypted`.\n"]
    pub fn set_kms_encrypted(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().kms_encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_policy`.\n"]
    pub fn set_notification_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().notification_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_acl`.\n"]
    pub fn set_object_acl(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().object_acl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requester_pays`.\n"]
    pub fn set_requester_pays(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().requester_pays = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `squash`.\n"]
    pub fn set_squash(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().squash = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_endpoint_dns_name`.\n"]
    pub fn set_vpc_endpoint_dns_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_endpoint_dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cache_attributes`.\n"]
    pub fn set_cache_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StoragegatewayNfsFileShare_CacheAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cache_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cache_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `nfs_file_share_defaults`.\n"]
    pub fn set_nfs_file_share_defaults(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<StoragegatewayNfsFileShare_NfsFileShareDefaults>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().nfs_file_share_defaults = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.nfs_file_share_defaults = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StoragegatewayNfsFileShare_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_list` after provisioning.\n"]
    pub fn client_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.client_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_storage_class` after provisioning.\n"]
    pub fn default_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_storage_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_share_name` after provisioning.\n"]
    pub fn file_share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fileshare_id` after provisioning.\n"]
    pub fn fileshare_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fileshare_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guess_mime_type_enabled` after provisioning.\n"]
    pub fn guess_mime_type_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guess_mime_type_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_encrypted` after provisioning.\n"]
    pub fn kms_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_arn` after provisioning.\n"]
    pub fn location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_policy` after provisioning.\n"]
    pub fn notification_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_acl` after provisioning.\n"]
    pub fn object_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_acl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\n"]
    pub fn requester_pays(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `squash` after provisioning.\n"]
    pub fn squash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_dns_name` after provisioning.\n"]
    pub fn vpc_endpoint_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(&self) -> terrars::ListRef<StoragegatewayNfsFileShare_CacheAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_file_share_defaults` after provisioning.\n"]
    pub fn nfs_file_share_defaults(&self) -> terrars::ListRef<StoragegatewayNfsFileShare_NfsFileShareDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_file_share_defaults", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayNfsFileShare_TimeoutsRef {
        StoragegatewayNfsFileShare_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for StoragegatewayNfsFileShare {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StoragegatewayNfsFileShare { }

impl terrars::ToListMappable for StoragegatewayNfsFileShare {
    type O = terrars::ListRef<StoragegatewayNfsFileShareRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StoragegatewayNfsFileShare_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_storagegateway_nfs_file_share".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewayNfsFileShare {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub client_list: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub gateway_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildStoragegatewayNfsFileShare {
    pub fn build(self, stack: &mut terrars::Stack) -> StoragegatewayNfsFileShare {
        let out = StoragegatewayNfsFileShare(std::rc::Rc::new(StoragegatewayNfsFileShare_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StoragegatewayNfsFileShareData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                audit_destination_arn: core::default::Default::default(),
                bucket_region: core::default::Default::default(),
                client_list: self.client_list,
                default_storage_class: core::default::Default::default(),
                file_share_name: core::default::Default::default(),
                gateway_arn: self.gateway_arn,
                guess_mime_type_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                kms_encrypted: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                location_arn: self.location_arn,
                notification_policy: core::default::Default::default(),
                object_acl: core::default::Default::default(),
                read_only: core::default::Default::default(),
                region: core::default::Default::default(),
                requester_pays: core::default::Default::default(),
                role_arn: self.role_arn,
                squash: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_endpoint_dns_name: core::default::Default::default(),
                cache_attributes: core::default::Default::default(),
                nfs_file_share_defaults: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewayNfsFileShareRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayNfsFileShareRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StoragegatewayNfsFileShareRef {
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

    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `client_list` after provisioning.\n"]
    pub fn client_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.client_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_storage_class` after provisioning.\n"]
    pub fn default_storage_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_storage_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_share_name` after provisioning.\n"]
    pub fn file_share_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_share_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fileshare_id` after provisioning.\n"]
    pub fn fileshare_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fileshare_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gateway_arn` after provisioning.\n"]
    pub fn gateway_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `guess_mime_type_enabled` after provisioning.\n"]
    pub fn guess_mime_type_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.guess_mime_type_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_encrypted` after provisioning.\n"]
    pub fn kms_encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_encrypted", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location_arn` after provisioning.\n"]
    pub fn location_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_policy` after provisioning.\n"]
    pub fn notification_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.notification_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `object_acl` after provisioning.\n"]
    pub fn object_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_acl", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\n"]
    pub fn requester_pays(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `squash` after provisioning.\n"]
    pub fn squash(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.squash", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_dns_name` after provisioning.\n"]
    pub fn vpc_endpoint_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(&self) -> terrars::ListRef<StoragegatewayNfsFileShare_CacheAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nfs_file_share_defaults` after provisioning.\n"]
    pub fn nfs_file_share_defaults(&self) -> terrars::ListRef<StoragegatewayNfsFileShare_NfsFileShareDefaultsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nfs_file_share_defaults", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewayNfsFileShare_TimeoutsRef {
        StoragegatewayNfsFileShare_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct StoragegatewayNfsFileShareDynamic {
    cache_attributes: std::option::Option<terrars::DynamicBlock<StoragegatewayNfsFileShare_CacheAttributes>>,
    nfs_file_share_defaults: std
    ::option
    ::Option<terrars::DynamicBlock<StoragegatewayNfsFileShare_NfsFileShareDefaults>>,
}

#[derive(Serialize)]
pub struct StoragegatewayNfsFileShare_CacheAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_stale_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl StoragegatewayNfsFileShare_CacheAttributes {
    #[doc = "Set the field `cache_stale_timeout_in_seconds`.\n"]
    pub fn set_cache_stale_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cache_stale_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewayNfsFileShare_CacheAttributes {
    type O = terrars::BlockAssignable<StoragegatewayNfsFileShare_CacheAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayNfsFileShare_CacheAttributes {}

impl BuildStoragegatewayNfsFileShare_CacheAttributes {
    pub fn build(self) -> StoragegatewayNfsFileShare_CacheAttributes {
        StoragegatewayNfsFileShare_CacheAttributes {
            cache_stale_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayNfsFileShare_CacheAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayNfsFileShare_CacheAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StoragegatewayNfsFileShare_CacheAttributesRef {
        StoragegatewayNfsFileShare_CacheAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayNfsFileShare_CacheAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_stale_timeout_in_seconds` after provisioning.\n"]
    pub fn cache_stale_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_stale_timeout_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayNfsFileShare_NfsFileShareDefaults {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    directory_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StoragegatewayNfsFileShare_NfsFileShareDefaults {
    #[doc = "Set the field `directory_mode`.\n"]
    pub fn set_directory_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.directory_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_mode`.\n"]
    pub fn set_file_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewayNfsFileShare_NfsFileShareDefaults {
    type O = terrars::BlockAssignable<StoragegatewayNfsFileShare_NfsFileShareDefaults>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayNfsFileShare_NfsFileShareDefaults {}

impl BuildStoragegatewayNfsFileShare_NfsFileShareDefaults {
    pub fn build(self) -> StoragegatewayNfsFileShare_NfsFileShareDefaults {
        StoragegatewayNfsFileShare_NfsFileShareDefaults {
            directory_mode: core::default::Default::default(),
            file_mode: core::default::Default::default(),
            group_id: core::default::Default::default(),
            owner_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayNfsFileShare_NfsFileShareDefaultsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayNfsFileShare_NfsFileShareDefaultsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StoragegatewayNfsFileShare_NfsFileShareDefaultsRef {
        StoragegatewayNfsFileShare_NfsFileShareDefaultsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayNfsFileShare_NfsFileShareDefaultsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `directory_mode` after provisioning.\n"]
    pub fn directory_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `file_mode` after provisioning.\n"]
    pub fn file_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewayNfsFileShare_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StoragegatewayNfsFileShare_Timeouts {
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

impl terrars::ToListMappable for StoragegatewayNfsFileShare_Timeouts {
    type O = terrars::BlockAssignable<StoragegatewayNfsFileShare_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewayNfsFileShare_Timeouts {}

impl BuildStoragegatewayNfsFileShare_Timeouts {
    pub fn build(self) -> StoragegatewayNfsFileShare_Timeouts {
        StoragegatewayNfsFileShare_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct StoragegatewayNfsFileShare_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewayNfsFileShare_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StoragegatewayNfsFileShare_TimeoutsRef {
        StoragegatewayNfsFileShare_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewayNfsFileShare_TimeoutsRef {
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
