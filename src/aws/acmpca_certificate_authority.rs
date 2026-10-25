#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AcmpcaCertificateAuthorityData_ {
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
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_storage_security_standard: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permanent_deletion_time_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    usage_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_authority_configuration: std
    ::option
    ::Option<std::vec::Vec<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    revocation_configuration: std::option::Option<std::vec::Vec<AcmpcaCertificateAuthority_RevocationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<AcmpcaCertificateAuthority_Timeouts>,
    dynamic: AcmpcaCertificateAuthorityDynamic,
}

struct AcmpcaCertificateAuthority_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AcmpcaCertificateAuthorityData_>,
}

#[derive(Clone)]
pub struct AcmpcaCertificateAuthority(std::rc::Rc<AcmpcaCertificateAuthority_>);

impl AcmpcaCertificateAuthority {
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

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_storage_security_standard`.\n"]
    pub fn set_key_storage_security_standard(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().key_storage_security_standard = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `permanent_deletion_time_in_days`.\n"]
    pub fn set_permanent_deletion_time_in_days(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().permanent_deletion_time_in_days = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `usage_mode`.\n"]
    pub fn set_usage_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().usage_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_authority_configuration`.\n"]
    pub fn set_certificate_authority_configuration(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().certificate_authority_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.certificate_authority_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `revocation_configuration`.\n"]
    pub fn set_revocation_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AcmpcaCertificateAuthority_RevocationConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().revocation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.revocation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<AcmpcaCertificateAuthority_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_storage_security_standard` after provisioning.\n"]
    pub fn key_storage_security_standard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_storage_security_standard", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permanent_deletion_time_in_days` after provisioning.\n"]
    pub fn permanent_deletion_time_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.permanent_deletion_time_in_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_configuration` after provisioning.\n"]
    pub fn certificate_authority_configuration(
        &self,
    ) -> terrars::ListRef<AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(&self) -> terrars::ListRef<AcmpcaCertificateAuthority_RevocationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AcmpcaCertificateAuthority_TimeoutsRef {
        AcmpcaCertificateAuthority_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for AcmpcaCertificateAuthority {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AcmpcaCertificateAuthority { }

impl terrars::ToListMappable for AcmpcaCertificateAuthority {
    type O = terrars::ListRef<AcmpcaCertificateAuthorityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AcmpcaCertificateAuthority_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_acmpca_certificate_authority".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAcmpcaCertificateAuthority {
    pub tf_id: std::string::String,
}

impl BuildAcmpcaCertificateAuthority {
    pub fn build(self, stack: &mut terrars::Stack) -> AcmpcaCertificateAuthority {
        let out = AcmpcaCertificateAuthority(std::rc::Rc::new(AcmpcaCertificateAuthority_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AcmpcaCertificateAuthorityData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                key_storage_security_standard: core::default::Default::default(),
                permanent_deletion_time_in_days: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                usage_mode: core::default::Default::default(),
                certificate_authority_configuration: core::default::Default::default(),
                revocation_configuration: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AcmpcaCertificateAuthorityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthorityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AcmpcaCertificateAuthorityRef {
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

    #[doc = "Get a reference to the value of field `certificate` after provisioning.\n"]
    pub fn certificate(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_storage_security_standard` after provisioning.\n"]
    pub fn key_storage_security_standard(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.key_storage_security_standard", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permanent_deletion_time_in_days` after provisioning.\n"]
    pub fn permanent_deletion_time_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.permanent_deletion_time_in_days", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial` after provisioning.\n"]
    pub fn serial(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `usage_mode` after provisioning.\n"]
    pub fn usage_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_configuration` after provisioning.\n"]
    pub fn certificate_authority_configuration(
        &self,
    ) -> terrars::ListRef<AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.certificate_authority_configuration", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `revocation_configuration` after provisioning.\n"]
    pub fn revocation_configuration(&self) -> terrars::ListRef<AcmpcaCertificateAuthority_RevocationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.revocation_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> AcmpcaCertificateAuthority_TimeoutsRef {
        AcmpcaCertificateAuthority_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateAuthorityDynamic {
    certificate_authority_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration>>,
    revocation_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AcmpcaCertificateAuthority_RevocationConfiguration>>,
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    common_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    country: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    distinguished_name_qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generation_qualifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    given_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initials: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    locality: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    organizational_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pseudonym: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    surname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {
    #[doc = "Set the field `common_name`.\n"]
    pub fn set_common_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.common_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `country`.\n"]
    pub fn set_country(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.country = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `distinguished_name_qualifier`.\n"]
    pub fn set_distinguished_name_qualifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.distinguished_name_qualifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `generation_qualifier`.\n"]
    pub fn set_generation_qualifier(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.generation_qualifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `given_name`.\n"]
    pub fn set_given_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.given_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `initials`.\n"]
    pub fn set_initials(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.initials = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `locality`.\n"]
    pub fn set_locality(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.locality = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organization`.\n"]
    pub fn set_organization(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.organization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `organizational_unit`.\n"]
    pub fn set_organizational_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.organizational_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pseudonym`.\n"]
    pub fn set_pseudonym(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.pseudonym = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `surname`.\n"]
    pub fn set_surname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.surname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.title = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {
    type O = terrars::BlockAssignable<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {}

impl BuildAcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {
    pub fn build(self) -> AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {
        AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject {
            common_name: core::default::Default::default(),
            country: core::default::Default::default(),
            distinguished_name_qualifier: core::default::Default::default(),
            generation_qualifier: core::default::Default::default(),
            given_name: core::default::Default::default(),
            initials: core::default::Default::default(),
            locality: core::default::Default::default(),
            organization: core::default::Default::default(),
            organizational_unit: core::default::Default::default(),
            pseudonym: core::default::Default::default(),
            state: core::default::Default::default(),
            surname: core::default::Default::default(),
            title: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_SubjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_SubjectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_SubjectRef {
        AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_SubjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_SubjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `common_name` after provisioning.\n"]
    pub fn common_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.common_name", self.base))
    }

    #[doc = "Get a reference to the value of field `country` after provisioning.\n"]
    pub fn country(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.country", self.base))
    }

    #[doc = "Get a reference to the value of field `distinguished_name_qualifier` after provisioning.\n"]
    pub fn distinguished_name_qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.distinguished_name_qualifier", self.base))
    }

    #[doc = "Get a reference to the value of field `generation_qualifier` after provisioning.\n"]
    pub fn generation_qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_qualifier", self.base))
    }

    #[doc = "Get a reference to the value of field `given_name` after provisioning.\n"]
    pub fn given_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.given_name", self.base))
    }

    #[doc = "Get a reference to the value of field `initials` after provisioning.\n"]
    pub fn initials(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initials", self.base))
    }

    #[doc = "Get a reference to the value of field `locality` after provisioning.\n"]
    pub fn locality(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locality", self.base))
    }

    #[doc = "Get a reference to the value of field `organization` after provisioning.\n"]
    pub fn organization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization", self.base))
    }

    #[doc = "Get a reference to the value of field `organizational_unit` after provisioning.\n"]
    pub fn organizational_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organizational_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `pseudonym` after provisioning.\n"]
    pub fn pseudonym(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pseudonym", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `surname` after provisioning.\n"]
    pub fn surname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.surname", self.base))
    }

    #[doc = "Get a reference to the value of field `title` after provisioning.\n"]
    pub fn title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.title", self.base))
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateAuthority_CertificateAuthorityConfigurationDynamic {
    subject: std
    ::option
    ::Option<terrars::DynamicBlock<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject>>,
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
    key_algorithm: terrars::PrimField<std::string::String>,
    signing_algorithm: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject: std::option::Option<std::vec::Vec<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject>>,
    dynamic: AcmpcaCertificateAuthority_CertificateAuthorityConfigurationDynamic,
}

impl AcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
    #[doc = "Set the field `subject`.\n"]
    pub fn set_subject(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_Subject>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.subject = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.subject = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
    type O = terrars::BlockAssignable<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
    #[doc = ""]
    pub key_algorithm: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub signing_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildAcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
    pub fn build(self) -> AcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
        AcmpcaCertificateAuthority_CertificateAuthorityConfiguration {
            key_algorithm: self.key_algorithm,
            signing_algorithm: self.signing_algorithm,
            subject: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef {
        AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthority_CertificateAuthorityConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `subject` after provisioning.\n"]
    pub fn subject(&self) -> terrars::ListRef<AcmpcaCertificateAuthority_CertificateAuthorityConfiguration_SubjectRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subject", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_cname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expiration_in_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_object_acl: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {
    #[doc = "Set the field `custom_cname`.\n"]
    pub fn set_custom_cname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_cname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_path`.\n"]
    pub fn set_custom_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.custom_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `expiration_in_days`.\n"]
    pub fn set_expiration_in_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.expiration_in_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_bucket_name`.\n"]
    pub fn set_s3_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_object_acl`.\n"]
    pub fn set_s3_object_acl(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_object_acl = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {
    type O = terrars::BlockAssignable<AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {}

impl BuildAcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {
    pub fn build(self) -> AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {
        AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration {
            custom_cname: core::default::Default::default(),
            custom_path: core::default::Default::default(),
            enabled: core::default::Default::default(),
            expiration_in_days: core::default::Default::default(),
            s3_bucket_name: core::default::Default::default(),
            s3_object_acl: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfigurationRef {
        AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_cname` after provisioning.\n"]
    pub fn custom_cname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_cname", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_path` after provisioning.\n"]
    pub fn custom_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_path", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `expiration_in_days` after provisioning.\n"]
    pub fn expiration_in_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expiration_in_days", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_object_acl` after provisioning.\n"]
    pub fn s3_object_acl(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_object_acl", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_custom_cname: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
    #[doc = "Set the field `ocsp_custom_cname`.\n"]
    pub fn set_ocsp_custom_cname(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ocsp_custom_cname = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
    type O = terrars::BlockAssignable<AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildAcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
    pub fn build(self) -> AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
        AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration {
            enabled: self.enabled,
            ocsp_custom_cname: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfigurationRef {
        AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_custom_cname` after provisioning.\n"]
    pub fn ocsp_custom_cname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsp_custom_cname", self.base))
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateAuthority_RevocationConfigurationDynamic {
    crl_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration>>,
    ocsp_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration>>,
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthority_RevocationConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crl_configuration: std
    ::option
    ::Option<std::vec::Vec<AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ocsp_configuration: std
    ::option
    ::Option<std::vec::Vec<AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration>>,
    dynamic: AcmpcaCertificateAuthority_RevocationConfigurationDynamic,
}

impl AcmpcaCertificateAuthority_RevocationConfiguration {
    #[doc = "Set the field `crl_configuration`.\n"]
    pub fn set_crl_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crl_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crl_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ocsp_configuration`.\n"]
    pub fn set_ocsp_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ocsp_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ocsp_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AcmpcaCertificateAuthority_RevocationConfiguration {
    type O = terrars::BlockAssignable<AcmpcaCertificateAuthority_RevocationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthority_RevocationConfiguration {}

impl BuildAcmpcaCertificateAuthority_RevocationConfiguration {
    pub fn build(self) -> AcmpcaCertificateAuthority_RevocationConfiguration {
        AcmpcaCertificateAuthority_RevocationConfiguration {
            crl_configuration: core::default::Default::default(),
            ocsp_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmpcaCertificateAuthority_RevocationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthority_RevocationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AcmpcaCertificateAuthority_RevocationConfigurationRef {
        AcmpcaCertificateAuthority_RevocationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthority_RevocationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `crl_configuration` after provisioning.\n"]
    pub fn crl_configuration(
        &self,
    ) -> terrars::ListRef<AcmpcaCertificateAuthority_RevocationConfiguration_CrlConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crl_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `ocsp_configuration` after provisioning.\n"]
    pub fn ocsp_configuration(
        &self,
    ) -> terrars::ListRef<AcmpcaCertificateAuthority_RevocationConfiguration_OcspConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ocsp_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateAuthority_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmpcaCertificateAuthority_Timeouts {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmpcaCertificateAuthority_Timeouts {
    type O = terrars::BlockAssignable<AcmpcaCertificateAuthority_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateAuthority_Timeouts {}

impl BuildAcmpcaCertificateAuthority_Timeouts {
    pub fn build(self) -> AcmpcaCertificateAuthority_Timeouts {
        AcmpcaCertificateAuthority_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct AcmpcaCertificateAuthority_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateAuthority_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AcmpcaCertificateAuthority_TimeoutsRef {
        AcmpcaCertificateAuthority_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateAuthority_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
