use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrserverlessApplicationData {
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
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    release_label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_start_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplicationAutoStartConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_stop_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplicationAutoStopConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_capacity: std::option::Option<std::vec::Vec<EmrserverlessApplicationInitialCapacityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_capacity: std::option::Option<std::vec::Vec<EmrserverlessApplicationMaximumCapacityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<EmrserverlessApplicationNetworkConfigurationEl>>,
    dynamic: EmrserverlessApplicationDynamic,
}

struct EmrserverlessApplication_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrserverlessApplicationData>,
}

#[derive(Clone)]
pub struct EmrserverlessApplication(std::rc::Rc<EmrserverlessApplication_>);

impl EmrserverlessApplication {
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

    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().architecture = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `auto_start_configuration`.\n"]
    pub fn set_auto_start_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplicationAutoStartConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_start_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_start_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_stop_configuration`.\n"]
    pub fn set_auto_stop_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplicationAutoStopConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_stop_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_stop_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `initial_capacity`.\n"]
    pub fn set_initial_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplicationInitialCapacityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maximum_capacity`.\n"]
    pub fn set_maximum_capacity(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplicationMaximumCapacityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maximum_capacity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maximum_capacity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrserverlessApplicationNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> terrars::ListRef<EmrserverlessApplicationAutoStartConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> terrars::ListRef<EmrserverlessApplicationAutoStopConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> terrars::ListRef<EmrserverlessApplicationMaximumCapacityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EmrserverlessApplicationNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for EmrserverlessApplication {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrserverlessApplication { }

impl terrars::ToListMappable for EmrserverlessApplication {
    type O = terrars::ListRef<EmrserverlessApplicationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrserverlessApplication_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emrserverless_application".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrserverlessApplication {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub release_label: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplication {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrserverlessApplication {
        let out = EmrserverlessApplication(std::rc::Rc::new(EmrserverlessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrserverlessApplicationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                architecture: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                release_label: self.release_label,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: self.type_,
                auto_start_configuration: core::default::Default::default(),
                auto_stop_configuration: core::default::Default::default(),
                initial_capacity: core::default::Default::default(),
                maximum_capacity: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrserverlessApplicationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrserverlessApplicationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> terrars::ListRef<EmrserverlessApplicationAutoStartConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> terrars::ListRef<EmrserverlessApplicationAutoStopConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> terrars::ListRef<EmrserverlessApplicationMaximumCapacityElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EmrserverlessApplicationNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationAutoStartConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl EmrserverlessApplicationAutoStartConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationAutoStartConfigurationEl {
    type O = terrars::BlockAssignable<EmrserverlessApplicationAutoStartConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationAutoStartConfigurationEl {}

impl BuildEmrserverlessApplicationAutoStartConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationAutoStartConfigurationEl {
        EmrserverlessApplicationAutoStartConfigurationEl { enabled: core::default::Default::default() }
    }
}

pub struct EmrserverlessApplicationAutoStartConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationAutoStartConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplicationAutoStartConfigurationElRef {
        EmrserverlessApplicationAutoStartConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationAutoStartConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationAutoStopConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrserverlessApplicationAutoStopConfigurationEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_timeout_minutes`.\n"]
    pub fn set_idle_timeout_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationAutoStopConfigurationEl {
    type O = terrars::BlockAssignable<EmrserverlessApplicationAutoStopConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationAutoStopConfigurationEl {}

impl BuildEmrserverlessApplicationAutoStopConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationAutoStopConfigurationEl {
        EmrserverlessApplicationAutoStopConfigurationEl {
            enabled: core::default::Default::default(),
            idle_timeout_minutes: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationAutoStopConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationAutoStopConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplicationAutoStopConfigurationElRef {
        EmrserverlessApplicationAutoStopConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationAutoStopConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `idle_timeout_minutes` after provisioning.\n"]
    pub fn idle_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    cpu: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<std::string::String>>,
    memory: terrars::PrimField<std::string::String>,
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    #[doc = ""]
    pub cpu: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub memory: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
            cpu: self.cpu,
            disk: core::default::Default::default(),
            memory: self.memory,
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElDynamic {
    worker_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    worker_count: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    worker_configuration: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>>,
    dynamic: EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElDynamic,
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    #[doc = "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.worker_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    type O = terrars::BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    #[doc = ""]
    pub worker_count: terrars::PrimField<f64>,
}

impl BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
            worker_count: self.worker_count,
            worker_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `worker_count` after provisioning.\n"]
    pub fn worker_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.worker_count", self.base))
    }

    #[doc = "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationInitialCapacityElDynamic {
    initial_capacity_config: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityEl {
    initial_capacity_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    initial_capacity_config: std
    ::option
    ::Option<std::vec::Vec<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
    dynamic: EmrserverlessApplicationInitialCapacityElDynamic,
}

impl EmrserverlessApplicationInitialCapacityEl {
    #[doc = "Set the field `initial_capacity_config`.\n"]
    pub fn set_initial_capacity_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.initial_capacity_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_capacity_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationInitialCapacityEl {
    type O = terrars::BlockAssignable<EmrserverlessApplicationInitialCapacityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityEl {
    #[doc = ""]
    pub initial_capacity_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplicationInitialCapacityEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityEl {
        EmrserverlessApplicationInitialCapacityEl {
            initial_capacity_type: self.initial_capacity_type,
            initial_capacity_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationInitialCapacityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrserverlessApplicationInitialCapacityElRef {
        EmrserverlessApplicationInitialCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `initial_capacity_type` after provisioning.\n"]
    pub fn initial_capacity_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.initial_capacity_type", self.base))
    }

    #[doc = "Get a reference to the value of field `initial_capacity_config` after provisioning.\n"]
    pub fn initial_capacity_config(
        &self,
    ) -> terrars::ListRef<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.initial_capacity_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMaximumCapacityEl {
    cpu: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<std::string::String>>,
    memory: terrars::PrimField<std::string::String>,
}

impl EmrserverlessApplicationMaximumCapacityEl {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationMaximumCapacityEl {
    type O = terrars::BlockAssignable<EmrserverlessApplicationMaximumCapacityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMaximumCapacityEl {
    #[doc = ""]
    pub cpu: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub memory: terrars::PrimField<std::string::String>,
}

impl BuildEmrserverlessApplicationMaximumCapacityEl {
    pub fn build(self) -> EmrserverlessApplicationMaximumCapacityEl {
        EmrserverlessApplicationMaximumCapacityEl {
            cpu: self.cpu,
            disk: core::default::Default::default(),
            memory: self.memory,
        }
    }
}

pub struct EmrserverlessApplicationMaximumCapacityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationMaximumCapacityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrserverlessApplicationMaximumCapacityElRef {
        EmrserverlessApplicationMaximumCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMaximumCapacityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationNetworkConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EmrserverlessApplicationNetworkConfigurationEl {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrserverlessApplicationNetworkConfigurationEl {
    type O = terrars::BlockAssignable<EmrserverlessApplicationNetworkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationNetworkConfigurationEl {}

impl BuildEmrserverlessApplicationNetworkConfigurationEl {
    pub fn build(self) -> EmrserverlessApplicationNetworkConfigurationEl {
        EmrserverlessApplicationNetworkConfigurationEl {
            security_group_ids: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationNetworkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrserverlessApplicationNetworkConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrserverlessApplicationNetworkConfigurationElRef {
        EmrserverlessApplicationNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationNetworkConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationDynamic {
    auto_start_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplicationAutoStartConfigurationEl>>,
    auto_stop_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplicationAutoStopConfigurationEl>>,
    initial_capacity: std::option::Option<terrars::DynamicBlock<EmrserverlessApplicationInitialCapacityEl>>,
    maximum_capacity: std::option::Option<terrars::DynamicBlock<EmrserverlessApplicationMaximumCapacityEl>>,
    network_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EmrserverlessApplicationNetworkConfigurationEl>>,
}
