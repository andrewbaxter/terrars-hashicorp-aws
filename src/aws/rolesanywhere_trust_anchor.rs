use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RolesanywhereTrustAnchorData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<RolesanywhereTrustAnchorSourceEl>>,
    dynamic: RolesanywhereTrustAnchorDynamic,
}

struct RolesanywhereTrustAnchor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RolesanywhereTrustAnchorData>,
}

#[derive(Clone)]
pub struct RolesanywhereTrustAnchor(std::rc::Rc<RolesanywhereTrustAnchor_>);

impl RolesanywhereTrustAnchor {
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

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RolesanywhereTrustAnchorSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<RolesanywhereTrustAnchorSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for RolesanywhereTrustAnchor {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for RolesanywhereTrustAnchor { }

impl terrars::ToListMappable for RolesanywhereTrustAnchor {
    type O = terrars::ListRef<RolesanywhereTrustAnchorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for RolesanywhereTrustAnchor_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_rolesanywhere_trust_anchor".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildRolesanywhereTrustAnchor {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildRolesanywhereTrustAnchor {
    pub fn build(self, stack: &mut terrars::Stack) -> RolesanywhereTrustAnchor {
        let out = RolesanywhereTrustAnchor(std::rc::Rc::new(RolesanywhereTrustAnchor_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(RolesanywhereTrustAnchorData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct RolesanywhereTrustAnchorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RolesanywhereTrustAnchorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl RolesanywhereTrustAnchorRef {
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

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<RolesanywhereTrustAnchorSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchorSourceElSourceDataEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm_pca_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_certificate_data: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RolesanywhereTrustAnchorSourceElSourceDataEl {
    #[doc = "Set the field `acm_pca_arn`.\n"]
    pub fn set_acm_pca_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.acm_pca_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `x509_certificate_data`.\n"]
    pub fn set_x509_certificate_data(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.x509_certificate_data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RolesanywhereTrustAnchorSourceElSourceDataEl {
    type O = terrars::BlockAssignable<RolesanywhereTrustAnchorSourceElSourceDataEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchorSourceElSourceDataEl {}

impl BuildRolesanywhereTrustAnchorSourceElSourceDataEl {
    pub fn build(self) -> RolesanywhereTrustAnchorSourceElSourceDataEl {
        RolesanywhereTrustAnchorSourceElSourceDataEl {
            acm_pca_arn: core::default::Default::default(),
            x509_certificate_data: core::default::Default::default(),
        }
    }
}

pub struct RolesanywhereTrustAnchorSourceElSourceDataElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RolesanywhereTrustAnchorSourceElSourceDataElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RolesanywhereTrustAnchorSourceElSourceDataElRef {
        RolesanywhereTrustAnchorSourceElSourceDataElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchorSourceElSourceDataElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acm_pca_arn` after provisioning.\n"]
    pub fn acm_pca_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.acm_pca_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `x509_certificate_data` after provisioning.\n"]
    pub fn x509_certificate_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.x509_certificate_data", self.base))
    }
}

#[derive(Serialize, Default)]
struct RolesanywhereTrustAnchorSourceElDynamic {
    source_data: std::option::Option<terrars::DynamicBlock<RolesanywhereTrustAnchorSourceElSourceDataEl>>,
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchorSourceEl {
    source_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_data: std::option::Option<std::vec::Vec<RolesanywhereTrustAnchorSourceElSourceDataEl>>,
    dynamic: RolesanywhereTrustAnchorSourceElDynamic,
}

impl RolesanywhereTrustAnchorSourceEl {
    #[doc = "Set the field `source_data`.\n"]
    pub fn set_source_data(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RolesanywhereTrustAnchorSourceElSourceDataEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_data = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_data = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for RolesanywhereTrustAnchorSourceEl {
    type O = terrars::BlockAssignable<RolesanywhereTrustAnchorSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchorSourceEl {
    #[doc = ""]
    pub source_type: terrars::PrimField<std::string::String>,
}

impl BuildRolesanywhereTrustAnchorSourceEl {
    pub fn build(self) -> RolesanywhereTrustAnchorSourceEl {
        RolesanywhereTrustAnchorSourceEl {
            source_type: self.source_type,
            source_data: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct RolesanywhereTrustAnchorSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RolesanywhereTrustAnchorSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RolesanywhereTrustAnchorSourceElRef {
        RolesanywhereTrustAnchorSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchorSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_data` after provisioning.\n"]
    pub fn source_data(&self) -> terrars::ListRef<RolesanywhereTrustAnchorSourceElSourceDataElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_data", self.base))
    }
}

#[derive(Serialize, Default)]
struct RolesanywhereTrustAnchorDynamic {
    source: std::option::Option<terrars::DynamicBlock<RolesanywhereTrustAnchorSourceEl>>,
}
