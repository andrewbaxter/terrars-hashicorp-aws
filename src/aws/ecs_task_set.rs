use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsTaskSetData {
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
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    force_delete: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    service: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    task_definition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_until_stable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_until_stable_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std::option::Option<std::vec::Vec<EcsTaskSetCapacityProviderStrategyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer: std::option::Option<std::vec::Vec<EcsTaskSetLoadBalancerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<EcsTaskSetNetworkConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale: std::option::Option<std::vec::Vec<EcsTaskSetScaleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_registries: std::option::Option<std::vec::Vec<EcsTaskSetServiceRegistriesEl>>,
    dynamic: EcsTaskSetDynamic,
}

struct EcsTaskSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsTaskSetData>,
}

#[derive(Clone)]
pub struct EcsTaskSet(std::rc::Rc<EcsTaskSet_>);

impl EcsTaskSet {
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

    #[doc = "Set the field `external_id`.\n"]
    pub fn set_external_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().external_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `force_delete`.\n"]
    pub fn set_force_delete(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().force_delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_type`.\n"]
    pub fn set_launch_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().launch_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `platform_version`.\n"]
    pub fn set_platform_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().platform_version = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `wait_until_stable`.\n"]
    pub fn set_wait_until_stable(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_until_stable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_until_stable_timeout`.\n"]
    pub fn set_wait_until_stable_timeout(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().wait_until_stable_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskSetCapacityProviderStrategyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_provider_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_provider_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer`.\n"]
    pub fn set_load_balancer(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskSetLoadBalancerEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskSetNetworkConfigurationEl>>,
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

    #[doc = "Set the field `scale`.\n"]
    pub fn set_scale(self, v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskSetScaleEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().scale = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.scale = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `service_registries`.\n"]
    pub fn set_service_registries(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsTaskSetServiceRegistriesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().service_registries = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.service_registries = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stability_status` after provisioning.\n"]
    pub fn stability_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stability_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_set_id` after provisioning.\n"]
    pub fn task_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_until_stable` after provisioning.\n"]
    pub fn wait_until_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_until_stable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_until_stable_timeout` after provisioning.\n"]
    pub fn wait_until_stable_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_until_stable_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsTaskSetNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> terrars::ListRef<EcsTaskSetScaleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<EcsTaskSetServiceRegistriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }
}

impl terrars::Referable for EcsTaskSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsTaskSet { }

impl terrars::ToListMappable for EcsTaskSet {
    type O = terrars::ListRef<EcsTaskSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsTaskSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_task_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsTaskSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub task_definition: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskSet {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsTaskSet {
        let out = EcsTaskSet(std::rc::Rc::new(EcsTaskSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsTaskSetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: self.cluster,
                external_id: core::default::Default::default(),
                force_delete: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_type: core::default::Default::default(),
                platform_version: core::default::Default::default(),
                service: self.service,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                task_definition: self.task_definition,
                wait_until_stable: core::default::Default::default(),
                wait_until_stable_timeout: core::default::Default::default(),
                capacity_provider_strategy: core::default::Default::default(),
                load_balancer: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                scale: core::default::Default::default(),
                service_registries: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsTaskSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsTaskSetRef {
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

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `external_id` after provisioning.\n"]
    pub fn external_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.external_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `force_delete` after provisioning.\n"]
    pub fn force_delete(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.force_delete", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service` after provisioning.\n"]
    pub fn service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stability_status` after provisioning.\n"]
    pub fn stability_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stability_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_set_id` after provisioning.\n"]
    pub fn task_set_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_set_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_until_stable` after provisioning.\n"]
    pub fn wait_until_stable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_until_stable", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_until_stable_timeout` after provisioning.\n"]
    pub fn wait_until_stable_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_until_stable_timeout", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsTaskSetNetworkConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale` after provisioning.\n"]
    pub fn scale(&self) -> terrars::ListRef<EcsTaskSetScaleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scale", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<EcsTaskSetServiceRegistriesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsTaskSetCapacityProviderStrategyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    weight: terrars::PrimField<f64>,
}

impl EcsTaskSetCapacityProviderStrategyEl {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskSetCapacityProviderStrategyEl {
    type O = terrars::BlockAssignable<EcsTaskSetCapacityProviderStrategyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskSetCapacityProviderStrategyEl {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub weight: terrars::PrimField<f64>,
}

impl BuildEcsTaskSetCapacityProviderStrategyEl {
    pub fn build(self) -> EcsTaskSetCapacityProviderStrategyEl {
        EcsTaskSetCapacityProviderStrategyEl {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: self.weight,
        }
    }
}

pub struct EcsTaskSetCapacityProviderStrategyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskSetCapacityProviderStrategyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskSetCapacityProviderStrategyElRef {
        EcsTaskSetCapacityProviderStrategyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskSetCapacityProviderStrategyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base` after provisioning.\n"]
    pub fn base(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_provider` after provisioning.\n"]
    pub fn capacity_provider(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskSetLoadBalancerEl {
    container_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsTaskSetLoadBalancerEl {
    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_name`.\n"]
    pub fn set_load_balancer_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.load_balancer_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_group_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskSetLoadBalancerEl {
    type O = terrars::BlockAssignable<EcsTaskSetLoadBalancerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskSetLoadBalancerEl {
    #[doc = ""]
    pub container_name: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskSetLoadBalancerEl {
    pub fn build(self) -> EcsTaskSetLoadBalancerEl {
        EcsTaskSetLoadBalancerEl {
            container_name: self.container_name,
            container_port: core::default::Default::default(),
            load_balancer_name: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskSetLoadBalancerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskSetLoadBalancerElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskSetLoadBalancerElRef {
        EcsTaskSetLoadBalancerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskSetLoadBalancerElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_name` after provisioning.\n"]
    pub fn load_balancer_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskSetNetworkConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EcsTaskSetNetworkConfigurationEl {
    #[doc = "Set the field `assign_public_ip`.\n"]
    pub fn set_assign_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.assign_public_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskSetNetworkConfigurationEl {
    type O = terrars::BlockAssignable<EcsTaskSetNetworkConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskSetNetworkConfigurationEl {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEcsTaskSetNetworkConfigurationEl {
    pub fn build(self) -> EcsTaskSetNetworkConfigurationEl {
        EcsTaskSetNetworkConfigurationEl {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

pub struct EcsTaskSetNetworkConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskSetNetworkConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskSetNetworkConfigurationElRef {
        EcsTaskSetNetworkConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskSetNetworkConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `assign_public_ip` after provisioning.\n"]
    pub fn assign_public_ip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.assign_public_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskSetScaleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsTaskSetScaleEl {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskSetScaleEl {
    type O = terrars::BlockAssignable<EcsTaskSetScaleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskSetScaleEl {}

impl BuildEcsTaskSetScaleEl {
    pub fn build(self) -> EcsTaskSetScaleEl {
        EcsTaskSetScaleEl {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct EcsTaskSetScaleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskSetScaleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskSetScaleElRef {
        EcsTaskSetScaleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskSetScaleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsTaskSetServiceRegistriesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    registry_arn: terrars::PrimField<std::string::String>,
}

impl EcsTaskSetServiceRegistriesEl {
    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsTaskSetServiceRegistriesEl {
    type O = terrars::BlockAssignable<EcsTaskSetServiceRegistriesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsTaskSetServiceRegistriesEl {
    #[doc = ""]
    pub registry_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsTaskSetServiceRegistriesEl {
    pub fn build(self) -> EcsTaskSetServiceRegistriesEl {
        EcsTaskSetServiceRegistriesEl {
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            port: core::default::Default::default(),
            registry_arn: self.registry_arn,
        }
    }
}

pub struct EcsTaskSetServiceRegistriesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsTaskSetServiceRegistriesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsTaskSetServiceRegistriesElRef {
        EcsTaskSetServiceRegistriesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsTaskSetServiceRegistriesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `registry_arn` after provisioning.\n"]
    pub fn registry_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registry_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsTaskSetDynamic {
    capacity_provider_strategy: std::option::Option<terrars::DynamicBlock<EcsTaskSetCapacityProviderStrategyEl>>,
    load_balancer: std::option::Option<terrars::DynamicBlock<EcsTaskSetLoadBalancerEl>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<EcsTaskSetNetworkConfigurationEl>>,
    scale: std::option::Option<terrars::DynamicBlock<EcsTaskSetScaleEl>>,
    service_registries: std::option::Option<terrars::DynamicBlock<EcsTaskSetServiceRegistriesEl>>,
}
