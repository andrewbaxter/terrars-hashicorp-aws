#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcsServiceData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    cluster_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataEcsService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcsServiceData_>,
}

#[derive(Clone)]
pub struct DataEcsService(std::rc::Rc<DataEcsService_>);

impl DataEcsService {
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

    #[doc = "Get a reference to the value of field `availability_zone_rebalancing` after provisioning.\n"]
    pub fn availability_zone_rebalancing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_provider_strategy` after provisioning.\n"]
    pub fn capacity_provider_strategy(&self) -> terrars::SetRef<DataEcsService_CapacityProviderStrategyRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_provider_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_configuration` after provisioning.\n"]
    pub fn deployment_configuration(&self) -> terrars::ListRef<DataEcsService_DeploymentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> terrars::ListRef<DataEcsService_DeploymentControllerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployments` after provisioning.\n"]
    pub fn deployments(&self) -> terrars::ListRef<DataEcsService_DeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::ListRef<DataEcsService_EventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::SetRef<DataEcsService_LoadBalancerRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<DataEcsService_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> terrars::ListRef<DataEcsService_OrderedPlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> terrars::SetRef<DataEcsService_PlacementConstraintsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.placement_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_family` after provisioning.\n"]
    pub fn platform_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<DataEcsService_ServiceRegistriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_sets` after provisioning.\n"]
    pub fn task_sets(&self) -> terrars::ListRef<DataEcsService_TaskSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_sets", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcsService {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcsService { }

impl terrars::ToListMappable for DataEcsService {
    type O = terrars::ListRef<DataEcsServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcsService_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecs_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcsService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsService {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcsService {
        let out = DataEcsService(std::rc::Rc::new(DataEcsService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcsServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cluster_arn: self.cluster_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                service_name: self.service_name,
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcsServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcsServiceRef {
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

    #[doc = "Get a reference to the value of field `availability_zone_rebalancing` after provisioning.\n"]
    pub fn availability_zone_rebalancing(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_rebalancing", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `capacity_provider_strategy` after provisioning.\n"]
    pub fn capacity_provider_strategy(&self) -> terrars::SetRef<DataEcsService_CapacityProviderStrategyRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_provider_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_arn` after provisioning.\n"]
    pub fn cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_by` after provisioning.\n"]
    pub fn created_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_configuration` after provisioning.\n"]
    pub fn deployment_configuration(&self) -> terrars::ListRef<DataEcsService_DeploymentConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_controller` after provisioning.\n"]
    pub fn deployment_controller(&self) -> terrars::ListRef<DataEcsService_DeploymentControllerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_controller", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployments` after provisioning.\n"]
    pub fn deployments(&self) -> terrars::ListRef<DataEcsService_DeploymentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_ecs_managed_tags` after provisioning.\n"]
    pub fn enable_ecs_managed_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_ecs_managed_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_execute_command` after provisioning.\n"]
    pub fn enable_execute_command(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_execute_command", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::ListRef<DataEcsService_EventsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.events", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_grace_period_seconds` after provisioning.\n"]
    pub fn health_check_grace_period_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.health_check_grace_period_seconds", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `iam_role` after provisioning.\n"]
    pub fn iam_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_type` after provisioning.\n"]
    pub fn launch_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer` after provisioning.\n"]
    pub fn load_balancer(&self) -> terrars::SetRef<DataEcsService_LoadBalancerRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.load_balancer", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<DataEcsService_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ordered_placement_strategy` after provisioning.\n"]
    pub fn ordered_placement_strategy(&self) -> terrars::ListRef<DataEcsService_OrderedPlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ordered_placement_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> terrars::SetRef<DataEcsService_PlacementConstraintsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.placement_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_family` after provisioning.\n"]
    pub fn platform_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform_version` after provisioning.\n"]
    pub fn platform_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scheduling_strategy` after provisioning.\n"]
    pub fn scheduling_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheduling_strategy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_registries` after provisioning.\n"]
    pub fn service_registries(&self) -> terrars::ListRef<DataEcsService_ServiceRegistriesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_registries", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_sets` after provisioning.\n"]
    pub fn task_sets(&self) -> terrars::ListRef<DataEcsService_TaskSetsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_sets", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_CapacityProviderStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsService_CapacityProviderStrategy {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_provider`.\n"]
    pub fn set_capacity_provider(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.capacity_provider = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_CapacityProviderStrategy {
    type O = terrars::BlockAssignable<DataEcsService_CapacityProviderStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_CapacityProviderStrategy {}

impl BuildDataEcsService_CapacityProviderStrategy {
    pub fn build(self) -> DataEcsService_CapacityProviderStrategy {
        DataEcsService_CapacityProviderStrategy {
            base: core::default::Default::default(),
            capacity_provider: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_CapacityProviderStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_CapacityProviderStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_CapacityProviderStrategyRef {
        DataEcsService_CapacityProviderStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_CapacityProviderStrategyRef {
    #[allow(dead_code)]
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
pub struct DataEcsService_DeploymentConfiguration_Alarms {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarm_names: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEcsService_DeploymentConfiguration_Alarms {
    #[doc = "Set the field `alarm_names`.\n"]
    pub fn set_alarm_names(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alarm_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollback`.\n"]
    pub fn set_rollback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rollback = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration_Alarms {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration_Alarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration_Alarms {}

impl BuildDataEcsService_DeploymentConfiguration_Alarms {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration_Alarms {
        DataEcsService_DeploymentConfiguration_Alarms {
            alarm_names: core::default::Default::default(),
            enable: core::default::Default::default(),
            rollback: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfiguration_AlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfiguration_AlarmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_DeploymentConfiguration_AlarmsRef {
        DataEcsService_DeploymentConfiguration_AlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfiguration_AlarmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_names` after provisioning.\n"]
    pub fn alarm_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_names", self.base))
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentConfiguration_CanaryConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_bake_time_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsService_DeploymentConfiguration_CanaryConfiguration {
    #[doc = "Set the field `canary_bake_time_in_minutes`.\n"]
    pub fn set_canary_bake_time_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.canary_bake_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canary_percent`.\n"]
    pub fn set_canary_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.canary_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration_CanaryConfiguration {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration_CanaryConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration_CanaryConfiguration {}

impl BuildDataEcsService_DeploymentConfiguration_CanaryConfiguration {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration_CanaryConfiguration {
        DataEcsService_DeploymentConfiguration_CanaryConfiguration {
            canary_bake_time_in_minutes: core::default::Default::default(),
            canary_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfiguration_CanaryConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfiguration_CanaryConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_DeploymentConfiguration_CanaryConfigurationRef {
        DataEcsService_DeploymentConfiguration_CanaryConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfiguration_CanaryConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `canary_bake_time_in_minutes` after provisioning.\n"]
    pub fn canary_bake_time_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canary_bake_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `canary_percent` after provisioning.\n"]
    pub fn canary_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.canary_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback: std::option::Option<terrars::PrimField<bool>>,
}

impl DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {
    #[doc = "Set the field `enable`.\n"]
    pub fn set_enable(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rollback`.\n"]
    pub fn set_rollback(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.rollback = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {}

impl BuildDataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {
        DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker {
            enable: core::default::Default::default(),
            rollback: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfiguration_DeploymentCircuitBreakerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfiguration_DeploymentCircuitBreakerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_DeploymentConfiguration_DeploymentCircuitBreakerRef {
        DataEcsService_DeploymentConfiguration_DeploymentCircuitBreakerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfiguration_DeploymentCircuitBreakerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable` after provisioning.\n"]
    pub fn enable(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback` after provisioning.\n"]
    pub fn rollback(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rollback", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_in_minutes`.\n"]
    pub fn set_timeout_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.timeout_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {}

impl BuildDataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
        DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration {
            action: core::default::Default::default(),
            timeout_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
        DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_in_minutes` after provisioning.\n"]
    pub fn timeout_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentConfiguration_LifecycleHook {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hook_details: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hook_target_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_stages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>>,
}

impl DataEcsService_DeploymentConfiguration_LifecycleHook {
    #[doc = "Set the field `hook_details`.\n"]
    pub fn set_hook_details(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hook_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hook_target_arn`.\n"]
    pub fn set_hook_target_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hook_target_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_stages`.\n"]
    pub fn set_lifecycle_stages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.lifecycle_stages = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_type`.\n"]
    pub fn set_target_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout_configuration`.\n"]
    pub fn set_timeout_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfiguration>,
                    >,
    ) -> Self {
        self.timeout_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration_LifecycleHook {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration_LifecycleHook>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration_LifecycleHook {}

impl BuildDataEcsService_DeploymentConfiguration_LifecycleHook {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration_LifecycleHook {
        DataEcsService_DeploymentConfiguration_LifecycleHook {
            hook_details: core::default::Default::default(),
            hook_target_arn: core::default::Default::default(),
            lifecycle_stages: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            target_type: core::default::Default::default(),
            timeout_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfiguration_LifecycleHookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfiguration_LifecycleHookRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_DeploymentConfiguration_LifecycleHookRef {
        DataEcsService_DeploymentConfiguration_LifecycleHookRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfiguration_LifecycleHookRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `hook_details` after provisioning.\n"]
    pub fn hook_details(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hook_details", self.base))
    }

    #[doc = "Get a reference to the value of field `hook_target_arn` after provisioning.\n"]
    pub fn hook_target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hook_target_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_stages` after provisioning.\n"]
    pub fn lifecycle_stages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_stages", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_configuration` after provisioning.\n"]
    pub fn timeout_configuration(
        &self,
    ) -> terrars::ListRef<DataEcsService_DeploymentConfiguration_LifecycleHook_TimeoutConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentConfiguration_LinearConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_bake_time_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_percent: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsService_DeploymentConfiguration_LinearConfiguration {
    #[doc = "Set the field `step_bake_time_in_minutes`.\n"]
    pub fn set_step_bake_time_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.step_bake_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step_percent`.\n"]
    pub fn set_step_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.step_percent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration_LinearConfiguration {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration_LinearConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration_LinearConfiguration {}

impl BuildDataEcsService_DeploymentConfiguration_LinearConfiguration {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration_LinearConfiguration {
        DataEcsService_DeploymentConfiguration_LinearConfiguration {
            step_bake_time_in_minutes: core::default::Default::default(),
            step_percent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfiguration_LinearConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfiguration_LinearConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_DeploymentConfiguration_LinearConfigurationRef {
        DataEcsService_DeploymentConfiguration_LinearConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfiguration_LinearConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `step_bake_time_in_minutes` after provisioning.\n"]
    pub fn step_bake_time_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_bake_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `step_percent` after provisioning.\n"]
    pub fn step_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_percent", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<terrars::ListField<DataEcsService_DeploymentConfiguration_Alarms>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bake_time_in_minutes: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsService_DeploymentConfiguration_CanaryConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_circuit_breaker: std
    ::option
    ::Option<terrars::ListField<DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_hook: std::option::Option<terrars::SetField<DataEcsService_DeploymentConfiguration_LifecycleHook>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linear_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsService_DeploymentConfiguration_LinearConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_healthy_percent: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_DeploymentConfiguration {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsService_DeploymentConfiguration_Alarms>>,
    ) -> Self {
        self.alarms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bake_time_in_minutes`.\n"]
    pub fn set_bake_time_in_minutes(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bake_time_in_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `canary_configuration`.\n"]
    pub fn set_canary_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsService_DeploymentConfiguration_CanaryConfiguration>>,
    ) -> Self {
        self.canary_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_circuit_breaker`.\n"]
    pub fn set_deployment_circuit_breaker(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEcsService_DeploymentConfiguration_DeploymentCircuitBreaker>>,
    ) -> Self {
        self.deployment_circuit_breaker = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lifecycle_hook`.\n"]
    pub fn set_lifecycle_hook(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataEcsService_DeploymentConfiguration_LifecycleHook>>,
    ) -> Self {
        self.lifecycle_hook = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `linear_configuration`.\n"]
    pub fn set_linear_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsService_DeploymentConfiguration_LinearConfiguration>>,
    ) -> Self {
        self.linear_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_percent`.\n"]
    pub fn set_maximum_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_healthy_percent`.\n"]
    pub fn set_minimum_healthy_percent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.minimum_healthy_percent = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `strategy`.\n"]
    pub fn set_strategy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentConfiguration {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentConfiguration {}

impl BuildDataEcsService_DeploymentConfiguration {
    pub fn build(self) -> DataEcsService_DeploymentConfiguration {
        DataEcsService_DeploymentConfiguration {
            alarms: core::default::Default::default(),
            bake_time_in_minutes: core::default::Default::default(),
            canary_configuration: core::default::Default::default(),
            deployment_circuit_breaker: core::default::Default::default(),
            lifecycle_hook: core::default::Default::default(),
            linear_configuration: core::default::Default::default(),
            maximum_percent: core::default::Default::default(),
            minimum_healthy_percent: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_DeploymentConfigurationRef {
        DataEcsService_DeploymentConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::ListRef<DataEcsService_DeploymentConfiguration_AlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc = "Get a reference to the value of field `bake_time_in_minutes` after provisioning.\n"]
    pub fn bake_time_in_minutes(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bake_time_in_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `canary_configuration` after provisioning.\n"]
    pub fn canary_configuration(
        &self,
    ) -> terrars::ListRef<DataEcsService_DeploymentConfiguration_CanaryConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_circuit_breaker` after provisioning.\n"]
    pub fn deployment_circuit_breaker(
        &self,
    ) -> terrars::ListRef<DataEcsService_DeploymentConfiguration_DeploymentCircuitBreakerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_circuit_breaker", self.base))
    }

    #[doc = "Get a reference to the value of field `lifecycle_hook` after provisioning.\n"]
    pub fn lifecycle_hook(&self) -> terrars::SetRef<DataEcsService_DeploymentConfiguration_LifecycleHookRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.lifecycle_hook", self.base))
    }

    #[doc = "Get a reference to the value of field `linear_configuration` after provisioning.\n"]
    pub fn linear_configuration(
        &self,
    ) -> terrars::ListRef<DataEcsService_DeploymentConfiguration_LinearConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linear_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_percent` after provisioning.\n"]
    pub fn maximum_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_percent` after provisioning.\n"]
    pub fn minimum_healthy_percent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.minimum_healthy_percent", self.base))
    }

    #[doc = "Get a reference to the value of field `strategy` after provisioning.\n"]
    pub fn strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_DeploymentController {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_DeploymentController {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_DeploymentController {
    type O = terrars::BlockAssignable<DataEcsService_DeploymentController>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_DeploymentController {}

impl BuildDataEcsService_DeploymentController {
    pub fn build(self) -> DataEcsService_DeploymentController {
        DataEcsService_DeploymentController { type_: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentControllerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentControllerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_DeploymentControllerRef {
        DataEcsService_DeploymentControllerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentControllerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_Deployments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    running_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_Deployments {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_count`.\n"]
    pub fn set_desired_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.desired_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_count`.\n"]
    pub fn set_pending_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `running_count`.\n"]
    pub fn set_running_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.running_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_definition`.\n"]
    pub fn set_task_definition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.task_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_Deployments {
    type O = terrars::BlockAssignable<DataEcsService_Deployments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_Deployments {}

impl BuildDataEcsService_Deployments {
    pub fn build(self) -> DataEcsService_Deployments {
        DataEcsService_Deployments {
            created_at: core::default::Default::default(),
            desired_count: core::default::Default::default(),
            id: core::default::Default::default(),
            pending_count: core::default::Default::default(),
            running_count: core::default::Default::default(),
            status: core::default::Default::default(),
            task_definition: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_DeploymentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_DeploymentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_DeploymentsRef {
        DataEcsService_DeploymentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_DeploymentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_count` after provisioning.\n"]
    pub fn desired_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_count", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_count", self.base))
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_count", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_Events {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_Events {
    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_Events {
    type O = terrars::BlockAssignable<DataEcsService_Events>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_Events {}

impl BuildDataEcsService_Events {
    pub fn build(self) -> DataEcsService_Events {
        DataEcsService_Events {
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            message: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_EventsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_EventsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_EventsRef {
        DataEcsService_EventsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_EventsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_LoadBalancer_AdvancedConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alternate_target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    production_listener_rule: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_listener_rule: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_LoadBalancer_AdvancedConfiguration {
    #[doc = "Set the field `alternate_target_group_arn`.\n"]
    pub fn set_alternate_target_group_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.alternate_target_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `production_listener_rule`.\n"]
    pub fn set_production_listener_rule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.production_listener_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `role_arn`.\n"]
    pub fn set_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `test_listener_rule`.\n"]
    pub fn set_test_listener_rule(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.test_listener_rule = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_LoadBalancer_AdvancedConfiguration {
    type O = terrars::BlockAssignable<DataEcsService_LoadBalancer_AdvancedConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_LoadBalancer_AdvancedConfiguration {}

impl BuildDataEcsService_LoadBalancer_AdvancedConfiguration {
    pub fn build(self) -> DataEcsService_LoadBalancer_AdvancedConfiguration {
        DataEcsService_LoadBalancer_AdvancedConfiguration {
            alternate_target_group_arn: core::default::Default::default(),
            production_listener_rule: core::default::Default::default(),
            role_arn: core::default::Default::default(),
            test_listener_rule: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_LoadBalancer_AdvancedConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_LoadBalancer_AdvancedConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsService_LoadBalancer_AdvancedConfigurationRef {
        DataEcsService_LoadBalancer_AdvancedConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_LoadBalancer_AdvancedConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternate_target_group_arn` after provisioning.\n"]
    pub fn alternate_target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alternate_target_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `production_listener_rule` after provisioning.\n"]
    pub fn production_listener_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.production_listener_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `test_listener_rule` after provisioning.\n"]
    pub fn test_listener_rule(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.test_listener_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_LoadBalancer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    advanced_configuration: std::option::Option<terrars::ListField<DataEcsService_LoadBalancer_AdvancedConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elb_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_LoadBalancer {
    #[doc = "Set the field `advanced_configuration`.\n"]
    pub fn set_advanced_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsService_LoadBalancer_AdvancedConfiguration>>,
    ) -> Self {
        self.advanced_configuration = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `elb_name`.\n"]
    pub fn set_elb_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.elb_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_group_arn`.\n"]
    pub fn set_target_group_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_group_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_LoadBalancer {
    type O = terrars::BlockAssignable<DataEcsService_LoadBalancer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_LoadBalancer {}

impl BuildDataEcsService_LoadBalancer {
    pub fn build(self) -> DataEcsService_LoadBalancer {
        DataEcsService_LoadBalancer {
            advanced_configuration: core::default::Default::default(),
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            elb_name: core::default::Default::default(),
            target_group_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_LoadBalancerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_LoadBalancerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_LoadBalancerRef {
        DataEcsService_LoadBalancerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_LoadBalancerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `advanced_configuration` after provisioning.\n"]
    pub fn advanced_configuration(&self) -> terrars::ListRef<DataEcsService_LoadBalancer_AdvancedConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.advanced_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `elb_name` after provisioning.\n"]
    pub fn elb_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elb_name", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group_arn` after provisioning.\n"]
    pub fn target_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_group_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataEcsService_NetworkConfiguration {
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

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_NetworkConfiguration {
    type O = terrars::BlockAssignable<DataEcsService_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_NetworkConfiguration {}

impl BuildDataEcsService_NetworkConfiguration {
    pub fn build(self) -> DataEcsService_NetworkConfiguration {
        DataEcsService_NetworkConfiguration {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_NetworkConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_NetworkConfigurationRef {
        DataEcsService_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_NetworkConfigurationRef {
    #[allow(dead_code)]
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
pub struct DataEcsService_OrderedPlacementStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_OrderedPlacementStrategy {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_OrderedPlacementStrategy {
    type O = terrars::BlockAssignable<DataEcsService_OrderedPlacementStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_OrderedPlacementStrategy {}

impl BuildDataEcsService_OrderedPlacementStrategy {
    pub fn build(self) -> DataEcsService_OrderedPlacementStrategy {
        DataEcsService_OrderedPlacementStrategy {
            field: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_OrderedPlacementStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_OrderedPlacementStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_OrderedPlacementStrategyRef {
        DataEcsService_OrderedPlacementStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_OrderedPlacementStrategyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsService_PlacementConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_PlacementConstraints {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_PlacementConstraints {
    type O = terrars::BlockAssignable<DataEcsService_PlacementConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_PlacementConstraints {}

impl BuildDataEcsService_PlacementConstraints {
    pub fn build(self) -> DataEcsService_PlacementConstraints {
        DataEcsService_PlacementConstraints {
            expression: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_PlacementConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_PlacementConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_PlacementConstraintsRef {
        DataEcsService_PlacementConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_PlacementConstraintsRef {
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
pub struct DataEcsService_ServiceRegistries {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    registry_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_ServiceRegistries {
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

    #[doc = "Set the field `registry_arn`.\n"]
    pub fn set_registry_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.registry_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_ServiceRegistries {
    type O = terrars::BlockAssignable<DataEcsService_ServiceRegistries>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_ServiceRegistries {}

impl BuildDataEcsService_ServiceRegistries {
    pub fn build(self) -> DataEcsService_ServiceRegistries {
        DataEcsService_ServiceRegistries {
            container_name: core::default::Default::default(),
            container_port: core::default::Default::default(),
            port: core::default::Default::default(),
            registry_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_ServiceRegistriesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_ServiceRegistriesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_ServiceRegistriesRef {
        DataEcsService_ServiceRegistriesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_ServiceRegistriesRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataEcsService_TaskSets {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pending_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    running_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stability_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_definition: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsService_TaskSets {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `pending_count`.\n"]
    pub fn set_pending_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.pending_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `running_count`.\n"]
    pub fn set_running_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.running_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stability_status`.\n"]
    pub fn set_stability_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.stability_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_definition`.\n"]
    pub fn set_task_definition(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.task_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.updated_at = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsService_TaskSets {
    type O = terrars::BlockAssignable<DataEcsService_TaskSets>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsService_TaskSets {}

impl BuildDataEcsService_TaskSets {
    pub fn build(self) -> DataEcsService_TaskSets {
        DataEcsService_TaskSets {
            arn: core::default::Default::default(),
            created_at: core::default::Default::default(),
            id: core::default::Default::default(),
            pending_count: core::default::Default::default(),
            running_count: core::default::Default::default(),
            stability_status: core::default::Default::default(),
            status: core::default::Default::default(),
            task_definition: core::default::Default::default(),
            updated_at: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsService_TaskSetsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsService_TaskSetsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsService_TaskSetsRef {
        DataEcsService_TaskSetsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsService_TaskSetsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `pending_count` after provisioning.\n"]
    pub fn pending_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pending_count", self.base))
    }

    #[doc = "Get a reference to the value of field `running_count` after provisioning.\n"]
    pub fn running_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.running_count", self.base))
    }

    #[doc = "Get a reference to the value of field `stability_status` after provisioning.\n"]
    pub fn stability_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stability_status", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.base))
    }
}
