#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct RolesanywhereTrustAnchorData_ {
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
    notification_settings: std::option::Option<std::vec::Vec<RolesanywhereTrustAnchor_NotificationSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<RolesanywhereTrustAnchor_Source>>,
    dynamic: RolesanywhereTrustAnchorDynamic,
}

struct RolesanywhereTrustAnchor_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<RolesanywhereTrustAnchorData_>,
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

    #[doc = "Set the field `notification_settings`.\n"]
    pub fn set_notification_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RolesanywhereTrustAnchor_NotificationSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<RolesanywhereTrustAnchor_Source>>,
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
    pub fn source(&self) -> terrars::ListRef<RolesanywhereTrustAnchor_SourceRef> {
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
            data: std::cell::RefCell::new(RolesanywhereTrustAnchorData_ {
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
                notification_settings: core::default::Default::default(),
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
    pub fn source(&self) -> terrars::ListRef<RolesanywhereTrustAnchor_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct RolesanywhereTrustAnchorDynamic {
    notification_settings: std::option::Option<terrars::DynamicBlock<RolesanywhereTrustAnchor_NotificationSettings>>,
    source: std::option::Option<terrars::DynamicBlock<RolesanywhereTrustAnchor_Source>>,
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchor_NotificationSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    channel: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold: std::option::Option<terrars::PrimField<f64>>,
}

impl RolesanywhereTrustAnchor_NotificationSettings {
    #[doc = "Set the field `channel`.\n"]
    pub fn set_channel(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.channel = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `event`.\n"]
    pub fn set_event(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.event = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threshold = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for RolesanywhereTrustAnchor_NotificationSettings {
    type O = terrars::BlockAssignable<RolesanywhereTrustAnchor_NotificationSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchor_NotificationSettings {}

impl BuildRolesanywhereTrustAnchor_NotificationSettings {
    pub fn build(self) -> RolesanywhereTrustAnchor_NotificationSettings {
        RolesanywhereTrustAnchor_NotificationSettings {
            channel: core::default::Default::default(),
            enabled: core::default::Default::default(),
            event: core::default::Default::default(),
            threshold: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RolesanywhereTrustAnchor_NotificationSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RolesanywhereTrustAnchor_NotificationSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> RolesanywhereTrustAnchor_NotificationSettingsRef {
        RolesanywhereTrustAnchor_NotificationSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchor_NotificationSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `channel` after provisioning.\n"]
    pub fn channel(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.channel", self.base))
    }

    #[doc = "Get a reference to the value of field `configured_by` after provisioning.\n"]
    pub fn configured_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configured_by", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `event` after provisioning.\n"]
    pub fn event(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event", self.base))
    }

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.base))
    }
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchor_Source_SourceData {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acm_pca_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    x509_certificate_data: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl RolesanywhereTrustAnchor_Source_SourceData {
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

impl terrars::ToListMappable for RolesanywhereTrustAnchor_Source_SourceData {
    type O = terrars::BlockAssignable<RolesanywhereTrustAnchor_Source_SourceData>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchor_Source_SourceData {}

impl BuildRolesanywhereTrustAnchor_Source_SourceData {
    pub fn build(self) -> RolesanywhereTrustAnchor_Source_SourceData {
        RolesanywhereTrustAnchor_Source_SourceData {
            acm_pca_arn: core::default::Default::default(),
            x509_certificate_data: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RolesanywhereTrustAnchor_Source_SourceDataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RolesanywhereTrustAnchor_Source_SourceDataRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RolesanywhereTrustAnchor_Source_SourceDataRef {
        RolesanywhereTrustAnchor_Source_SourceDataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchor_Source_SourceDataRef {
    #[allow(dead_code)]
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
struct RolesanywhereTrustAnchor_SourceDynamic {
    source_data: std::option::Option<terrars::DynamicBlock<RolesanywhereTrustAnchor_Source_SourceData>>,
}

#[derive(Serialize)]
pub struct RolesanywhereTrustAnchor_Source {
    source_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_data: std::option::Option<std::vec::Vec<RolesanywhereTrustAnchor_Source_SourceData>>,
    dynamic: RolesanywhereTrustAnchor_SourceDynamic,
}

impl RolesanywhereTrustAnchor_Source {
    #[doc = "Set the field `source_data`.\n"]
    pub fn set_source_data(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<RolesanywhereTrustAnchor_Source_SourceData>>,
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

impl terrars::ToListMappable for RolesanywhereTrustAnchor_Source {
    type O = terrars::BlockAssignable<RolesanywhereTrustAnchor_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildRolesanywhereTrustAnchor_Source {
    #[doc = ""]
    pub source_type: terrars::PrimField<std::string::String>,
}

impl BuildRolesanywhereTrustAnchor_Source {
    pub fn build(self) -> RolesanywhereTrustAnchor_Source {
        RolesanywhereTrustAnchor_Source {
            source_type: self.source_type,
            source_data: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct RolesanywhereTrustAnchor_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for RolesanywhereTrustAnchor_SourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> RolesanywhereTrustAnchor_SourceRef {
        RolesanywhereTrustAnchor_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl RolesanywhereTrustAnchor_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_data` after provisioning.\n"]
    pub fn source_data(&self) -> terrars::ListRef<RolesanywhereTrustAnchor_Source_SourceDataRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_data", self.base))
    }
}
