use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct StoragegatewaySmbFileShareData {
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
    access_based_enumeration: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    admin_user_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    audit_destination_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authentication: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    case_sensitivity: std::option::Option<terrars::PrimField<std::string::String>>,
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
    invalid_user_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    oplocks_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read_only: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requester_pays: std::option::Option<terrars::PrimField<bool>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    smb_acl_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_user_list: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint_dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_attributes: std::option::Option<std::vec::Vec<StoragegatewaySmbFileShareCacheAttributesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<StoragegatewaySmbFileShareTimeoutsEl>,
    dynamic: StoragegatewaySmbFileShareDynamic,
}

struct StoragegatewaySmbFileShare_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<StoragegatewaySmbFileShareData>,
}

#[derive(Clone)]
pub struct StoragegatewaySmbFileShare(std::rc::Rc<StoragegatewaySmbFileShare_>);

impl StoragegatewaySmbFileShare {
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

    #[doc = "Set the field `access_based_enumeration`.\n"]
    pub fn set_access_based_enumeration(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().access_based_enumeration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `admin_user_list`.\n"]
    pub fn set_admin_user_list(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().admin_user_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `audit_destination_arn`.\n"]
    pub fn set_audit_destination_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().audit_destination_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authentication`.\n"]
    pub fn set_authentication(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().authentication = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_region`.\n"]
    pub fn set_bucket_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bucket_region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `case_sensitivity`.\n"]
    pub fn set_case_sensitivity(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().case_sensitivity = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `invalid_user_list`.\n"]
    pub fn set_invalid_user_list(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().invalid_user_list = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `oplocks_enabled`.\n"]
    pub fn set_oplocks_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().oplocks_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `read_only`.\n"]
    pub fn set_read_only(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().read_only = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requester_pays`.\n"]
    pub fn set_requester_pays(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().requester_pays = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `smb_acl_enabled`.\n"]
    pub fn set_smb_acl_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().smb_acl_enabled = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `valid_user_list`.\n"]
    pub fn set_valid_user_list(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().valid_user_list = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<StoragegatewaySmbFileShareCacheAttributesEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<StoragegatewaySmbFileShareTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `access_based_enumeration` after provisioning.\n"]
    pub fn access_based_enumeration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_based_enumeration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_user_list` after provisioning.\n"]
    pub fn admin_user_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.admin_user_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `case_sensitivity` after provisioning.\n"]
    pub fn case_sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitivity", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `invalid_user_list` after provisioning.\n"]
    pub fn invalid_user_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.invalid_user_list", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `oplocks_enabled` after provisioning.\n"]
    pub fn oplocks_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oplocks_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\n"]
    pub fn requester_pays(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_acl_enabled` after provisioning.\n"]
    pub fn smb_acl_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_acl_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_user_list` after provisioning.\n"]
    pub fn valid_user_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.valid_user_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_dns_name` after provisioning.\n"]
    pub fn vpc_endpoint_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(&self) -> terrars::ListRef<StoragegatewaySmbFileShareCacheAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewaySmbFileShareTimeoutsElRef {
        StoragegatewaySmbFileShareTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for StoragegatewaySmbFileShare {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for StoragegatewaySmbFileShare { }

impl terrars::ToListMappable for StoragegatewaySmbFileShare {
    type O = terrars::ListRef<StoragegatewaySmbFileShareRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for StoragegatewaySmbFileShare_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_storagegateway_smb_file_share".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildStoragegatewaySmbFileShare {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub gateway_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub location_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildStoragegatewaySmbFileShare {
    pub fn build(self, stack: &mut terrars::Stack) -> StoragegatewaySmbFileShare {
        let out = StoragegatewaySmbFileShare(std::rc::Rc::new(StoragegatewaySmbFileShare_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(StoragegatewaySmbFileShareData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                access_based_enumeration: core::default::Default::default(),
                admin_user_list: core::default::Default::default(),
                audit_destination_arn: core::default::Default::default(),
                authentication: core::default::Default::default(),
                bucket_region: core::default::Default::default(),
                case_sensitivity: core::default::Default::default(),
                default_storage_class: core::default::Default::default(),
                file_share_name: core::default::Default::default(),
                gateway_arn: self.gateway_arn,
                guess_mime_type_enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                invalid_user_list: core::default::Default::default(),
                kms_encrypted: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                location_arn: self.location_arn,
                notification_policy: core::default::Default::default(),
                object_acl: core::default::Default::default(),
                oplocks_enabled: core::default::Default::default(),
                read_only: core::default::Default::default(),
                requester_pays: core::default::Default::default(),
                role_arn: self.role_arn,
                smb_acl_enabled: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                valid_user_list: core::default::Default::default(),
                vpc_endpoint_dns_name: core::default::Default::default(),
                cache_attributes: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct StoragegatewaySmbFileShareRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewaySmbFileShareRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl StoragegatewaySmbFileShareRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_based_enumeration` after provisioning.\n"]
    pub fn access_based_enumeration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_based_enumeration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `admin_user_list` after provisioning.\n"]
    pub fn admin_user_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.admin_user_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `audit_destination_arn` after provisioning.\n"]
    pub fn audit_destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.audit_destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `authentication` after provisioning.\n"]
    pub fn authentication(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.authentication", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_region` after provisioning.\n"]
    pub fn bucket_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `case_sensitivity` after provisioning.\n"]
    pub fn case_sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.case_sensitivity", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `invalid_user_list` after provisioning.\n"]
    pub fn invalid_user_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.invalid_user_list", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `oplocks_enabled` after provisioning.\n"]
    pub fn oplocks_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oplocks_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `read_only` after provisioning.\n"]
    pub fn read_only(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read_only", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_pays` after provisioning.\n"]
    pub fn requester_pays(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_pays", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `smb_acl_enabled` after provisioning.\n"]
    pub fn smb_acl_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smb_acl_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_user_list` after provisioning.\n"]
    pub fn valid_user_list(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.valid_user_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint_dns_name` after provisioning.\n"]
    pub fn vpc_endpoint_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_endpoint_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cache_attributes` after provisioning.\n"]
    pub fn cache_attributes(&self) -> terrars::ListRef<StoragegatewaySmbFileShareCacheAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cache_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> StoragegatewaySmbFileShareTimeoutsElRef {
        StoragegatewaySmbFileShareTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct StoragegatewaySmbFileShareCacheAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cache_stale_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl StoragegatewaySmbFileShareCacheAttributesEl {
    #[doc = "Set the field `cache_stale_timeout_in_seconds`.\n"]
    pub fn set_cache_stale_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cache_stale_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for StoragegatewaySmbFileShareCacheAttributesEl {
    type O = terrars::BlockAssignable<StoragegatewaySmbFileShareCacheAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewaySmbFileShareCacheAttributesEl {}

impl BuildStoragegatewaySmbFileShareCacheAttributesEl {
    pub fn build(self) -> StoragegatewaySmbFileShareCacheAttributesEl {
        StoragegatewaySmbFileShareCacheAttributesEl {
            cache_stale_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewaySmbFileShareCacheAttributesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewaySmbFileShareCacheAttributesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> StoragegatewaySmbFileShareCacheAttributesElRef {
        StoragegatewaySmbFileShareCacheAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewaySmbFileShareCacheAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cache_stale_timeout_in_seconds` after provisioning.\n"]
    pub fn cache_stale_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cache_stale_timeout_in_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct StoragegatewaySmbFileShareTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl StoragegatewaySmbFileShareTimeoutsEl {
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

impl terrars::ToListMappable for StoragegatewaySmbFileShareTimeoutsEl {
    type O = terrars::BlockAssignable<StoragegatewaySmbFileShareTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildStoragegatewaySmbFileShareTimeoutsEl {}

impl BuildStoragegatewaySmbFileShareTimeoutsEl {
    pub fn build(self) -> StoragegatewaySmbFileShareTimeoutsEl {
        StoragegatewaySmbFileShareTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct StoragegatewaySmbFileShareTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for StoragegatewaySmbFileShareTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> StoragegatewaySmbFileShareTimeoutsElRef {
        StoragegatewaySmbFileShareTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl StoragegatewaySmbFileShareTimeoutsElRef {
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
struct StoragegatewaySmbFileShareDynamic {
    cache_attributes: std::option::Option<terrars::DynamicBlock<StoragegatewaySmbFileShareCacheAttributesEl>>,
}
