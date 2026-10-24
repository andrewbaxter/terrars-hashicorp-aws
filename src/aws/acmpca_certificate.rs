use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AcmpcaCertificateData {
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
    certificate_authority_arn: terrars::PrimField<std::string::String>,
    certificate_signing_request: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    signing_algorithm: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    validity: std::option::Option<std::vec::Vec<AcmpcaCertificateValidityEl>>,
    dynamic: AcmpcaCertificateDynamic,
}

struct AcmpcaCertificate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AcmpcaCertificateData>,
}

#[derive(Clone)]
pub struct AcmpcaCertificate(std::rc::Rc<AcmpcaCertificate_>);

impl AcmpcaCertificate {
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

    #[doc = "Set the field `template_arn`.\n"]
    pub fn set_template_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().template_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `validity`.\n"]
    pub fn set_validity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AcmpcaCertificateValidityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().validity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.validity = std::option::Option::Some(d);
            },
        }
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

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_arn` after provisioning.\n"]
    pub fn template_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> terrars::ListRef<AcmpcaCertificateValidityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }
}

impl terrars::Referable for AcmpcaCertificate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AcmpcaCertificate { }

impl terrars::ToListMappable for AcmpcaCertificate {
    type O = terrars::ListRef<AcmpcaCertificateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AcmpcaCertificate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_acmpca_certificate".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAcmpcaCertificate {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub certificate_authority_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub certificate_signing_request: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub signing_algorithm: terrars::PrimField<std::string::String>,
}

impl BuildAcmpcaCertificate {
    pub fn build(self, stack: &mut terrars::Stack) -> AcmpcaCertificate {
        let out = AcmpcaCertificate(std::rc::Rc::new(AcmpcaCertificate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AcmpcaCertificateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                certificate_authority_arn: self.certificate_authority_arn,
                certificate_signing_request: self.certificate_signing_request,
                id: core::default::Default::default(),
                signing_algorithm: self.signing_algorithm,
                template_arn: core::default::Default::default(),
                validity: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AcmpcaCertificateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AcmpcaCertificateRef {
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

    #[doc = "Get a reference to the value of field `certificate_authority_arn` after provisioning.\n"]
    pub fn certificate_authority_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_authority_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_chain` after provisioning.\n"]
    pub fn certificate_chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_chain", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `certificate_signing_request` after provisioning.\n"]
    pub fn certificate_signing_request(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.certificate_signing_request", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `signing_algorithm` after provisioning.\n"]
    pub fn signing_algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_algorithm", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `template_arn` after provisioning.\n"]
    pub fn template_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `validity` after provisioning.\n"]
    pub fn validity(&self) -> terrars::ListRef<AcmpcaCertificateValidityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.validity", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AcmpcaCertificateValidityEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl AcmpcaCertificateValidityEl { }

impl terrars::ToListMappable for AcmpcaCertificateValidityEl {
    type O = terrars::BlockAssignable<AcmpcaCertificateValidityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAcmpcaCertificateValidityEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildAcmpcaCertificateValidityEl {
    pub fn build(self) -> AcmpcaCertificateValidityEl {
        AcmpcaCertificateValidityEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct AcmpcaCertificateValidityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AcmpcaCertificateValidityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AcmpcaCertificateValidityElRef {
        AcmpcaCertificateValidityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AcmpcaCertificateValidityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AcmpcaCertificateDynamic {
    validity: std::option::Option<terrars::DynamicBlock<AcmpcaCertificateValidityEl>>,
}
