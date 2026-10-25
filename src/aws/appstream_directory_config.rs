#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AppstreamDirectoryConfigData_ {
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
    directory_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    organizational_unit_distinguished_names: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_based_auth_properties: std
    ::option
    ::Option<std::vec::Vec<AppstreamDirectoryConfig_CertificateBasedAuthProperties>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_account_credentials: std::option::Option<std::vec::Vec<AppstreamDirectoryConfig_ServiceAccountCredentials>>,
    dynamic: AppstreamDirectoryConfigDynamic,
}

struct AppstreamDirectoryConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AppstreamDirectoryConfigData_>,
}

#[derive(Clone)]
pub struct AppstreamDirectoryConfig(std::rc::Rc<AppstreamDirectoryConfig_>);

impl AppstreamDirectoryConfig {
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

    #[doc = "Set the field `certificate_based_auth_properties`.\n"]
    pub fn set_certificate_based_auth_properties(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AppstreamDirectoryConfig_CertificateBasedAuthProperties>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_based_auth_properties = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_based_auth_properties = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_account_credentials`.\n"]
    pub fn set_service_account_credentials(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AppstreamDirectoryConfig_ServiceAccountCredentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_account_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_account_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_names` after provisioning.\n"]
    pub fn organizational_unit_distinguished_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.organizational_unit_distinguished_names", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> terrars::ListRef<AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_based_auth_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_account_credentials` after provisioning.\n"]
    pub fn service_account_credentials(&self) -> terrars::ListRef<AppstreamDirectoryConfig_ServiceAccountCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_credentials", self.extract_ref()))
    }
}

impl terrars::Referable for AppstreamDirectoryConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AppstreamDirectoryConfig { }

impl terrars::ToListMappable for AppstreamDirectoryConfig {
    type O = terrars::ListRef<AppstreamDirectoryConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AppstreamDirectoryConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_appstream_directory_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAppstreamDirectoryConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub directory_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub organizational_unit_distinguished_names: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildAppstreamDirectoryConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> AppstreamDirectoryConfig {
        let out = AppstreamDirectoryConfig(std::rc::Rc::new(AppstreamDirectoryConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AppstreamDirectoryConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                directory_name: self.directory_name,
                id: core::default::Default::default(),
                organizational_unit_distinguished_names: self.organizational_unit_distinguished_names,
                region: core::default::Default::default(),
                certificate_based_auth_properties: core::default::Default::default(),
                service_account_credentials: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AppstreamDirectoryConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamDirectoryConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AppstreamDirectoryConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_time` after provisioning.\n"]
    pub fn created_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `directory_name` after provisioning.\n"]
    pub fn directory_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.directory_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `organizational_unit_distinguished_names` after provisioning.\n"]
    pub fn organizational_unit_distinguished_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.organizational_unit_distinguished_names", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_based_auth_properties` after provisioning.\n"]
    pub fn certificate_based_auth_properties(
        &self,
    ) -> terrars::ListRef<AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_based_auth_properties", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `service_account_credentials` after provisioning.\n"]
    pub fn service_account_credentials(&self) -> terrars::ListRef<AppstreamDirectoryConfig_ServiceAccountCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_account_credentials", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AppstreamDirectoryConfigDynamic {
    certificate_based_auth_properties: std
    ::option
    ::Option<terrars::DynamicBlock<AppstreamDirectoryConfig_CertificateBasedAuthProperties>>,
    service_account_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<AppstreamDirectoryConfig_ServiceAccountCredentials>>,
}

#[derive(Serialize)]
pub struct AppstreamDirectoryConfig_CertificateBasedAuthProperties {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AppstreamDirectoryConfig_CertificateBasedAuthProperties {
    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_authority_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AppstreamDirectoryConfig_CertificateBasedAuthProperties {
    type O = terrars::BlockAssignable<AppstreamDirectoryConfig_CertificateBasedAuthProperties>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamDirectoryConfig_CertificateBasedAuthProperties {}

impl BuildAppstreamDirectoryConfig_CertificateBasedAuthProperties {
    pub fn build(self) -> AppstreamDirectoryConfig_CertificateBasedAuthProperties {
        AppstreamDirectoryConfig_CertificateBasedAuthProperties {
            certificate_authority_arn: core::default::Default::default(),
            status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef {
        AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamDirectoryConfig_CertificateBasedAuthPropertiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }
}

#[derive(Serialize)]
pub struct AppstreamDirectoryConfig_ServiceAccountCredentials {
    account_name: terrars::PrimField<std::string::String>,
    account_password: terrars::PrimField<std::string::String>,
}

impl AppstreamDirectoryConfig_ServiceAccountCredentials { }

impl terrars::ToListMappable for AppstreamDirectoryConfig_ServiceAccountCredentials {
    type O = terrars::BlockAssignable<AppstreamDirectoryConfig_ServiceAccountCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAppstreamDirectoryConfig_ServiceAccountCredentials {
    #[doc = ""]
    pub account_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub account_password: terrars::PrimField<std::string::String>,
}

impl BuildAppstreamDirectoryConfig_ServiceAccountCredentials {
    pub fn build(self) -> AppstreamDirectoryConfig_ServiceAccountCredentials {
        AppstreamDirectoryConfig_ServiceAccountCredentials {
            account_name: self.account_name,
            account_password: self.account_password,
        }
    }
}

#[allow(dead_code)]
pub struct AppstreamDirectoryConfig_ServiceAccountCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AppstreamDirectoryConfig_ServiceAccountCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AppstreamDirectoryConfig_ServiceAccountCredentialsRef {
        AppstreamDirectoryConfig_ServiceAccountCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AppstreamDirectoryConfig_ServiceAccountCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_name` after provisioning.\n"]
    pub fn account_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_name", self.base))
    }

    #[doc = "Get a reference to the value of field `account_password` after provisioning.\n"]
    pub fn account_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_password", self.base))
    }
}
