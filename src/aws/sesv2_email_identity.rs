use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Sesv2EmailIdentityData {
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
    configuration_set_name: std::option::Option<terrars::PrimField<std::string::String>>,
    email_identity: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dkim_signing_attributes: std::option::Option<std::vec::Vec<Sesv2EmailIdentityDkimSigningAttributesEl>>,
    dynamic: Sesv2EmailIdentityDynamic,
}

struct Sesv2EmailIdentity_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Sesv2EmailIdentityData>,
}

#[derive(Clone)]
pub struct Sesv2EmailIdentity(std::rc::Rc<Sesv2EmailIdentity_>);

impl Sesv2EmailIdentity {
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

    #[doc = "Set the field `configuration_set_name`.\n"]
    pub fn set_configuration_set_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().configuration_set_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `dkim_signing_attributes`.\n"]
    pub fn set_dkim_signing_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Sesv2EmailIdentityDkimSigningAttributesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dkim_signing_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dkim_signing_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_for_sending_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(&self) -> terrars::ListRef<Sesv2EmailIdentityDkimSigningAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dkim_signing_attributes", self.extract_ref()))
    }
}

impl terrars::Referable for Sesv2EmailIdentity {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Sesv2EmailIdentity { }

impl terrars::ToListMappable for Sesv2EmailIdentity {
    type O = terrars::ListRef<Sesv2EmailIdentityRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Sesv2EmailIdentity_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sesv2_email_identity".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesv2EmailIdentity {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub email_identity: terrars::PrimField<std::string::String>,
}

impl BuildSesv2EmailIdentity {
    pub fn build(self, stack: &mut terrars::Stack) -> Sesv2EmailIdentity {
        let out = Sesv2EmailIdentity(std::rc::Rc::new(Sesv2EmailIdentity_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Sesv2EmailIdentityData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                configuration_set_name: core::default::Default::default(),
                email_identity: self.email_identity,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                dkim_signing_attributes: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Sesv2EmailIdentityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2EmailIdentityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Sesv2EmailIdentityRef {
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

    #[doc = "Get a reference to the value of field `configuration_set_name` after provisioning.\n"]
    pub fn configuration_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `email_identity` after provisioning.\n"]
    pub fn email_identity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.email_identity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_type` after provisioning.\n"]
    pub fn identity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `verified_for_sending_status` after provisioning.\n"]
    pub fn verified_for_sending_status(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.verified_for_sending_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dkim_signing_attributes` after provisioning.\n"]
    pub fn dkim_signing_attributes(&self) -> terrars::ListRef<Sesv2EmailIdentityDkimSigningAttributesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dkim_signing_attributes", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Sesv2EmailIdentityDkimSigningAttributesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_signing_private_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain_signing_selector: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    next_signing_key_length: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Sesv2EmailIdentityDkimSigningAttributesEl {
    #[doc = "Set the field `domain_signing_private_key`.\n"]
    pub fn set_domain_signing_private_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.domain_signing_private_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain_signing_selector`.\n"]
    pub fn set_domain_signing_selector(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.domain_signing_selector = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `next_signing_key_length`.\n"]
    pub fn set_next_signing_key_length(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.next_signing_key_length = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Sesv2EmailIdentityDkimSigningAttributesEl {
    type O = terrars::BlockAssignable<Sesv2EmailIdentityDkimSigningAttributesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesv2EmailIdentityDkimSigningAttributesEl {}

impl BuildSesv2EmailIdentityDkimSigningAttributesEl {
    pub fn build(self) -> Sesv2EmailIdentityDkimSigningAttributesEl {
        Sesv2EmailIdentityDkimSigningAttributesEl {
            domain_signing_private_key: core::default::Default::default(),
            domain_signing_selector: core::default::Default::default(),
            next_signing_key_length: core::default::Default::default(),
        }
    }
}

pub struct Sesv2EmailIdentityDkimSigningAttributesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Sesv2EmailIdentityDkimSigningAttributesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Sesv2EmailIdentityDkimSigningAttributesElRef {
        Sesv2EmailIdentityDkimSigningAttributesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Sesv2EmailIdentityDkimSigningAttributesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_signing_key_length` after provisioning.\n"]
    pub fn current_signing_key_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_signing_key_length", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_signing_private_key` after provisioning.\n"]
    pub fn domain_signing_private_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_signing_private_key", self.base))
    }

    #[doc = "Get a reference to the value of field `domain_signing_selector` after provisioning.\n"]
    pub fn domain_signing_selector(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_signing_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `last_key_generation_timestamp` after provisioning.\n"]
    pub fn last_key_generation_timestamp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_key_generation_timestamp", self.base))
    }

    #[doc = "Get a reference to the value of field `next_signing_key_length` after provisioning.\n"]
    pub fn next_signing_key_length(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.next_signing_key_length", self.base))
    }

    #[doc = "Get a reference to the value of field `signing_attributes_origin` after provisioning.\n"]
    pub fn signing_attributes_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.signing_attributes_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `tokens` after provisioning.\n"]
    pub fn tokens(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tokens", self.base))
    }
}

#[derive(Serialize, Default)]
struct Sesv2EmailIdentityDynamic {
    dkim_signing_attributes: std::option::Option<terrars::DynamicBlock<Sesv2EmailIdentityDkimSigningAttributesEl>>,
}
