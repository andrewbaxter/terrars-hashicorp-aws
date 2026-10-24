use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsTaskDefinitionData {
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
    container_definitions: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    family: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipc_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pid_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    requires_compatibilities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    skip_destroy: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std::option::Option<std::vec::Vec<EcsTaskDefinitionEphemeralStorageEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_accelerator: std::option::Option<std::vec::Vec<EcsTaskDefinitionInferenceAcceleratorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraints: std::option::Option<std::vec::Vec<EcsTaskDefinitionPlacementConstraintsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinitionProxyConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_platform: std::option::Option<std::vec::Vec<EcsTaskDefinitionRuntimePlatformEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume: std::option::Option<std::vec::Vec<EcsTaskDefinitionVolumeEl>>,
    dynamic: EcsTaskDefinitionDynamic,
}

struct EcsTaskDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsTaskDefinitionData>,
}

#[derive(Clone)]
pub struct EcsTaskDefinition(std::rc::Rc<EcsTaskDefinition_>);

impl EcsTaskDefinition {
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

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipc_mode`.\n"]
    pub fn set_ipc_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipc_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_mode`.\n"]
    pub fn set_network_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().network_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pid_mode`.\n"]
    pub fn set_pid_mode(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().pid_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `requires_compatibilities`.\n"]
    pub fn set_requires_compatibilities(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().requires_compatibilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `skip_destroy`.\n"]
    pub fn set_skip_destroy(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().skip_destroy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().task_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionEphemeralStorageEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_storage = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_storage = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inference_accelerator`.\n"]
    pub fn set_inference_accelerator(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionInferenceAcceleratorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().inference_accelerator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.inference_accelerator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionPlacementConstraintsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `proxy_configuration`.\n"]
    pub fn set_proxy_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionProxyConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().proxy_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.proxy_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `runtime_platform`.\n"]
    pub fn set_runtime_platform(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionRuntimePlatformEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_platform = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_platform = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `volume`.\n"]
    pub fn set_volume(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionVolumeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().volume = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.volume = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipc_mode` after provisioning.\n"]
    pub fn ipc_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pid_mode` after provisioning.\n"]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requires_compatibilities` after provisioning.\n"]
    pub fn requires_compatibilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.requires_compatibilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<EcsTaskDefinitionEphemeralStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> terrars::ListRef<EcsTaskDefinitionProxyConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> terrars::ListRef<EcsTaskDefinitionRuntimePlatformElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }
}

impl terrars::Referable for EcsTaskDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsTaskDefinition { }

impl terrars::ToListMappable for EcsTaskDefinition {
    type O = terrars::ListRef<EcsTaskDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsTaskDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_task_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsTaskDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub container_definitions: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub family: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsTaskDefinition {
        let out = EcsTaskDefinition(std::rc::Rc::new(EcsTaskDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsTaskDefinitionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_definitions: self.container_definitions,
                cpu: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                family: self.family,
                id: core::default::Default::default(),
                ipc_mode: core::default::Default::default(),
                memory: core::default::Default::default(),
                network_mode: core::default::Default::default(),
                pid_mode: core::default::Default::default(),
                requires_compatibilities: core::default::Default::default(),
                skip_destroy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_role_arn: core::default::Default::default(),
                ephemeral_storage: core::default::Default::default(),
                inference_accelerator: core::default::Default::default(),
                placement_constraints: core::default::Default::default(),
                proxy_configuration: core::default::Default::default(),
                runtime_platform: core::default::Default::default(),
                volume: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsTaskDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsTaskDefinitionRef {
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

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipc_mode` after provisioning.\n"]
    pub fn ipc_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pid_mode` after provisioning.\n"]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requires_compatibilities` after provisioning.\n"]
    pub fn requires_compatibilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.requires_compatibilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `skip_destroy` after provisioning.\n"]
    pub fn skip_destroy(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.skip_destroy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<EcsTaskDefinitionEphemeralStorageElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> terrars::ListRef<EcsTaskDefinitionProxyConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> terrars::ListRef<EcsTaskDefinitionRuntimePlatformElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionEphemeralStorageEl {
    size_in_gib: terrars::PrimField<f64>,
}

impl EcsTaskDefinitionEphemeralStorageEl { }

impl terrars::ToListMappable for EcsTaskDefinitionEphemeralStorageEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionEphemeralStorageEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionEphemeralStorageEl {
    #[doc = ""]
    pub size_in_gib: terrars::PrimField<f64>,
}

impl BuildEcsTaskDefinitionEphemeralStorageEl {
    pub fn build(self) -> EcsTaskDefinitionEphemeralStorageEl {
        EcsTaskDefinitionEphemeralStorageEl { size_in_gib: self.size_in_gib }
    }
}

pub struct EcsTaskDefinitionEphemeralStorageElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionEphemeralStorageElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinitionEphemeralStorageElRef {
        EcsTaskDefinitionEphemeralStorageElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionEphemeralStorageElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionInferenceAcceleratorEl {
    device_name: terrars::PrimField<std::string::String>,
    device_type: terrars::PrimField<std::string::String>,
}

impl EcsTaskDefinitionInferenceAcceleratorEl { }

impl terrars::ToListMappable for EcsTaskDefinitionInferenceAcceleratorEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionInferenceAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionInferenceAcceleratorEl {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub device_type: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionInferenceAcceleratorEl {
    pub fn build(self) -> EcsTaskDefinitionInferenceAcceleratorEl {
        EcsTaskDefinitionInferenceAcceleratorEl {
            device_name: self.device_name,
            device_type: self.device_type,
        }
    }
}

pub struct EcsTaskDefinitionInferenceAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionInferenceAcceleratorElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinitionInferenceAcceleratorElRef {
        EcsTaskDefinitionInferenceAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionInferenceAcceleratorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `device_type` after provisioning.\n"]
    pub fn device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionPlacementConstraintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsTaskDefinitionPlacementConstraintsEl {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionPlacementConstraintsEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionPlacementConstraintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionPlacementConstraintsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionPlacementConstraintsEl {
    pub fn build(self) -> EcsTaskDefinitionPlacementConstraintsEl {
        EcsTaskDefinitionPlacementConstraintsEl {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct EcsTaskDefinitionPlacementConstraintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionPlacementConstraintsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinitionPlacementConstraintsElRef {
        EcsTaskDefinitionPlacementConstraintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionPlacementConstraintsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `expression` after provisioning.\n"]
    pub fn expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.expression", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionProxyConfigurationEl {
    container_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinitionProxyConfigurationEl {
    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionProxyConfigurationEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionProxyConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionProxyConfigurationEl {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionProxyConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionProxyConfigurationEl {
        EcsTaskDefinitionProxyConfigurationEl {
            container_name: self.container_name,
            properties: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionProxyConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionProxyConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinitionProxyConfigurationElRef {
        EcsTaskDefinitionProxyConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionProxyConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionRuntimePlatformEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinitionRuntimePlatformEl {
    #[doc = "Set the field `cpu_architecture`.\n"]
    pub fn set_cpu_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system_family`.\n"]
    pub fn set_operating_system_family(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.operating_system_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionRuntimePlatformEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionRuntimePlatformEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionRuntimePlatformEl {}

impl BuildEcsTaskDefinitionRuntimePlatformEl {
    pub fn build(self) -> EcsTaskDefinitionRuntimePlatformEl {
        EcsTaskDefinitionRuntimePlatformEl {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionRuntimePlatformElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionRuntimePlatformElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinitionRuntimePlatformElRef {
        EcsTaskDefinitionRuntimePlatformElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionRuntimePlatformElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_architecture` after provisioning.\n"]
    pub fn cpu_architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system_family` after provisioning.\n"]
    pub fn operating_system_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system_family", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoprovision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_opts: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    #[doc = "Set the field `autoprovision`.\n"]
    pub fn set_autoprovision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.autoprovision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver`.\n"]
    pub fn set_driver(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver_opts`.\n"]
    pub fn set_driver_opts(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {}

impl BuildEcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
        EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl {
            autoprovision: core::default::Default::default(),
            driver: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            labels: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
        EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoprovision` after provisioning.\n"]
    pub fn autoprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoprovision", self.base))
    }

    #[doc = "Get a reference to the value of field `driver` after provisioning.\n"]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.base))
    }

    #[doc = "Get a reference to the value of field `driver_opts` after provisioning.\n"]
    pub fn driver_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    #[doc = "Set the field `access_point_id`.\n"]
    pub fn set_access_point_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iam = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {}

impl BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl {
            access_point_id: core::default::Default::default(),
            iam: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElDynamic {
    authorization_config: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>>,
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    file_system_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std
    ::option
    ::Option<std::vec::Vec<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>>,
    dynamic: EcsTaskDefinitionVolumeElEfsVolumeConfigurationElDynamic,
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption`.\n"]
    pub fn set_transit_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption_port`.\n"]
    pub fn set_transit_encryption_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transit_encryption_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl {
            file_system_id: self.file_system_id,
            root_directory: core::default::Default::default(),
            transit_encryption: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
            authorization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
        EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption` after provisioning.\n"]
    pub fn transit_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"]
    pub fn transit_encryption_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_port", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> terrars::ListRef<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElAuthorizationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    credentials_parameter: terrars::PrimField<std::string::String>,
    domain: terrars::PrimField<std::string::String>,
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl { }

impl terrars
::ToListMappable for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    type O =
        terrars
        ::BlockAssignable<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    #[doc = ""]
    pub credentials_parameter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl {
            credentials_parameter: self.credentials_parameter,
            domain: self.domain,
        }
    }
}

pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_parameter` after provisioning.\n"]
    pub fn credentials_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElDynamic {
    authorization_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>,
    >,
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    file_system_id: terrars::PrimField<std::string::String>,
    root_directory: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std
    ::option
    ::Option<std::vec::Vec<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl>>,
    dynamic: EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElDynamic,
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.authorization_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.authorization_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub root_directory: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl {
            file_system_id: self.file_system_id,
            root_directory: self.root_directory,
            authorization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
        EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> terrars::ListRef<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElAuthorizationConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionVolumeElDynamic {
    docker_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    efs_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    fsx_windows_file_server_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>>,
}

#[derive(Serialize)]
pub struct EcsTaskDefinitionVolumeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_volume_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_volume_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fsx_windows_file_server_volume_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>>,
    dynamic: EcsTaskDefinitionVolumeElDynamic,
}

impl EcsTaskDefinitionVolumeEl {
    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_volume_configuration`.\n"]
    pub fn set_docker_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionVolumeElDockerVolumeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.docker_volume_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.docker_volume_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `efs_volume_configuration`.\n"]
    pub fn set_efs_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinitionVolumeElEfsVolumeConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.efs_volume_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.efs_volume_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fsx_windows_file_server_volume_configuration`.\n"]
    pub fn set_fsx_windows_file_server_volume_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fsx_windows_file_server_volume_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fsx_windows_file_server_volume_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinitionVolumeEl {
    type O = terrars::BlockAssignable<EcsTaskDefinitionVolumeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinitionVolumeEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinitionVolumeEl {
    pub fn build(self) -> EcsTaskDefinitionVolumeEl {
        EcsTaskDefinitionVolumeEl {
            host_path: core::default::Default::default(),
            name: self.name,
            docker_volume_configuration: core::default::Default::default(),
            efs_volume_configuration: core::default::Default::default(),
            fsx_windows_file_server_volume_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskDefinitionVolumeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinitionVolumeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinitionVolumeElRef {
        EcsTaskDefinitionVolumeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinitionVolumeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_volume_configuration` after provisioning.\n"]
    pub fn docker_volume_configuration(
        &self,
    ) -> terrars::ListRef<EcsTaskDefinitionVolumeElDockerVolumeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]
    pub fn efs_volume_configuration(&self) -> terrars::ListRef<EcsTaskDefinitionVolumeElEfsVolumeConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_volume_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fsx_windows_file_server_volume_configuration` after provisioning.\n"]
    pub fn fsx_windows_file_server_volume_configuration(
        &self,
    ) -> terrars::ListRef<EcsTaskDefinitionVolumeElFsxWindowsFileServerVolumeConfigurationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.fsx_windows_file_server_volume_configuration", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionDynamic {
    ephemeral_storage: std::option::Option<terrars::DynamicBlock<EcsTaskDefinitionEphemeralStorageEl>>,
    inference_accelerator: std::option::Option<terrars::DynamicBlock<EcsTaskDefinitionInferenceAcceleratorEl>>,
    placement_constraints: std::option::Option<terrars::DynamicBlock<EcsTaskDefinitionPlacementConstraintsEl>>,
    proxy_configuration: std::option::Option<terrars::DynamicBlock<EcsTaskDefinitionProxyConfigurationEl>>,
    runtime_platform: std::option::Option<terrars::DynamicBlock<EcsTaskDefinitionRuntimePlatformEl>>,
    volume: std::option::Option<terrars::DynamicBlock<EcsTaskDefinitionVolumeEl>>,
}
