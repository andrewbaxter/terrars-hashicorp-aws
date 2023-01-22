use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrserverlessApplicationData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    architecture: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    release_label: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_start_configuration: Option<Vec<EmrserverlessApplicationAutoStartConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_stop_configuration: Option<Vec<EmrserverlessApplicationAutoStopConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_capacity: Option<Vec<EmrserverlessApplicationInitialCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_capacity: Option<Vec<EmrserverlessApplicationMaximumCapacityEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    network_configuration: Option<Vec<EmrserverlessApplicationNetworkConfigurationEl>>,
    dynamic: EmrserverlessApplicationDynamic,
}

struct EmrserverlessApplication_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EmrserverlessApplicationData>,
}

#[derive(Clone)]
pub struct EmrserverlessApplication(Rc<EmrserverlessApplication_>);

impl EmrserverlessApplication {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().architecture = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `auto_start_configuration`.\n"]
    pub fn set_auto_start_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationAutoStartConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_start_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_start_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `auto_stop_configuration`.\n"]
    pub fn set_auto_stop_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationAutoStopConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_stop_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_stop_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `initial_capacity`.\n"]
    pub fn set_initial_capacity(self, v: impl Into<BlockAssignable<EmrserverlessApplicationInitialCapacityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().initial_capacity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.initial_capacity = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `maximum_capacity`.\n"]
    pub fn set_maximum_capacity(self, v: impl Into<BlockAssignable<EmrserverlessApplicationMaximumCapacityEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maximum_capacity = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maximum_capacity = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationNetworkConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStartConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStopConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> ListRef<EmrserverlessApplicationMaximumCapacityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EmrserverlessApplicationNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }
}

impl Resource for EmrserverlessApplication {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for EmrserverlessApplication {
    type O = ListRef<EmrserverlessApplicationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for EmrserverlessApplication_ {
    fn extract_resource_type(&self) -> String {
        "aws_emrserverless_application".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrserverlessApplication {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub release_label: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildEmrserverlessApplication {
    pub fn build(self, stack: &mut Stack) -> EmrserverlessApplication {
        let out = EmrserverlessApplication(Rc::new(EmrserverlessApplication_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EmrserverlessApplicationData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrserverlessApplicationRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrserverlessApplicationRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_start_configuration` after provisioning.\n"]
    pub fn auto_start_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStartConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_start_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_stop_configuration` after provisioning.\n"]
    pub fn auto_stop_configuration(&self) -> ListRef<EmrserverlessApplicationAutoStopConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_stop_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_capacity` after provisioning.\n"]
    pub fn maximum_capacity(&self) -> ListRef<EmrserverlessApplicationMaximumCapacityElRef> {
        ListRef::new(self.shared().clone(), format!("{}.maximum_capacity", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> ListRef<EmrserverlessApplicationNetworkConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationAutoStartConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl EmrserverlessApplicationAutoStartConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationAutoStartConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationAutoStartConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationAutoStartConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationAutoStartConfigurationElRef {
        EmrserverlessApplicationAutoStartConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationAutoStartConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationAutoStopConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    idle_timeout_minutes: Option<PrimField<f64>>,
}

impl EmrserverlessApplicationAutoStopConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `idle_timeout_minutes`.\n"]
    pub fn set_idle_timeout_minutes(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.idle_timeout_minutes = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationAutoStopConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationAutoStopConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationAutoStopConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationAutoStopConfigurationElRef {
        EmrserverlessApplicationAutoStopConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationAutoStopConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `idle_timeout_minutes` after provisioning.\n"]
    pub fn idle_timeout_minutes(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    cpu: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<String>>,
    memory: PrimField<String>,
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    #[doc= "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl {
    #[doc= ""]
    pub cpu: PrimField<String>,
    #[doc= ""]
    pub memory: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    fn new(
        shared: StackShared,
        base: String,
    ) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElDynamic {
    worker_configuration: Option<
        DynamicBlock<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    worker_count: PrimField<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_configuration: Option<
        Vec<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl>,
    >,
    dynamic: EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElDynamic,
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    #[doc= "Set the field `worker_configuration`.\n"]
    pub fn set_worker_configuration(
        mut self,
        v:
            impl

                    Into<
                        BlockAssignable<
                            EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.worker_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.worker_configuration = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    type O = BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    #[doc= ""]
    pub worker_count: PrimField<f64>,
}

impl BuildEmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl {
            worker_count: self.worker_count,
            worker_configuration: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
        EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `worker_count` after provisioning.\n"]
    pub fn worker_count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.worker_count", self.base))
    }

    #[doc= "Get a reference to the value of field `worker_configuration` after provisioning.\n"]
    pub fn worker_configuration(
        &self,
    ) -> ListRef<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElWorkerConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.worker_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationInitialCapacityElDynamic {
    initial_capacity_config: Option<DynamicBlock<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationInitialCapacityEl {
    initial_capacity_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_capacity_config: Option<Vec<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
    dynamic: EmrserverlessApplicationInitialCapacityElDynamic,
}

impl EmrserverlessApplicationInitialCapacityEl {
    #[doc= "Set the field `initial_capacity_config`.\n"]
    pub fn set_initial_capacity_config(
        mut self,
        v: impl Into<BlockAssignable<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.initial_capacity_config = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.initial_capacity_config = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EmrserverlessApplicationInitialCapacityEl {
    type O = BlockAssignable<EmrserverlessApplicationInitialCapacityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationInitialCapacityEl {
    #[doc= ""]
    pub initial_capacity_type: PrimField<String>,
}

impl BuildEmrserverlessApplicationInitialCapacityEl {
    pub fn build(self) -> EmrserverlessApplicationInitialCapacityEl {
        EmrserverlessApplicationInitialCapacityEl {
            initial_capacity_type: self.initial_capacity_type,
            initial_capacity_config: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EmrserverlessApplicationInitialCapacityElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationInitialCapacityElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationInitialCapacityElRef {
        EmrserverlessApplicationInitialCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationInitialCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `initial_capacity_type` after provisioning.\n"]
    pub fn initial_capacity_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.initial_capacity_type", self.base))
    }

    #[doc= "Get a reference to the value of field `initial_capacity_config` after provisioning.\n"]
    pub fn initial_capacity_config(
        &self,
    ) -> ListRef<EmrserverlessApplicationInitialCapacityElInitialCapacityConfigElRef> {
        ListRef::new(self.shared().clone(), format!("{}.initial_capacity_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationMaximumCapacityEl {
    cpu: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disk: Option<PrimField<String>>,
    memory: PrimField<String>,
}

impl EmrserverlessApplicationMaximumCapacityEl {
    #[doc= "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.disk = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationMaximumCapacityEl {
    type O = BlockAssignable<EmrserverlessApplicationMaximumCapacityEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrserverlessApplicationMaximumCapacityEl {
    #[doc= ""]
    pub cpu: PrimField<String>,
    #[doc= ""]
    pub memory: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationMaximumCapacityElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationMaximumCapacityElRef {
        EmrserverlessApplicationMaximumCapacityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationMaximumCapacityElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc= "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc= "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrserverlessApplicationNetworkConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    security_group_ids: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_ids: Option<SetField<PrimField<String>>>,
}

impl EmrserverlessApplicationNetworkConfigurationEl {
    #[doc= "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.security_group_ids = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.subnet_ids = Some(v.into());
        self
    }
}

impl ToListMappable for EmrserverlessApplicationNetworkConfigurationEl {
    type O = BlockAssignable<EmrserverlessApplicationNetworkConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for EmrserverlessApplicationNetworkConfigurationElRef {
    fn new(shared: StackShared, base: String) -> EmrserverlessApplicationNetworkConfigurationElRef {
        EmrserverlessApplicationNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrserverlessApplicationNetworkConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrserverlessApplicationDynamic {
    auto_start_configuration: Option<DynamicBlock<EmrserverlessApplicationAutoStartConfigurationEl>>,
    auto_stop_configuration: Option<DynamicBlock<EmrserverlessApplicationAutoStopConfigurationEl>>,
    initial_capacity: Option<DynamicBlock<EmrserverlessApplicationInitialCapacityEl>>,
    maximum_capacity: Option<DynamicBlock<EmrserverlessApplicationMaximumCapacityEl>>,
    network_configuration: Option<DynamicBlock<EmrserverlessApplicationNetworkConfigurationEl>>,
}
