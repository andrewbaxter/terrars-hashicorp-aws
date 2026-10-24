use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CognitoIdentityPoolRolesAttachmentData {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    identity_pool_id: terrars::PrimField<std::string::String>,
    roles: terrars::RecField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_mapping: std::option::Option<std::vec::Vec<CognitoIdentityPoolRolesAttachmentRoleMappingEl>>,
    dynamic: CognitoIdentityPoolRolesAttachmentDynamic,
}

struct CognitoIdentityPoolRolesAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CognitoIdentityPoolRolesAttachmentData>,
}

#[derive(Clone)]
pub struct CognitoIdentityPoolRolesAttachment(std::rc::Rc<CognitoIdentityPoolRolesAttachment_>);

impl CognitoIdentityPoolRolesAttachment {
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

    #[doc = "Set the field `role_mapping`.\n"]
    pub fn set_role_mapping(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().role_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.role_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }
}

impl terrars::Referable for CognitoIdentityPoolRolesAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CognitoIdentityPoolRolesAttachment { }

impl terrars::ToListMappable for CognitoIdentityPoolRolesAttachment {
    type O = terrars::ListRef<CognitoIdentityPoolRolesAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CognitoIdentityPoolRolesAttachment_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cognito_identity_pool_roles_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCognitoIdentityPoolRolesAttachment {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub identity_pool_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub roles: terrars::RecField<terrars::PrimField<std::string::String>>,
}

impl BuildCognitoIdentityPoolRolesAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> CognitoIdentityPoolRolesAttachment {
        let out = CognitoIdentityPoolRolesAttachment(std::rc::Rc::new(CognitoIdentityPoolRolesAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CognitoIdentityPoolRolesAttachmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                identity_pool_id: self.identity_pool_id,
                roles: self.roles,
                role_mapping: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CognitoIdentityPoolRolesAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoIdentityPoolRolesAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CognitoIdentityPoolRolesAttachmentRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `identity_pool_id` after provisioning.\n"]
    pub fn identity_pool_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_pool_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `roles` after provisioning.\n"]
    pub fn roles(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.roles", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    claim: terrars::PrimField<std::string::String>,
    match_type: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl { }

impl terrars::ToListMappable for CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    type O = terrars::BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    #[doc = ""]
    pub claim: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub match_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
    pub fn build(self) -> CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
        CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl {
            claim: self.claim,
            match_type: self.match_type,
            role_arn: self.role_arn,
            value: self.value,
        }
    }
}

pub struct CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
        CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `claim` after provisioning.\n"]
    pub fn claim(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.claim", self.base))
    }

    #[doc = "Get a reference to the value of field `match_type` after provisioning.\n"]
    pub fn match_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.match_type", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoIdentityPoolRolesAttachmentRoleMappingElDynamic {
    mapping_rule: std
    ::option
    ::Option<terrars::DynamicBlock<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>>,
}

#[derive(Serialize)]
pub struct CognitoIdentityPoolRolesAttachmentRoleMappingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ambiguous_role_resolution: std::option::Option<terrars::PrimField<std::string::String>>,
    identity_provider: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mapping_rule: std::option::Option<std::vec::Vec<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>>,
    dynamic: CognitoIdentityPoolRolesAttachmentRoleMappingElDynamic,
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingEl {
    #[doc = "Set the field `ambiguous_role_resolution`.\n"]
    pub fn set_ambiguous_role_resolution(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ambiguous_role_resolution = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mapping_rule`.\n"]
    pub fn set_mapping_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.mapping_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.mapping_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CognitoIdentityPoolRolesAttachmentRoleMappingEl {
    type O = terrars::BlockAssignable<CognitoIdentityPoolRolesAttachmentRoleMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCognitoIdentityPoolRolesAttachmentRoleMappingEl {
    #[doc = ""]
    pub identity_provider: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCognitoIdentityPoolRolesAttachmentRoleMappingEl {
    pub fn build(self) -> CognitoIdentityPoolRolesAttachmentRoleMappingEl {
        CognitoIdentityPoolRolesAttachmentRoleMappingEl {
            ambiguous_role_resolution: core::default::Default::default(),
            identity_provider: self.identity_provider,
            type_: self.type_,
            mapping_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
        CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CognitoIdentityPoolRolesAttachmentRoleMappingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ambiguous_role_resolution` after provisioning.\n"]
    pub fn ambiguous_role_resolution(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ambiguous_role_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `identity_provider` after provisioning.\n"]
    pub fn identity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.identity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `mapping_rule` after provisioning.\n"]
    pub fn mapping_rule(&self) -> terrars::ListRef<CognitoIdentityPoolRolesAttachmentRoleMappingElMappingRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.mapping_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CognitoIdentityPoolRolesAttachmentDynamic {
    role_mapping: std::option::Option<terrars::DynamicBlock<CognitoIdentityPoolRolesAttachmentRoleMappingEl>>,
}
