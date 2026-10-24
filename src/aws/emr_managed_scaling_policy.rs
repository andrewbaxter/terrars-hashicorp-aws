use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrManagedScalingPolicyData {
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
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_limits: std::option::Option<std::vec::Vec<EmrManagedScalingPolicyComputeLimitsEl>>,
    dynamic: EmrManagedScalingPolicyDynamic,
}

struct EmrManagedScalingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrManagedScalingPolicyData>,
}

#[derive(Clone)]
pub struct EmrManagedScalingPolicy(std::rc::Rc<EmrManagedScalingPolicy_>);

impl EmrManagedScalingPolicy {
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

    #[doc = "Set the field `compute_limits`.\n"]
    pub fn set_compute_limits(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrManagedScalingPolicyComputeLimitsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().compute_limits = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.compute_limits = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

impl terrars::Referable for EmrManagedScalingPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrManagedScalingPolicy { }

impl terrars::ToListMappable for EmrManagedScalingPolicy {
    type O = terrars::ListRef<EmrManagedScalingPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrManagedScalingPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emr_managed_scaling_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrManagedScalingPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildEmrManagedScalingPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrManagedScalingPolicy {
        let out = EmrManagedScalingPolicy(std::rc::Rc::new(EmrManagedScalingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrManagedScalingPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                compute_limits: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrManagedScalingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrManagedScalingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrManagedScalingPolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct EmrManagedScalingPolicyComputeLimitsEl {
    maximum_capacity_units: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_core_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_ondemand_capacity_units: std::option::Option<terrars::PrimField<f64>>,
    minimum_capacity_units: terrars::PrimField<f64>,
    unit_type: terrars::PrimField<std::string::String>,
}

impl EmrManagedScalingPolicyComputeLimitsEl {
    #[doc = "Set the field `maximum_core_capacity_units`.\n"]
    pub fn set_maximum_core_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_core_capacity_units = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_ondemand_capacity_units`.\n"]
    pub fn set_maximum_ondemand_capacity_units(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_ondemand_capacity_units = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrManagedScalingPolicyComputeLimitsEl {
    type O = terrars::BlockAssignable<EmrManagedScalingPolicyComputeLimitsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrManagedScalingPolicyComputeLimitsEl {
    #[doc = ""]
    pub maximum_capacity_units: terrars::PrimField<f64>,
    #[doc = ""]
    pub minimum_capacity_units: terrars::PrimField<f64>,
    #[doc = ""]
    pub unit_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrManagedScalingPolicyComputeLimitsEl {
    pub fn build(self) -> EmrManagedScalingPolicyComputeLimitsEl {
        EmrManagedScalingPolicyComputeLimitsEl {
            maximum_capacity_units: self.maximum_capacity_units,
            maximum_core_capacity_units: core::default::Default::default(),
            maximum_ondemand_capacity_units: core::default::Default::default(),
            minimum_capacity_units: self.minimum_capacity_units,
            unit_type: self.unit_type,
        }
    }
}

pub struct EmrManagedScalingPolicyComputeLimitsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrManagedScalingPolicyComputeLimitsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrManagedScalingPolicyComputeLimitsElRef {
        EmrManagedScalingPolicyComputeLimitsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrManagedScalingPolicyComputeLimitsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_capacity_units` after provisioning.\n"]
    pub fn maximum_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_core_capacity_units` after provisioning.\n"]
    pub fn maximum_core_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_core_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_ondemand_capacity_units` after provisioning.\n"]
    pub fn maximum_ondemand_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_ondemand_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_capacity_units` after provisioning.\n"]
    pub fn minimum_capacity_units(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_capacity_units", self.base))
    }

    #[doc = "Get a reference to the value of field `unit_type` after provisioning.\n"]
    pub fn unit_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit_type", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrManagedScalingPolicyDynamic {
    compute_limits: std::option::Option<terrars::DynamicBlock<EmrManagedScalingPolicyComputeLimitsEl>>,
}
