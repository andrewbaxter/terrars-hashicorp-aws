use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsoadminInstanceAccessControlAttributesData {
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
    instance_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<std::vec::Vec<SsoadminInstanceAccessControlAttributesAttributeEl>>,
    dynamic: SsoadminInstanceAccessControlAttributesDynamic,
}

struct SsoadminInstanceAccessControlAttributes_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsoadminInstanceAccessControlAttributesData>,
}

#[derive(Clone)]
pub struct SsoadminInstanceAccessControlAttributes(std::rc::Rc<SsoadminInstanceAccessControlAttributes_>);

impl SsoadminInstanceAccessControlAttributes {
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

    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().attribute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.attribute = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }
}

impl terrars::Referable for SsoadminInstanceAccessControlAttributes {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsoadminInstanceAccessControlAttributes { }

impl terrars::ToListMappable for SsoadminInstanceAccessControlAttributes {
    type O = terrars::ListRef<SsoadminInstanceAccessControlAttributesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsoadminInstanceAccessControlAttributes_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssoadmin_instance_access_control_attributes".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsoadminInstanceAccessControlAttributes {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_arn: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminInstanceAccessControlAttributes {
    pub fn build(self, stack: &mut terrars::Stack) -> SsoadminInstanceAccessControlAttributes {
        let out = SsoadminInstanceAccessControlAttributes(std::rc::Rc::new(SsoadminInstanceAccessControlAttributes_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsoadminInstanceAccessControlAttributesData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_arn: self.instance_arn,
                attribute: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsoadminInstanceAccessControlAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminInstanceAccessControlAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsoadminInstanceAccessControlAttributesRef {
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

    #[doc = "Get a reference to the value of field `instance_arn` after provisioning.\n"]
    pub fn instance_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_reason` after provisioning.\n"]
    pub fn status_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsoadminInstanceAccessControlAttributesAttributeElValueEl {
    source: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl SsoadminInstanceAccessControlAttributesAttributeElValueEl { }

impl terrars::ToListMappable for SsoadminInstanceAccessControlAttributesAttributeElValueEl {
    type O = terrars::BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeElValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminInstanceAccessControlAttributesAttributeElValueEl {
    #[doc = ""]
    pub source: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildSsoadminInstanceAccessControlAttributesAttributeElValueEl {
    pub fn build(self) -> SsoadminInstanceAccessControlAttributesAttributeElValueEl {
        SsoadminInstanceAccessControlAttributesAttributeElValueEl { source: self.source }
    }
}

pub struct SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
        SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminInstanceAccessControlAttributesAttributeElValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminInstanceAccessControlAttributesAttributeElDynamic {
    value: std::option::Option<terrars::DynamicBlock<SsoadminInstanceAccessControlAttributesAttributeElValueEl>>,
}

#[derive(Serialize)]
pub struct SsoadminInstanceAccessControlAttributesAttributeEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<std::vec::Vec<SsoadminInstanceAccessControlAttributesAttributeElValueEl>>,
    dynamic: SsoadminInstanceAccessControlAttributesAttributeElDynamic,
}

impl SsoadminInstanceAccessControlAttributesAttributeEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeElValueEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsoadminInstanceAccessControlAttributesAttributeEl {
    type O = terrars::BlockAssignable<SsoadminInstanceAccessControlAttributesAttributeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsoadminInstanceAccessControlAttributesAttributeEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildSsoadminInstanceAccessControlAttributesAttributeEl {
    pub fn build(self) -> SsoadminInstanceAccessControlAttributesAttributeEl {
        SsoadminInstanceAccessControlAttributesAttributeEl {
            key: self.key,
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SsoadminInstanceAccessControlAttributesAttributeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsoadminInstanceAccessControlAttributesAttributeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsoadminInstanceAccessControlAttributesAttributeElRef {
        SsoadminInstanceAccessControlAttributesAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsoadminInstanceAccessControlAttributesAttributeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsoadminInstanceAccessControlAttributesDynamic {
    attribute: std::option::Option<terrars::DynamicBlock<SsoadminInstanceAccessControlAttributesAttributeEl>>,
}
