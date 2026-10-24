use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BatchSchedulingPolicyData {
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
    fair_share_policy: std::option::Option<std::vec::Vec<BatchSchedulingPolicyFairSharePolicyEl>>,
    dynamic: BatchSchedulingPolicyDynamic,
}

struct BatchSchedulingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BatchSchedulingPolicyData>,
}

#[derive(Clone)]
pub struct BatchSchedulingPolicy(std::rc::Rc<BatchSchedulingPolicy_>);

impl BatchSchedulingPolicy {
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

    #[doc = "Set the field `fair_share_policy`.\n"]
    pub fn set_fair_share_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BatchSchedulingPolicyFairSharePolicyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().fair_share_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.fair_share_policy = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> terrars::ListRef<BatchSchedulingPolicyFairSharePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fair_share_policy", self.extract_ref()))
    }
}

impl terrars::Referable for BatchSchedulingPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BatchSchedulingPolicy { }

impl terrars::ToListMappable for BatchSchedulingPolicy {
    type O = terrars::ListRef<BatchSchedulingPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BatchSchedulingPolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_batch_scheduling_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBatchSchedulingPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBatchSchedulingPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> BatchSchedulingPolicy {
        let out = BatchSchedulingPolicy(std::rc::Rc::new(BatchSchedulingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BatchSchedulingPolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                fair_share_policy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BatchSchedulingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchSchedulingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BatchSchedulingPolicyRef {
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

    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> terrars::ListRef<BatchSchedulingPolicyFairSharePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fair_share_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    share_identifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    #[doc = "Set the field `weight_factor`.\n"]
    pub fn set_weight_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight_factor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    type O = terrars::BlockAssignable<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    #[doc = ""]
    pub share_identifier: terrars::PrimField<std::string::String>,
}

impl BuildBatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
    pub fn build(self) -> BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
        BatchSchedulingPolicyFairSharePolicyElShareDistributionEl {
            share_identifier: self.share_identifier,
            weight_factor: core::default::Default::default(),
        }
    }
}

pub struct BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
        BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchSchedulingPolicyFairSharePolicyElShareDistributionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `share_identifier` after provisioning.\n"]
    pub fn share_identifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_identifier", self.base))
    }

    #[doc = "Get a reference to the value of field `weight_factor` after provisioning.\n"]
    pub fn weight_factor(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight_factor", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchSchedulingPolicyFairSharePolicyElDynamic {
    share_distribution: std
    ::option
    ::Option<terrars::DynamicBlock<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
}

#[derive(Serialize)]
pub struct BatchSchedulingPolicyFairSharePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_reservation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_decay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_distribution: std::option::Option<std::vec::Vec<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
    dynamic: BatchSchedulingPolicyFairSharePolicyElDynamic,
}

impl BatchSchedulingPolicyFairSharePolicyEl {
    #[doc = "Set the field `compute_reservation`.\n"]
    pub fn set_compute_reservation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.compute_reservation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_decay_seconds`.\n"]
    pub fn set_share_decay_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.share_decay_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `share_distribution`.\n"]
    pub fn set_share_distribution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BatchSchedulingPolicyFairSharePolicyElShareDistributionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.share_distribution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.share_distribution = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BatchSchedulingPolicyFairSharePolicyEl {
    type O = terrars::BlockAssignable<BatchSchedulingPolicyFairSharePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBatchSchedulingPolicyFairSharePolicyEl {}

impl BuildBatchSchedulingPolicyFairSharePolicyEl {
    pub fn build(self) -> BatchSchedulingPolicyFairSharePolicyEl {
        BatchSchedulingPolicyFairSharePolicyEl {
            compute_reservation: core::default::Default::default(),
            share_decay_seconds: core::default::Default::default(),
            share_distribution: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BatchSchedulingPolicyFairSharePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BatchSchedulingPolicyFairSharePolicyElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BatchSchedulingPolicyFairSharePolicyElRef {
        BatchSchedulingPolicyFairSharePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BatchSchedulingPolicyFairSharePolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `compute_reservation` after provisioning.\n"]
    pub fn compute_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `share_decay_seconds` after provisioning.\n"]
    pub fn share_decay_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.share_decay_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct BatchSchedulingPolicyDynamic {
    fair_share_policy: std::option::Option<terrars::DynamicBlock<BatchSchedulingPolicyFairSharePolicyEl>>,
}
