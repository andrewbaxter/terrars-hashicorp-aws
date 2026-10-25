#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct IotCaCertificateData_ {
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
    active: terrars::PrimField<bool>,
    allow_auto_registration: terrars::PrimField<bool>,
    ca_certificate_pem: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    certificate_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_certificate_pem: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registration_config: std::option::Option<std::vec::Vec<IotCaCertificate_RegistrationConfig>>,
    dynamic: IotCaCertificateDynamic,
}

struct IotCaCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<IotCaCertificateData_>,
}

#[derive(Clone)]
pub struct IotCaCertificate(std::rc::Rc<IotCaCertificate_>);

impl IotCaCertificate {
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

    #[doc = "Set the field `certificate_mode`.\n"]
    pub fn set_certificate_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().certificate_mode = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `verification_certificate_pem`.\n"]
    pub fn set_verification_certificate_pem(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().verification_certificate_pem = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `registration_config`.\n"]
    pub fn set_registration_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<IotCaCertificate_RegistrationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().registration_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.registration_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_auto_registration` after provisioning.\n"]
    pub fn allow_auto_registration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_registration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_pem` after provisioning.\n"]
    pub fn ca_certificate_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_version` after provisioning.\n"]
    pub fn customer_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generation_id` after provisioning.\n"]
    pub fn generation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> terrars::ListRef<IotCaCertificate_ValidityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_certificate_pem` after provisioning.\n"]
    pub fn verification_certificate_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_config` after provisioning.\n"]
    pub fn registration_config(&self) -> terrars::ListRef<IotCaCertificate_RegistrationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registration_config", self.extract_ref()))
    }
}

impl terrars::Referable for IotCaCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for IotCaCertificate { }

impl terrars::ToListMappable for IotCaCertificate {
    type O = terrars::ListRef<IotCaCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for IotCaCertificate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_iot_ca_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildIotCaCertificate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub active: terrars::PrimField<bool>,
    #[doc = ""]
    pub allow_auto_registration: terrars::PrimField<bool>,
    #[doc = ""]
    pub ca_certificate_pem: terrars::PrimField<std::string::String>,
}

impl BuildIotCaCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> IotCaCertificate {
        let out = IotCaCertificate(std::rc::Rc::new(IotCaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(IotCaCertificateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                active: self.active,
                allow_auto_registration: self.allow_auto_registration,
                ca_certificate_pem: self.ca_certificate_pem,
                certificate_mode: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                verification_certificate_pem: core::default::Default::default(),
                registration_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct IotCaCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotCaCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl IotCaCertificateRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `active` after provisioning.\n"]
    pub fn active(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.active", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allow_auto_registration` after provisioning.\n"]
    pub fn allow_auto_registration(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allow_auto_registration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ca_certificate_pem` after provisioning.\n"]
    pub fn ca_certificate_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ca_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_mode` after provisioning.\n"]
    pub fn certificate_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_version` after provisioning.\n"]
    pub fn customer_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `generation_id` after provisioning.\n"]
    pub fn generation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.generation_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> terrars::ListRef<IotCaCertificate_ValidityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verification_certificate_pem` after provisioning.\n"]
    pub fn verification_certificate_pem(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verification_certificate_pem", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registration_config` after provisioning.\n"]
    pub fn registration_config(&self) -> terrars::ListRef<IotCaCertificate_RegistrationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.registration_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct IotCaCertificateDynamic {
    registration_config: std::option::Option<terrars::DynamicBlock<IotCaCertificate_RegistrationConfig>>,
}

#[derive(Serialize)]
pub struct IotCaCertificate_Validity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_before: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotCaCertificate_Validity {
    #[doc = "Set the field `not_after`.\n"]
    pub fn set_not_after(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.not_after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_before`.\n"]
    pub fn set_not_before(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.not_before = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotCaCertificate_Validity {
    type O = terrars::BlockAssignable<IotCaCertificate_Validity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotCaCertificate_Validity {}

impl BuildIotCaCertificate_Validity {
    pub fn build(self) -> IotCaCertificate_Validity {
        IotCaCertificate_Validity {
            not_after: core::default::Default::default(),
            not_before: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotCaCertificate_ValidityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotCaCertificate_ValidityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotCaCertificate_ValidityRef {
        IotCaCertificate_ValidityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotCaCertificate_ValidityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `not_after` after provisioning.\n"]
    pub fn not_after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_after", self.base))
    }

    #[doc = "Get a reference to the value of field `not_before` after provisioning.\n"]
    pub fn not_before(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.not_before", self.base))
    }
}

#[derive(Serialize)]
pub struct IotCaCertificate_RegistrationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_body: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl IotCaCertificate_RegistrationConfig {
    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template_body`.\n"]
    pub fn set_template_body(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.template_body = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template_name`.\n"]
    pub fn set_template_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.template_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for IotCaCertificate_RegistrationConfig {
    type O = terrars::BlockAssignable<IotCaCertificate_RegistrationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildIotCaCertificate_RegistrationConfig {}

impl BuildIotCaCertificate_RegistrationConfig {
    pub fn build(self) -> IotCaCertificate_RegistrationConfig {
        IotCaCertificate_RegistrationConfig {
            role_arn: core::default::Default::default(),
            template_body: core::default::Default::default(),
            template_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct IotCaCertificate_RegistrationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for IotCaCertificate_RegistrationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> IotCaCertificate_RegistrationConfigRef {
        IotCaCertificate_RegistrationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl IotCaCertificate_RegistrationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `template_body` after provisioning.\n"]
    pub fn template_body(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_body", self.base))
    }

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.base))
    }
}
