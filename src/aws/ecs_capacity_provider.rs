#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsCapacityProviderData_ {
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
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_group_provider: std::option::Option<std::vec::Vec<EcsCapacityProvider_AutoScalingGroupProvider>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_instances_provider: std::option::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider>>,
    dynamic: EcsCapacityProviderDynamic,
}

struct EcsCapacityProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsCapacityProviderData_>,
}

#[derive(Clone)]
pub struct EcsCapacityProvider(std::rc::Rc<EcsCapacityProvider_>);

impl EcsCapacityProvider {
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

    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `auto_scaling_group_provider`.\n"]
    pub fn set_auto_scaling_group_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCapacityProvider_AutoScalingGroupProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_group_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_group_provider = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `managed_instances_provider`.\n"]
    pub fn set_managed_instances_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().managed_instances_provider = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.managed_instances_provider = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(&self) -> terrars::ListRef<EcsCapacityProvider_AutoScalingGroupProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_group_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_instances_provider` after provisioning.\n"]
    pub fn managed_instances_provider(&self) -> terrars::ListRef<EcsCapacityProvider_ManagedInstancesProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instances_provider", self.extract_ref()))
    }
}

impl terrars::Referable for EcsCapacityProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsCapacityProvider { }

impl terrars::ToListMappable for EcsCapacityProvider {
    type O = terrars::ListRef<EcsCapacityProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsCapacityProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_capacity_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsCapacityProvider {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildEcsCapacityProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsCapacityProvider {
        let out = EcsCapacityProvider(std::rc::Rc::new(EcsCapacityProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsCapacityProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                auto_scaling_group_provider: core::default::Default::default(),
                managed_instances_provider: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsCapacityProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsCapacityProviderRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(&self) -> terrars::ListRef<EcsCapacityProvider_AutoScalingGroupProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_group_provider", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `managed_instances_provider` after provisioning.\n"]
    pub fn managed_instances_provider(&self) -> terrars::ListRef<EcsCapacityProvider_ManagedInstancesProviderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_instances_provider", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProviderDynamic {
    auto_scaling_group_provider: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProvider_AutoScalingGroupProvider>>,
    managed_instances_provider: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider>>,
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_warmup_period: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_scaling_step_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_scaling_step_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_capacity: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {
    #[doc = "Set the field `instance_warmup_period`.\n"]
    pub fn set_instance_warmup_period(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_warmup_period = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_scaling_step_size`.\n"]
    pub fn set_maximum_scaling_step_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_scaling_step_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_scaling_step_size`.\n"]
    pub fn set_minimum_scaling_step_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_scaling_step_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_capacity`.\n"]
    pub fn set_target_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {
    type O = terrars::BlockAssignable<EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {}

impl BuildEcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {
    pub fn build(self) -> EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {
        EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling {
            instance_warmup_period: core::default::Default::default(),
            maximum_scaling_step_size: core::default::Default::default(),
            minimum_scaling_step_size: core::default::Default::default(),
            status: core::default::Default::default(),
            target_capacity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_AutoScalingGroupProvider_ManagedScalingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_AutoScalingGroupProvider_ManagedScalingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_AutoScalingGroupProvider_ManagedScalingRef {
        EcsCapacityProvider_AutoScalingGroupProvider_ManagedScalingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_AutoScalingGroupProvider_ManagedScalingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_warmup_period` after provisioning.\n"]
    pub fn instance_warmup_period(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_warmup_period", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_scaling_step_size` after provisioning.\n"]
    pub fn maximum_scaling_step_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_scaling_step_size", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_scaling_step_size` after provisioning.\n"]
    pub fn minimum_scaling_step_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_scaling_step_size", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_capacity", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProvider_AutoScalingGroupProviderDynamic {
    managed_scaling: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling>>,
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_AutoScalingGroupProvider {
    auto_scaling_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_draining: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_termination_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_scaling: std::option::Option<std::vec::Vec<EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling>>,
    dynamic: EcsCapacityProvider_AutoScalingGroupProviderDynamic,
}

impl EcsCapacityProvider_AutoScalingGroupProvider {
    #[doc = "Set the field `managed_draining`.\n"]
    pub fn set_managed_draining(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.managed_draining = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_termination_protection`.\n"]
    pub fn set_managed_termination_protection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.managed_termination_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `managed_scaling`.\n"]
    pub fn set_managed_scaling(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsCapacityProvider_AutoScalingGroupProvider_ManagedScaling>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.managed_scaling = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_scaling = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsCapacityProvider_AutoScalingGroupProvider {
    type O = terrars::BlockAssignable<EcsCapacityProvider_AutoScalingGroupProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_AutoScalingGroupProvider {
    #[doc = ""]
    pub auto_scaling_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsCapacityProvider_AutoScalingGroupProvider {
    pub fn build(self) -> EcsCapacityProvider_AutoScalingGroupProvider {
        EcsCapacityProvider_AutoScalingGroupProvider {
            auto_scaling_group_arn: self.auto_scaling_group_arn,
            managed_draining: core::default::Default::default(),
            managed_termination_protection: core::default::Default::default(),
            managed_scaling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_AutoScalingGroupProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_AutoScalingGroupProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_AutoScalingGroupProviderRef {
        EcsCapacityProvider_AutoScalingGroupProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_AutoScalingGroupProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_draining` after provisioning.\n"]
    pub fn managed_draining(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_draining", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_termination_protection` after provisioning.\n"]
    pub fn managed_termination_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_termination_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_scaling` after provisioning.\n"]
    pub fn managed_scaling(&self) -> terrars::ListRef<EcsCapacityProvider_AutoScalingGroupProvider_ManagedScalingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_scaling", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    actions_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {
    #[doc = "Set the field `actions_status`.\n"]
    pub fn set_actions_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.actions_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {
    type O = terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {
        EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration {
            actions_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfigurationRef {
        EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `actions_status` after provisioning.\n"]
    pub fn actions_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.actions_status", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_in_after: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {
    #[doc = "Set the field `scale_in_after`.\n"]
    pub fn set_scale_in_after(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.scale_in_after = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {
    type O = terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {
        EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization {
            scale_in_after: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimizationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimizationRef {
        EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimizationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimizationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `scale_in_after` after provisioning.\n"]
    pub fn scale_in_after(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_in_after", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {
    #[doc = "Set the field `reservation_group_arn`.\n"]
    pub fn set_reservation_group_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.reservation_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reservation_preference`.\n"]
    pub fn set_reservation_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.reservation_preference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {
    type O =
        terrars
        ::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations {
            reservation_group_arn: core::default::Default::default(),
            reservation_preference: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservationsRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `reservation_group_arn` after provisioning.\n"]
    pub fn reservation_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `reservation_preference` after provisioning.\n"]
    pub fn reservation_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_preference", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCountRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMibRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMibRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCountRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsDynamic {
    accelerator_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount,
        >,
    >,
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib,
        >,
    >,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps,
        >,
    >,
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu,
        >,
    >,
    memory_mib: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib,
        >,
    >,
    network_bandwidth_gbps: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps,
        >,
    >,
    network_interface_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount,
        >,
    >,
    total_local_storage_gb: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb,
        >,
    >,
    vcpu_count: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount,
        >,
    >,
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bare_metal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burstable_performance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_generations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_spot_price_as_percentage_of_optimal_on_demand_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount>,
    >,
    dynamic: EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsDynamic,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bare_metal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.burstable_performance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cpu_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_generations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.local_storage_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_hibernate_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMib,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interface_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGb,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCount,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vcpu_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {
    type O =
        terrars
        ::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_names", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_types", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.burstable_performance", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cpu_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_generations", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_storage_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_spot_price_as_percentage_of_optimal_on_demand_price", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_hibernate_support", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorCountRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_MemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_bandwidth_gbps", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars
    ::ListRef<
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements_VcpuCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    use_local_storage: std::option::Option<terrars::PrimField<bool>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {
    #[doc = "Set the field `use_local_storage`.\n"]
    pub fn set_use_local_storage(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.use_local_storage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {
    pub fn build(
        self,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration {
            use_local_storage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfigurationRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `use_local_storage` after provisioning.\n"]
    pub fn use_local_storage(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.use_local_storage", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
    type O =
        terrars
        ::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration {
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfigurationRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration {
    storage_size_gib: terrars::PrimField<f64>,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration { }

impl terrars
::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration {
    type O =
        terrars
        ::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration {
    #[doc = ""]
    pub storage_size_gib: terrars::PrimField<f64>,
}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration {
            storage_size_gib: self.storage_size_gib,
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfigurationRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `storage_size_gib` after provisioning.\n"]
    pub fn storage_size_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_size_gib", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateDynamic {
    capacity_reservations: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations>,
    >,
    instance_requirements: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements>,
    >,
    local_storage_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration>,
    >,
    network_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration>,
    >,
    storage_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_option_type: std::option::Option<terrars::PrimField<std::string::String>>,
    ec2_instance_profile_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservations: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage_configuration: std
    ::option
    ::Option<
        std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration>>,
    dynamic: EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateDynamic,
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
    #[doc = "Set the field `capacity_option_type`.\n"]
    pub fn set_capacity_option_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_option_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservations`.\n"]
    pub fn set_capacity_reservations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservations,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_reservations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirements,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_requirements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `local_storage_configuration`.\n"]
    pub fn set_local_storage_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.local_storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.local_storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `storage_configuration`.\n"]
    pub fn set_storage_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.storage_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.storage_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
    type O = terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
    #[doc = ""]
    pub ec2_instance_profile_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate {
            capacity_option_type: core::default::Default::default(),
            ec2_instance_profile_arn: self.ec2_instance_profile_arn,
            monitoring: core::default::Default::default(),
            capacity_reservations: core::default::Default::default(),
            instance_requirements: core::default::Default::default(),
            local_storage_configuration: core::default::Default::default(),
            network_configuration: core::default::Default::default(),
            storage_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateRef {
        EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_option_type` after provisioning.\n"]
    pub fn capacity_option_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_option_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_profile_arn` after provisioning.\n"]
    pub fn ec2_instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_profile_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservations` after provisioning.\n"]
    pub fn capacity_reservations(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_CapacityReservationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservations", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage_configuration` after provisioning.\n"]
    pub fn local_storage_configuration(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_LocalStorageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.local_storage_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_configuration` after provisioning.\n"]
    pub fn storage_configuration(
        &self,
    ) -> terrars
    ::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate_StorageConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.storage_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProvider_ManagedInstancesProviderDynamic {
    auto_repair_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration>>,
    infrastructure_optimization: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization>>,
    instance_launch_template: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate>>,
}

#[derive(Serialize)]
pub struct EcsCapacityProvider_ManagedInstancesProvider {
    infrastructure_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_repair_configuration: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infrastructure_optimization: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_launch_template: std
    ::option
    ::Option<std::vec::Vec<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate>>,
    dynamic: EcsCapacityProvider_ManagedInstancesProviderDynamic,
}

impl EcsCapacityProvider_ManagedInstancesProvider {
    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_repair_configuration`.\n"]
    pub fn set_auto_repair_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_repair_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_repair_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `infrastructure_optimization`.\n"]
    pub fn set_infrastructure_optimization(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimization>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.infrastructure_optimization = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.infrastructure_optimization = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_launch_template`.\n"]
    pub fn set_instance_launch_template(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplate>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_launch_template = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsCapacityProvider_ManagedInstancesProvider {
    type O = terrars::BlockAssignable<EcsCapacityProvider_ManagedInstancesProvider>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProvider_ManagedInstancesProvider {
    #[doc = ""]
    pub infrastructure_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsCapacityProvider_ManagedInstancesProvider {
    pub fn build(self) -> EcsCapacityProvider_ManagedInstancesProvider {
        EcsCapacityProvider_ManagedInstancesProvider {
            infrastructure_role_arn: self.infrastructure_role_arn,
            propagate_tags: core::default::Default::default(),
            auto_repair_configuration: core::default::Default::default(),
            infrastructure_optimization: core::default::Default::default(),
            instance_launch_template: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsCapacityProvider_ManagedInstancesProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProvider_ManagedInstancesProviderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProvider_ManagedInstancesProviderRef {
        EcsCapacityProvider_ManagedInstancesProviderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProvider_ManagedInstancesProviderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `infrastructure_role_arn` after provisioning.\n"]
    pub fn infrastructure_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_repair_configuration` after provisioning.\n"]
    pub fn auto_repair_configuration(
        &self,
    ) -> terrars::ListRef<EcsCapacityProvider_ManagedInstancesProvider_AutoRepairConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_repair_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `infrastructure_optimization` after provisioning.\n"]
    pub fn infrastructure_optimization(
        &self,
    ) -> terrars::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InfrastructureOptimizationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.infrastructure_optimization", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_launch_template` after provisioning.\n"]
    pub fn instance_launch_template(
        &self,
    ) -> terrars::ListRef<EcsCapacityProvider_ManagedInstancesProvider_InstanceLaunchTemplateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_launch_template", self.base))
    }
}
