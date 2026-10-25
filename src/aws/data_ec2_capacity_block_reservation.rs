#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2CapacityBlockReservationData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2CapacityBlockReservation_Filter>>,
    dynamic: DataEc2CapacityBlockReservationDynamic,
}

struct DataEc2CapacityBlockReservation_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2CapacityBlockReservationData_>,
}

#[derive(Clone)]
pub struct DataEc2CapacityBlockReservation(std::rc::Rc<DataEc2CapacityBlockReservation_>);

impl DataEc2CapacityBlockReservation {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2CapacityBlockReservation_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_instance_count` after provisioning.\n"]
    pub fn available_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_instance_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_block_id` after provisioning.\n"]
    pub fn capacity_block_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_block_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `commitment_info` after provisioning.\n"]
    pub fn commitment_info(&self) -> DataEc2CapacityBlockReservation_CommitmentInfoRef {
        DataEc2CapacityBlockReservation_CommitmentInfoRef::new(
            self.shared().clone(),
            format!("{}.commitment_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_preference` after provisioning.\n"]
    pub fn delivery_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date_type` after provisioning.\n"]
    pub fn end_date_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_match_criteria` after provisioning.\n"]
    pub fn instance_match_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_match_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_platform` after provisioning.\n"]
    pub fn instance_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interruptible_capacity_allocation` after provisioning.\n"]
    pub fn interruptible_capacity_allocation(&self) -> DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
        DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef::new(
            self.shared().clone(),
            format!("{}.interruptible_capacity_allocation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `interruption_info` after provisioning.\n"]
    pub fn interruption_info(&self) -> DataEc2CapacityBlockReservation_InterruptionInfoRef {
        DataEc2CapacityBlockReservation_InterruptionInfoRef::new(
            self.shared().clone(),
            format!("{}.interruption_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_arn` after provisioning.\n"]
    pub fn placement_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_type` after provisioning.\n"]
    pub fn reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2CapacityBlockReservation {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2CapacityBlockReservation { }

impl terrars::ToListMappable for DataEc2CapacityBlockReservation {
    type O = terrars::ListRef<DataEc2CapacityBlockReservationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2CapacityBlockReservation_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_capacity_block_reservation".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2CapacityBlockReservation {
    pub tf_id: std::string::String,
}

impl BuildDataEc2CapacityBlockReservation {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2CapacityBlockReservation {
        let out = DataEc2CapacityBlockReservation(std::rc::Rc::new(DataEc2CapacityBlockReservation_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2CapacityBlockReservationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2CapacityBlockReservationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2CapacityBlockReservationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2CapacityBlockReservationRef {
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

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_instance_count` after provisioning.\n"]
    pub fn available_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_instance_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_block_id` after provisioning.\n"]
    pub fn capacity_block_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_block_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `commitment_info` after provisioning.\n"]
    pub fn commitment_info(&self) -> DataEc2CapacityBlockReservation_CommitmentInfoRef {
        DataEc2CapacityBlockReservation_CommitmentInfoRef::new(
            self.shared().clone(),
            format!("{}.commitment_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delivery_preference` after provisioning.\n"]
    pub fn delivery_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delivery_preference", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date` after provisioning.\n"]
    pub fn end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `end_date_type` after provisioning.\n"]
    pub fn end_date_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_match_criteria` after provisioning.\n"]
    pub fn instance_match_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_match_criteria", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_platform` after provisioning.\n"]
    pub fn instance_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interruptible_capacity_allocation` after provisioning.\n"]
    pub fn interruptible_capacity_allocation(&self) -> DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
        DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef::new(
            self.shared().clone(),
            format!("{}.interruptible_capacity_allocation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `interruption_info` after provisioning.\n"]
    pub fn interruption_info(&self) -> DataEc2CapacityBlockReservation_InterruptionInfoRef {
        DataEc2CapacityBlockReservation_InterruptionInfoRef::new(
            self.shared().clone(),
            format!("{}.interruption_info", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_arn` after provisioning.\n"]
    pub fn placement_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reservation_type` after provisioning.\n"]
    pub fn reservation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reservation_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEc2CapacityBlockReservationDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2CapacityBlockReservation_Filter>>,
}

#[derive(Serialize)]
pub struct DataEc2CapacityBlockReservation_CommitmentInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    commitment_end_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    committed_instance_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2CapacityBlockReservation_CommitmentInfo {
    #[doc = "Set the field `commitment_end_date`.\n"]
    pub fn set_commitment_end_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.commitment_end_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `committed_instance_count`.\n"]
    pub fn set_committed_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.committed_instance_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2CapacityBlockReservation_CommitmentInfo {
    type O = terrars::BlockAssignable<DataEc2CapacityBlockReservation_CommitmentInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2CapacityBlockReservation_CommitmentInfo {}

impl BuildDataEc2CapacityBlockReservation_CommitmentInfo {
    pub fn build(self) -> DataEc2CapacityBlockReservation_CommitmentInfo {
        DataEc2CapacityBlockReservation_CommitmentInfo {
            commitment_end_date: core::default::Default::default(),
            committed_instance_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2CapacityBlockReservation_CommitmentInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2CapacityBlockReservation_CommitmentInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2CapacityBlockReservation_CommitmentInfoRef {
        DataEc2CapacityBlockReservation_CommitmentInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2CapacityBlockReservation_CommitmentInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `commitment_end_date` after provisioning.\n"]
    pub fn commitment_end_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.commitment_end_date", self.base))
    }

    #[doc = "Get a reference to the value of field `committed_instance_count` after provisioning.\n"]
    pub fn committed_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.committed_instance_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interruptible_capacity_reservation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interruption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_instance_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {
    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interruptible_capacity_reservation_id`.\n"]
    pub fn set_interruptible_capacity_reservation_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.interruptible_capacity_reservation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interruption_type`.\n"]
    pub fn set_interruption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interruption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_instance_count`.\n"]
    pub fn set_target_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_instance_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {
    type O = terrars::BlockAssignable<DataEc2CapacityBlockReservation_InterruptibleCapacityAllocation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {}

impl BuildDataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {
    pub fn build(self) -> DataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {
        DataEc2CapacityBlockReservation_InterruptibleCapacityAllocation {
            instance_count: core::default::Default::default(),
            interruptible_capacity_reservation_id: core::default::Default::default(),
            interruption_type: core::default::Default::default(),
            status: core::default::Default::default(),
            target_instance_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
        DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2CapacityBlockReservation_InterruptibleCapacityAllocationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `interruptible_capacity_reservation_id` after provisioning.\n"]
    pub fn interruptible_capacity_reservation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interruptible_capacity_reservation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `interruption_type` after provisioning.\n"]
    pub fn interruption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interruption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `target_instance_count` after provisioning.\n"]
    pub fn target_instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_instance_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2CapacityBlockReservation_InterruptionInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interruption_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_capacity_reservation_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2CapacityBlockReservation_InterruptionInfo {
    #[doc = "Set the field `interruption_type`.\n"]
    pub fn set_interruption_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interruption_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_capacity_reservation_id`.\n"]
    pub fn set_source_capacity_reservation_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_capacity_reservation_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2CapacityBlockReservation_InterruptionInfo {
    type O = terrars::BlockAssignable<DataEc2CapacityBlockReservation_InterruptionInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2CapacityBlockReservation_InterruptionInfo {}

impl BuildDataEc2CapacityBlockReservation_InterruptionInfo {
    pub fn build(self) -> DataEc2CapacityBlockReservation_InterruptionInfo {
        DataEc2CapacityBlockReservation_InterruptionInfo {
            interruption_type: core::default::Default::default(),
            source_capacity_reservation_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2CapacityBlockReservation_InterruptionInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2CapacityBlockReservation_InterruptionInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2CapacityBlockReservation_InterruptionInfoRef {
        DataEc2CapacityBlockReservation_InterruptionInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2CapacityBlockReservation_InterruptionInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interruption_type` after provisioning.\n"]
    pub fn interruption_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interruption_type", self.base))
    }

    #[doc = "Get a reference to the value of field `source_capacity_reservation_id` after provisioning.\n"]
    pub fn source_capacity_reservation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_capacity_reservation_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2CapacityBlockReservation_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2CapacityBlockReservation_Filter { }

impl terrars::ToListMappable for DataEc2CapacityBlockReservation_Filter {
    type O = terrars::BlockAssignable<DataEc2CapacityBlockReservation_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2CapacityBlockReservation_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2CapacityBlockReservation_Filter {
    pub fn build(self) -> DataEc2CapacityBlockReservation_Filter {
        DataEc2CapacityBlockReservation_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2CapacityBlockReservation_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2CapacityBlockReservation_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2CapacityBlockReservation_FilterRef {
        DataEc2CapacityBlockReservation_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2CapacityBlockReservation_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}
