#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCodebuildFleetData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataCodebuildFleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCodebuildFleetData_>,
}

#[derive(Clone)]
pub struct DataCodebuildFleet(std::rc::Rc<DataCodebuildFleet_>);

impl DataCodebuildFleet {
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

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> terrars::ListRef<DataCodebuildFleet_ComputeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overflow_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> terrars::ListRef<DataCodebuildFleet_ScalingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::SetRef<DataCodebuildFleet_StatusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataCodebuildFleet_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for DataCodebuildFleet {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCodebuildFleet { }

impl terrars::ToListMappable for DataCodebuildFleet {
    type O = terrars::ListRef<DataCodebuildFleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCodebuildFleet_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_codebuild_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCodebuildFleet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataCodebuildFleet {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCodebuildFleet {
        let out = DataCodebuildFleet(std::rc::Rc::new(DataCodebuildFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataCodebuildFleetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCodebuildFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodebuildFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCodebuildFleetRef {
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

    #[doc = "Get a reference to the value of field `base_capacity` after provisioning.\n"]
    pub fn base_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_configuration` after provisioning.\n"]
    pub fn compute_configuration(&self) -> terrars::ListRef<DataCodebuildFleet_ComputeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_type` after provisioning.\n"]
    pub fn compute_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created` after provisioning.\n"]
    pub fn created(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `environment_type` after provisioning.\n"]
    pub fn environment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.environment_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `fleet_service_role` after provisioning.\n"]
    pub fn fleet_service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.fleet_service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified` after provisioning.\n"]
    pub fn last_modified(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overflow_behavior` after provisioning.\n"]
    pub fn overflow_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.overflow_behavior", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_configuration` after provisioning.\n"]
    pub fn scaling_configuration(&self) -> terrars::ListRef<DataCodebuildFleet_ScalingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::SetRef<DataCodebuildFleet_StatusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<DataCodebuildFleet_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCodebuildFleet_ComputeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disk: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    machine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCodebuildFleet_ComputeConfiguration {
    #[doc = "Set the field `disk`.\n"]
    pub fn set_disk(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.disk = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `machine_type`.\n"]
    pub fn set_machine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.machine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu`.\n"]
    pub fn set_vcpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.vcpu = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodebuildFleet_ComputeConfiguration {
    type O = terrars::BlockAssignable<DataCodebuildFleet_ComputeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodebuildFleet_ComputeConfiguration {}

impl BuildDataCodebuildFleet_ComputeConfiguration {
    pub fn build(self) -> DataCodebuildFleet_ComputeConfiguration {
        DataCodebuildFleet_ComputeConfiguration {
            disk: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            machine_type: core::default::Default::default(),
            memory: core::default::Default::default(),
            vcpu: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodebuildFleet_ComputeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodebuildFleet_ComputeConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCodebuildFleet_ComputeConfigurationRef {
        DataCodebuildFleet_ComputeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodebuildFleet_ComputeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `disk` after provisioning.\n"]
    pub fn disk(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disk", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `machine_type` after provisioning.\n"]
    pub fn machine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.machine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu` after provisioning.\n"]
    pub fn vcpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vcpu", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    #[doc = "Set the field `metric_type`.\n"]
    pub fn set_metric_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.metric_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_value`.\n"]
    pub fn set_target_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    type O = terrars::BlockAssignable<DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {}

impl BuildDataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
    pub fn build(self) -> DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
        DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs {
            metric_type: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
        DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_type` after provisioning.\n"]
    pub fn metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodebuildFleet_ScalingConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tracking_scaling_configs: std
    ::option
    ::Option<terrars::ListField<DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>>,
}

impl DataCodebuildFleet_ScalingConfiguration {
    #[doc = "Set the field `desired_capacity`.\n"]
    pub fn set_desired_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_capacity`.\n"]
    pub fn set_max_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_type`.\n"]
    pub fn set_scaling_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scaling_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_tracking_scaling_configs`.\n"]
    pub fn set_target_tracking_scaling_configs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigs>>,
    ) -> Self {
        self.target_tracking_scaling_configs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodebuildFleet_ScalingConfiguration {
    type O = terrars::BlockAssignable<DataCodebuildFleet_ScalingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodebuildFleet_ScalingConfiguration {}

impl BuildDataCodebuildFleet_ScalingConfiguration {
    pub fn build(self) -> DataCodebuildFleet_ScalingConfiguration {
        DataCodebuildFleet_ScalingConfiguration {
            desired_capacity: core::default::Default::default(),
            max_capacity: core::default::Default::default(),
            scaling_type: core::default::Default::default(),
            target_tracking_scaling_configs: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodebuildFleet_ScalingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodebuildFleet_ScalingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCodebuildFleet_ScalingConfigurationRef {
        DataCodebuildFleet_ScalingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodebuildFleet_ScalingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `desired_capacity` after provisioning.\n"]
    pub fn desired_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `max_capacity` after provisioning.\n"]
    pub fn max_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_type` after provisioning.\n"]
    pub fn scaling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tracking_scaling_configs` after provisioning.\n"]
    pub fn target_tracking_scaling_configs(
        &self,
    ) -> terrars::ListRef<DataCodebuildFleet_ScalingConfiguration_TargetTrackingScalingConfigsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_tracking_scaling_configs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodebuildFleet_Status {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    context: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCodebuildFleet_Status {
    #[doc = "Set the field `context`.\n"]
    pub fn set_context(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.context = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodebuildFleet_Status {
    type O = terrars::BlockAssignable<DataCodebuildFleet_Status>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodebuildFleet_Status {}

impl BuildDataCodebuildFleet_Status {
    pub fn build(self) -> DataCodebuildFleet_Status {
        DataCodebuildFleet_Status {
            context: core::default::Default::default(),
            message: core::default::Default::default(),
            status_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodebuildFleet_StatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodebuildFleet_StatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCodebuildFleet_StatusRef {
        DataCodebuildFleet_StatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodebuildFleet_StatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `context` after provisioning.\n"]
    pub fn context(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.context", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCodebuildFleet_VpcConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCodebuildFleet_VpcConfig {
    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.vpc_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCodebuildFleet_VpcConfig {
    type O = terrars::BlockAssignable<DataCodebuildFleet_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCodebuildFleet_VpcConfig {}

impl BuildDataCodebuildFleet_VpcConfig {
    pub fn build(self) -> DataCodebuildFleet_VpcConfig {
        DataCodebuildFleet_VpcConfig {
            security_group_ids: core::default::Default::default(),
            subnets: core::default::Default::default(),
            vpc_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCodebuildFleet_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCodebuildFleet_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataCodebuildFleet_VpcConfigRef {
        DataCodebuildFleet_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCodebuildFleet_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.base))
    }
}
