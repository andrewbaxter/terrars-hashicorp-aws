#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchomniDomainData_ {
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
    identity_providers: terrars::SetField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_provider_configuration: std
    ::option
    ::Option<std::vec::Vec<CloudwatchomniDomain_IdentityProviderConfiguration>>,
    dynamic: CloudwatchomniDomainDynamic,
}

struct CloudwatchomniDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchomniDomainData_>,
}

#[derive(Clone)]
pub struct CloudwatchomniDomain(std::rc::Rc<CloudwatchomniDomain_>);

impl CloudwatchomniDomain {
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

    #[doc = "Set the field `identity_provider_configuration`.\n"]
    pub fn set_identity_provider_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchomniDomain_IdentityProviderConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().identity_provider_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.identity_provider_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_endpoint_urls` after provisioning.\n"]
    pub fn custom_endpoint_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_endpoint_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_url` after provisioning.\n"]
    pub fn domain_endpoint_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_endpoint_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_application_arn` after provisioning.\n"]
    pub fn identity_center_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_center_application_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity_providers` after provisioning.\n"]
    pub fn identity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identity_providers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `identity_provider_configuration` after provisioning.\n"]
    pub fn identity_provider_configuration(
        &self,
    ) -> terrars::ListRef<CloudwatchomniDomain_IdentityProviderConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.identity_provider_configuration", self.extract_ref()),
        )
    }
}

impl terrars::Referable for CloudwatchomniDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchomniDomain { }

impl terrars::ToListMappable for CloudwatchomniDomain {
    type O = terrars::ListRef<CloudwatchomniDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchomniDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatchomni_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchomniDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_providers: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchomniDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchomniDomain {
        let out = CloudwatchomniDomain(std::rc::Rc::new(CloudwatchomniDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchomniDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                identity_providers: self.identity_providers,
                name: self.name,
                region: core::default::Default::default(),
                identity_provider_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchomniDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchomniDomainRef {
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

    #[doc = "Get a reference to the value of field `custom_endpoint_urls` after provisioning.\n"]
    pub fn custom_endpoint_urls(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_endpoint_urls", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_endpoint_url` after provisioning.\n"]
    pub fn domain_endpoint_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_endpoint_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_center_application_arn` after provisioning.\n"]
    pub fn identity_center_application_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.identity_center_application_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `identity_providers` after provisioning.\n"]
    pub fn identity_providers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.identity_providers", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `identity_provider_configuration` after provisioning.\n"]
    pub fn identity_provider_configuration(
        &self,
    ) -> terrars::ListRef<CloudwatchomniDomain_IdentityProviderConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.identity_provider_configuration", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniDomainDynamic {
    identity_provider_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchomniDomain_IdentityProviderConfiguration>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration {
    identity_center_instance_arn: terrars::PrimField<std::string::String>,
}

impl CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration { }

impl terrars::ToListMappable for CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration {
    type O =
        terrars::BlockAssignable<CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration {
    #[doc = ""]
    pub identity_center_instance_arn: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration {
    pub fn build(self) -> CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration {
        CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration {
            identity_center_instance_arn: self.identity_center_instance_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfigurationRef {
        CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center_instance_arn` after provisioning.\n"]
    pub fn identity_center_instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_center_instance_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniDomain_IdentityProviderConfigurationDynamic {
    identity_center_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniDomain_IdentityProviderConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    identity_center_configuration: std
    ::option
    ::Option<std::vec::Vec<CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration>>,
    dynamic: CloudwatchomniDomain_IdentityProviderConfigurationDynamic,
}

impl CloudwatchomniDomain_IdentityProviderConfiguration {
    #[doc = "Set the field `identity_center_configuration`.\n"]
    pub fn set_identity_center_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.identity_center_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.identity_center_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchomniDomain_IdentityProviderConfiguration {
    type O = terrars::BlockAssignable<CloudwatchomniDomain_IdentityProviderConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniDomain_IdentityProviderConfiguration {}

impl BuildCloudwatchomniDomain_IdentityProviderConfiguration {
    pub fn build(self) -> CloudwatchomniDomain_IdentityProviderConfiguration {
        CloudwatchomniDomain_IdentityProviderConfiguration {
            identity_center_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniDomain_IdentityProviderConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniDomain_IdentityProviderConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchomniDomain_IdentityProviderConfigurationRef {
        CloudwatchomniDomain_IdentityProviderConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniDomain_IdentityProviderConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identity_center_configuration` after provisioning.\n"]
    pub fn identity_center_configuration(
        &self,
    ) -> terrars::ListRef<CloudwatchomniDomain_IdentityProviderConfiguration_IdentityCenterConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identity_center_configuration", self.base))
    }
}
