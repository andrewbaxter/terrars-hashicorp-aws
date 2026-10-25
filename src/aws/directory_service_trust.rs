#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DirectoryServiceTrustData_ {
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
    conditional_forwarder_ip_addrs: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_associated_conditional_forwarder: std::option::Option<terrars::PrimField<bool>>,
    directory_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    remote_domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selective_auth: std::option::Option<terrars::PrimField<std::string::String>>,
    trust_direction: terrars::PrimField<std::string::String>,
    trust_password: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trust_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DirectoryServiceTrust_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DirectoryServiceTrustData_>,
}

#[derive(Clone)]
pub struct DirectoryServiceTrust(std::rc::Rc<DirectoryServiceTrust_>);

impl DirectoryServiceTrust {
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

    #[doc = "Set the field `conditional_forwarder_ip_addrs`.\n"]
    pub fn set_conditional_forwarder_ip_addrs(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().conditional_forwarder_ip_addrs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_associated_conditional_forwarder`.\n"]
    pub fn set_delete_associated_conditional_forwarder(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().delete_associated_conditional_forwarder = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `selective_auth`.\n"]
    pub fn set_selective_auth(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().selective_auth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `trust_type`.\n"]
    pub fn set_trust_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().trust_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `conditional_forwarder_ip_addrs` after provisioning.\n"]
    pub fn conditional_forwarder_ip_addrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.conditional_forwarder_ip_addrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date_time` after provisioning.\n"]
    pub fn created_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_associated_conditional_forwarder` after provisioning.\n"]
    pub fn delete_associated_conditional_forwarder(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_associated_conditional_forwarder", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date_time` after provisioning.\n"]
    pub fn last_updated_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_name` after provisioning.\n"]
    pub fn remote_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selective_auth` after provisioning.\n"]
    pub fn selective_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selective_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_last_updated_date_time` after provisioning.\n"]
    pub fn state_last_updated_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_last_updated_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_direction` after provisioning.\n"]
    pub fn trust_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_password` after provisioning.\n"]
    pub fn trust_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state` after provisioning.\n"]
    pub fn trust_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state_reason` after provisioning.\n"]
    pub fn trust_state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_type` after provisioning.\n"]
    pub fn trust_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_type", self.extract_ref()))
    }
}

impl terrars::Referable for DirectoryServiceTrust {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DirectoryServiceTrust { }

impl terrars::ToListMappable for DirectoryServiceTrust {
    type O = terrars::ListRef<DirectoryServiceTrustRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DirectoryServiceTrust_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_directory_service_trust".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDirectoryServiceTrust {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub directory_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub remote_domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub trust_direction: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub trust_password: terrars::PrimField<std::string::String>,
}

impl BuildDirectoryServiceTrust {
    pub fn build(self, stack: &mut terrars::Stack) -> DirectoryServiceTrust {
        let out = DirectoryServiceTrust(std::rc::Rc::new(DirectoryServiceTrust_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DirectoryServiceTrustData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                conditional_forwarder_ip_addrs: core::default::Default::default(),
                delete_associated_conditional_forwarder: core::default::Default::default(),
                directory_id: self.directory_id,
                region: core::default::Default::default(),
                remote_domain_name: self.remote_domain_name,
                selective_auth: core::default::Default::default(),
                trust_direction: self.trust_direction,
                trust_password: self.trust_password,
                trust_type: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DirectoryServiceTrustRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DirectoryServiceTrustRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DirectoryServiceTrustRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conditional_forwarder_ip_addrs` after provisioning.\n"]
    pub fn conditional_forwarder_ip_addrs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.conditional_forwarder_ip_addrs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date_time` after provisioning.\n"]
    pub fn created_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_associated_conditional_forwarder` after provisioning.\n"]
    pub fn delete_associated_conditional_forwarder(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.delete_associated_conditional_forwarder", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `directory_id` after provisioning.\n"]
    pub fn directory_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date_time` after provisioning.\n"]
    pub fn last_updated_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date_time", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `remote_domain_name` after provisioning.\n"]
    pub fn remote_domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.remote_domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `selective_auth` after provisioning.\n"]
    pub fn selective_auth(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.selective_auth", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state_last_updated_date_time` after provisioning.\n"]
    pub fn state_last_updated_date_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state_last_updated_date_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_direction` after provisioning.\n"]
    pub fn trust_direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_direction", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_password` after provisioning.\n"]
    pub fn trust_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_password", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state` after provisioning.\n"]
    pub fn trust_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_state_reason` after provisioning.\n"]
    pub fn trust_state_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_state_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `trust_type` after provisioning.\n"]
    pub fn trust_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trust_type", self.extract_ref()))
    }
}
