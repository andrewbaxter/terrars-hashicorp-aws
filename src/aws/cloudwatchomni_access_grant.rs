#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CloudwatchomniAccessGrantData_ {
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
    domain_id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    permission: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    space_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal: std::option::Option<std::vec::Vec<CloudwatchomniAccessGrant_Principal>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scoped_action: std::option::Option<std::vec::Vec<CloudwatchomniAccessGrant_ScopedAction>>,
    dynamic: CloudwatchomniAccessGrantDynamic,
}

struct CloudwatchomniAccessGrant_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CloudwatchomniAccessGrantData_>,
}

#[derive(Clone)]
pub struct CloudwatchomniAccessGrant(std::rc::Rc<CloudwatchomniAccessGrant_>);

impl CloudwatchomniAccessGrant {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal`.\n"]
    pub fn set_principal(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchomniAccessGrant_Principal>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().principal = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.principal = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scoped_action`.\n"]
    pub fn set_scoped_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scoped_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scoped_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_id` after provisioning.\n"]
    pub fn grant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_type` after provisioning.\n"]
    pub fn grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_id` after provisioning.\n"]
    pub fn space_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scoped_action` after provisioning.\n"]
    pub fn scoped_action(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_ScopedActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoped_action", self.extract_ref()))
    }
}

impl terrars::Referable for CloudwatchomniAccessGrant {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CloudwatchomniAccessGrant { }

impl terrars::ToListMappable for CloudwatchomniAccessGrant {
    type O = terrars::ListRef<CloudwatchomniAccessGrantRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CloudwatchomniAccessGrant_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_cloudwatchomni_access_grant".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCloudwatchomniAccessGrant {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub domain_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub permission: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub space_id: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchomniAccessGrant {
    pub fn build(self, stack: &mut terrars::Stack) -> CloudwatchomniAccessGrant {
        let out = CloudwatchomniAccessGrant(std::rc::Rc::new(CloudwatchomniAccessGrant_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CloudwatchomniAccessGrantData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                domain_id: self.domain_id,
                name: self.name,
                permission: self.permission,
                region: core::default::Default::default(),
                space_id: self.space_id,
                tags: core::default::Default::default(),
                principal: core::default::Default::default(),
                scoped_action: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CloudwatchomniAccessGrantRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrantRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CloudwatchomniAccessGrantRef {
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

    #[doc = "Get a reference to the value of field `domain_id` after provisioning.\n"]
    pub fn domain_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_id` after provisioning.\n"]
    pub fn grant_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `grant_type` after provisioning.\n"]
    pub fn grant_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.grant_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `permission` after provisioning.\n"]
    pub fn permission(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.permission", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `space_id` after provisioning.\n"]
    pub fn space_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.space_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `principal` after provisioning.\n"]
    pub fn principal(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_PrincipalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scoped_action` after provisioning.\n"]
    pub fn scoped_action(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_ScopedActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scoped_action", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniAccessGrantDynamic {
    principal: std::option::Option<terrars::DynamicBlock<CloudwatchomniAccessGrant_Principal>>,
    scoped_action: std::option::Option<terrars::DynamicBlock<CloudwatchomniAccessGrant_ScopedAction>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniAccessGrant_Principal_PrincipalAttribute {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl CloudwatchomniAccessGrant_Principal_PrincipalAttribute { }

impl terrars::ToListMappable for CloudwatchomniAccessGrant_Principal_PrincipalAttribute {
    type O = terrars::BlockAssignable<CloudwatchomniAccessGrant_Principal_PrincipalAttribute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniAccessGrant_Principal_PrincipalAttribute {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchomniAccessGrant_Principal_PrincipalAttribute {
    pub fn build(self) -> CloudwatchomniAccessGrant_Principal_PrincipalAttribute {
        CloudwatchomniAccessGrant_Principal_PrincipalAttribute {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniAccessGrant_Principal_PrincipalAttributeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrant_Principal_PrincipalAttributeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchomniAccessGrant_Principal_PrincipalAttributeRef {
        CloudwatchomniAccessGrant_Principal_PrincipalAttributeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniAccessGrant_Principal_PrincipalAttributeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniAccessGrant_PrincipalDynamic {
    principal_attribute: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchomniAccessGrant_Principal_PrincipalAttribute>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniAccessGrant_Principal {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_id: std::option::Option<terrars::PrimField<std::string::String>>,
    principal_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    principal_attribute: std::option::Option<std::vec::Vec<CloudwatchomniAccessGrant_Principal_PrincipalAttribute>>,
    dynamic: CloudwatchomniAccessGrant_PrincipalDynamic,
}

impl CloudwatchomniAccessGrant_Principal {
    #[doc = "Set the field `principal_id`.\n"]
    pub fn set_principal_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.principal_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `principal_attribute`.\n"]
    pub fn set_principal_attribute(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchomniAccessGrant_Principal_PrincipalAttribute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.principal_attribute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.principal_attribute = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchomniAccessGrant_Principal {
    type O = terrars::BlockAssignable<CloudwatchomniAccessGrant_Principal>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniAccessGrant_Principal {
    #[doc = ""]
    pub principal_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchomniAccessGrant_Principal {
    pub fn build(self) -> CloudwatchomniAccessGrant_Principal {
        CloudwatchomniAccessGrant_Principal {
            principal_id: core::default::Default::default(),
            principal_type: self.principal_type,
            principal_attribute: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniAccessGrant_PrincipalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrant_PrincipalRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchomniAccessGrant_PrincipalRef {
        CloudwatchomniAccessGrant_PrincipalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniAccessGrant_PrincipalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `principal_id` after provisioning.\n"]
    pub fn principal_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_id", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_type` after provisioning.\n"]
    pub fn principal_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.principal_type", self.base))
    }

    #[doc = "Get a reference to the value of field `principal_attribute` after provisioning.\n"]
    pub fn principal_attribute(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_Principal_PrincipalAttributeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.principal_attribute", self.base))
    }
}

#[derive(Serialize)]
pub struct CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope {
    field: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope { }

impl terrars::ToListMappable for CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope {
    type O = terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope {
    #[doc = ""]
    pub field: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope {
    pub fn build(self) -> CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope {
        CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope {
            field: self.field,
            operator: self.operator,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScopeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScopeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScopeRef {
        CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScopeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScopeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupDynamic {
    row_scope: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_scope: std::option::Option<std::vec::Vec<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope>>,
    dynamic: CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupDynamic,
}

impl CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {
    #[doc = "Set the field `row_scope`.\n"]
    pub fn set_row_scope(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScope>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.row_scope = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.row_scope = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {
    type O = terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {}

impl BuildCloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {
    pub fn build(self) -> CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {
        CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup {
            row_scope: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupRef {
        CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `row_scope` after provisioning.\n"]
    pub fn row_scope(
        &self,
    ) -> terrars::ListRef<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup_RowScopeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_scope", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniAccessGrant_ScopedAction_ResourceDynamic {
    row_scope_group: std
    ::option
    ::Option<terrars::DynamicBlock<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniAccessGrant_ScopedAction_Resource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    resource_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    signal_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    row_scope_group: std::option::Option<std::vec::Vec<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup>>,
    dynamic: CloudwatchomniAccessGrant_ScopedAction_ResourceDynamic,
}

impl CloudwatchomniAccessGrant_ScopedAction_Resource {
    #[doc = "Set the field `resource_arns`.\n"]
    pub fn set_resource_arns(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `signal_types`.\n"]
    pub fn set_signal_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.signal_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `row_scope_group`.\n"]
    pub fn set_row_scope_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.row_scope_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.row_scope_group = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CloudwatchomniAccessGrant_ScopedAction_Resource {
    type O = terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction_Resource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniAccessGrant_ScopedAction_Resource {
    #[doc = ""]
    pub resource_type: terrars::PrimField<std::string::String>,
}

impl BuildCloudwatchomniAccessGrant_ScopedAction_Resource {
    pub fn build(self) -> CloudwatchomniAccessGrant_ScopedAction_Resource {
        CloudwatchomniAccessGrant_ScopedAction_Resource {
            resource_arns: core::default::Default::default(),
            resource_type: self.resource_type,
            signal_types: core::default::Default::default(),
            tags: core::default::Default::default(),
            row_scope_group: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniAccessGrant_ScopedAction_ResourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrant_ScopedAction_ResourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CloudwatchomniAccessGrant_ScopedAction_ResourceRef {
        CloudwatchomniAccessGrant_ScopedAction_ResourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniAccessGrant_ScopedAction_ResourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arns` after provisioning.\n"]
    pub fn resource_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_arns", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `signal_types` after provisioning.\n"]
    pub fn signal_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.signal_types", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `row_scope_group` after provisioning.\n"]
    pub fn row_scope_group(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_ScopedAction_Resource_RowScopeGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.row_scope_group", self.base))
    }
}

#[derive(Serialize, Default)]
struct CloudwatchomniAccessGrant_ScopedActionDynamic {
    resource: std::option::Option<terrars::DynamicBlock<CloudwatchomniAccessGrant_ScopedAction_Resource>>,
}

#[derive(Serialize)]
pub struct CloudwatchomniAccessGrant_ScopedAction {
    actions: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource: std::option::Option<std::vec::Vec<CloudwatchomniAccessGrant_ScopedAction_Resource>>,
    dynamic: CloudwatchomniAccessGrant_ScopedActionDynamic,
}

impl CloudwatchomniAccessGrant_ScopedAction {
    #[doc = "Set the field `resource`.\n"]
    pub fn set_resource(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction_Resource>>,
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

impl terrars::ToListMappable for CloudwatchomniAccessGrant_ScopedAction {
    type O = terrars::BlockAssignable<CloudwatchomniAccessGrant_ScopedAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCloudwatchomniAccessGrant_ScopedAction {
    #[doc = ""]
    pub actions: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCloudwatchomniAccessGrant_ScopedAction {
    pub fn build(self) -> CloudwatchomniAccessGrant_ScopedAction {
        CloudwatchomniAccessGrant_ScopedAction {
            actions: self.actions,
            resource: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CloudwatchomniAccessGrant_ScopedActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CloudwatchomniAccessGrant_ScopedActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CloudwatchomniAccessGrant_ScopedActionRef {
        CloudwatchomniAccessGrant_ScopedActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CloudwatchomniAccessGrant_ScopedActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions` after provisioning.\n"]
    pub fn actions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.actions", self.base))
    }

    #[doc = "Get a reference to the value of field `resource` after provisioning.\n"]
    pub fn resource(&self) -> terrars::ListRef<CloudwatchomniAccessGrant_ScopedAction_ResourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource", self.base))
    }
}
