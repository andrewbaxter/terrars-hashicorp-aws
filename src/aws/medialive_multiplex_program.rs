#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MedialiveMultiplexProgramData_ {
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
    multiplex_id: terrars::PrimField<std::string::String>,
    program_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    multiplex_program_settings: std::option::Option<std::vec::Vec<MedialiveMultiplexProgram_MultiplexProgramSettings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<MedialiveMultiplexProgram_Timeouts>,
    dynamic: MedialiveMultiplexProgramDynamic,
}

struct MedialiveMultiplexProgram_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MedialiveMultiplexProgramData_>,
}

#[derive(Clone)]
pub struct MedialiveMultiplexProgram(std::rc::Rc<MedialiveMultiplexProgram_>);

impl MedialiveMultiplexProgram {
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

    #[doc = "Set the field `multiplex_program_settings`.\n"]
    pub fn set_multiplex_program_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().multiplex_program_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.multiplex_program_settings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<MedialiveMultiplexProgram_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multiplex_id` after provisioning.\n"]
    pub fn multiplex_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiplex_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `program_name` after provisioning.\n"]
    pub fn program_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multiplex_program_settings` after provisioning.\n"]
    pub fn multiplex_program_settings(&self) -> terrars::ListRef<MedialiveMultiplexProgram_MultiplexProgramSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_program_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveMultiplexProgram_TimeoutsRef {
        MedialiveMultiplexProgram_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for MedialiveMultiplexProgram {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MedialiveMultiplexProgram { }

impl terrars::ToListMappable for MedialiveMultiplexProgram {
    type O = terrars::ListRef<MedialiveMultiplexProgramRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MedialiveMultiplexProgram_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_medialive_multiplex_program".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMedialiveMultiplexProgram {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub multiplex_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub program_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveMultiplexProgram {
    pub fn build(self, stack: &mut terrars::Stack) -> MedialiveMultiplexProgram {
        let out = MedialiveMultiplexProgram(std::rc::Rc::new(MedialiveMultiplexProgram_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MedialiveMultiplexProgramData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                multiplex_id: self.multiplex_id,
                program_name: self.program_name,
                region: core::default::Default::default(),
                multiplex_program_settings: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MedialiveMultiplexProgramRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgramRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MedialiveMultiplexProgramRef {
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

    #[doc = "Get a reference to the value of field `multiplex_id` after provisioning.\n"]
    pub fn multiplex_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multiplex_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `program_name` after provisioning.\n"]
    pub fn program_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multiplex_program_settings` after provisioning.\n"]
    pub fn multiplex_program_settings(&self) -> terrars::ListRef<MedialiveMultiplexProgram_MultiplexProgramSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_program_settings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> MedialiveMultiplexProgram_TimeoutsRef {
        MedialiveMultiplexProgram_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MedialiveMultiplexProgramDynamic {
    multiplex_program_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveMultiplexProgram_MultiplexProgramSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor {
    provider_name: terrars::PrimField<std::string::String>,
    service_name: terrars::PrimField<std::string::String>,
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor { }

impl terrars::ToListMappable for MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor {
    #[doc = ""]
    pub provider_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor {
    pub fn build(self) -> MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor {
        MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor {
            provider_name: self.provider_name,
            service_name: self.service_name,
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptorRef {
        MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `provider_name` after provisioning.\n"]
    pub fn provider_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provider_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {
    #[doc = "Set the field `maximum_bitrate`.\n"]
    pub fn set_maximum_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_bitrate`.\n"]
    pub fn set_minimum_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {
    type O =
        terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {}

impl BuildMedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {
    pub fn build(self) -> MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {
        MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings {
            maximum_bitrate: core::default::Default::default(),
            minimum_bitrate: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettingsRef {
        MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_bitrate` after provisioning.\n"]
    pub fn maximum_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_bitrate` after provisioning.\n"]
    pub fn minimum_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsDynamic {
    statmux_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constant_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statmux_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings>>,
    dynamic: MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsDynamic,
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {
    #[doc = "Set the field `constant_bitrate`.\n"]
    pub fn set_constant_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constant_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statmux_settings`.\n"]
    pub fn set_statmux_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettings,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statmux_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statmux_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {}

impl BuildMedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {
    pub fn build(self) -> MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {
        MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings {
            constant_bitrate: core::default::Default::default(),
            statmux_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsRef {
        MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constant_bitrate` after provisioning.\n"]
    pub fn constant_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constant_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `statmux_settings` after provisioning.\n"]
    pub fn statmux_settings(
        &self,
    ) -> terrars::ListRef<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings_StatmuxSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statmux_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveMultiplexProgram_MultiplexProgramSettingsDynamic {
    service_descriptor: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor>>,
    video_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings>>,
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettings {
    preferred_channel_pipeline: terrars::PrimField<std::string::String>,
    program_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_descriptor: std
    ::option
    ::Option<std::vec::Vec<MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_settings: std::option::Option<std::vec::Vec<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings>>,
    dynamic: MedialiveMultiplexProgram_MultiplexProgramSettingsDynamic,
}

impl MedialiveMultiplexProgram_MultiplexProgramSettings {
    #[doc = "Set the field `service_descriptor`.\n"]
    pub fn set_service_descriptor(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptor>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.service_descriptor = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.service_descriptor = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `video_settings`.\n"]
    pub fn set_video_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettings>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.video_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.video_settings = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MedialiveMultiplexProgram_MultiplexProgramSettings {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgram_MultiplexProgramSettings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgram_MultiplexProgramSettings {
    #[doc = ""]
    pub preferred_channel_pipeline: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub program_number: terrars::PrimField<f64>,
}

impl BuildMedialiveMultiplexProgram_MultiplexProgramSettings {
    pub fn build(self) -> MedialiveMultiplexProgram_MultiplexProgramSettings {
        MedialiveMultiplexProgram_MultiplexProgramSettings {
            preferred_channel_pipeline: self.preferred_channel_pipeline,
            program_number: self.program_number,
            service_descriptor: core::default::Default::default(),
            video_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MedialiveMultiplexProgram_MultiplexProgramSettingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgram_MultiplexProgramSettingsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgram_MultiplexProgramSettingsRef {
        MedialiveMultiplexProgram_MultiplexProgramSettingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgram_MultiplexProgramSettingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `preferred_channel_pipeline` after provisioning.\n"]
    pub fn preferred_channel_pipeline(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.preferred_channel_pipeline", self.base))
    }

    #[doc = "Get a reference to the value of field `program_number` after provisioning.\n"]
    pub fn program_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.program_number", self.base))
    }

    #[doc = "Get a reference to the value of field `service_descriptor` after provisioning.\n"]
    pub fn service_descriptor(
        &self,
    ) -> terrars::ListRef<MedialiveMultiplexProgram_MultiplexProgramSettings_ServiceDescriptorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_descriptor", self.base))
    }

    #[doc = "Get a reference to the value of field `video_settings` after provisioning.\n"]
    pub fn video_settings(
        &self,
    ) -> terrars::ListRef<MedialiveMultiplexProgram_MultiplexProgramSettings_VideoSettingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_settings", self.base))
    }
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgram_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MedialiveMultiplexProgram_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MedialiveMultiplexProgram_Timeouts {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgram_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgram_Timeouts {}

impl BuildMedialiveMultiplexProgram_Timeouts {
    pub fn build(self) -> MedialiveMultiplexProgram_Timeouts {
        MedialiveMultiplexProgram_Timeouts { create: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MedialiveMultiplexProgram_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgram_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MedialiveMultiplexProgram_TimeoutsRef {
        MedialiveMultiplexProgram_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgram_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }
}
