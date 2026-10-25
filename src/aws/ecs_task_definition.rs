#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsTaskDefinitionData_ {
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
    enable_fault_injection: std::option::Option<terrars::PrimField<bool>>,
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
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
    track_latest: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_storage: std::option::Option<std::vec::Vec<EcsTaskDefinition_EphemeralStorage>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraints: std::option::Option<std::vec::Vec<EcsTaskDefinition_PlacementConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    proxy_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinition_ProxyConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_platform: std::option::Option<std::vec::Vec<EcsTaskDefinition_RuntimePlatform>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume: std::option::Option<std::vec::Vec<EcsTaskDefinition_Volume>>,
    dynamic: EcsTaskDefinitionDynamic,
}

struct EcsTaskDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsTaskDefinitionData_>,
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

    #[doc = "Set the field `enable_fault_injection`.\n"]
    pub fn set_enable_fault_injection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_fault_injection = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `track_latest`.\n"]
    pub fn set_track_latest(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().track_latest = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ephemeral_storage`.\n"]
    pub fn set_ephemeral_storage(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_EphemeralStorage>>,
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

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_PlacementConstraints>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_ProxyConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_RuntimePlatform>>,
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
    pub fn set_volume(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_Volume>>) -> Self {
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

    #[doc = "Get a reference to the value of field `arn_without_revision` after provisioning.\n"]
    pub fn arn_without_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_without_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_fault_injection` after provisioning.\n"]
    pub fn enable_fault_injection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fault_injection", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `track_latest` after provisioning.\n"]
    pub fn track_latest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_latest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<EcsTaskDefinition_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> terrars::ListRef<EcsTaskDefinition_ProxyConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> terrars::ListRef<EcsTaskDefinition_RuntimePlatformRef> {
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
            data: std::cell::RefCell::new(EcsTaskDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                container_definitions: self.container_definitions,
                cpu: core::default::Default::default(),
                enable_fault_injection: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                family: self.family,
                id: core::default::Default::default(),
                ipc_mode: core::default::Default::default(),
                memory: core::default::Default::default(),
                network_mode: core::default::Default::default(),
                pid_mode: core::default::Default::default(),
                region: core::default::Default::default(),
                requires_compatibilities: core::default::Default::default(),
                skip_destroy: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_role_arn: core::default::Default::default(),
                track_latest: core::default::Default::default(),
                ephemeral_storage: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `arn_without_revision` after provisioning.\n"]
    pub fn arn_without_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_without_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_fault_injection` after provisioning.\n"]
    pub fn enable_fault_injection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fault_injection", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `track_latest` after provisioning.\n"]
    pub fn track_latest(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.track_latest", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<EcsTaskDefinition_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> terrars::ListRef<EcsTaskDefinition_ProxyConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> terrars::ListRef<EcsTaskDefinition_RuntimePlatformRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinitionDynamic {
    ephemeral_storage: std::option::Option<terrars::DynamicBlock<EcsTaskDefinition_EphemeralStorage>>,
    placement_constraints: std::option::Option<terrars::DynamicBlock<EcsTaskDefinition_PlacementConstraints>>,
    proxy_configuration: std::option::Option<terrars::DynamicBlock<EcsTaskDefinition_ProxyConfiguration>>,
    runtime_platform: std::option::Option<terrars::DynamicBlock<EcsTaskDefinition_RuntimePlatform>>,
    volume: std::option::Option<terrars::DynamicBlock<EcsTaskDefinition_Volume>>,
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_EphemeralStorage {
    size_in_gib: terrars::PrimField<f64>,
}

impl EcsTaskDefinition_EphemeralStorage { }

impl terrars::ToListMappable for EcsTaskDefinition_EphemeralStorage {
    type O = terrars::BlockAssignable<EcsTaskDefinition_EphemeralStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_EphemeralStorage {
    #[doc = ""]
    pub size_in_gib: terrars::PrimField<f64>,
}

impl BuildEcsTaskDefinition_EphemeralStorage {
    pub fn build(self) -> EcsTaskDefinition_EphemeralStorage {
        EcsTaskDefinition_EphemeralStorage { size_in_gib: self.size_in_gib }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_EphemeralStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_EphemeralStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinition_EphemeralStorageRef {
        EcsTaskDefinition_EphemeralStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_EphemeralStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_PlacementConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl EcsTaskDefinition_PlacementConstraints {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinition_PlacementConstraints {
    type O = terrars::BlockAssignable<EcsTaskDefinition_PlacementConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_PlacementConstraints {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_PlacementConstraints {
    pub fn build(self) -> EcsTaskDefinition_PlacementConstraints {
        EcsTaskDefinition_PlacementConstraints {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_PlacementConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_PlacementConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinition_PlacementConstraintsRef {
        EcsTaskDefinition_PlacementConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_PlacementConstraintsRef {
    #[allow(dead_code)]
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
pub struct EcsTaskDefinition_ProxyConfiguration {
    container_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinition_ProxyConfiguration {
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

impl terrars::ToListMappable for EcsTaskDefinition_ProxyConfiguration {
    type O = terrars::BlockAssignable<EcsTaskDefinition_ProxyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_ProxyConfiguration {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_ProxyConfiguration {
    pub fn build(self) -> EcsTaskDefinition_ProxyConfiguration {
        EcsTaskDefinition_ProxyConfiguration {
            container_name: self.container_name,
            properties: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_ProxyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_ProxyConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinition_ProxyConfigurationRef {
        EcsTaskDefinition_ProxyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_ProxyConfigurationRef {
    #[allow(dead_code)]
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
pub struct EcsTaskDefinition_RuntimePlatform {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinition_RuntimePlatform {
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

impl terrars::ToListMappable for EcsTaskDefinition_RuntimePlatform {
    type O = terrars::BlockAssignable<EcsTaskDefinition_RuntimePlatform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_RuntimePlatform {}

impl BuildEcsTaskDefinition_RuntimePlatform {
    pub fn build(self) -> EcsTaskDefinition_RuntimePlatform {
        EcsTaskDefinition_RuntimePlatform {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_RuntimePlatformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_RuntimePlatformRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinition_RuntimePlatformRef {
        EcsTaskDefinition_RuntimePlatformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_RuntimePlatformRef {
    #[allow(dead_code)]
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
pub struct EcsTaskDefinition_Volume_DockerVolumeConfiguration {
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

impl EcsTaskDefinition_Volume_DockerVolumeConfiguration {
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

impl terrars::ToListMappable for EcsTaskDefinition_Volume_DockerVolumeConfiguration {
    type O = terrars::BlockAssignable<EcsTaskDefinition_Volume_DockerVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume_DockerVolumeConfiguration {}

impl BuildEcsTaskDefinition_Volume_DockerVolumeConfiguration {
    pub fn build(self) -> EcsTaskDefinition_Volume_DockerVolumeConfiguration {
        EcsTaskDefinition_Volume_DockerVolumeConfiguration {
            autoprovision: core::default::Default::default(),
            driver: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            labels: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
        EcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
    #[allow(dead_code)]
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
pub struct EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
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

impl terrars::ToListMappable for EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    type O = terrars::BlockAssignable<EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {}

impl BuildEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    pub fn build(self) -> EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
        EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
            access_point_id: core::default::Default::default(),
            iam: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
        EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
    #[allow(dead_code)]
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
struct EcsTaskDefinition_Volume_EfsVolumeConfigurationDynamic {
    authorization_config: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>>,
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_Volume_EfsVolumeConfiguration {
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
    ::Option<std::vec::Vec<EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>>,
    dynamic: EcsTaskDefinition_Volume_EfsVolumeConfigurationDynamic,
}

impl EcsTaskDefinition_Volume_EfsVolumeConfiguration {
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
                        terrars::BlockAssignable<EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>,
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

impl terrars::ToListMappable for EcsTaskDefinition_Volume_EfsVolumeConfiguration {
    type O = terrars::BlockAssignable<EcsTaskDefinition_Volume_EfsVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume_EfsVolumeConfiguration {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_Volume_EfsVolumeConfiguration {
    pub fn build(self) -> EcsTaskDefinition_Volume_EfsVolumeConfiguration {
        EcsTaskDefinition_Volume_EfsVolumeConfiguration {
            file_system_id: self.file_system_id,
            root_directory: core::default::Default::default(),
            transit_encryption: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
            authorization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
        EcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<EcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    credentials_parameter: terrars::PrimField<std::string::String>,
    domain: terrars::PrimField<std::string::String>,
}

impl EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig { }

impl terrars::ToListMappable for EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    type O =
        terrars
        ::BlockAssignable<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    #[doc = ""]
    pub credentials_parameter: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub domain: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    pub fn build(self) -> EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
        EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
            credentials_parameter: self.credentials_parameter,
            domain: self.domain,
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
        EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
    #[allow(dead_code)]
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
struct EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationDynamic {
    authorization_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig>,
    >,
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    file_system_id: terrars::PrimField<std::string::String>,
    root_directory: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std
    ::option
    ::Option<std::vec::Vec<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig>>,
    dynamic: EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationDynamic,
}

impl EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
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
                            EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig,
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

impl terrars::ToListMappable for EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    type O = terrars::BlockAssignable<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    #[doc = ""]
    pub file_system_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub root_directory: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    pub fn build(self) -> EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
        EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
            file_system_id: self.file_system_id,
            root_directory: self.root_directory,
            authorization_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
        EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    file_system_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_port: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    #[doc = "Set the field `access_point_arn`.\n"]
    pub fn set_access_point_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption_port`.\n"]
    pub fn set_transit_encryption_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transit_encryption_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    type O = terrars::BlockAssignable<EcsTaskDefinition_Volume_S3filesVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    #[doc = ""]
    pub file_system_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    pub fn build(self) -> EcsTaskDefinition_Volume_S3filesVolumeConfiguration {
        EcsTaskDefinition_Volume_S3filesVolumeConfiguration {
            access_point_arn: core::default::Default::default(),
            file_system_arn: self.file_system_arn,
            root_directory: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
        EcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_arn` after provisioning.\n"]
    pub fn file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"]
    pub fn transit_encryption_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_port", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskDefinition_VolumeDynamic {
    docker_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinition_Volume_DockerVolumeConfiguration>>,
    efs_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinition_Volume_EfsVolumeConfiguration>>,
    fsx_windows_file_server_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>>,
    s3files_volume_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsTaskDefinition_Volume_S3filesVolumeConfiguration>>,
}

#[derive(Serialize)]
pub struct EcsTaskDefinition_Volume {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configure_at_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_volume_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinition_Volume_DockerVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_volume_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinition_Volume_EfsVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fsx_windows_file_server_volume_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3files_volume_configuration: std::option::Option<std::vec::Vec<EcsTaskDefinition_Volume_S3filesVolumeConfiguration>>,
    dynamic: EcsTaskDefinition_VolumeDynamic,
}

impl EcsTaskDefinition_Volume {
    #[doc = "Set the field `configure_at_launch`.\n"]
    pub fn set_configure_at_launch(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.configure_at_launch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_volume_configuration`.\n"]
    pub fn set_docker_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_Volume_DockerVolumeConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_Volume_EfsVolumeConfiguration>>,
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
                        terrars::BlockAssignable<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>,
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

    #[doc = "Set the field `s3files_volume_configuration`.\n"]
    pub fn set_s3files_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskDefinition_Volume_S3filesVolumeConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3files_volume_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3files_volume_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsTaskDefinition_Volume {
    type O = terrars::BlockAssignable<EcsTaskDefinition_Volume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskDefinition_Volume {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskDefinition_Volume {
    pub fn build(self) -> EcsTaskDefinition_Volume {
        EcsTaskDefinition_Volume {
            configure_at_launch: core::default::Default::default(),
            host_path: core::default::Default::default(),
            name: self.name,
            docker_volume_configuration: core::default::Default::default(),
            efs_volume_configuration: core::default::Default::default(),
            fsx_windows_file_server_volume_configuration: core::default::Default::default(),
            s3files_volume_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsTaskDefinition_VolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskDefinition_VolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskDefinition_VolumeRef {
        EcsTaskDefinition_VolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskDefinition_VolumeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configure_at_launch` after provisioning.\n"]
    pub fn configure_at_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configure_at_launch", self.base))
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
    pub fn docker_volume_configuration(&self) -> terrars::ListRef<EcsTaskDefinition_Volume_DockerVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]
    pub fn efs_volume_configuration(&self) -> terrars::ListRef<EcsTaskDefinition_Volume_EfsVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_volume_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fsx_windows_file_server_volume_configuration` after provisioning.\n"]
    pub fn fsx_windows_file_server_volume_configuration(
        &self,
    ) -> terrars::ListRef<EcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.fsx_windows_file_server_volume_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `s3files_volume_configuration` after provisioning.\n"]
    pub fn s3files_volume_configuration(
        &self,
    ) -> terrars::ListRef<EcsTaskDefinition_Volume_S3filesVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3files_volume_configuration", self.base))
    }
}
