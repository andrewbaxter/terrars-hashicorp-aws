#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AmplifyDomainAssociationData_ {
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
    app_id: terrars::PrimField<std::string::String>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_auto_sub_domain: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_verification: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_settings: std::option::Option<std::vec::Vec<AmplifyDomainAssociation_CertificateSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sub_domain: std::option::Option<std::vec::Vec<AmplifyDomainAssociation_SubDomain>>,
    dynamic: AmplifyDomainAssociationDynamic,
}

struct AmplifyDomainAssociation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AmplifyDomainAssociationData_>,
}

#[derive(Clone)]
pub struct AmplifyDomainAssociation(std::rc::Rc<AmplifyDomainAssociation_>);

impl AmplifyDomainAssociation {
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

    #[doc = "Set the field `enable_auto_sub_domain`.\n"]
    pub fn set_enable_auto_sub_domain(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_auto_sub_domain = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `wait_for_verification`.\n"]
    pub fn set_wait_for_verification(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_verification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_settings`.\n"]
    pub fn set_certificate_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AmplifyDomainAssociation_CertificateSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sub_domain`.\n"]
    pub fn set_sub_domain(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AmplifyDomainAssociation_SubDomain>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sub_domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sub_domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_verification_dns_record` after provisioning.\n"]
    pub fn certificate_verification_dns_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_verification_dns_record", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_sub_domain` after provisioning.\n"]
    pub fn enable_auto_sub_domain(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_sub_domain", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `wait_for_verification` after provisioning.\n"]
    pub fn wait_for_verification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_verification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_settings` after provisioning.\n"]
    pub fn certificate_settings(&self) -> terrars::ListRef<AmplifyDomainAssociation_CertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_settings", self.extract_ref()))
    }
}

impl terrars::Referable for AmplifyDomainAssociation {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AmplifyDomainAssociation { }

impl terrars::ToListMappable for AmplifyDomainAssociation {
    type O = terrars::ListRef<AmplifyDomainAssociationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AmplifyDomainAssociation_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_amplify_domain_association".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAmplifyDomainAssociation {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildAmplifyDomainAssociation {
    pub fn build(self, stack: &mut terrars::Stack) -> AmplifyDomainAssociation {
        let out = AmplifyDomainAssociation(std::rc::Rc::new(AmplifyDomainAssociation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AmplifyDomainAssociationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_id: self.app_id,
                domain_name: self.domain_name,
                enable_auto_sub_domain: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                wait_for_verification: core::default::Default::default(),
                certificate_settings: core::default::Default::default(),
                sub_domain: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AmplifyDomainAssociationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyDomainAssociationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AmplifyDomainAssociationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_id` after provisioning.\n"]
    pub fn app_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_verification_dns_record` after provisioning.\n"]
    pub fn certificate_verification_dns_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_verification_dns_record", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_auto_sub_domain` after provisioning.\n"]
    pub fn enable_auto_sub_domain(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_auto_sub_domain", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `wait_for_verification` after provisioning.\n"]
    pub fn wait_for_verification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_verification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_settings` after provisioning.\n"]
    pub fn certificate_settings(&self) -> terrars::ListRef<AmplifyDomainAssociation_CertificateSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.certificate_settings", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AmplifyDomainAssociationDynamic {
    certificate_settings: std::option::Option<terrars::DynamicBlock<AmplifyDomainAssociation_CertificateSettings>>,
    sub_domain: std::option::Option<terrars::DynamicBlock<AmplifyDomainAssociation_SubDomain>>,
}

#[derive(Serialize)]
pub struct AmplifyDomainAssociation_CertificateSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_certificate_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl AmplifyDomainAssociation_CertificateSettings {
    #[doc = "Set the field `custom_certificate_arn`.\n"]
    pub fn set_custom_certificate_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_certificate_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AmplifyDomainAssociation_CertificateSettings {
    type O = terrars::BlockAssignable<AmplifyDomainAssociation_CertificateSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyDomainAssociation_CertificateSettings {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildAmplifyDomainAssociation_CertificateSettings {
    pub fn build(self) -> AmplifyDomainAssociation_CertificateSettings {
        AmplifyDomainAssociation_CertificateSettings {
            custom_certificate_arn: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct AmplifyDomainAssociation_CertificateSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyDomainAssociation_CertificateSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AmplifyDomainAssociation_CertificateSettingsRef {
        AmplifyDomainAssociation_CertificateSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyDomainAssociation_CertificateSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_verification_dns_record` after provisioning.\n"]
    pub fn certificate_verification_dns_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_verification_dns_record", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_certificate_arn` after provisioning.\n"]
    pub fn custom_certificate_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_certificate_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct AmplifyDomainAssociation_SubDomain {
    branch_name: terrars::PrimField<std::string::String>,
    prefix: terrars::PrimField<std::string::String>,
}

impl AmplifyDomainAssociation_SubDomain { }

impl terrars::ToListMappable for AmplifyDomainAssociation_SubDomain {
    type O = terrars::BlockAssignable<AmplifyDomainAssociation_SubDomain>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAmplifyDomainAssociation_SubDomain {
    #[doc = ""]
    pub branch_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub prefix: terrars::PrimField<std::string::String>,
}

impl BuildAmplifyDomainAssociation_SubDomain {
    pub fn build(self) -> AmplifyDomainAssociation_SubDomain {
        AmplifyDomainAssociation_SubDomain {
            branch_name: self.branch_name,
            prefix: self.prefix,
        }
    }
}

#[allow(dead_code)]
pub struct AmplifyDomainAssociation_SubDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AmplifyDomainAssociation_SubDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AmplifyDomainAssociation_SubDomainRef {
        AmplifyDomainAssociation_SubDomainRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AmplifyDomainAssociation_SubDomainRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_name` after provisioning.\n"]
    pub fn branch_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_name", self.base))
    }

    #[doc = "Get a reference to the value of field `dns_record` after provisioning.\n"]
    pub fn dns_record(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_record", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `verified` after provisioning.\n"]
    pub fn verified(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified", self.base))
    }
}
