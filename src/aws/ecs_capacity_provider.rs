use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsCapacityProviderData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_scaling_group_provider: Option<Vec<EcsCapacityProviderAutoScalingGroupProviderEl>>,
    dynamic: EcsCapacityProviderDynamic,
}

struct EcsCapacityProvider_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<EcsCapacityProviderData>,
}

#[derive(Clone)]
pub struct EcsCapacityProvider(Rc<EcsCapacityProvider_>);

impl EcsCapacityProvider {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `auto_scaling_group_provider`.\n"]
    pub fn set_auto_scaling_group_provider(
        self,
        v: impl Into<BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_scaling_group_provider = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_scaling_group_provider = Some(d);
            },
        }
        self
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

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(&self) -> ListRef<EcsCapacityProviderAutoScalingGroupProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_scaling_group_provider", self.extract_ref()))
    }
}

impl Resource for EcsCapacityProvider {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for EcsCapacityProvider {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for EcsCapacityProvider {
    type O = ListRef<EcsCapacityProviderRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for EcsCapacityProvider_ {
    fn extract_resource_type(&self) -> String {
        "aws_ecs_capacity_provider".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsCapacityProvider {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildEcsCapacityProvider {
    pub fn build(self, stack: &mut Stack) -> EcsCapacityProvider {
        let out = EcsCapacityProvider(Rc::new(EcsCapacityProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(EcsCapacityProviderData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                auto_scaling_group_provider: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsCapacityProviderRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsCapacityProviderRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsCapacityProviderRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
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

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `auto_scaling_group_provider` after provisioning.\n"]
    pub fn auto_scaling_group_provider(&self) -> ListRef<EcsCapacityProviderAutoScalingGroupProviderElRef> {
        ListRef::new(self.shared().clone(), format!("{}.auto_scaling_group_provider", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    instance_warmup_period: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum_scaling_step_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_scaling_step_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_capacity: Option<PrimField<f64>>,
}

impl EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    #[doc= "Set the field `instance_warmup_period`.\n"]
    pub fn set_instance_warmup_period(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.instance_warmup_period = Some(v.into());
        self
    }

    #[doc= "Set the field `maximum_scaling_step_size`.\n"]
    pub fn set_maximum_scaling_step_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.maximum_scaling_step_size = Some(v.into());
        self
    }

    #[doc= "Set the field `minimum_scaling_step_size`.\n"]
    pub fn set_minimum_scaling_step_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.minimum_scaling_step_size = Some(v.into());
        self
    }

    #[doc= "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.status = Some(v.into());
        self
    }

    #[doc= "Set the field `target_capacity`.\n"]
    pub fn set_target_capacity(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.target_capacity = Some(v.into());
        self
    }
}

impl ToListMappable for EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl {
    type O = BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    fn new(shared: StackShared, base: String) -> EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
        EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `instance_warmup_period` after provisioning.\n"]
    pub fn instance_warmup_period(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_warmup_period", self.base))
    }

    #[doc= "Get a reference to the value of field `maximum_scaling_step_size` after provisioning.\n"]
    pub fn maximum_scaling_step_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_scaling_step_size", self.base))
    }

    #[doc= "Get a reference to the value of field `minimum_scaling_step_size` after provisioning.\n"]
    pub fn minimum_scaling_step_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.minimum_scaling_step_size", self.base))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc= "Get a reference to the value of field `target_capacity` after provisioning.\n"]
    pub fn target_capacity(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.target_capacity", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProviderAutoScalingGroupProviderElDynamic {
    managed_scaling: Option<DynamicBlock<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
}

#[derive(Serialize)]
pub struct EcsCapacityProviderAutoScalingGroupProviderEl {
    auto_scaling_group_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_termination_protection: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    managed_scaling: Option<Vec<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
    dynamic: EcsCapacityProviderAutoScalingGroupProviderElDynamic,
}

impl EcsCapacityProviderAutoScalingGroupProviderEl {
    #[doc= "Set the field `managed_termination_protection`.\n"]
    pub fn set_managed_termination_protection(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.managed_termination_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `managed_scaling`.\n"]
    pub fn set_managed_scaling(
        mut self,
        v: impl Into<BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.managed_scaling = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.managed_scaling = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for EcsCapacityProviderAutoScalingGroupProviderEl {
    type O = BlockAssignable<EcsCapacityProviderAutoScalingGroupProviderEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsCapacityProviderAutoScalingGroupProviderEl {
    #[doc= ""]
    pub auto_scaling_group_arn: PrimField<String>,
}

impl BuildEcsCapacityProviderAutoScalingGroupProviderEl {
    pub fn build(self) -> EcsCapacityProviderAutoScalingGroupProviderEl {
        EcsCapacityProviderAutoScalingGroupProviderEl {
            auto_scaling_group_arn: self.auto_scaling_group_arn,
            managed_termination_protection: core::default::Default::default(),
            managed_scaling: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct EcsCapacityProviderAutoScalingGroupProviderElRef {
    shared: StackShared,
    base: String,
}

impl Ref for EcsCapacityProviderAutoScalingGroupProviderElRef {
    fn new(shared: StackShared, base: String) -> EcsCapacityProviderAutoScalingGroupProviderElRef {
        EcsCapacityProviderAutoScalingGroupProviderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsCapacityProviderAutoScalingGroupProviderElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `auto_scaling_group_arn` after provisioning.\n"]
    pub fn auto_scaling_group_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_group_arn", self.base))
    }

    #[doc= "Get a reference to the value of field `managed_termination_protection` after provisioning.\n"]
    pub fn managed_termination_protection(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.managed_termination_protection", self.base))
    }

    #[doc= "Get a reference to the value of field `managed_scaling` after provisioning.\n"]
    pub fn managed_scaling(&self) -> ListRef<EcsCapacityProviderAutoScalingGroupProviderElManagedScalingElRef> {
        ListRef::new(self.shared().clone(), format!("{}.managed_scaling", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsCapacityProviderDynamic {
    auto_scaling_group_provider: Option<DynamicBlock<EcsCapacityProviderAutoScalingGroupProviderEl>>,
}
