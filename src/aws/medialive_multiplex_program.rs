use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MedialiveMultiplexProgramData {
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
    multiplex_program_settings: std::option::Option<std::vec::Vec<MedialiveMultiplexProgramMultiplexProgramSettingsEl>>,
    dynamic: MedialiveMultiplexProgramDynamic,
}

struct MedialiveMultiplexProgram_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MedialiveMultiplexProgramData>,
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

    #[doc = "Set the field `multiplex_program_settings`.\n"]
    pub fn set_multiplex_program_settings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsEl>>,
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

    #[doc = "Get a reference to the value of field `multiplex_program_settings` after provisioning.\n"]
    pub fn multiplex_program_settings(&self) -> terrars::ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_program_settings", self.extract_ref()))
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
            data: std::cell::RefCell::new(MedialiveMultiplexProgramData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                multiplex_id: self.multiplex_id,
                program_name: self.program_name,
                multiplex_program_settings: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `multiplex_program_settings` after provisioning.\n"]
    pub fn multiplex_program_settings(&self) -> terrars::ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.multiplex_program_settings", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    provider_name: terrars::PrimField<std::string::String>,
    service_name: terrars::PrimField<std::string::String>,
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl { }

impl terrars::ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    #[doc = ""]
    pub provider_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl {
            provider_name: self.provider_name,
            service_name: self.service_name,
        }
    }
}

pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef {
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
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {
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

impl terrars::ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {}

impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl {
            maximum_bitrate: core::default::Default::default(),
            minimum_bitrate: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsElRef {
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

#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
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

impl terrars::ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
    type O =
        terrars
        ::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {}

impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl {
            maximum_bitrate: core::default::Default::default(),
            minimum_bitrate: core::default::Default::default(),
            priority: core::default::Default::default(),
        }
    }
}

pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef {
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
struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElDynamic {
    statemux_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl>,
    >,
    statmux_settings: std
    ::option
    ::Option<
        terrars::DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl>,
    >,
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    constant_bitrate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statemux_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    statmux_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl>>,
    dynamic: MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElDynamic,
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    #[doc = "Set the field `constant_bitrate`.\n"]
    pub fn set_constant_bitrate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.constant_bitrate = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `statemux_settings`.\n"]
    pub fn set_statemux_settings(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.statemux_settings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.statemux_settings = std::option::Option::Some(d);
            },
        }
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
                            MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsEl,
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

impl terrars::ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {}

impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl {
            constant_bitrate: core::default::Default::default(),
            statemux_settings: core::default::Default::default(),
            statmux_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `constant_bitrate` after provisioning.\n"]
    pub fn constant_bitrate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.constant_bitrate", self.base))
    }

    #[doc = "Get a reference to the value of field `statemux_settings` after provisioning.\n"]
    pub fn statemux_settings(
        &self,
    ) -> terrars::ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatemuxSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statemux_settings", self.base))
    }

    #[doc = "Get a reference to the value of field `statmux_settings` after provisioning.\n"]
    pub fn statmux_settings(
        &self,
    ) -> terrars::ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElStatmuxSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.statmux_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveMultiplexProgramMultiplexProgramSettingsElDynamic {
    service_descriptor: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>>,
    video_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>>,
}

#[derive(Serialize)]
pub struct MedialiveMultiplexProgramMultiplexProgramSettingsEl {
    preferred_channel_pipeline: terrars::PrimField<std::string::String>,
    program_number: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_descriptor: std
    ::option
    ::Option<std::vec::Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    video_settings: std
    ::option
    ::Option<std::vec::Vec<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>>,
    dynamic: MedialiveMultiplexProgramMultiplexProgramSettingsElDynamic,
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsEl {
    #[doc = "Set the field `service_descriptor`.\n"]
    pub fn set_service_descriptor(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorEl>,
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
                        terrars::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsEl>,
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

impl terrars::ToListMappable for MedialiveMultiplexProgramMultiplexProgramSettingsEl {
    type O = terrars::BlockAssignable<MedialiveMultiplexProgramMultiplexProgramSettingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMedialiveMultiplexProgramMultiplexProgramSettingsEl {
    #[doc = ""]
    pub preferred_channel_pipeline: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub program_number: terrars::PrimField<f64>,
}

impl BuildMedialiveMultiplexProgramMultiplexProgramSettingsEl {
    pub fn build(self) -> MedialiveMultiplexProgramMultiplexProgramSettingsEl {
        MedialiveMultiplexProgramMultiplexProgramSettingsEl {
            preferred_channel_pipeline: self.preferred_channel_pipeline,
            program_number: self.program_number,
            service_descriptor: core::default::Default::default(),
            video_settings: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
        MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MedialiveMultiplexProgramMultiplexProgramSettingsElRef {
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
    ) -> terrars::ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElServiceDescriptorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_descriptor", self.base))
    }

    #[doc = "Get a reference to the value of field `video_settings` after provisioning.\n"]
    pub fn video_settings(
        &self,
    ) -> terrars::ListRef<MedialiveMultiplexProgramMultiplexProgramSettingsElVideoSettingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.video_settings", self.base))
    }
}

#[derive(Serialize, Default)]
struct MedialiveMultiplexProgramDynamic {
    multiplex_program_settings: std
    ::option
    ::Option<terrars::DynamicBlock<MedialiveMultiplexProgramMultiplexProgramSettingsEl>>,
}
