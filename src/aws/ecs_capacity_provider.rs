use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsCapacityProviderData {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_group_provider: std::option::Option<std::vec::Vec<EcsCapacityProviderAutoScalingGroupProviderEl>>,
    dynamic: EcsCapacityProviderDynamic,
}

struct EcsCapacityProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsCapacityProviderData>,
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

    #[doc = "Set the field `auto_scaling_group_provider`.\n"]
    pub fn set_auto_scaling_group_provider(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderEl>>,
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(&self) -> terrars::ListRef<EcsCapacityProviderAutoScalingGroupProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_group_provider", self.extract_ref()))
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
            data: std::cell::RefCell::new(EcsCapacityProviderData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                auto_scaling_group_provider: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(&self) -> terrars::ListRef<EcsCapacityProviderAutoScalingGroupProviderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_scaling_group_provider", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
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

impl EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
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

impl terrars::ToListMappable for EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    type O = terrars::BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {}

impl BuildEcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    pub fn build(self) -> EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
        EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
            instance_warmup_period: core::default::Default::default(),
            maximum_scaling_step_size: core::default::Default::default(),
            minimum_scaling_step_size: core::default::Default::default(),
            status: core::default::Default::default(),
            target_capacity: core::default::Default::default(),
        }
    }
}

pub struct EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
        EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
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
struct EcsCapacityProviderAutoScalingGroupProviderElDynamic {
    managed_scaling: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
}

#[derive(Serialize)]
pub struct EcsCapacityProviderAutoScalingGroupProviderEl {
    auto_scaling_group_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_termination_protection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    managed_scaling: std::option::Option<std::vec::Vec<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
    dynamic: EcsCapacityProviderAutoScalingGroupProviderElDynamic,
}

impl EcsCapacityProviderAutoScalingGroupProviderEl {
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
                    ::Into<terrars::BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
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

impl terrars::ToListMappable for EcsCapacityProviderAutoScalingGroupProviderEl {
    type O = terrars::BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProviderAutoScalingGroupProviderEl {
    #[doc = ""]
    pub auto_scaling_group_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsCapacityProviderAutoScalingGroupProviderEl {
    pub fn build(self) -> EcsCapacityProviderAutoScalingGroupProviderEl {
        EcsCapacityProviderAutoScalingGroupProviderEl {
            auto_scaling_group_arn: self.auto_scaling_group_arn,
            managed_termination_protection: core::default::Default::default(),
            managed_scaling: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct EcsCapacityProviderAutoScalingGroupProviderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsCapacityProviderAutoScalingGroupProviderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsCapacityProviderAutoScalingGroupProviderElRef {
        EcsCapacityProviderAutoScalingGroupProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProviderAutoScalingGroupProviderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_termination_protection` after provisioning.\n"]
    pub fn managed_termination_protection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.managed_termination_protection", self.base))
    }

    #[doc = "Get a reference to the value of field `managed_scaling` after provisioning.\n"]
    pub fn managed_scaling(&self) -> terrars::ListRef<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_scaling", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProviderDynamic {
    auto_scaling_group_provider: std
    ::option
    ::Option<terrars::DynamicBlock<EcsCapacityProviderAutoScalingGroupProviderEl>>,
}
