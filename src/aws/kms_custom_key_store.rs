#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct KmsCustomKeyStoreData_ {
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
    cloud_hsm_cluster_id: std::option::Option<terrars::PrimField<std::string::String>>,
    custom_key_store_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key_store_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_store_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_anchor_certificate: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xks_proxy_connectivity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xks_proxy_uri_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xks_proxy_uri_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xks_proxy_vpc_endpoint_service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<KmsCustomKeyStore_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xks_proxy_authentication_credential: std
    ::option
    ::Option<std::vec::Vec<KmsCustomKeyStore_XksProxyAuthenticationCredential>>,
    dynamic: KmsCustomKeyStoreDynamic,
}

struct KmsCustomKeyStore_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<KmsCustomKeyStoreData_>,
}

#[derive(Clone)]
pub struct KmsCustomKeyStore(std::rc::Rc<KmsCustomKeyStore_>);

impl KmsCustomKeyStore {
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

    #[doc = "Set the field `cloud_hsm_cluster_id`.\n"]
    pub fn set_cloud_hsm_cluster_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cloud_hsm_cluster_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key_store_type`.\n"]
    pub fn set_custom_key_store_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_key_store_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_store_password`.\n"]
    pub fn set_key_store_password(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_store_password = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_anchor_certificate`.\n"]
    pub fn set_trust_anchor_certificate(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().trust_anchor_certificate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xks_proxy_connectivity`.\n"]
    pub fn set_xks_proxy_connectivity(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().xks_proxy_connectivity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xks_proxy_uri_endpoint`.\n"]
    pub fn set_xks_proxy_uri_endpoint(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().xks_proxy_uri_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xks_proxy_uri_path`.\n"]
    pub fn set_xks_proxy_uri_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().xks_proxy_uri_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xks_proxy_vpc_endpoint_service_name`.\n"]
    pub fn set_xks_proxy_vpc_endpoint_service_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().xks_proxy_vpc_endpoint_service_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<KmsCustomKeyStore_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `xks_proxy_authentication_credential`.\n"]
    pub fn set_xks_proxy_authentication_credential(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<KmsCustomKeyStore_XksProxyAuthenticationCredential>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().xks_proxy_authentication_credential = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.xks_proxy_authentication_credential = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_hsm_cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_name` after provisioning.\n"]
    pub fn custom_key_store_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_type` after provisioning.\n"]
    pub fn custom_key_store_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_store_password` after provisioning.\n"]
    pub fn key_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_anchor_certificate` after provisioning.\n"]
    pub fn trust_anchor_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_anchor_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_connectivity` after provisioning.\n"]
    pub fn xks_proxy_connectivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xks_proxy_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_uri_endpoint` after provisioning.\n"]
    pub fn xks_proxy_uri_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xks_proxy_uri_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_uri_path` after provisioning.\n"]
    pub fn xks_proxy_uri_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xks_proxy_uri_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_vpc_endpoint_service_name` after provisioning.\n"]
    pub fn xks_proxy_vpc_endpoint_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.xks_proxy_vpc_endpoint_service_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCustomKeyStore_TimeoutsRef {
        KmsCustomKeyStore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_authentication_credential` after provisioning.\n"]
    pub fn xks_proxy_authentication_credential(
        &self,
    ) -> terrars::ListRef<KmsCustomKeyStore_XksProxyAuthenticationCredentialRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.xks_proxy_authentication_credential", self.extract_ref()),
        )
    }
}

impl terrars::Referable for KmsCustomKeyStore {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for KmsCustomKeyStore { }

impl terrars::ToListMappable for KmsCustomKeyStore {
    type O = terrars::ListRef<KmsCustomKeyStoreRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for KmsCustomKeyStore_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_kms_custom_key_store".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildKmsCustomKeyStore {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub custom_key_store_name: terrars::PrimField<std::string::String>,
}

impl BuildKmsCustomKeyStore {
    pub fn build(self, stack: &mut terrars::Stack) -> KmsCustomKeyStore {
        let out = KmsCustomKeyStore(std::rc::Rc::new(KmsCustomKeyStore_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(KmsCustomKeyStoreData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cloud_hsm_cluster_id: core::default::Default::default(),
                custom_key_store_name: self.custom_key_store_name,
                custom_key_store_type: core::default::Default::default(),
                id: core::default::Default::default(),
                key_store_password: core::default::Default::default(),
                region: core::default::Default::default(),
                trust_anchor_certificate: core::default::Default::default(),
                xks_proxy_connectivity: core::default::Default::default(),
                xks_proxy_uri_endpoint: core::default::Default::default(),
                xks_proxy_uri_path: core::default::Default::default(),
                xks_proxy_vpc_endpoint_service_name: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                xks_proxy_authentication_credential: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct KmsCustomKeyStoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCustomKeyStoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl KmsCustomKeyStoreRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloud_hsm_cluster_id` after provisioning.\n"]
    pub fn cloud_hsm_cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloud_hsm_cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_name` after provisioning.\n"]
    pub fn custom_key_store_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_key_store_type` after provisioning.\n"]
    pub fn custom_key_store_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_key_store_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_store_password` after provisioning.\n"]
    pub fn key_store_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_store_password", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_anchor_certificate` after provisioning.\n"]
    pub fn trust_anchor_certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_anchor_certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_connectivity` after provisioning.\n"]
    pub fn xks_proxy_connectivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xks_proxy_connectivity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_uri_endpoint` after provisioning.\n"]
    pub fn xks_proxy_uri_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xks_proxy_uri_endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_uri_path` after provisioning.\n"]
    pub fn xks_proxy_uri_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.xks_proxy_uri_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_vpc_endpoint_service_name` after provisioning.\n"]
    pub fn xks_proxy_vpc_endpoint_service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.xks_proxy_vpc_endpoint_service_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> KmsCustomKeyStore_TimeoutsRef {
        KmsCustomKeyStore_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `xks_proxy_authentication_credential` after provisioning.\n"]
    pub fn xks_proxy_authentication_credential(
        &self,
    ) -> terrars::ListRef<KmsCustomKeyStore_XksProxyAuthenticationCredentialRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.xks_proxy_authentication_credential", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct KmsCustomKeyStoreDynamic {
    xks_proxy_authentication_credential: std
    ::option
    ::Option<terrars::DynamicBlock<KmsCustomKeyStore_XksProxyAuthenticationCredential>>,
}

#[derive(Serialize)]
pub struct KmsCustomKeyStore_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl KmsCustomKeyStore_Timeouts {
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

impl terrars::ToListMappable for KmsCustomKeyStore_Timeouts {
    type O = terrars::BlockAssignable<KmsCustomKeyStore_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCustomKeyStore_Timeouts {}

impl BuildKmsCustomKeyStore_Timeouts {
    pub fn build(self) -> KmsCustomKeyStore_Timeouts {
        KmsCustomKeyStore_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct KmsCustomKeyStore_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCustomKeyStore_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> KmsCustomKeyStore_TimeoutsRef {
        KmsCustomKeyStore_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCustomKeyStore_TimeoutsRef {
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

#[derive(Serialize)]
pub struct KmsCustomKeyStore_XksProxyAuthenticationCredential {
    access_key_id: terrars::PrimField<std::string::String>,
    raw_secret_access_key: terrars::PrimField<std::string::String>,
}

impl KmsCustomKeyStore_XksProxyAuthenticationCredential { }

impl terrars::ToListMappable for KmsCustomKeyStore_XksProxyAuthenticationCredential {
    type O = terrars::BlockAssignable<KmsCustomKeyStore_XksProxyAuthenticationCredential>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildKmsCustomKeyStore_XksProxyAuthenticationCredential {
    #[doc = ""]
    pub access_key_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub raw_secret_access_key: terrars::PrimField<std::string::String>,
}

impl BuildKmsCustomKeyStore_XksProxyAuthenticationCredential {
    pub fn build(self) -> KmsCustomKeyStore_XksProxyAuthenticationCredential {
        KmsCustomKeyStore_XksProxyAuthenticationCredential {
            access_key_id: self.access_key_id,
            raw_secret_access_key: self.raw_secret_access_key,
        }
    }
}

#[allow(dead_code)]
pub struct KmsCustomKeyStore_XksProxyAuthenticationCredentialRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for KmsCustomKeyStore_XksProxyAuthenticationCredentialRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> KmsCustomKeyStore_XksProxyAuthenticationCredentialRef {
        KmsCustomKeyStore_XksProxyAuthenticationCredentialRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl KmsCustomKeyStore_XksProxyAuthenticationCredentialRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_key_id` after provisioning.\n"]
    pub fn access_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `raw_secret_access_key` after provisioning.\n"]
    pub fn raw_secret_access_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.raw_secret_access_key", self.base))
    }
}
