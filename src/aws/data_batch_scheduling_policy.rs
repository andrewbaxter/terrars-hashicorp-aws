#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataBatchSchedulingPolicyData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataBatchSchedulingPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBatchSchedulingPolicyData_>,
}

#[derive(Clone)]
pub struct DataBatchSchedulingPolicy(std::rc::Rc<DataBatchSchedulingPolicy_>);

impl DataBatchSchedulingPolicy {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> terrars::ListRef<DataBatchSchedulingPolicy_FairSharePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fair_share_policy", self.extract_ref()))
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
}

impl terrars::Referable for DataBatchSchedulingPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBatchSchedulingPolicy { }

impl terrars::ToListMappable for DataBatchSchedulingPolicy {
    type O = terrars::ListRef<DataBatchSchedulingPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBatchSchedulingPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_batch_scheduling_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBatchSchedulingPolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub arn: terrars::PrimField<std::string::String>,
}

impl BuildDataBatchSchedulingPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBatchSchedulingPolicy {
        let out = DataBatchSchedulingPolicy(std::rc::Rc::new(DataBatchSchedulingPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBatchSchedulingPolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: self.arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBatchSchedulingPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchSchedulingPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBatchSchedulingPolicyRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fair_share_policy` after provisioning.\n"]
    pub fn fair_share_policy(&self) -> terrars::ListRef<DataBatchSchedulingPolicy_FairSharePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fair_share_policy", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_identifier: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight_factor: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {
    #[doc = "Set the field `share_identifier`.\n"]
    pub fn set_share_identifier(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.share_identifier = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight_factor`.\n"]
    pub fn set_weight_factor(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight_factor = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {
    type O = terrars::BlockAssignable<DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {}

impl BuildDataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {
    pub fn build(self) -> DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {
        DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution {
            share_identifier: core::default::Default::default(),
            weight_factor: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchSchedulingPolicy_FairSharePolicy_ShareDistributionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchSchedulingPolicy_FairSharePolicy_ShareDistributionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBatchSchedulingPolicy_FairSharePolicy_ShareDistributionRef {
        DataBatchSchedulingPolicy_FairSharePolicy_ShareDistributionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchSchedulingPolicy_FairSharePolicy_ShareDistributionRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataBatchSchedulingPolicy_FairSharePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_reservation: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_decay_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_distribution: std
    ::option
    ::Option<terrars::SetField<DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution>>,
}

impl DataBatchSchedulingPolicy_FairSharePolicy {
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
        v: impl std::convert::Into<terrars::SetField<DataBatchSchedulingPolicy_FairSharePolicy_ShareDistribution>>,
    ) -> Self {
        self.share_distribution = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBatchSchedulingPolicy_FairSharePolicy {
    type O = terrars::BlockAssignable<DataBatchSchedulingPolicy_FairSharePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBatchSchedulingPolicy_FairSharePolicy {}

impl BuildDataBatchSchedulingPolicy_FairSharePolicy {
    pub fn build(self) -> DataBatchSchedulingPolicy_FairSharePolicy {
        DataBatchSchedulingPolicy_FairSharePolicy {
            compute_reservation: core::default::Default::default(),
            share_decay_seconds: core::default::Default::default(),
            share_distribution: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBatchSchedulingPolicy_FairSharePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBatchSchedulingPolicy_FairSharePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBatchSchedulingPolicy_FairSharePolicyRef {
        DataBatchSchedulingPolicy_FairSharePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBatchSchedulingPolicy_FairSharePolicyRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `share_distribution` after provisioning.\n"]
    pub fn share_distribution(&self) -> terrars::SetRef<DataBatchSchedulingPolicy_FairSharePolicy_ShareDistributionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.share_distribution", self.base))
    }
}
