#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Wafv2WebAclData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_json: std::option::Option<terrars::PrimField<std::string::String>>,
    scope: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    token_domains: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captcha_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_CaptchaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_ChallengeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response_body: std::option::Option<std::vec::Vec<Wafv2WebAcl_CustomResponseBody>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_protection_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_DataProtectionConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action: std::option::Option<std::vec::Vec<Wafv2WebAcl_DefaultAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monetization_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_MonetizationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_VisibilityConfig>>,
    dynamic: Wafv2WebAclDynamic,
}

struct Wafv2WebAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Wafv2WebAclData_>,
}

#[derive(Clone)]
pub struct Wafv2WebAcl(std::rc::Rc<Wafv2WebAcl_>);

impl Wafv2WebAcl {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_json`.\n"]
    pub fn set_rule_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_json = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `token_domains`.\n"]
    pub fn set_token_domains(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().token_domains = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `association_config`.\n"]
    pub fn set_association_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().association_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.association_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `captcha_config`.\n"]
    pub fn set_captcha_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_CaptchaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().captcha_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.captcha_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `challenge_config`.\n"]
    pub fn set_challenge_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_ChallengeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().challenge_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.challenge_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `custom_response_body`.\n"]
    pub fn set_custom_response_body(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_CustomResponseBody>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().custom_response_body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.custom_response_body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_protection_config`.\n"]
    pub fn set_data_protection_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_DataProtectionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_protection_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_protection_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_DefaultAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monetization_config`.\n"]
    pub fn set_monetization_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monetization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monetization_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `visibility_config`.\n"]
    pub fn set_visibility_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_VisibilityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().visibility_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.visibility_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `application_integration_url` after provisioning.\n"]
    pub fn application_integration_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_integration_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_token` after provisioning.\n"]
    pub fn lock_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_json` after provisioning.\n"]
    pub fn rule_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_domains` after provisioning.\n"]
    pub fn token_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.token_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_config` after provisioning.\n"]
    pub fn association_config(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.association_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `captcha_config` after provisioning.\n"]
    pub fn captcha_config(&self) -> terrars::ListRef<Wafv2WebAcl_CaptchaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `challenge_config` after provisioning.\n"]
    pub fn challenge_config(&self) -> terrars::ListRef<Wafv2WebAcl_ChallengeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_protection_config` after provisioning.\n"]
    pub fn data_protection_config(&self) -> terrars::ListRef<Wafv2WebAcl_DataProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<Wafv2WebAcl_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monetization_config` after provisioning.\n"]
    pub fn monetization_config(&self) -> terrars::ListRef<Wafv2WebAcl_MonetizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monetization_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility_config` after provisioning.\n"]
    pub fn visibility_config(&self) -> terrars::ListRef<Wafv2WebAcl_VisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visibility_config", self.extract_ref()))
    }
}

impl terrars::Referable for Wafv2WebAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Wafv2WebAcl { }

impl terrars::ToListMappable for Wafv2WebAcl {
    type O = terrars::ListRef<Wafv2WebAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Wafv2WebAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_wafv2_web_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafv2WebAcl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> Wafv2WebAcl {
        let out = Wafv2WebAcl(std::rc::Rc::new(Wafv2WebAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Wafv2WebAclData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                region: core::default::Default::default(),
                rule_json: core::default::Default::default(),
                scope: self.scope,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                token_domains: core::default::Default::default(),
                association_config: core::default::Default::default(),
                captcha_config: core::default::Default::default(),
                challenge_config: core::default::Default::default(),
                custom_response_body: core::default::Default::default(),
                data_protection_config: core::default::Default::default(),
                default_action: core::default::Default::default(),
                monetization_config: core::default::Default::default(),
                rule: core::default::Default::default(),
                visibility_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Wafv2WebAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Wafv2WebAclRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `application_integration_url` after provisioning.\n"]
    pub fn application_integration_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_integration_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity` after provisioning.\n"]
    pub fn capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lock_token` after provisioning.\n"]
    pub fn lock_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lock_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_json` after provisioning.\n"]
    pub fn rule_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `token_domains` after provisioning.\n"]
    pub fn token_domains(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.token_domains", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_config` after provisioning.\n"]
    pub fn association_config(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.association_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `captcha_config` after provisioning.\n"]
    pub fn captcha_config(&self) -> terrars::ListRef<Wafv2WebAcl_CaptchaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `challenge_config` after provisioning.\n"]
    pub fn challenge_config(&self) -> terrars::ListRef<Wafv2WebAcl_ChallengeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `data_protection_config` after provisioning.\n"]
    pub fn data_protection_config(&self) -> terrars::ListRef<Wafv2WebAcl_DataProtectionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_protection_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<Wafv2WebAcl_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monetization_config` after provisioning.\n"]
    pub fn monetization_config(&self) -> terrars::ListRef<Wafv2WebAcl_MonetizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monetization_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visibility_config` after provisioning.\n"]
    pub fn visibility_config(&self) -> terrars::ListRef<Wafv2WebAcl_VisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visibility_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclDynamic {
    association_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig>>,
    captcha_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_CaptchaConfig>>,
    challenge_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_ChallengeConfig>>,
    custom_response_body: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_CustomResponseBody>>,
    data_protection_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_DataProtectionConfig>>,
    default_action: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_DefaultAction>>,
    monetization_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_MonetizationConfig>>,
    rule: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule>>,
    visibility_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_VisibilityConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway {
    default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway { }

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig_RequestBody_ApiGateway {
    #[doc = ""]
    pub default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_AssociationConfig_RequestBody_ApiGateway {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway {
        Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway {
            default_size_inspection_limit: self.default_size_inspection_limit,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_ApiGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfig_RequestBody_ApiGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_AssociationConfig_RequestBody_ApiGatewayRef {
        Wafv2WebAcl_AssociationConfig_RequestBody_ApiGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_ApiGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_size_inspection_limit` after provisioning.\n"]
    pub fn default_size_inspection_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_size_inspection_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService {
    default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService { }

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService {
    #[doc = ""]
    pub default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService {
        Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService {
            default_size_inspection_limit: self.default_size_inspection_limit,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerServiceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerServiceRef {
        Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_size_inspection_limit` after provisioning.\n"]
    pub fn default_size_inspection_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_size_inspection_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront {
    default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront { }

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig_RequestBody_Cloudfront {
    #[doc = ""]
    pub default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_AssociationConfig_RequestBody_Cloudfront {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront {
        Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront {
            default_size_inspection_limit: self.default_size_inspection_limit,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_CloudfrontRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfig_RequestBody_CloudfrontRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_AssociationConfig_RequestBody_CloudfrontRef {
        Wafv2WebAcl_AssociationConfig_RequestBody_CloudfrontRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_CloudfrontRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_size_inspection_limit` after provisioning.\n"]
    pub fn default_size_inspection_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_size_inspection_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool {
    default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool { }

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool {
    #[doc = ""]
    pub default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool {
        Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool {
            default_size_inspection_limit: self.default_size_inspection_limit,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPoolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPoolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPoolRef {
        Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPoolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPoolRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_size_inspection_limit` after provisioning.\n"]
    pub fn default_size_inspection_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_size_inspection_limit", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance {
    default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance { }

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance {
    #[doc = ""]
    pub default_size_inspection_limit: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance {
        Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance {
            default_size_inspection_limit: self.default_size_inspection_limit,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstanceRef {
        Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_size_inspection_limit` after provisioning.\n"]
    pub fn default_size_inspection_limit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_size_inspection_limit", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_AssociationConfig_RequestBodyDynamic {
    api_gateway: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway>>,
    app_runner_service: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService>>,
    cloudfront: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront>>,
    cognito_user_pool: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool>>,
    verified_access_instance: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBody {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    api_gateway: std::option::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    app_runner_service: std::option::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudfront: std::option::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cognito_user_pool: std::option::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verified_access_instance: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance>>,
    dynamic: Wafv2WebAcl_AssociationConfig_RequestBodyDynamic,
}

impl Wafv2WebAcl_AssociationConfig_RequestBody {
    #[doc = "Set the field `api_gateway`.\n"]
    pub fn set_api_gateway(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_ApiGateway>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.api_gateway = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.api_gateway = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `app_runner_service`.\n"]
    pub fn set_app_runner_service(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.app_runner_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.app_runner_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cloudfront`.\n"]
    pub fn set_cloudfront(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_Cloudfront>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudfront = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudfront = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cognito_user_pool`.\n"]
    pub fn set_cognito_user_pool(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPool>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cognito_user_pool = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cognito_user_pool = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verified_access_instance`.\n"]
    pub fn set_verified_access_instance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstance>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verified_access_instance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verified_access_instance = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig_RequestBody {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig_RequestBody {}

impl BuildWafv2WebAcl_AssociationConfig_RequestBody {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig_RequestBody {
        Wafv2WebAcl_AssociationConfig_RequestBody {
            api_gateway: core::default::Default::default(),
            app_runner_service: core::default::Default::default(),
            cloudfront: core::default::Default::default(),
            cognito_user_pool: core::default::Default::default(),
            verified_access_instance: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfig_RequestBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfig_RequestBodyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_AssociationConfig_RequestBodyRef {
        Wafv2WebAcl_AssociationConfig_RequestBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfig_RequestBodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `api_gateway` after provisioning.\n"]
    pub fn api_gateway(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfig_RequestBody_ApiGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.api_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `app_runner_service` after provisioning.\n"]
    pub fn app_runner_service(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfig_RequestBody_AppRunnerServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.app_runner_service", self.base))
    }

    #[doc = "Get a reference to the value of field `cloudfront` after provisioning.\n"]
    pub fn cloudfront(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfig_RequestBody_CloudfrontRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudfront", self.base))
    }

    #[doc = "Get a reference to the value of field `cognito_user_pool` after provisioning.\n"]
    pub fn cognito_user_pool(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfig_RequestBody_CognitoUserPoolRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cognito_user_pool", self.base))
    }

    #[doc = "Get a reference to the value of field `verified_access_instance` after provisioning.\n"]
    pub fn verified_access_instance(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_AssociationConfig_RequestBody_VerifiedAccessInstanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verified_access_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_AssociationConfigDynamic {
    request_body: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_AssociationConfig_RequestBody>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_AssociationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_body: std::option::Option<std::vec::Vec<Wafv2WebAcl_AssociationConfig_RequestBody>>,
    dynamic: Wafv2WebAcl_AssociationConfigDynamic,
}

impl Wafv2WebAcl_AssociationConfig {
    #[doc = "Set the field `request_body`.\n"]
    pub fn set_request_body(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig_RequestBody>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_body = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_AssociationConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_AssociationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_AssociationConfig {}

impl BuildWafv2WebAcl_AssociationConfig {
    pub fn build(self) -> Wafv2WebAcl_AssociationConfig {
        Wafv2WebAcl_AssociationConfig {
            request_body: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_AssociationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_AssociationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_AssociationConfigRef {
        Wafv2WebAcl_AssociationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_AssociationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `request_body` after provisioning.\n"]
    pub fn request_body(&self) -> terrars::ListRef<Wafv2WebAcl_AssociationConfig_RequestBodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_body", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immunity_time: std::option::Option<terrars::PrimField<f64>>,
}

impl Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty {
    #[doc = "Set the field `immunity_time`.\n"]
    pub fn set_immunity_time(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.immunity_time = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty {
    type O = terrars::BlockAssignable<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_CaptchaConfig_ImmunityTimeProperty {}

impl BuildWafv2WebAcl_CaptchaConfig_ImmunityTimeProperty {
    pub fn build(self) -> Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty {
        Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty { immunity_time: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef {
        Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `immunity_time` after provisioning.\n"]
    pub fn immunity_time(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.immunity_time", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_CaptchaConfigDynamic {
    immunity_time_property: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_CaptchaConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immunity_time_property: std::option::Option<std::vec::Vec<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>>,
    dynamic: Wafv2WebAcl_CaptchaConfigDynamic,
}

impl Wafv2WebAcl_CaptchaConfig {
    #[doc = "Set the field `immunity_time_property`.\n"]
    pub fn set_immunity_time_property(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.immunity_time_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.immunity_time_property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_CaptchaConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_CaptchaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_CaptchaConfig {}

impl BuildWafv2WebAcl_CaptchaConfig {
    pub fn build(self) -> Wafv2WebAcl_CaptchaConfig {
        Wafv2WebAcl_CaptchaConfig {
            immunity_time_property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_CaptchaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_CaptchaConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_CaptchaConfigRef {
        Wafv2WebAcl_CaptchaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_CaptchaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `immunity_time_property` after provisioning.\n"]
    pub fn immunity_time_property(&self) -> terrars::ListRef<Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.immunity_time_property", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_ChallengeConfigDynamic {
    immunity_time_property: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_ChallengeConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    immunity_time_property: std::option::Option<std::vec::Vec<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>>,
    dynamic: Wafv2WebAcl_ChallengeConfigDynamic,
}

impl Wafv2WebAcl_ChallengeConfig {
    #[doc = "Set the field `immunity_time_property`.\n"]
    pub fn set_immunity_time_property(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_CaptchaConfig_ImmunityTimeProperty>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.immunity_time_property = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.immunity_time_property = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_ChallengeConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_ChallengeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_ChallengeConfig {}

impl BuildWafv2WebAcl_ChallengeConfig {
    pub fn build(self) -> Wafv2WebAcl_ChallengeConfig {
        Wafv2WebAcl_ChallengeConfig {
            immunity_time_property: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_ChallengeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_ChallengeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_ChallengeConfigRef {
        Wafv2WebAcl_ChallengeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_ChallengeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `immunity_time_property` after provisioning.\n"]
    pub fn immunity_time_property(&self) -> terrars::ListRef<Wafv2WebAcl_CaptchaConfig_ImmunityTimePropertyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.immunity_time_property", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_CustomResponseBody {
    content: terrars::PrimField<std::string::String>,
    content_type: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_CustomResponseBody { }

impl terrars::ToListMappable for Wafv2WebAcl_CustomResponseBody {
    type O = terrars::BlockAssignable<Wafv2WebAcl_CustomResponseBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_CustomResponseBody {
    #[doc = ""]
    pub content: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub content_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_CustomResponseBody {
    pub fn build(self) -> Wafv2WebAcl_CustomResponseBody {
        Wafv2WebAcl_CustomResponseBody {
            content: self.content,
            content_type: self.content_type,
            key: self.key,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_CustomResponseBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_CustomResponseBodyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_CustomResponseBodyRef {
        Wafv2WebAcl_CustomResponseBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_CustomResponseBodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_DataProtectionConfig_DataProtection_Field {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_keys: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    field_type: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_DataProtectionConfig_DataProtection_Field {
    #[doc = "Set the field `field_keys`.\n"]
    pub fn set_field_keys(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.field_keys = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_DataProtectionConfig_DataProtection_Field {
    type O = terrars::BlockAssignable<Wafv2WebAcl_DataProtectionConfig_DataProtection_Field>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_DataProtectionConfig_DataProtection_Field {
    #[doc = ""]
    pub field_type: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_DataProtectionConfig_DataProtection_Field {
    pub fn build(self) -> Wafv2WebAcl_DataProtectionConfig_DataProtection_Field {
        Wafv2WebAcl_DataProtectionConfig_DataProtection_Field {
            field_keys: core::default::Default::default(),
            field_type: self.field_type,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_DataProtectionConfig_DataProtection_FieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_DataProtectionConfig_DataProtection_FieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_DataProtectionConfig_DataProtection_FieldRef {
        Wafv2WebAcl_DataProtectionConfig_DataProtection_FieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_DataProtectionConfig_DataProtection_FieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_keys` after provisioning.\n"]
    pub fn field_keys(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_keys", self.base))
    }

    #[doc = "Get a reference to the value of field `field_type` after provisioning.\n"]
    pub fn field_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_DataProtectionConfig_DataProtectionDynamic {
    field: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_DataProtectionConfig_DataProtection_Field>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_DataProtectionConfig_DataProtection {
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_rate_based_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_rule_match_details: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<std::vec::Vec<Wafv2WebAcl_DataProtectionConfig_DataProtection_Field>>,
    dynamic: Wafv2WebAcl_DataProtectionConfig_DataProtectionDynamic,
}

impl Wafv2WebAcl_DataProtectionConfig_DataProtection {
    #[doc = "Set the field `exclude_rate_based_details`.\n"]
    pub fn set_exclude_rate_based_details(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_rate_based_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_rule_match_details`.\n"]
    pub fn set_exclude_rule_match_details(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_rule_match_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_DataProtectionConfig_DataProtection_Field>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_DataProtectionConfig_DataProtection {
    type O = terrars::BlockAssignable<Wafv2WebAcl_DataProtectionConfig_DataProtection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_DataProtectionConfig_DataProtection {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_DataProtectionConfig_DataProtection {
    pub fn build(self) -> Wafv2WebAcl_DataProtectionConfig_DataProtection {
        Wafv2WebAcl_DataProtectionConfig_DataProtection {
            action: self.action,
            exclude_rate_based_details: core::default::Default::default(),
            exclude_rule_match_details: core::default::Default::default(),
            field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_DataProtectionConfig_DataProtectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_DataProtectionConfig_DataProtectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_DataProtectionConfig_DataProtectionRef {
        Wafv2WebAcl_DataProtectionConfig_DataProtectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_DataProtectionConfig_DataProtectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_rate_based_details` after provisioning.\n"]
    pub fn exclude_rate_based_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_rate_based_details", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_rule_match_details` after provisioning.\n"]
    pub fn exclude_rule_match_details(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_rule_match_details", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::ListRef<Wafv2WebAcl_DataProtectionConfig_DataProtection_FieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_DataProtectionConfigDynamic {
    data_protection: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_DataProtectionConfig_DataProtection>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_DataProtectionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_protection: std::option::Option<std::vec::Vec<Wafv2WebAcl_DataProtectionConfig_DataProtection>>,
    dynamic: Wafv2WebAcl_DataProtectionConfigDynamic,
}

impl Wafv2WebAcl_DataProtectionConfig {
    #[doc = "Set the field `data_protection`.\n"]
    pub fn set_data_protection(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_DataProtectionConfig_DataProtection>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_protection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_protection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_DataProtectionConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_DataProtectionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_DataProtectionConfig {}

impl BuildWafv2WebAcl_DataProtectionConfig {
    pub fn build(self) -> Wafv2WebAcl_DataProtectionConfig {
        Wafv2WebAcl_DataProtectionConfig {
            data_protection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_DataProtectionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_DataProtectionConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_DataProtectionConfigRef {
        Wafv2WebAcl_DataProtectionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_DataProtectionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_protection` after provisioning.\n"]
    pub fn data_protection(&self) -> terrars::ListRef<Wafv2WebAcl_DataProtectionConfig_DataProtectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_protection", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader {
        Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeaderRef {
        Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingDynamic {
    insert_header: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    insert_header: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader>>,
    dynamic: Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingDynamic,
}

impl Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {
    #[doc = "Set the field `insert_header`.\n"]
    pub fn set_insert_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling_InsertHeader>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.insert_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.insert_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {}

impl BuildWafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {
        Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling {
            insert_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef {
        Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_AllowDynamic {
    custom_request_handling: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Allow {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    dynamic: Wafv2WebAcl_Rule_Action_AllowDynamic,
}

impl Wafv2WebAcl_Rule_Action_Allow {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Allow {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Allow {}

impl BuildWafv2WebAcl_Rule_Action_Allow {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Allow {
        Wafv2WebAcl_Rule_Action_Allow {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_AllowRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_AllowRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Action_AllowRef {
        Wafv2WebAcl_Rule_Action_AllowRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_AllowRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader {
        Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeaderRef {
        Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_Block_CustomResponseDynamic {
    response_header: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Block_CustomResponse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response_body_key: std::option::Option<terrars::PrimField<std::string::String>>,
    response_code: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_header: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader>>,
    dynamic: Wafv2WebAcl_Rule_Action_Block_CustomResponseDynamic,
}

impl Wafv2WebAcl_Rule_Action_Block_CustomResponse {
    #[doc = "Set the field `custom_response_body_key`.\n"]
    pub fn set_custom_response_body_key(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.custom_response_body_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `response_header`.\n"]
    pub fn set_response_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block_CustomResponse_ResponseHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_header = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Block_CustomResponse {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block_CustomResponse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Block_CustomResponse {
    #[doc = ""]
    pub response_code: terrars::PrimField<f64>,
}

impl BuildWafv2WebAcl_Rule_Action_Block_CustomResponse {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Block_CustomResponse {
        Wafv2WebAcl_Rule_Action_Block_CustomResponse {
            custom_response_body_key: core::default::Default::default(),
            response_code: self.response_code,
            response_header: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_Block_CustomResponseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_Block_CustomResponseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Action_Block_CustomResponseRef {
        Wafv2WebAcl_Rule_Action_Block_CustomResponseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_Block_CustomResponseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_response_body_key` after provisioning.\n"]
    pub fn custom_response_body_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_response_body_key", self.base))
    }

    #[doc = "Get a reference to the value of field `response_code` after provisioning.\n"]
    pub fn response_code(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.response_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_BlockDynamic {
    custom_response: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Block_CustomResponse>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Block {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_response: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Block_CustomResponse>>,
    dynamic: Wafv2WebAcl_Rule_Action_BlockDynamic,
}

impl Wafv2WebAcl_Rule_Action_Block {
    #[doc = "Set the field `custom_response`.\n"]
    pub fn set_custom_response(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block_CustomResponse>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_response = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_response = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Block {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Block {}

impl BuildWafv2WebAcl_Rule_Action_Block {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Block {
        Wafv2WebAcl_Rule_Action_Block {
            custom_response: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_BlockRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_BlockRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Action_BlockRef {
        Wafv2WebAcl_Rule_Action_BlockRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_BlockRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_response` after provisioning.\n"]
    pub fn custom_response(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_Block_CustomResponseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_response", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_DefaultActionDynamic {
    allow: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow>>,
    block: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Block>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_DefaultAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Block>>,
    dynamic: Wafv2WebAcl_DefaultActionDynamic,
}

impl Wafv2WebAcl_DefaultAction {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_DefaultAction {
    type O = terrars::BlockAssignable<Wafv2WebAcl_DefaultAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_DefaultAction {}

impl BuildWafv2WebAcl_DefaultAction {
    pub fn build(self) -> Wafv2WebAcl_DefaultAction {
        Wafv2WebAcl_DefaultAction {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_DefaultActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_DefaultActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_DefaultActionRef {
        Wafv2WebAcl_DefaultActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_DefaultActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_BlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices {
    amount: terrars::PrimField<std::string::String>,
    currency: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices { }

impl terrars::ToListMappable for Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices {
    type O = terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices {
    #[doc = ""]
    pub amount: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub currency: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices {
    pub fn build(self) -> Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices {
        Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices {
            amount: self.amount,
            currency: self.currency,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_PricesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_PricesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_PricesRef {
        Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_PricesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_PricesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount` after provisioning.\n"]
    pub fn amount(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amount", self.base))
    }

    #[doc = "Get a reference to the value of field `currency` after provisioning.\n"]
    pub fn currency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkDynamic {
    prices: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
    chain: terrars::PrimField<std::string::String>,
    wallet_address: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prices: std::option::Option<std::vec::Vec<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices>>,
    dynamic: Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkDynamic,
}

impl Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
    #[doc = "Set the field `prices`.\n"]
    pub fn set_prices(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_Prices>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prices = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prices = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
    type O = terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
    #[doc = ""]
    pub chain: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub wallet_address: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
    pub fn build(self) -> Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
        Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork {
            chain: self.chain,
            wallet_address: self.wallet_address,
            prices: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkRef {
        Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `chain` after provisioning.\n"]
    pub fn chain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.chain", self.base))
    }

    #[doc = "Get a reference to the value of field `wallet_address` after provisioning.\n"]
    pub fn wallet_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wallet_address", self.base))
    }

    #[doc = "Get a reference to the value of field `prices` after provisioning.\n"]
    pub fn prices(&self) -> terrars::ListRef<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork_PricesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prices", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_MonetizationConfig_CryptoConfigDynamic {
    payment_network: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_MonetizationConfig_CryptoConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payment_network: std::option::Option<std::vec::Vec<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork>>,
    dynamic: Wafv2WebAcl_MonetizationConfig_CryptoConfigDynamic,
}

impl Wafv2WebAcl_MonetizationConfig_CryptoConfig {
    #[doc = "Set the field `payment_network`.\n"]
    pub fn set_payment_network(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetwork>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.payment_network = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.payment_network = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_MonetizationConfig_CryptoConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig_CryptoConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_MonetizationConfig_CryptoConfig {}

impl BuildWafv2WebAcl_MonetizationConfig_CryptoConfig {
    pub fn build(self) -> Wafv2WebAcl_MonetizationConfig_CryptoConfig {
        Wafv2WebAcl_MonetizationConfig_CryptoConfig {
            payment_network: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_MonetizationConfig_CryptoConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_MonetizationConfig_CryptoConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_MonetizationConfig_CryptoConfigRef {
        Wafv2WebAcl_MonetizationConfig_CryptoConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_MonetizationConfig_CryptoConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payment_network` after provisioning.\n"]
    pub fn payment_network(&self) -> terrars::ListRef<Wafv2WebAcl_MonetizationConfig_CryptoConfig_PaymentNetworkRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.payment_network", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_MonetizationConfigDynamic {
    crypto_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_MonetizationConfig_CryptoConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_MonetizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    currency_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    crypto_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_MonetizationConfig_CryptoConfig>>,
    dynamic: Wafv2WebAcl_MonetizationConfigDynamic,
}

impl Wafv2WebAcl_MonetizationConfig {
    #[doc = "Set the field `currency_mode`.\n"]
    pub fn set_currency_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.currency_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `crypto_config`.\n"]
    pub fn set_crypto_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig_CryptoConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.crypto_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.crypto_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_MonetizationConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_MonetizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_MonetizationConfig {}

impl BuildWafv2WebAcl_MonetizationConfig {
    pub fn build(self) -> Wafv2WebAcl_MonetizationConfig {
        Wafv2WebAcl_MonetizationConfig {
            currency_mode: core::default::Default::default(),
            crypto_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_MonetizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_MonetizationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_MonetizationConfigRef {
        Wafv2WebAcl_MonetizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_MonetizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `currency_mode` after provisioning.\n"]
    pub fn currency_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.currency_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `crypto_config` after provisioning.\n"]
    pub fn crypto_config(&self) -> terrars::ListRef<Wafv2WebAcl_MonetizationConfig_CryptoConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.crypto_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_CaptchaDynamic {
    custom_request_handling: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Captcha {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    dynamic: Wafv2WebAcl_Rule_Action_CaptchaDynamic,
}

impl Wafv2WebAcl_Rule_Action_Captcha {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Captcha {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Captcha>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Captcha {}

impl BuildWafv2WebAcl_Rule_Action_Captcha {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Captcha {
        Wafv2WebAcl_Rule_Action_Captcha {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_CaptchaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_CaptchaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Action_CaptchaRef {
        Wafv2WebAcl_Rule_Action_CaptchaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_CaptchaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_ChallengeDynamic {
    custom_request_handling: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Challenge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    dynamic: Wafv2WebAcl_Rule_Action_ChallengeDynamic,
}

impl Wafv2WebAcl_Rule_Action_Challenge {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Challenge {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Challenge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Challenge {}

impl BuildWafv2WebAcl_Rule_Action_Challenge {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Challenge {
        Wafv2WebAcl_Rule_Action_Challenge {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_ChallengeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_ChallengeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Action_ChallengeRef {
        Wafv2WebAcl_Rule_Action_ChallengeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_ChallengeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Action_CountDynamic {
    custom_request_handling: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Count {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_request_handling: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    dynamic: Wafv2WebAcl_Rule_Action_CountDynamic,
}

impl Wafv2WebAcl_Rule_Action_Count {
    #[doc = "Set the field `custom_request_handling`.\n"]
    pub fn set_custom_request_handling(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_request_handling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_request_handling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Count {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Count>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Count {}

impl BuildWafv2WebAcl_Rule_Action_Count {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Count {
        Wafv2WebAcl_Rule_Action_Count {
            custom_request_handling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_CountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_CountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Action_CountRef {
        Wafv2WebAcl_Rule_Action_CountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_CountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_request_handling` after provisioning.\n"]
    pub fn custom_request_handling(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_Allow_CustomRequestHandlingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_request_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action_Monetize {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    price_multiplier: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Action_Monetize {
    #[doc = "Set the field `price_multiplier`.\n"]
    pub fn set_price_multiplier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.price_multiplier = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action_Monetize {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Monetize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action_Monetize {}

impl BuildWafv2WebAcl_Rule_Action_Monetize {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action_Monetize {
        Wafv2WebAcl_Rule_Action_Monetize { price_multiplier: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Action_MonetizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Action_MonetizeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Action_MonetizeRef {
        Wafv2WebAcl_Rule_Action_MonetizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Action_MonetizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `price_multiplier` after provisioning.\n"]
    pub fn price_multiplier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.price_multiplier", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_ActionDynamic {
    allow: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow>>,
    block: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Block>>,
    captcha: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Captcha>>,
    challenge: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Challenge>>,
    count: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Count>>,
    monetize: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Monetize>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Block>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captcha: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Captcha>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Challenge>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Count>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monetize: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Monetize>>,
    dynamic: Wafv2WebAcl_Rule_ActionDynamic,
}

impl Wafv2WebAcl_Rule_Action {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `captcha`.\n"]
    pub fn set_captcha(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Captcha>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.captcha = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.captcha = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Challenge>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Count>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monetize`.\n"]
    pub fn set_monetize(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Monetize>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.monetize = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.monetize = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Action {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Action {}

impl BuildWafv2WebAcl_Rule_Action {
    pub fn build(self) -> Wafv2WebAcl_Rule_Action {
        Wafv2WebAcl_Rule_Action {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            captcha: core::default::Default::default(),
            challenge: core::default::Default::default(),
            count: core::default::Default::default(),
            monetize: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_ActionRef {
        Wafv2WebAcl_Rule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_BlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }

    #[doc = "Get a reference to the value of field `captcha` after provisioning.\n"]
    pub fn captcha(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_CaptchaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]
    pub fn challenge(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_ChallengeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_CountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `monetize` after provisioning.\n"]
    pub fn monetize(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_MonetizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monetize", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_OverrideAction_Count {}

impl Wafv2WebAcl_Rule_OverrideAction_Count { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_OverrideAction_Count {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_OverrideAction_Count>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_OverrideAction_Count {}

impl BuildWafv2WebAcl_Rule_OverrideAction_Count {
    pub fn build(self) -> Wafv2WebAcl_Rule_OverrideAction_Count {
        Wafv2WebAcl_Rule_OverrideAction_Count {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_OverrideAction_CountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_OverrideAction_CountRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_OverrideAction_CountRef {
        Wafv2WebAcl_Rule_OverrideAction_CountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_OverrideAction_CountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_OverrideAction_None {}

impl Wafv2WebAcl_Rule_OverrideAction_None { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_OverrideAction_None {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_OverrideAction_None>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_OverrideAction_None {}

impl BuildWafv2WebAcl_Rule_OverrideAction_None {
    pub fn build(self) -> Wafv2WebAcl_Rule_OverrideAction_None {
        Wafv2WebAcl_Rule_OverrideAction_None {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_OverrideAction_NoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_OverrideAction_NoneRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_OverrideAction_NoneRef {
        Wafv2WebAcl_Rule_OverrideAction_NoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_OverrideAction_NoneRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_OverrideActionDynamic {
    count: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_OverrideAction_Count>>,
    none: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_OverrideAction_None>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_OverrideAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_OverrideAction_Count>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    none: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_OverrideAction_None>>,
    dynamic: Wafv2WebAcl_Rule_OverrideActionDynamic,
}

impl Wafv2WebAcl_Rule_OverrideAction {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_OverrideAction_Count>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `none`.\n"]
    pub fn set_none(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_OverrideAction_None>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.none = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.none = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_OverrideAction {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_OverrideAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_OverrideAction {}

impl BuildWafv2WebAcl_Rule_OverrideAction {
    pub fn build(self) -> Wafv2WebAcl_Rule_OverrideAction {
        Wafv2WebAcl_Rule_OverrideAction {
            count: core::default::Default::default(),
            none: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_OverrideActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_OverrideActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_OverrideActionRef {
        Wafv2WebAcl_Rule_OverrideActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_OverrideActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_OverrideAction_CountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `none` after provisioning.\n"]
    pub fn none(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_OverrideAction_NoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.none", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_RuleLabel {
    name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_RuleLabel { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_RuleLabel {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_RuleLabel>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_RuleLabel {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_RuleLabel {
    pub fn build(self) -> Wafv2WebAcl_Rule_RuleLabel {
        Wafv2WebAcl_Rule_RuleLabel { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_RuleLabelRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_RuleLabelRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_RuleLabelRef {
        Wafv2WebAcl_Rule_RuleLabelRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_RuleLabelRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig {
    fallback_behavior: terrars::PrimField<std::string::String>,
    header_name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig {
    #[doc = ""]
    pub fallback_behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub header_name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig {
        Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig {
            fallback_behavior: self.fallback_behavior,
            header_name: self.header_name,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef {
        Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fallback_behavior` after provisioning.\n"]
    pub fn fallback_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_AsnMatchStatementDynamic {
    forwarded_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_AsnMatchStatement {
    asn_list: terrars::ListField<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_ip_config: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    dynamic: Wafv2WebAcl_Rule_Statement_AsnMatchStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_AsnMatchStatement {
    #[doc = "Set the field `forwarded_ip_config`.\n"]
    pub fn set_forwarded_ip_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_AsnMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_AsnMatchStatement {
    #[doc = ""]
    pub asn_list: terrars::ListField<terrars::PrimField<f64>>,
}

impl BuildWafv2WebAcl_Rule_Statement_AsnMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_AsnMatchStatement {
        Wafv2WebAcl_Rule_Statement_AsnMatchStatement {
            asn_list: self.asn_list,
            forwarded_ip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn_list` after provisioning.\n"]
    pub fn asn_list(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn_list", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_ip_config` after provisioning.\n"]
    pub fn forwarded_ip_config(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_ip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments {}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArgumentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArgumentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArgumentsRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArgumentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArgumentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oversize_handling: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {
    #[doc = "Set the field `oversize_handling`.\n"]
    pub fn set_oversize_handling(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oversize_handling = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body {
            oversize_handling: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_BodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_BodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_BodyRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_BodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_BodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oversize_handling` after provisioning.\n"]
    pub fn oversize_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oversize_handling", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All {}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All {
    type O =
        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternDynamic {
    all: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_cookies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_cookies: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {
    #[doc = "Set the field `excluded_cookies`.\n"]
    pub fn set_excluded_cookies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_cookies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `included_cookies`.\n"]
    pub fn set_included_cookies(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_cookies = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all`.\n"]
    pub fn set_all(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {
    type O =
        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern {
            excluded_cookies: core::default::Default::default(),
            included_cookies: core::default::Default::default(),
            all: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_cookies` after provisioning.\n"]
    pub fn excluded_cookies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `included_cookies` after provisioning.\n"]
    pub fn included_cookies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesDynamic {
    match_pattern: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
    match_scope: terrars::PrimField<std::string::String>,
    oversize_handling: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_pattern: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
    #[doc = "Set the field `match_pattern`.\n"]
    pub fn set_match_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
    #[doc = ""]
    pub match_scope: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub oversize_handling: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies {
            match_scope: self.match_scope,
            oversize_handling: self.oversize_handling,
            match_pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_scope` after provisioning.\n"]
    pub fn match_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `oversize_handling` after provisioning.\n"]
    pub fn oversize_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oversize_handling", self.base))
    }

    #[doc = "Get a reference to the value of field `match_pattern` after provisioning.\n"]
    pub fn match_pattern(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder {
    oversize_handling: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder {
    #[doc = ""]
    pub oversize_handling: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder {
            oversize_handling: self.oversize_handling,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrderRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `oversize_handling` after provisioning.\n"]
    pub fn oversize_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oversize_handling", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternDynamic {
    all: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {
    #[doc = "Set the field `excluded_headers`.\n"]
    pub fn set_excluded_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `included_headers`.\n"]
    pub fn set_included_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all`.\n"]
    pub fn set_all(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {
    type O =
        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern {
            excluded_headers: core::default::Default::default(),
            included_headers: core::default::Default::default(),
            all: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `excluded_headers` after provisioning.\n"]
    pub fn excluded_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `included_headers` after provisioning.\n"]
    pub fn included_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersDynamic {
    match_pattern: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
    match_scope: terrars::PrimField<std::string::String>,
    oversize_handling: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_pattern: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
    #[doc = "Set the field `match_pattern`.\n"]
    pub fn set_match_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPattern,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
    #[doc = ""]
    pub match_scope: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub oversize_handling: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers {
            match_scope: self.match_scope,
            oversize_handling: self.oversize_handling,
            match_pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_scope` after provisioning.\n"]
    pub fn match_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `oversize_handling` after provisioning.\n"]
    pub fn oversize_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oversize_handling", self.base))
    }

    #[doc = "Get a reference to the value of field `match_pattern` after provisioning.\n"]
    pub fn match_pattern(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers_MatchPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint {
    fallback_behavior: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint {
    #[doc = ""]
    pub fallback_behavior: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint {
            fallback_behavior: self.fallback_behavior,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fallback_behavior` after provisioning.\n"]
    pub fn fallback_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint {
    fallback_behavior: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint {
    #[doc = ""]
    pub fallback_behavior: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint {
            fallback_behavior: self.fallback_behavior,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fallback_behavior` after provisioning.\n"]
    pub fn fallback_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_behavior", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternDynamic {
    all: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    included_paths: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {
    #[doc = "Set the field `included_paths`.\n"]
    pub fn set_included_paths(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.included_paths = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `all`.\n"]
    pub fn set_all(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_All,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {
    type O =
        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern {
            included_paths: core::default::Default::default(),
            all: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `included_paths` after provisioning.\n"]
    pub fn included_paths(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.included_paths", self.base))
    }

    #[doc = "Get a reference to the value of field `all` after provisioning.\n"]
    pub fn all(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies_MatchPattern_AllRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyDynamic {
    match_pattern: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invalid_fallback_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    match_scope: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    oversize_handling: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_pattern: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
    #[doc = "Set the field `invalid_fallback_behavior`.\n"]
    pub fn set_invalid_fallback_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.invalid_fallback_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `oversize_handling`.\n"]
    pub fn set_oversize_handling(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.oversize_handling = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_pattern`.\n"]
    pub fn set_match_pattern(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPattern,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.match_pattern = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.match_pattern = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
    #[doc = ""]
    pub match_scope: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody {
            invalid_fallback_behavior: core::default::Default::default(),
            match_scope: self.match_scope,
            oversize_handling: core::default::Default::default(),
            match_pattern: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invalid_fallback_behavior` after provisioning.\n"]
    pub fn invalid_fallback_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invalid_fallback_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `match_scope` after provisioning.\n"]
    pub fn match_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `oversize_handling` after provisioning.\n"]
    pub fn oversize_handling(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.oversize_handling", self.base))
    }

    #[doc = "Get a reference to the value of field `match_pattern` after provisioning.\n"]
    pub fn match_pattern(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody_MatchPatternRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method {}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_MethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_MethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_MethodRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_MethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_MethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString {}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryStringRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader {
    name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeaderRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument {
    name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgumentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgumentRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgumentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgumentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fallback_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {
    #[doc = "Set the field `fallback_behavior`.\n"]
    pub fn set_fallback_behavior(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.fallback_behavior = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment {
            fallback_behavior: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragmentRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fallback_behavior` after provisioning.\n"]
    pub fn fallback_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath {}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPathRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchDynamic {
    all_query_arguments: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments>>,
    body: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body>>,
    cookies: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies>>,
    header_order: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder>>,
    headers: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers>>,
    ja3_fingerprint: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>>,
    ja4_fingerprint: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>>,
    json_body: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody>>,
    method: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString>>,
    single_header: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader>>,
    single_query_argument: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument>>,
    uri_fragment: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment>>,
    uri_path: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_query_arguments: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookies: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header_order: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    headers: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ja3_fingerprint: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ja4_fingerprint: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json_body: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_header: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_query_argument: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_fragment: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_path: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {
    #[doc = "Set the field `all_query_arguments`.\n"]
    pub fn set_all_query_arguments(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArguments>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all_query_arguments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all_query_arguments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Body>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cookies`.\n"]
    pub fn set_cookies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Cookies>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cookies = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cookies = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header_order`.\n"]
    pub fn set_header_order(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrder>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header_order = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header_order = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `headers`.\n"]
    pub fn set_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Headers>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.headers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ja3_fingerprint`.\n"]
    pub fn set_ja3_fingerprint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ja3_fingerprint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ja3_fingerprint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ja4_fingerprint`.\n"]
    pub fn set_ja4_fingerprint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ja4_fingerprint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ja4_fingerprint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json_body`.\n"]
    pub fn set_json_body(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBody>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json_body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json_body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_Method>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryString>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_header`.\n"]
    pub fn set_single_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeader>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_query_argument`.\n"]
    pub fn set_single_query_argument(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgument,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_query_argument = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_query_argument = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uri_fragment`.\n"]
    pub fn set_uri_fragment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragment>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uri_fragment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_fragment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uri_path`.\n"]
    pub fn set_uri_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPath>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uri_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_path = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch {
            all_query_arguments: core::default::Default::default(),
            body: core::default::Default::default(),
            cookies: core::default::Default::default(),
            header_order: core::default::Default::default(),
            headers: core::default::Default::default(),
            ja3_fingerprint: core::default::Default::default(),
            ja4_fingerprint: core::default::Default::default(),
            json_body: core::default::Default::default(),
            method: core::default::Default::default(),
            query_string: core::default::Default::default(),
            single_header: core::default::Default::default(),
            single_query_argument: core::default::Default::default(),
            uri_fragment: core::default::Default::default(),
            uri_path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_query_arguments` after provisioning.\n"]
    pub fn all_query_arguments(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_AllQueryArgumentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_query_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_BodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc = "Get a reference to the value of field `cookies` after provisioning.\n"]
    pub fn cookies(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_CookiesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookies", self.base))
    }

    #[doc = "Get a reference to the value of field `header_order` after provisioning.\n"]
    pub fn header_order(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeaderOrderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header_order", self.base))
    }

    #[doc = "Get a reference to the value of field `headers` after provisioning.\n"]
    pub fn headers(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_HeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.headers", self.base))
    }

    #[doc = "Get a reference to the value of field `ja3_fingerprint` after provisioning.\n"]
    pub fn ja3_fingerprint(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ja3_fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `ja4_fingerprint` after provisioning.\n"]
    pub fn ja4_fingerprint(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ja4_fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `json_body` after provisioning.\n"]
    pub fn json_body(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_JsonBodyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json_body", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_MethodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_QueryStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `single_header` after provisioning.\n"]
    pub fn single_header(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_header", self.base))
    }

    #[doc = "Get a reference to the value of field `single_query_argument` after provisioning.\n"]
    pub fn single_query_argument(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_SingleQueryArgumentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_query_argument", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_fragment` after provisioning.\n"]
    pub fn uri_fragment(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriFragmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri_fragment", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_path` after provisioning.\n"]
    pub fn uri_path(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch_UriPathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri_path", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation {
    priority: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation {
            priority: self.priority,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformationRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation {
    priority: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation {
            priority: self.priority,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformationRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ByteMatchStatementDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    pre_parse_text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ByteMatchStatement {
    positional_constraint: terrars::PrimField<std::string::String>,
    search_string: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_parse_text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_ByteMatchStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ByteMatchStatement {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_parse_text_transformation`.\n"]
    pub fn set_pre_parse_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_parse_text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_parse_text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_ByteMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ByteMatchStatement {
    #[doc = ""]
    pub positional_constraint: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub search_string: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ByteMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ByteMatchStatement {
        Wafv2WebAcl_Rule_Statement_ByteMatchStatement {
            positional_constraint: self.positional_constraint,
            search_string: self.search_string,
            field_to_match: core::default::Default::default(),
            pre_parse_text_transformation: core::default::Default::default(),
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `positional_constraint` after provisioning.\n"]
    pub fn positional_constraint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.positional_constraint", self.base))
    }

    #[doc = "Get a reference to the value of field `search_string` after provisioning.\n"]
    pub fn search_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.search_string", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_GeoMatchStatementDynamic {
    forwarded_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_GeoMatchStatement {
    country_codes: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_ip_config: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    dynamic: Wafv2WebAcl_Rule_Statement_GeoMatchStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_GeoMatchStatement {
    #[doc = "Set the field `forwarded_ip_config`.\n"]
    pub fn set_forwarded_ip_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_GeoMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_GeoMatchStatement {
    #[doc = ""]
    pub country_codes: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAcl_Rule_Statement_GeoMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_GeoMatchStatement {
        Wafv2WebAcl_Rule_Statement_GeoMatchStatement {
            country_codes: self.country_codes,
            forwarded_ip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `country_codes` after provisioning.\n"]
    pub fn country_codes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.country_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_ip_config` after provisioning.\n"]
    pub fn forwarded_ip_config(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_ip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig {
    fallback_behavior: terrars::PrimField<std::string::String>,
    header_name: terrars::PrimField<std::string::String>,
    position: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig {
    #[doc = ""]
    pub fallback_behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub position: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig {
        Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig {
            fallback_behavior: self.fallback_behavior,
            header_name: self.header_name,
            position: self.position,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfigRef {
        Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `fallback_behavior` after provisioning.\n"]
    pub fn fallback_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fallback_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementDynamic {
    ip_set_forwarded_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_forwarded_ip_config: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig>>,
    dynamic: Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
    #[doc = "Set the field `ip_set_forwarded_ip_config`.\n"]
    pub fn set_ip_set_forwarded_ip_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_forwarded_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_forwarded_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
        Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement {
            arn: self.arn,
            ip_set_forwarded_ip_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef {
        Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_forwarded_ip_config` after provisioning.\n"]
    pub fn ip_set_forwarded_ip_config(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement_IpSetForwardedIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_forwarded_ip_config", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_LabelMatchStatement {
    key: terrars::PrimField<std::string::String>,
    scope: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_LabelMatchStatement { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_LabelMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_LabelMatchStatement {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_LabelMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_LabelMatchStatement {
        Wafv2WebAcl_Rule_Statement_LabelMatchStatement {
            key: self.key,
            scope: self.scope,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RegexMatchStatementDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    pre_parse_text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RegexMatchStatement {
    regex_string: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_parse_text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RegexMatchStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RegexMatchStatement {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_parse_text_transformation`.\n"]
    pub fn set_pre_parse_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_parse_text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_parse_text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RegexMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RegexMatchStatement {
    #[doc = ""]
    pub regex_string: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RegexMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RegexMatchStatement {
        Wafv2WebAcl_Rule_Statement_RegexMatchStatement {
            regex_string: self.regex_string,
            field_to_match: core::default::Default::default(),
            pre_parse_text_transformation: core::default::Default::default(),
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_string` after provisioning.\n"]
    pub fn regex_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_string", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    pre_parse_text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_parse_text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_parse_text_transformation`.\n"]
    pub fn set_pre_parse_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_parse_text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_parse_text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
        Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement {
            arn: self.arn,
            field_to_match: core::default::Default::default(),
            pre_parse_text_transformation: core::default::Default::default(),
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef {
        Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_SizeConstraintStatementDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    pre_parse_text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_SizeConstraintStatement {
    comparison_operator: terrars::PrimField<std::string::String>,
    size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_parse_text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_SizeConstraintStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_SizeConstraintStatement {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_parse_text_transformation`.\n"]
    pub fn set_pre_parse_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_parse_text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_parse_text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_SizeConstraintStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_SizeConstraintStatement {
    #[doc = ""]
    pub comparison_operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
}

impl BuildWafv2WebAcl_Rule_Statement_SizeConstraintStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_SizeConstraintStatement {
        Wafv2WebAcl_Rule_Statement_SizeConstraintStatement {
            comparison_operator: self.comparison_operator,
            size: self.size,
            field_to_match: core::default::Default::default(),
            pre_parse_text_transformation: core::default::Default::default(),
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef {
        Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison_operator` after provisioning.\n"]
    pub fn comparison_operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison_operator", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_SqliMatchStatementDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    pre_parse_text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_SqliMatchStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_parse_text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_SqliMatchStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_SqliMatchStatement {
    #[doc = "Set the field `sensitivity_level`.\n"]
    pub fn set_sensitivity_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_parse_text_transformation`.\n"]
    pub fn set_pre_parse_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_parse_text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_parse_text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_SqliMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_SqliMatchStatement {}

impl BuildWafv2WebAcl_Rule_Statement_SqliMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_SqliMatchStatement {
        Wafv2WebAcl_Rule_Statement_SqliMatchStatement {
            sensitivity_level: core::default::Default::default(),
            field_to_match: core::default::Default::default(),
            pre_parse_text_transformation: core::default::Default::default(),
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sensitivity_level` after provisioning.\n"]
    pub fn sensitivity_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_level", self.base))
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_XssMatchStatementDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    pre_parse_text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pre_parse_text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_XssMatchStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatement {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `pre_parse_text_transformation`.\n"]
    pub fn set_pre_parse_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_PreParseTextTransformation>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.pre_parse_text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.pre_parse_text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_XssMatchStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_XssMatchStatement {}

impl BuildWafv2WebAcl_Rule_Statement_XssMatchStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_XssMatchStatement {
        Wafv2WebAcl_Rule_Statement_XssMatchStatement {
            field_to_match: core::default::Default::default(),
            pre_parse_text_transformation: core::default::Default::default(),
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_XssMatchStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_XssMatchStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_XssMatchStatementRef {
        Wafv2WebAcl_Rule_Statement_XssMatchStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_XssMatchStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field_to_match` after provisioning.\n"]
    pub fn field_to_match(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatement_FieldToMatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.field_to_match", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementDynamic {
    asn_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    byte_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    geo_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ip_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    label_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    regex_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    size_constraint_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    sqli_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    xss_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byte_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_constraint_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqli_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {
    #[doc = "Set the field `asn_match_statement`.\n"]
    pub fn set_asn_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asn_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asn_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `byte_match_statement`.\n"]
    pub fn set_byte_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.byte_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.byte_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geo_match_statement`.\n"]
    pub fn set_geo_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_set_reference_statement`.\n"]
    pub fn set_ip_set_reference_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_match_statement`.\n"]
    pub fn set_label_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_match_statement`.\n"]
    pub fn set_regex_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_pattern_set_reference_statement`.\n"]
    pub fn set_regex_pattern_set_reference_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_pattern_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_pattern_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `size_constraint_statement`.\n"]
    pub fn set_size_constraint_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.size_constraint_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.size_constraint_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqli_match_statement`.\n"]
    pub fn set_sqli_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqli_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqli_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_match_statement`.\n"]
    pub fn set_xss_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xss_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement {
            asn_match_statement: core::default::Default::default(),
            byte_match_statement: core::default::Default::default(),
            geo_match_statement: core::default::Default::default(),
            ip_set_reference_statement: core::default::Default::default(),
            label_match_statement: core::default::Default::default(),
            regex_match_statement: core::default::Default::default(),
            regex_pattern_set_reference_statement: core::default::Default::default(),
            size_constraint_statement: core::default::Default::default(),
            sqli_match_statement: core::default::Default::default(),
            xss_match_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn_match_statement` after provisioning.\n"]
    pub fn asn_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `byte_match_statement` after provisioning.\n"]
    pub fn byte_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.byte_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_match_statement` after provisioning.\n"]
    pub fn geo_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_reference_statement` after provisioning.\n"]
    pub fn ip_set_reference_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `label_match_statement` after provisioning.\n"]
    pub fn label_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_match_statement` after provisioning.\n"]
    pub fn regex_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_pattern_set_reference_statement` after provisioning.\n"]
    pub fn regex_pattern_set_reference_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_pattern_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `size_constraint_statement` after provisioning.\n"]
    pub fn size_constraint_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_constraint_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `sqli_match_statement` after provisioning.\n"]
    pub fn sqli_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqli_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_match_statement` after provisioning.\n"]
    pub fn xss_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_match_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementDynamic {
    statement: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement>,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementDynamic {
    statement: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement>,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementDynamic {
    statement: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement>,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_Statement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {
    type O =
        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementDynamic {
    and_statement: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement>,
    >,
    asn_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    byte_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    geo_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ip_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    label_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    not_statement: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement>,
    >,
    or_statement: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement>,
    >,
    regex_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    size_constraint_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    sqli_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    xss_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byte_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_constraint_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqli_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {
    #[doc = "Set the field `and_statement`.\n"]
    pub fn set_and_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `asn_match_statement`.\n"]
    pub fn set_asn_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asn_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asn_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `byte_match_statement`.\n"]
    pub fn set_byte_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.byte_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.byte_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geo_match_statement`.\n"]
    pub fn set_geo_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_set_reference_statement`.\n"]
    pub fn set_ip_set_reference_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_match_statement`.\n"]
    pub fn set_label_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not_statement`.\n"]
    pub fn set_not_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or_statement`.\n"]
    pub fn set_or_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatement,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_match_statement`.\n"]
    pub fn set_regex_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_pattern_set_reference_statement`.\n"]
    pub fn set_regex_pattern_set_reference_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_pattern_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_pattern_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `size_constraint_statement`.\n"]
    pub fn set_size_constraint_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.size_constraint_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.size_constraint_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqli_match_statement`.\n"]
    pub fn set_sqli_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqli_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqli_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_match_statement`.\n"]
    pub fn set_xss_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xss_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement {
            and_statement: core::default::Default::default(),
            asn_match_statement: core::default::Default::default(),
            byte_match_statement: core::default::Default::default(),
            geo_match_statement: core::default::Default::default(),
            ip_set_reference_statement: core::default::Default::default(),
            label_match_statement: core::default::Default::default(),
            not_statement: core::default::Default::default(),
            or_statement: core::default::Default::default(),
            regex_match_statement: core::default::Default::default(),
            regex_pattern_set_reference_statement: core::default::Default::default(),
            size_constraint_statement: core::default::Default::default(),
            sqli_match_statement: core::default::Default::default(),
            xss_match_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and_statement` after provisioning.\n"]
    pub fn and_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_AndStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `asn_match_statement` after provisioning.\n"]
    pub fn asn_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `byte_match_statement` after provisioning.\n"]
    pub fn byte_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.byte_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_match_statement` after provisioning.\n"]
    pub fn geo_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_reference_statement` after provisioning.\n"]
    pub fn ip_set_reference_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `label_match_statement` after provisioning.\n"]
    pub fn label_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `not_statement` after provisioning.\n"]
    pub fn not_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_NotStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `or_statement` after provisioning.\n"]
    pub fn or_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement_OrStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_match_statement` after provisioning.\n"]
    pub fn regex_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_pattern_set_reference_statement` after provisioning.\n"]
    pub fn regex_pattern_set_reference_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_pattern_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `size_constraint_statement` after provisioning.\n"]
    pub fn size_constraint_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_constraint_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `sqli_match_statement` after provisioning.\n"]
    pub fn sqli_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqli_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_match_statement` after provisioning.\n"]
    pub fn xss_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_match_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementDynamic {
    statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementDynamic {
    statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementDynamic {
    statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_Statement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatement_StatementDynamic {
    and_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>>,
    asn_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    byte_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    geo_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ip_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    label_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    not_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>>,
    or_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>>,
    regex_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    size_constraint_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    sqli_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    xss_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_Statement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byte_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_constraint_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqli_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatement_StatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_Statement {
    #[doc = "Set the field `and_statement`.\n"]
    pub fn set_and_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `asn_match_statement`.\n"]
    pub fn set_asn_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asn_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asn_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `byte_match_statement`.\n"]
    pub fn set_byte_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.byte_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.byte_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geo_match_statement`.\n"]
    pub fn set_geo_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_set_reference_statement`.\n"]
    pub fn set_ip_set_reference_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_match_statement`.\n"]
    pub fn set_label_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not_statement`.\n"]
    pub fn set_not_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or_statement`.\n"]
    pub fn set_or_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_match_statement`.\n"]
    pub fn set_regex_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_pattern_set_reference_statement`.\n"]
    pub fn set_regex_pattern_set_reference_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_pattern_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_pattern_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `size_constraint_statement`.\n"]
    pub fn set_size_constraint_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.size_constraint_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.size_constraint_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqli_match_statement`.\n"]
    pub fn set_sqli_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqli_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqli_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_match_statement`.\n"]
    pub fn set_xss_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xss_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement_Statement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement_Statement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement_Statement {
        Wafv2WebAcl_Rule_Statement_OrStatement_Statement {
            and_statement: core::default::Default::default(),
            asn_match_statement: core::default::Default::default(),
            byte_match_statement: core::default::Default::default(),
            geo_match_statement: core::default::Default::default(),
            ip_set_reference_statement: core::default::Default::default(),
            label_match_statement: core::default::Default::default(),
            not_statement: core::default::Default::default(),
            or_statement: core::default::Default::default(),
            regex_match_statement: core::default::Default::default(),
            regex_pattern_set_reference_statement: core::default::Default::default(),
            size_constraint_statement: core::default::Default::default(),
            sqli_match_statement: core::default::Default::default(),
            xss_match_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and_statement` after provisioning.\n"]
    pub fn and_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `asn_match_statement` after provisioning.\n"]
    pub fn asn_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `byte_match_statement` after provisioning.\n"]
    pub fn byte_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.byte_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_match_statement` after provisioning.\n"]
    pub fn geo_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_reference_statement` after provisioning.\n"]
    pub fn ip_set_reference_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `label_match_statement` after provisioning.\n"]
    pub fn label_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `not_statement` after provisioning.\n"]
    pub fn not_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `or_statement` after provisioning.\n"]
    pub fn or_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_match_statement` after provisioning.\n"]
    pub fn regex_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_pattern_set_reference_statement` after provisioning.\n"]
    pub fn regex_pattern_set_reference_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_pattern_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `size_constraint_statement` after provisioning.\n"]
    pub fn size_constraint_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_constraint_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `sqli_match_statement` after provisioning.\n"]
    pub fn sqli_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqli_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_match_statement` after provisioning.\n"]
    pub fn xss_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_match_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_AndStatementDynamic {
    statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_AndStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_AndStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_AndStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_AndStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AndStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_AndStatement {}

impl BuildWafv2WebAcl_Rule_Statement_AndStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_AndStatement {
        Wafv2WebAcl_Rule_Statement_AndStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_AndStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_AndStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Statement_AndStatementRef {
        Wafv2WebAcl_Rule_Statement_AndStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_AndStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    #[doc = ""]
    pub identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields {
            identifiers: self.identifiers,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifiers` after provisioning.\n"]
    pub fn identifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    identifier: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField {
            identifier: self.identifier,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField {
    identifier: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField { }

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField {
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField {
            identifier: self.identifier,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    #[doc = ""]
    pub identifiers: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields {
            identifiers: self.identifiers,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifiers` after provisioning.\n"]
    pub fn identifiers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.identifiers", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField {
    identifier: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField { }

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField {
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField {
            identifier: self.identifier,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionDynamic {
    address_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
        >,
    >,
    email_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
        >,
    >,
    password_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>,
    >,
    phone_number_fields: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
        >,
    >,
    username_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    payload_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_field: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_field: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number_fields: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_field: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    #[doc = "Set the field `address_fields`.\n"]
    pub fn set_address_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.address_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.address_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `email_field`.\n"]
    pub fn set_email_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.email_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.email_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `password_field`.\n"]
    pub fn set_password_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `phone_number_fields`.\n"]
    pub fn set_phone_number_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFields,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.phone_number_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.phone_number_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_field`.\n"]
    pub fn set_username_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    #[doc = ""]
    pub payload_type: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection {
            payload_type: self.payload_type,
            address_fields: core::default::Default::default(),
            email_field: core::default::Default::default(),
            password_field: core::default::Default::default(),
            phone_number_fields: core::default::Default::default(),
            username_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload_type` after provisioning.\n"]
    pub fn payload_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_type", self.base))
    }

    #[doc = "Get a reference to the value of field `address_fields` after provisioning.\n"]
    pub fn address_fields(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_AddressFieldsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `email_field` after provisioning.\n"]
    pub fn email_field(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_EmailFieldRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_field", self.base))
    }

    #[doc = "Get a reference to the value of field `password_field` after provisioning.\n"]
    pub fn password_field(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_field", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number_fields` after provisioning.\n"]
    pub fn phone_number_fields(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection_PhoneNumberFieldsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_number_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `username_field` after provisioning.\n"]
    pub fn username_field(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_field", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    failure_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
    success_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    #[doc = ""]
    pub failure_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub success_strings: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains {
            failure_strings: self.failure_strings,
            success_strings: self.success_strings,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_strings` after provisioning.\n"]
    pub fn failure_strings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_strings", self.base))
    }

    #[doc = "Get a reference to the value of field `success_strings` after provisioning.\n"]
    pub fn success_strings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_strings", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    #[doc = ""]
    pub failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header {
            failure_values: self.failure_values,
            name: self.name,
            success_values: self.success_values,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_values` after provisioning.\n"]
    pub fn failure_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_values", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `success_values` after provisioning.\n"]
    pub fn success_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_values", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    identifier: terrars::PrimField<std::string::String>,
    success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    #[doc = ""]
    pub failure_values: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub identifier: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub success_values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json {
            failure_values: self.failure_values,
            identifier: self.identifier,
            success_values: self.success_values,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_values` after provisioning.\n"]
    pub fn failure_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_values", self.base))
    }

    #[doc = "Get a reference to the value of field `identifier` after provisioning.\n"]
    pub fn identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `success_values` after provisioning.\n"]
    pub fn success_values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_values", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    failure_codes: terrars::SetField<terrars::PrimField<f64>>,
    success_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {

}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    #[doc = ""]
    pub failure_codes: terrars::SetField<terrars::PrimField<f64>>,
    #[doc = ""]
    pub success_codes: terrars::SetField<terrars::PrimField<f64>>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode {
            failure_codes: self.failure_codes,
            success_codes: self.success_codes,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `failure_codes` after provisioning.\n"]
    pub fn failure_codes(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.failure_codes", self.base))
    }

    #[doc = "Get a reference to the value of field `success_codes` after provisioning.\n"]
    pub fn success_codes(&self) -> terrars::SetRef<terrars::PrimExpr<f64>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.success_codes", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionDynamic {
    body_contains: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
        >,
    >,
    header: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
        >,
    >,
    json: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
        >,
    >,
    status_code: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body_contains: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    json: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
        >,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    #[doc = "Set the field `body_contains`.\n"]
    pub fn set_body_contains(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContains,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.body_contains = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.body_contains = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Header,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `json`.\n"]
    pub fn set_json(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_Json,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.json = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.json = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCode,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.status_code = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.status_code = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection {
            body_contains: core::default::Default::default(),
            header: core::default::Default::default(),
            json: core::default::Default::default(),
            status_code: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `body_contains` after provisioning.\n"]
    pub fn body_contains(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_BodyContainsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body_contains", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_HeaderRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `json` after provisioning.\n"]
    pub fn json(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_JsonRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.json", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection_StatusCodeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetDynamic {
    request_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
        >,
    >,
    response_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    creation_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_regex_in_path: std::option::Option<terrars::PrimField<bool>>,
    registration_page_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    #[doc = "Set the field `enable_regex_in_path`.\n"]
    pub fn set_enable_regex_in_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_regex_in_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_inspection`.\n"]
    pub fn set_request_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_inspection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_inspection`.\n"]
    pub fn set_response_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_inspection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    #[doc = ""]
    pub creation_path: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub registration_page_path: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet {
            creation_path: self.creation_path,
            enable_regex_in_path: core::default::Default::default(),
            registration_page_path: self.registration_page_path,
            request_inspection: core::default::Default::default(),
            response_inspection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `creation_path` after provisioning.\n"]
    pub fn creation_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_path", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_regex_in_path` after provisioning.\n"]
    pub fn enable_regex_in_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_regex_in_path", self.base))
    }

    #[doc = "Get a reference to the value of field `registration_page_path` after provisioning.\n"]
    pub fn registration_page_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registration_page_path", self.base))
    }

    #[doc = "Get a reference to the value of field `request_inspection` after provisioning.\n"]
    pub fn request_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet_RequestInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_inspection", self.base))
    }

    #[doc = "Get a reference to the value of field `response_inspection` after provisioning.\n"]
    pub fn response_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_inspection", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_string: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    #[doc = "Set the field `regex_string`.\n"]
    pub fn set_regex_string(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex_string = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression {
            regex_string: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regex_string` after provisioning.\n"]
    pub fn regex_string(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex_string", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeDynamic {
    exempt_uri_regular_expression: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity: std::option::Option<terrars::PrimField<std::string::String>>,
    usage_of_action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exempt_uri_regular_expression: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
        >,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    #[doc = "Set the field `sensitivity`.\n"]
    pub fn set_sensitivity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.sensitivity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exempt_uri_regular_expression`.\n"]
    pub fn set_exempt_uri_regular_expression(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpression,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exempt_uri_regular_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exempt_uri_regular_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    #[doc = ""]
    pub usage_of_action: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge {
            sensitivity: core::default::Default::default(),
            usage_of_action: self.usage_of_action,
            exempt_uri_regular_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sensitivity` after provisioning.\n"]
    pub fn sensitivity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity", self.base))
    }

    #[doc = "Get a reference to the value of field `usage_of_action` after provisioning.\n"]
    pub fn usage_of_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.usage_of_action", self.base))
    }

    #[doc = "Get a reference to the value of field `exempt_uri_regular_expression` after provisioning.\n"]
    pub fn exempt_uri_regular_expression(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge_ExemptUriRegularExpressionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exempt_uri_regular_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigDynamic {
    challenge: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
        >,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_Challenge,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig {
            challenge: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]
    pub fn challenge(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig_ChallengeRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetDynamic {
    client_side_action_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sensitivity_to_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_side_action_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
        >,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    #[doc = "Set the field `sensitivity_to_block`.\n"]
    pub fn set_sensitivity_to_block(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.sensitivity_to_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_side_action_config`.\n"]
    pub fn set_client_side_action_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.client_side_action_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.client_side_action_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet {
            sensitivity_to_block: core::default::Default::default(),
            client_side_action_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sensitivity_to_block` after provisioning.\n"]
    pub fn sensitivity_to_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sensitivity_to_block", self.base))
    }

    #[doc = "Get a reference to the value of field `client_side_action_config` after provisioning.\n"]
    pub fn client_side_action_config(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet_ClientSideActionConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.client_side_action_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionDynamic {
    password_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>,
    >,
    username_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    payload_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_field: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_field: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    #[doc = "Set the field `password_field`.\n"]
    pub fn set_password_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_field`.\n"]
    pub fn set_username_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    #[doc = ""]
    pub payload_type: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection {
            payload_type: self.payload_type,
            password_field: core::default::Default::default(),
            username_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `payload_type` after provisioning.\n"]
    pub fn payload_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_type", self.base))
    }

    #[doc = "Get a reference to the value of field `password_field` after provisioning.\n"]
    pub fn password_field(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_field", self.base))
    }

    #[doc = "Get a reference to the value of field `username_field` after provisioning.\n"]
    pub fn username_field(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetDynamic {
    request_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
        >,
    >,
    response_inspection: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_regex_in_path: std::option::Option<terrars::PrimField<bool>>,
    login_path: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    request_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    response_inspection: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
        >,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    #[doc = "Set the field `enable_regex_in_path`.\n"]
    pub fn set_enable_regex_in_path(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_regex_in_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `request_inspection`.\n"]
    pub fn set_request_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.request_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.request_inspection = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `response_inspection`.\n"]
    pub fn set_response_inspection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspection,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.response_inspection = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.response_inspection = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    #[doc = ""]
    pub login_path: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet {
            enable_regex_in_path: core::default::Default::default(),
            login_path: self.login_path,
            request_inspection: core::default::Default::default(),
            response_inspection: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_regex_in_path` after provisioning.\n"]
    pub fn enable_regex_in_path(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_regex_in_path", self.base))
    }

    #[doc = "Get a reference to the value of field `login_path` after provisioning.\n"]
    pub fn login_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_path", self.base))
    }

    #[doc = "Get a reference to the value of field `request_inspection` after provisioning.\n"]
    pub fn request_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_RequestInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.request_inspection", self.base))
    }

    #[doc = "Get a reference to the value of field `response_inspection` after provisioning.\n"]
    pub fn response_inspection(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet_ResponseInspectionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.response_inspection", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_machine_learning: std::option::Option<terrars::PrimField<bool>>,
    inspection_level: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    #[doc = "Set the field `enable_machine_learning`.\n"]
    pub fn set_enable_machine_learning(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_machine_learning = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    type O =
        terrars
        ::BlockAssignable<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    #[doc = ""]
    pub inspection_level: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
    pub fn build(
        self,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet {
            enable_machine_learning: core::default::Default::default(),
            inspection_level: self.inspection_level,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_machine_learning` after provisioning.\n"]
    pub fn enable_machine_learning(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_machine_learning", self.base))
    }

    #[doc = "Get a reference to the value of field `inspection_level` after provisioning.\n"]
    pub fn inspection_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.inspection_level", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsDynamic {
    aws_managed_rules_acfp_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet,
        >,
    >,
    aws_managed_rules_anti_ddos_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
        >,
    >,
    aws_managed_rules_atp_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet,
        >,
    >,
    aws_managed_rules_bot_control_rule_set: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
        >,
    >,
    password_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>,
    >,
    username_field: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    login_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_acfp_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_anti_ddos_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_atp_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_managed_rules_bot_control_rule_set: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_field: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username_field: std
    ::option
    ::Option<
        std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField>,
    >,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {
    #[doc = "Set the field `login_path`.\n"]
    pub fn set_login_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.login_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload_type`.\n"]
    pub fn set_payload_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `aws_managed_rules_acfp_rule_set`.\n"]
    pub fn set_aws_managed_rules_acfp_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_acfp_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_acfp_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_managed_rules_anti_ddos_rule_set`.\n"]
    pub fn set_aws_managed_rules_anti_ddos_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_anti_ddos_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_anti_ddos_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_managed_rules_atp_rule_set`.\n"]
    pub fn set_aws_managed_rules_atp_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_atp_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_atp_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `aws_managed_rules_bot_control_rule_set`.\n"]
    pub fn set_aws_managed_rules_bot_control_rule_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSet,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_managed_rules_bot_control_rule_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_managed_rules_bot_control_rule_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `password_field`.\n"]
    pub fn set_password_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.password_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.password_field = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `username_field`.\n"]
    pub fn set_username_field(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameField,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.username_field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.username_field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs {
            login_path: core::default::Default::default(),
            payload_type: core::default::Default::default(),
            aws_managed_rules_acfp_rule_set: core::default::Default::default(),
            aws_managed_rules_anti_ddos_rule_set: core::default::Default::default(),
            aws_managed_rules_atp_rule_set: core::default::Default::default(),
            aws_managed_rules_bot_control_rule_set: core::default::Default::default(),
            password_field: core::default::Default::default(),
            username_field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `login_path` after provisioning.\n"]
    pub fn login_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.login_path", self.base))
    }

    #[doc = "Get a reference to the value of field `payload_type` after provisioning.\n"]
    pub fn payload_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_type", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_acfp_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_acfp_rule_set(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAcfpRuleSetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_acfp_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_anti_ddos_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_anti_ddos_rule_set(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAntiDdosRuleSetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_anti_ddos_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_atp_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_atp_rule_set(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesAtpRuleSetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_atp_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `aws_managed_rules_bot_control_rule_set` after provisioning.\n"]
    pub fn aws_managed_rules_bot_control_rule_set(
        &self,
    ) -> terrars
    ::ListRef<
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_AwsManagedRulesBotControlRuleSetRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_managed_rules_bot_control_rule_set", self.base))
    }

    #[doc = "Get a reference to the value of field `password_field` after provisioning.\n"]
    pub fn password_field(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_PasswordFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.password_field", self.base))
    }

    #[doc = "Get a reference to the value of field `username_field` after provisioning.\n"]
    pub fn username_field(
        &self,
    ) -> terrars
    ::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs_UsernameFieldRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.username_field", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseDynamic {
    allow: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Allow>>,
    block: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Block>>,
    captcha: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Captcha>>,
    challenge: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Challenge>>,
    count: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action_Count>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allow: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Allow>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Block>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captcha: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Captcha>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Challenge>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action_Count>>,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {
    #[doc = "Set the field `allow`.\n"]
    pub fn set_allow(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Allow>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.allow = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.allow = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `block`.\n"]
    pub fn set_block(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Block>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.block = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.block = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `captcha`.\n"]
    pub fn set_captcha(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Captcha>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.captcha = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.captcha = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `challenge`.\n"]
    pub fn set_challenge(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Challenge>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `count`.\n"]
    pub fn set_count(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action_Count>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.count = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse {
            allow: core::default::Default::default(),
            block: core::default::Default::default(),
            captcha: core::default::Default::default(),
            challenge: core::default::Default::default(),
            count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allow` after provisioning.\n"]
    pub fn allow(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_AllowRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allow", self.base))
    }

    #[doc = "Get a reference to the value of field `block` after provisioning.\n"]
    pub fn block(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_BlockRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block", self.base))
    }

    #[doc = "Get a reference to the value of field `captcha` after provisioning.\n"]
    pub fn captcha(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_CaptchaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge` after provisioning.\n"]
    pub fn challenge(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_ChallengeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Action_CountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.count", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideDynamic {
    action_to_use: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_to_use: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse>>,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
    #[doc = "Set the field `action_to_use`.\n"]
    pub fn set_action_to_use(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUse,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_to_use = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_to_use = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride {
            name: self.name,
            action_to_use: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `action_to_use` after provisioning.\n"]
    pub fn action_to_use(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride_ActionToUseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_to_use", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementDynamic {
    and_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>>,
    asn_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    byte_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    geo_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ip_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    label_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    not_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>>,
    or_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>>,
    regex_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    size_constraint_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    sqli_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    xss_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byte_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_constraint_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqli_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {
    #[doc = "Set the field `and_statement`.\n"]
    pub fn set_and_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `asn_match_statement`.\n"]
    pub fn set_asn_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asn_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asn_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `byte_match_statement`.\n"]
    pub fn set_byte_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.byte_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.byte_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geo_match_statement`.\n"]
    pub fn set_geo_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_set_reference_statement`.\n"]
    pub fn set_ip_set_reference_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_match_statement`.\n"]
    pub fn set_label_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not_statement`.\n"]
    pub fn set_not_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or_statement`.\n"]
    pub fn set_or_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_match_statement`.\n"]
    pub fn set_regex_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_pattern_set_reference_statement`.\n"]
    pub fn set_regex_pattern_set_reference_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_pattern_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_pattern_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `size_constraint_statement`.\n"]
    pub fn set_size_constraint_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.size_constraint_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.size_constraint_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqli_match_statement`.\n"]
    pub fn set_sqli_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqli_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqli_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_match_statement`.\n"]
    pub fn set_xss_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xss_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement {
            and_statement: core::default::Default::default(),
            asn_match_statement: core::default::Default::default(),
            byte_match_statement: core::default::Default::default(),
            geo_match_statement: core::default::Default::default(),
            ip_set_reference_statement: core::default::Default::default(),
            label_match_statement: core::default::Default::default(),
            not_statement: core::default::Default::default(),
            or_statement: core::default::Default::default(),
            regex_match_statement: core::default::Default::default(),
            regex_pattern_set_reference_statement: core::default::Default::default(),
            size_constraint_statement: core::default::Default::default(),
            sqli_match_statement: core::default::Default::default(),
            xss_match_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and_statement` after provisioning.\n"]
    pub fn and_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_AndStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `asn_match_statement` after provisioning.\n"]
    pub fn asn_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `byte_match_statement` after provisioning.\n"]
    pub fn byte_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.byte_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_match_statement` after provisioning.\n"]
    pub fn geo_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_reference_statement` after provisioning.\n"]
    pub fn ip_set_reference_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `label_match_statement` after provisioning.\n"]
    pub fn label_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `not_statement` after provisioning.\n"]
    pub fn not_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_NotStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `or_statement` after provisioning.\n"]
    pub fn or_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_Statement_OrStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_match_statement` after provisioning.\n"]
    pub fn regex_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_pattern_set_reference_statement` after provisioning.\n"]
    pub fn regex_pattern_set_reference_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_pattern_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `size_constraint_statement` after provisioning.\n"]
    pub fn size_constraint_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_constraint_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `sqli_match_statement` after provisioning.\n"]
    pub fn sqli_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqli_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_match_statement` after provisioning.\n"]
    pub fn xss_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_match_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementDynamic {
    managed_rule_group_configs: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs>>,
    rule_action_override: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>>,
    scope_down_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
    name: terrars::PrimField<std::string::String>,
    vendor_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_rule_group_configs: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action_override: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_down_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_rule_group_configs`.\n"]
    pub fn set_managed_rule_group_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigs,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_rule_group_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_rule_group_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_action_override`.\n"]
    pub fn set_rule_action_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_action_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_action_override = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope_down_statement`.\n"]
    pub fn set_scope_down_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scope_down_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scope_down_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vendor_name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement {
            name: self.name,
            vendor_name: self.vendor_name,
            version: core::default::Default::default(),
            managed_rule_group_configs: core::default::Default::default(),
            rule_action_override: core::default::Default::default(),
            scope_down_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementRef {
        Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `vendor_name` after provisioning.\n"]
    pub fn vendor_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor_name", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_rule_group_configs` after provisioning.\n"]
    pub fn managed_rule_group_configs(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_ManagedRuleGroupConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rule_group_configs", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action_override` after provisioning.\n"]
    pub fn rule_action_override(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_action_override", self.base))
    }

    #[doc = "Get a reference to the value of field `scope_down_statement` after provisioning.\n"]
    pub fn scope_down_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_down_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_NotStatementDynamic {
    statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_NotStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_NotStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_NotStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_NotStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_NotStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_NotStatement {}

impl BuildWafv2WebAcl_Rule_Statement_NotStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_NotStatement {
        Wafv2WebAcl_Rule_Statement_NotStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_NotStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_NotStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Statement_NotStatementRef {
        Wafv2WebAcl_Rule_Statement_NotStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_NotStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_OrStatementDynamic {
    statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_OrStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_OrStatement {
    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement_Statement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_OrStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_OrStatement {}

impl BuildWafv2WebAcl_Rule_Statement_OrStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_OrStatement {
        Wafv2WebAcl_Rule_Statement_OrStatement {
            statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_OrStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_OrStatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_Statement_OrStatementRef {
        Wafv2WebAcl_Rule_Statement_OrStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_OrStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatement_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn {}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_AsnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_AsnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_AsnRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_AsnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_AsnRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieDynamic {
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie {
            name: self.name,
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp {}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIpRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderDynamic {
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header {
            name: self.name,
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod {}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethodRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip {}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_IpRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_IpRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_IpRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_IpRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_IpRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace {
    namespace: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace { }

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace {
    #[doc = ""]
    pub namespace: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace { namespace: self.namespace }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespaceRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `namespace` after provisioning.\n"]
    pub fn namespace(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.namespace", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentDynamic {
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument {
            name: self.name,
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringDynamic {
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {
    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString {
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathDynamic {
    text_transformation: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    text_transformation: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {
    #[doc = "Set the field `text_transformation`.\n"]
    pub fn set_text_transformation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement_TextTransformation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.text_transformation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.text_transformation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath {
            text_transformation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyDynamic {
    asn: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn>>,
    cookie: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie>>,
    forwarded_ip: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp>>,
    header: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header>>,
    http_method: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod>>,
    ip: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip>>,
    ja3_fingerprint: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>>,
    ja4_fingerprint: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>>,
    label_namespace: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace>>,
    query_argument: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString>>,
    uri_path: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cookie: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_ip: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_method: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ja3_fingerprint: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ja4_fingerprint: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_namespace: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_argument: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_path: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Asn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cookie`.\n"]
    pub fn set_cookie(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Cookie>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cookie = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cookie = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarded_ip`.\n"]
    pub fn set_forwarded_ip(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIp>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_ip = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Header>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `http_method`.\n"]
    pub fn set_http_method(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethod>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.http_method = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.http_method = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ip>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ja3_fingerprint`.\n"]
    pub fn set_ja3_fingerprint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3Fingerprint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ja3_fingerprint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ja3_fingerprint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ja4_fingerprint`.\n"]
    pub fn set_ja4_fingerprint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4Fingerprint>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ja4_fingerprint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ja4_fingerprint = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_namespace`.\n"]
    pub fn set_label_namespace(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespace>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_namespace = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_namespace = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_argument`.\n"]
    pub fn set_query_argument(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgument>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_argument = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_argument = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryString>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uri_path`.\n"]
    pub fn set_uri_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uri_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_path = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey {
            asn: core::default::Default::default(),
            cookie: core::default::Default::default(),
            forwarded_ip: core::default::Default::default(),
            header: core::default::Default::default(),
            http_method: core::default::Default::default(),
            ip: core::default::Default::default(),
            ja3_fingerprint: core::default::Default::default(),
            ja4_fingerprint: core::default::Default::default(),
            label_namespace: core::default::Default::default(),
            query_argument: core::default::Default::default(),
            query_string: core::default::Default::default(),
            uri_path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_AsnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `cookie` after provisioning.\n"]
    pub fn cookie(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_CookieRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cookie", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_ip` after provisioning.\n"]
    pub fn forwarded_ip(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_ForwardedIpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `http_method` after provisioning.\n"]
    pub fn http_method(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_HttpMethodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_method", self.base))
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_IpRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ja3_fingerprint` after provisioning.\n"]
    pub fn ja3_fingerprint(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja3FingerprintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ja3_fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `ja4_fingerprint` after provisioning.\n"]
    pub fn ja4_fingerprint(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_Ja4FingerprintRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ja4_fingerprint", self.base))
    }

    #[doc = "Get a reference to the value of field `label_namespace` after provisioning.\n"]
    pub fn label_namespace(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_LabelNamespaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_namespace", self.base))
    }

    #[doc = "Get a reference to the value of field `query_argument` after provisioning.\n"]
    pub fn query_argument(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryArgumentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_argument", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_QueryStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_path` after provisioning.\n"]
    pub fn uri_path(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey_UriPathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RateBasedStatementDynamic {
    custom_key: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey>>,
    forwarded_ip_config: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    scope_down_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aggregate_key_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluation_window_sec: std::option::Option<terrars::PrimField<f64>>,
    limit: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_key: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    forwarded_ip_config: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope_down_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RateBasedStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatement {
    #[doc = "Set the field `aggregate_key_type`.\n"]
    pub fn set_aggregate_key_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.aggregate_key_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `evaluation_window_sec`.\n"]
    pub fn set_evaluation_window_sec(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.evaluation_window_sec = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_key`.\n"]
    pub fn set_custom_key(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKey>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_key = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_key = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `forwarded_ip_config`.\n"]
    pub fn set_forwarded_ip_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.forwarded_ip_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.forwarded_ip_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scope_down_statement`.\n"]
    pub fn set_scope_down_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatement>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scope_down_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scope_down_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RateBasedStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RateBasedStatement {
    #[doc = ""]
    pub limit: terrars::PrimField<f64>,
}

impl BuildWafv2WebAcl_Rule_Statement_RateBasedStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RateBasedStatement {
        Wafv2WebAcl_Rule_Statement_RateBasedStatement {
            aggregate_key_type: core::default::Default::default(),
            evaluation_window_sec: core::default::Default::default(),
            limit: self.limit,
            custom_key: core::default::Default::default(),
            forwarded_ip_config: core::default::Default::default(),
            scope_down_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RateBasedStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RateBasedStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RateBasedStatementRef {
        Wafv2WebAcl_Rule_Statement_RateBasedStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RateBasedStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aggregate_key_type` after provisioning.\n"]
    pub fn aggregate_key_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aggregate_key_type", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluation_window_sec` after provisioning.\n"]
    pub fn evaluation_window_sec(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluation_window_sec", self.base))
    }

    #[doc = "Get a reference to the value of field `limit` after provisioning.\n"]
    pub fn limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.limit", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_key` after provisioning.\n"]
    pub fn custom_key(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_CustomKeyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_key", self.base))
    }

    #[doc = "Get a reference to the value of field `forwarded_ip_config` after provisioning.\n"]
    pub fn forwarded_ip_config(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatement_ForwardedIpConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forwarded_ip_config", self.base))
    }

    #[doc = "Get a reference to the value of field `scope_down_statement` after provisioning.\n"]
    pub fn scope_down_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatement_ScopeDownStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scope_down_statement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementDynamic {
    rule_action_override: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action_override: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>>,
    dynamic: Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
    #[doc = "Set the field `rule_action_override`.\n"]
    pub fn set_rule_action_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverride>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_action_override = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_action_override = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
        Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement {
            arn: self.arn,
            rule_action_override: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementRef {
        Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action_override` after provisioning.\n"]
    pub fn rule_action_override(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement_RuleActionOverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_action_override", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_Rule_StatementDynamic {
    and_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AndStatement>>,
    asn_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    byte_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    geo_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ip_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    label_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    managed_rule_group_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement>>,
    not_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_NotStatement>>,
    or_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_OrStatement>>,
    rate_based_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RateBasedStatement>>,
    regex_match_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    rule_group_reference_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement>>,
    size_constraint_statement: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    sqli_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    xss_match_statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule_Statement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AndStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    byte_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    geo_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_reference_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_rule_group_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_NotStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_OrStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rate_based_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RateBasedStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex_pattern_set_reference_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_group_reference_statement: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_constraint_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sqli_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    xss_match_statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    dynamic: Wafv2WebAcl_Rule_StatementDynamic,
}

impl Wafv2WebAcl_Rule_Statement {
    #[doc = "Set the field `and_statement`.\n"]
    pub fn set_and_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AndStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `asn_match_statement`.\n"]
    pub fn set_asn_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_AsnMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.asn_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.asn_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `byte_match_statement`.\n"]
    pub fn set_byte_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ByteMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.byte_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.byte_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `geo_match_statement`.\n"]
    pub fn set_geo_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_GeoMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.geo_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.geo_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_set_reference_statement`.\n"]
    pub fn set_ip_set_reference_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_match_statement`.\n"]
    pub fn set_label_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_LabelMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_rule_group_statement`.\n"]
    pub fn set_managed_rule_group_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_rule_group_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_rule_group_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not_statement`.\n"]
    pub fn set_not_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_NotStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or_statement`.\n"]
    pub fn set_or_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_OrStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rate_based_statement`.\n"]
    pub fn set_rate_based_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RateBasedStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rate_based_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rate_based_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_match_statement`.\n"]
    pub fn set_regex_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `regex_pattern_set_reference_statement`.\n"]
    pub fn set_regex_pattern_set_reference_statement(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.regex_pattern_set_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.regex_pattern_set_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_group_reference_statement`.\n"]
    pub fn set_rule_group_reference_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_group_reference_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_group_reference_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `size_constraint_statement`.\n"]
    pub fn set_size_constraint_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SizeConstraintStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.size_constraint_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.size_constraint_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sqli_match_statement`.\n"]
    pub fn set_sqli_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_SqliMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sqli_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sqli_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `xss_match_statement`.\n"]
    pub fn set_xss_match_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement_XssMatchStatement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.xss_match_statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.xss_match_statement = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule_Statement {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule_Statement {}

impl BuildWafv2WebAcl_Rule_Statement {
    pub fn build(self) -> Wafv2WebAcl_Rule_Statement {
        Wafv2WebAcl_Rule_Statement {
            and_statement: core::default::Default::default(),
            asn_match_statement: core::default::Default::default(),
            byte_match_statement: core::default::Default::default(),
            geo_match_statement: core::default::Default::default(),
            ip_set_reference_statement: core::default::Default::default(),
            label_match_statement: core::default::Default::default(),
            managed_rule_group_statement: core::default::Default::default(),
            not_statement: core::default::Default::default(),
            or_statement: core::default::Default::default(),
            rate_based_statement: core::default::Default::default(),
            regex_match_statement: core::default::Default::default(),
            regex_pattern_set_reference_statement: core::default::Default::default(),
            rule_group_reference_statement: core::default::Default::default(),
            size_constraint_statement: core::default::Default::default(),
            sqli_match_statement: core::default::Default::default(),
            xss_match_statement: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_Rule_StatementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_Rule_StatementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_Rule_StatementRef {
        Wafv2WebAcl_Rule_StatementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_Rule_StatementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `and_statement` after provisioning.\n"]
    pub fn and_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AndStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.and_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `asn_match_statement` after provisioning.\n"]
    pub fn asn_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_AsnMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.asn_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `byte_match_statement` after provisioning.\n"]
    pub fn byte_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ByteMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.byte_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `geo_match_statement` after provisioning.\n"]
    pub fn geo_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_GeoMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.geo_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_reference_statement` after provisioning.\n"]
    pub fn ip_set_reference_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_IpSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `label_match_statement` after provisioning.\n"]
    pub fn label_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_LabelMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_rule_group_statement` after provisioning.\n"]
    pub fn managed_rule_group_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_ManagedRuleGroupStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rule_group_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `not_statement` after provisioning.\n"]
    pub fn not_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_NotStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `or_statement` after provisioning.\n"]
    pub fn or_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_OrStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.or_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `rate_based_statement` after provisioning.\n"]
    pub fn rate_based_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RateBasedStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rate_based_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_match_statement` after provisioning.\n"]
    pub fn regex_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `regex_pattern_set_reference_statement` after provisioning.\n"]
    pub fn regex_pattern_set_reference_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RegexPatternSetReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.regex_pattern_set_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_group_reference_statement` after provisioning.\n"]
    pub fn rule_group_reference_statement(
        &self,
    ) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_RuleGroupReferenceStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_group_reference_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `size_constraint_statement` after provisioning.\n"]
    pub fn size_constraint_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SizeConstraintStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.size_constraint_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `sqli_match_statement` after provisioning.\n"]
    pub fn sqli_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_SqliMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sqli_match_statement", self.base))
    }

    #[doc = "Get a reference to the value of field `xss_match_statement` after provisioning.\n"]
    pub fn xss_match_statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_Statement_XssMatchStatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.xss_match_statement", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_VisibilityConfig {
    cloudwatch_metrics_enabled: terrars::PrimField<bool>,
    metric_name: terrars::PrimField<std::string::String>,
    sampled_requests_enabled: terrars::PrimField<bool>,
}

impl Wafv2WebAcl_VisibilityConfig { }

impl terrars::ToListMappable for Wafv2WebAcl_VisibilityConfig {
    type O = terrars::BlockAssignable<Wafv2WebAcl_VisibilityConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_VisibilityConfig {
    #[doc = ""]
    pub cloudwatch_metrics_enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sampled_requests_enabled: terrars::PrimField<bool>,
}

impl BuildWafv2WebAcl_VisibilityConfig {
    pub fn build(self) -> Wafv2WebAcl_VisibilityConfig {
        Wafv2WebAcl_VisibilityConfig {
            cloudwatch_metrics_enabled: self.cloudwatch_metrics_enabled,
            metric_name: self.metric_name,
            sampled_requests_enabled: self.sampled_requests_enabled,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_VisibilityConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_VisibilityConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_VisibilityConfigRef {
        Wafv2WebAcl_VisibilityConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_VisibilityConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_metrics_enabled` after provisioning.\n"]
    pub fn cloudwatch_metrics_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cloudwatch_metrics_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.base))
    }

    #[doc = "Get a reference to the value of field `sampled_requests_enabled` after provisioning.\n"]
    pub fn sampled_requests_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampled_requests_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAcl_RuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Action>>,
    captcha_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_CaptchaConfig>>,
    challenge_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_ChallengeConfig>>,
    override_action: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_OverrideAction>>,
    rule_label: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_RuleLabel>>,
    statement: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_Rule_Statement>>,
    visibility_config: std::option::Option<terrars::DynamicBlock<Wafv2WebAcl_VisibilityConfig>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAcl_Rule {
    name: terrars::PrimField<std::string::String>,
    priority: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    captcha_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_CaptchaConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    challenge_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_ChallengeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_action: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_OverrideAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_label: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_RuleLabel>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statement: std::option::Option<std::vec::Vec<Wafv2WebAcl_Rule_Statement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visibility_config: std::option::Option<std::vec::Vec<Wafv2WebAcl_VisibilityConfig>>,
    dynamic: Wafv2WebAcl_RuleDynamic,
}

impl Wafv2WebAcl_Rule {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Action>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `captcha_config`.\n"]
    pub fn set_captcha_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_CaptchaConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.captcha_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.captcha_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `challenge_config`.\n"]
    pub fn set_challenge_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_ChallengeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.challenge_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.challenge_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_action`.\n"]
    pub fn set_override_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_OverrideAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_label`.\n"]
    pub fn set_rule_label(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_RuleLabel>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rule_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rule_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `statement`.\n"]
    pub fn set_statement(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_Rule_Statement>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `visibility_config`.\n"]
    pub fn set_visibility_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAcl_VisibilityConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.visibility_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.visibility_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAcl_Rule {
    type O = terrars::BlockAssignable<Wafv2WebAcl_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAcl_Rule {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
}

impl BuildWafv2WebAcl_Rule {
    pub fn build(self) -> Wafv2WebAcl_Rule {
        Wafv2WebAcl_Rule {
            name: self.name,
            priority: self.priority,
            action: core::default::Default::default(),
            captcha_config: core::default::Default::default(),
            challenge_config: core::default::Default::default(),
            override_action: core::default::Default::default(),
            rule_label: core::default::Default::default(),
            statement: core::default::Default::default(),
            visibility_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAcl_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAcl_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Wafv2WebAcl_RuleRef {
        Wafv2WebAcl_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAcl_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `captcha_config` after provisioning.\n"]
    pub fn captcha_config(&self) -> terrars::ListRef<Wafv2WebAcl_CaptchaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.captcha_config", self.base))
    }

    #[doc = "Get a reference to the value of field `challenge_config` after provisioning.\n"]
    pub fn challenge_config(&self) -> terrars::ListRef<Wafv2WebAcl_ChallengeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.challenge_config", self.base))
    }

    #[doc = "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_OverrideActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::ListRef<Wafv2WebAcl_Rule_StatementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statement", self.base))
    }

    #[doc = "Get a reference to the value of field `visibility_config` after provisioning.\n"]
    pub fn visibility_config(&self) -> terrars::ListRef<Wafv2WebAcl_VisibilityConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.visibility_config", self.base))
    }
}
