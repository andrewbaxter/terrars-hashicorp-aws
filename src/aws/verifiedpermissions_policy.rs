#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct VerifiedpermissionsPolicyData_ {
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
    policy_store_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std::option::Option<std::vec::Vec<VerifiedpermissionsPolicy_Definition>>,
    dynamic: VerifiedpermissionsPolicyDynamic,
}

struct VerifiedpermissionsPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<VerifiedpermissionsPolicyData_>,
}

#[derive(Clone)]
pub struct VerifiedpermissionsPolicy(std::rc::Rc<VerifiedpermissionsPolicy_>);

impl VerifiedpermissionsPolicy {
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

    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().definition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.definition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<VerifiedpermissionsPolicy_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }
}

impl terrars::Referable for VerifiedpermissionsPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for VerifiedpermissionsPolicy { }

impl terrars::ToListMappable for VerifiedpermissionsPolicy {
    type O = terrars::ListRef<VerifiedpermissionsPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for VerifiedpermissionsPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_verifiedpermissions_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildVerifiedpermissionsPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub policy_store_id: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> VerifiedpermissionsPolicy {
        let out = VerifiedpermissionsPolicy(std::rc::Rc::new(VerifiedpermissionsPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(VerifiedpermissionsPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                policy_store_id: self.policy_store_id,
                region: core::default::Default::default(),
                definition: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct VerifiedpermissionsPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl VerifiedpermissionsPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_id` after provisioning.\n"]
    pub fn policy_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_store_id` after provisioning.\n"]
    pub fn policy_store_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_store_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::ListRef<VerifiedpermissionsPolicy_DefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsPolicyDynamic {
    definition: std::option::Option<terrars::DynamicBlock<VerifiedpermissionsPolicy_Definition>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicy_Definition_Static {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    statement: terrars::PrimField<std::string::String>,
}

impl VerifiedpermissionsPolicy_Definition_Static {
    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for VerifiedpermissionsPolicy_Definition_Static {
    type O = terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_Static>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicy_Definition_Static {
    #[doc = ""]
    pub statement: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsPolicy_Definition_Static {
    pub fn build(self) -> VerifiedpermissionsPolicy_Definition_Static {
        VerifiedpermissionsPolicy_Definition_Static {
            description: core::default::Default::default(),
            statement: self.statement,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsPolicy_Definition_StaticRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsPolicy_Definition_StaticRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsPolicy_Definition_StaticRef {
        VerifiedpermissionsPolicy_Definition_StaticRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicy_Definition_StaticRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `statement` after provisioning.\n"]
    pub fn statement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.statement", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal {
    entity_id: terrars::PrimField<std::string::String>,
    entity_type: terrars::PrimField<std::string::String>,
}

impl VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal { }

impl terrars::ToListMappable for VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal {
    type O = terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicy_Definition_TemplateLinked_Principal {
    #[doc = ""]
    pub entity_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub entity_type: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsPolicy_Definition_TemplateLinked_Principal {
    pub fn build(self) -> VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal {
        VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsPolicy_Definition_TemplateLinked_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsPolicy_Definition_TemplateLinked_PrincipalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsPolicy_Definition_TemplateLinked_PrincipalRef {
        VerifiedpermissionsPolicy_Definition_TemplateLinked_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicy_Definition_TemplateLinked_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\n"]
    pub fn entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource {
    entity_id: terrars::PrimField<std::string::String>,
    entity_type: terrars::PrimField<std::string::String>,
}

impl VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource { }

impl terrars::ToListMappable for VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource {
    type O = terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicy_Definition_TemplateLinked_Resource {
    #[doc = ""]
    pub entity_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub entity_type: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsPolicy_Definition_TemplateLinked_Resource {
    pub fn build(self) -> VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource {
        VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsPolicy_Definition_TemplateLinked_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsPolicy_Definition_TemplateLinked_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsPolicy_Definition_TemplateLinked_ResourceRef {
        VerifiedpermissionsPolicy_Definition_TemplateLinked_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicy_Definition_TemplateLinked_ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `entity_id` after provisioning.\n"]
    pub fn entity_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `entity_type` after provisioning.\n"]
    pub fn entity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.entity_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsPolicy_Definition_TemplateLinkedDynamic {
    principal: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal>>,
    resource: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicy_Definition_TemplateLinked {
    policy_template_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource>>,
    dynamic: VerifiedpermissionsPolicy_Definition_TemplateLinkedDynamic,
}

impl VerifiedpermissionsPolicy_Definition_TemplateLinked {
    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_TemplateLinked_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_TemplateLinked_Resource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedpermissionsPolicy_Definition_TemplateLinked {
    type O = terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_TemplateLinked>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicy_Definition_TemplateLinked {
    #[doc = ""]
    pub policy_template_id: terrars::PrimField<std::string::String>,
}

impl BuildVerifiedpermissionsPolicy_Definition_TemplateLinked {
    pub fn build(self) -> VerifiedpermissionsPolicy_Definition_TemplateLinked {
        VerifiedpermissionsPolicy_Definition_TemplateLinked {
            policy_template_id: self.policy_template_id,
            principal: core::default::Default::default(),
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsPolicy_Definition_TemplateLinkedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsPolicy_Definition_TemplateLinkedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> VerifiedpermissionsPolicy_Definition_TemplateLinkedRef {
        VerifiedpermissionsPolicy_Definition_TemplateLinkedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicy_Definition_TemplateLinkedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_template_id` after provisioning.\n"]
    pub fn policy_template_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_template_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<VerifiedpermissionsPolicy_Definition_TemplateLinked_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<VerifiedpermissionsPolicy_Definition_TemplateLinked_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}

#[derive(Serialize, Default)]
struct VerifiedpermissionsPolicy_DefinitionDynamic {
    static_: std::option::Option<terrars::DynamicBlock<VerifiedpermissionsPolicy_Definition_Static>>,
    template_linked: std
    ::option
    ::Option<terrars::DynamicBlock<VerifiedpermissionsPolicy_Definition_TemplateLinked>>,
}

#[derive(Serialize)]
pub struct VerifiedpermissionsPolicy_Definition {
    #[serde(rename = "static", skip_serializing_if = "std::option::Option::is_none")]
    static_: std::option::Option<std::vec::Vec<VerifiedpermissionsPolicy_Definition_Static>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_linked: std::option::Option<std::vec::Vec<VerifiedpermissionsPolicy_Definition_TemplateLinked>>,
    dynamic: VerifiedpermissionsPolicy_DefinitionDynamic,
}

impl VerifiedpermissionsPolicy_Definition {
    #[doc = "Set the field `static_`.\n"]
    pub fn set_static(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_Static>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.static_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.static_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `template_linked`.\n"]
    pub fn set_template_linked(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition_TemplateLinked>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.template_linked = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.template_linked = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for VerifiedpermissionsPolicy_Definition {
    type O = terrars::BlockAssignable<VerifiedpermissionsPolicy_Definition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildVerifiedpermissionsPolicy_Definition {}

impl BuildVerifiedpermissionsPolicy_Definition {
    pub fn build(self) -> VerifiedpermissionsPolicy_Definition {
        VerifiedpermissionsPolicy_Definition {
            static_: core::default::Default::default(),
            template_linked: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct VerifiedpermissionsPolicy_DefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for VerifiedpermissionsPolicy_DefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> VerifiedpermissionsPolicy_DefinitionRef {
        VerifiedpermissionsPolicy_DefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl VerifiedpermissionsPolicy_DefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `static_` after provisioning.\n"]
    pub fn static_(&self) -> terrars::ListRef<VerifiedpermissionsPolicy_Definition_StaticRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static", self.base))
    }

    #[doc = "Get a reference to the value of field `template_linked` after provisioning.\n"]
    pub fn template_linked(&self) -> terrars::ListRef<VerifiedpermissionsPolicy_Definition_TemplateLinkedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.template_linked", self.base))
    }
}
