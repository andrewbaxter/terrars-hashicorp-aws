#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ChimesdkvoiceGlobalSettingsData_ {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    voice_connector: std::option::Option<std::vec::Vec<ChimesdkvoiceGlobalSettings_VoiceConnector>>,
    dynamic: ChimesdkvoiceGlobalSettingsDynamic,
}

struct ChimesdkvoiceGlobalSettings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ChimesdkvoiceGlobalSettingsData_>,
}

#[derive(Clone)]
pub struct ChimesdkvoiceGlobalSettings(std::rc::Rc<ChimesdkvoiceGlobalSettings_>);

impl ChimesdkvoiceGlobalSettings {
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

    #[doc = "Set the field `voice_connector`.\n"]
    pub fn set_voice_connector(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ChimesdkvoiceGlobalSettings_VoiceConnector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().voice_connector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.voice_connector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `voice_connector` after provisioning.\n"]
    pub fn voice_connector(&self) -> terrars::ListRef<ChimesdkvoiceGlobalSettings_VoiceConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.voice_connector", self.extract_ref()))
    }
}

impl terrars::Referable for ChimesdkvoiceGlobalSettings {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ChimesdkvoiceGlobalSettings { }

impl terrars::ToListMappable for ChimesdkvoiceGlobalSettings {
    type O = terrars::ListRef<ChimesdkvoiceGlobalSettingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ChimesdkvoiceGlobalSettings_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_chimesdkvoice_global_settings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildChimesdkvoiceGlobalSettings {
    pub tf_id: std::string::String,
}

impl BuildChimesdkvoiceGlobalSettings {
    pub fn build(self, stack: &mut terrars::Stack) -> ChimesdkvoiceGlobalSettings {
        let out = ChimesdkvoiceGlobalSettings(std::rc::Rc::new(ChimesdkvoiceGlobalSettings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ChimesdkvoiceGlobalSettingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                voice_connector: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ChimesdkvoiceGlobalSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkvoiceGlobalSettingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ChimesdkvoiceGlobalSettingsRef {
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

    #[doc = "Get a reference to the value of field `voice_connector` after provisioning.\n"]
    pub fn voice_connector(&self) -> terrars::ListRef<ChimesdkvoiceGlobalSettings_VoiceConnectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.voice_connector", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct ChimesdkvoiceGlobalSettingsDynamic {
    voice_connector: std::option::Option<terrars::DynamicBlock<ChimesdkvoiceGlobalSettings_VoiceConnector>>,
}

#[derive(Serialize)]
pub struct ChimesdkvoiceGlobalSettings_VoiceConnector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cdr_bucket: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ChimesdkvoiceGlobalSettings_VoiceConnector {
    #[doc = "Set the field `cdr_bucket`.\n"]
    pub fn set_cdr_bucket(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cdr_bucket = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ChimesdkvoiceGlobalSettings_VoiceConnector {
    type O = terrars::BlockAssignable<ChimesdkvoiceGlobalSettings_VoiceConnector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildChimesdkvoiceGlobalSettings_VoiceConnector {}

impl BuildChimesdkvoiceGlobalSettings_VoiceConnector {
    pub fn build(self) -> ChimesdkvoiceGlobalSettings_VoiceConnector {
        ChimesdkvoiceGlobalSettings_VoiceConnector { cdr_bucket: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct ChimesdkvoiceGlobalSettings_VoiceConnectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ChimesdkvoiceGlobalSettings_VoiceConnectorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ChimesdkvoiceGlobalSettings_VoiceConnectorRef {
        ChimesdkvoiceGlobalSettings_VoiceConnectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ChimesdkvoiceGlobalSettings_VoiceConnectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cdr_bucket` after provisioning.\n"]
    pub fn cdr_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cdr_bucket", self.base))
    }
}
