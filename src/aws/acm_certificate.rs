#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AcmCertificateData_ {
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
    certificate_authority_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_chain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    early_renewal_duration: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_wo: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_key_wo_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subject_alternative_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    options: std::option::Option<std::vec::Vec<AcmCertificate_Options>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validation_option: std::option::Option<std::vec::Vec<AcmCertificate_ValidationOption>>,
    dynamic: AcmCertificateDynamic,
}

struct AcmCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AcmCertificateData_>,
}

#[derive(Clone)]
pub struct AcmCertificate(std::rc::Rc<AcmCertificate_>);

impl AcmCertificate {
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

    #[doc = "Set the field `certificate_authority_arn`.\n"]
    pub fn set_certificate_authority_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().certificate_authority_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_body`.\n"]
    pub fn set_certificate_body(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `certificate_chain`.\n"]
    pub fn set_certificate_chain(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_chain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `early_renewal_duration`.\n"]
    pub fn set_early_renewal_duration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().early_renewal_duration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_algorithm`.\n"]
    pub fn set_key_algorithm(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key`.\n"]
    pub fn set_private_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_wo`.\n"]
    pub fn set_private_key_wo(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().private_key_wo = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_key_wo_version`.\n"]
    pub fn set_private_key_wo_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().private_key_wo_version = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subject_alternative_names`.\n"]
    pub fn set_subject_alternative_names(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().subject_alternative_names = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `validation_method`.\n"]
    pub fn set_validation_method(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().validation_method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `options`.\n"]
    pub fn set_options(self, v: impl std::convert::Into<terrars::BlockAssignable<AcmCertificate_Options>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `validation_option`.\n"]
    pub fn set_validation_option(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AcmCertificate_ValidationOption>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validation_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validation_option = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_body` after provisioning.\n"]
    pub fn certificate_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_validation_options` after provisioning.\n"]
    pub fn domain_validation_options(&self) -> terrars::SetRef<AcmCertificate_DomainValidationOptionsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domain_validation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `early_renewal_duration` after provisioning.\n"]
    pub fn early_renewal_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_renewal_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_renewal` after provisioning.\n"]
    pub fn pending_renewal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_renewal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key_wo` after provisioning.\n"]
    pub fn private_key_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key_wo_version` after provisioning.\n"]
    pub fn private_key_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `renewal_eligibility` after provisioning.\n"]
    pub fn renewal_eligibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renewal_eligibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `renewal_summary` after provisioning.\n"]
    pub fn renewal_summary(&self) -> terrars::ListRef<AcmCertificate_RenewalSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renewal_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `validation_emails` after provisioning.\n"]
    pub fn validation_emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_method` after provisioning.\n"]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<AcmCertificate_OptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }
}

impl terrars::Referable for AcmCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AcmCertificate { }

impl terrars::ToListMappable for AcmCertificate {
    type O = terrars::ListRef<AcmCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AcmCertificate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_acm_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAcmCertificate {
    pub tf_id: std::string::String,
}

impl BuildAcmCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> AcmCertificate {
        let out = AcmCertificate(std::rc::Rc::new(AcmCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AcmCertificateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                certificate_authority_arn: core::default::Default::default(),
                certificate_body: core::default::Default::default(),
                certificate_chain: core::default::Default::default(),
                domain_name: core::default::Default::default(),
                early_renewal_duration: core::default::Default::default(),
                id: core::default::Default::default(),
                key_algorithm: core::default::Default::default(),
                private_key: core::default::Default::default(),
                private_key_wo: core::default::Default::default(),
                private_key_wo_version: core::default::Default::default(),
                region: core::default::Default::default(),
                subject_alternative_names: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                validation_method: core::default::Default::default(),
                options: core::default::Default::default(),
                validation_option: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AcmCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AcmCertificateRef {
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

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_body` after provisioning.\n"]
    pub fn certificate_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_body", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_validation_options` after provisioning.\n"]
    pub fn domain_validation_options(&self) -> terrars::SetRef<AcmCertificate_DomainValidationOptionsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.domain_validation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `early_renewal_duration` after provisioning.\n"]
    pub fn early_renewal_duration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.early_renewal_duration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_algorithm` after provisioning.\n"]
    pub fn key_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_renewal` after provisioning.\n"]
    pub fn pending_renewal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_renewal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key` after provisioning.\n"]
    pub fn private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key_wo` after provisioning.\n"]
    pub fn private_key_wo(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_wo", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_key_wo_version` after provisioning.\n"]
    pub fn private_key_wo_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_key_wo_version", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `renewal_eligibility` after provisioning.\n"]
    pub fn renewal_eligibility(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renewal_eligibility", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `renewal_summary` after provisioning.\n"]
    pub fn renewal_summary(&self) -> terrars::ListRef<AcmCertificate_RenewalSummaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.renewal_summary", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subject_alternative_names` after provisioning.\n"]
    pub fn subject_alternative_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subject_alternative_names", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `validation_emails` after provisioning.\n"]
    pub fn validation_emails(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validation_emails", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validation_method` after provisioning.\n"]
    pub fn validation_method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_method", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\n"]
    pub fn options(&self) -> terrars::ListRef<AcmCertificate_OptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct AcmCertificateDynamic {
    options: std::option::Option<terrars::DynamicBlock<AcmCertificate_Options>>,
    validation_option: std::option::Option<terrars::DynamicBlock<AcmCertificate_ValidationOption>>,
}

#[derive(Serialize)]
pub struct AcmCertificate_DomainValidationOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_record_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_record_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_record_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmCertificate_DomainValidationOptions {
    #[doc = "Set the field `domain_name`.\n"]
    pub fn set_domain_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_record_name`.\n"]
    pub fn set_resource_record_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_record_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_record_type`.\n"]
    pub fn set_resource_record_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_record_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_record_value`.\n"]
    pub fn set_resource_record_value(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.resource_record_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmCertificate_DomainValidationOptions {
    type O = terrars::BlockAssignable<AcmCertificate_DomainValidationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificate_DomainValidationOptions {}

impl BuildAcmCertificate_DomainValidationOptions {
    pub fn build(self) -> AcmCertificate_DomainValidationOptions {
        AcmCertificate_DomainValidationOptions {
            domain_name: core::default::Default::default(),
            resource_record_name: core::default::Default::default(),
            resource_record_type: core::default::Default::default(),
            resource_record_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmCertificate_DomainValidationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmCertificate_DomainValidationOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AcmCertificate_DomainValidationOptionsRef {
        AcmCertificate_DomainValidationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificate_DomainValidationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_record_name` after provisioning.\n"]
    pub fn resource_record_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_record_name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_record_type` after provisioning.\n"]
    pub fn resource_record_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_record_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_record_value` after provisioning.\n"]
    pub fn resource_record_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_record_value", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmCertificate_RenewalSummary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renewal_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    renewal_status_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmCertificate_RenewalSummary {
    #[doc = "Set the field `renewal_status`.\n"]
    pub fn set_renewal_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.renewal_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `renewal_status_reason`.\n"]
    pub fn set_renewal_status_reason(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.renewal_status_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmCertificate_RenewalSummary {
    type O = terrars::BlockAssignable<AcmCertificate_RenewalSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificate_RenewalSummary {}

impl BuildAcmCertificate_RenewalSummary {
    pub fn build(self) -> AcmCertificate_RenewalSummary {
        AcmCertificate_RenewalSummary {
            renewal_status: core::default::Default::default(),
            renewal_status_reason: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmCertificate_RenewalSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmCertificate_RenewalSummaryRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AcmCertificate_RenewalSummaryRef {
        AcmCertificate_RenewalSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificate_RenewalSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `renewal_status` after provisioning.\n"]
    pub fn renewal_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renewal_status", self.base))
    }

    #[doc = "Get a reference to the value of field `renewal_status_reason` after provisioning.\n"]
    pub fn renewal_status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.renewal_status_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmCertificate_Options {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_transparency_logging_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    export: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl AcmCertificate_Options {
    #[doc = "Set the field `certificate_transparency_logging_preference`.\n"]
    pub fn set_certificate_transparency_logging_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.certificate_transparency_logging_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `export`.\n"]
    pub fn set_export(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.export = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for AcmCertificate_Options {
    type O = terrars::BlockAssignable<AcmCertificate_Options>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificate_Options {}

impl BuildAcmCertificate_Options {
    pub fn build(self) -> AcmCertificate_Options {
        AcmCertificate_Options {
            certificate_transparency_logging_preference: core::default::Default::default(),
            export: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct AcmCertificate_OptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmCertificate_OptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AcmCertificate_OptionsRef {
        AcmCertificate_OptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificate_OptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `certificate_transparency_logging_preference` after provisioning.\n"]
    pub fn certificate_transparency_logging_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.certificate_transparency_logging_preference", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `export` after provisioning.\n"]
    pub fn export(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.export", self.base))
    }
}

#[derive(Serialize)]
pub struct AcmCertificate_ValidationOption {
    domain_name: terrars::PrimField<std::string::String>,
    validation_domain: terrars::PrimField<std::string::String>,
}

impl AcmCertificate_ValidationOption { }

impl terrars::ToListMappable for AcmCertificate_ValidationOption {
    type O = terrars::BlockAssignable<AcmCertificate_ValidationOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmCertificate_ValidationOption {
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub validation_domain: terrars::PrimField<std::string::String>,
}

impl BuildAcmCertificate_ValidationOption {
    pub fn build(self) -> AcmCertificate_ValidationOption {
        AcmCertificate_ValidationOption {
            domain_name: self.domain_name,
            validation_domain: self.validation_domain,
        }
    }
}

#[allow(dead_code)]
pub struct AcmCertificate_ValidationOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmCertificate_ValidationOptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AcmCertificate_ValidationOptionRef {
        AcmCertificate_ValidationOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmCertificate_ValidationOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.base))
    }

    #[doc = "Get a reference to the value of field `validation_domain` after provisioning.\n"]
    pub fn validation_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.validation_domain", self.base))
    }
}
