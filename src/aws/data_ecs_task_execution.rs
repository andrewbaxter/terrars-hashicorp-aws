#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcsTaskExecutionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_token: std::option::Option<terrars::PrimField<std::string::String>>,
    cluster: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    desired_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_ecs_managed_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_execute_command: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    platform_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    propagate_tags: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reference_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    started_by: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    task_definition: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_provider_strategy: std::option::Option<std::vec::Vec<DataEcsTaskExecution_CapacityProviderStrategy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<std::vec::Vec<DataEcsTaskExecution_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    overrides: std::option::Option<std::vec::Vec<DataEcsTaskExecution_Overrides>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_constraints: std::option::Option<std::vec::Vec<DataEcsTaskExecution_PlacementConstraints>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_strategy: std::option::Option<std::vec::Vec<DataEcsTaskExecution_PlacementStrategy>>,
    dynamic: DataEcsTaskExecutionDynamic,
}

struct DataEcsTaskExecution_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcsTaskExecutionData_>,
}

#[derive(Clone)]
pub struct DataEcsTaskExecution(std::rc::Rc<DataEcsTaskExecution_>);

impl DataEcsTaskExecution {
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

    #[doc = "Set the field `client_token`.\n"]
    pub fn set_client_token(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().client_token = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `desired_count`.\n"]
    pub fn set_desired_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().desired_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_ecs_managed_tags`.\n"]
    pub fn set_enable_ecs_managed_tags(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_ecs_managed_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_execute_command`.\n"]
    pub fn set_enable_execute_command(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_execute_command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group`.\n"]
    pub fn set_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().group = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `propagate_tags`.\n"]
    pub fn set_propagate_tags(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().propagate_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `reference_id`.\n"]
    pub fn set_reference_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().reference_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `started_by`.\n"]
    pub fn set_started_by(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().started_by = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `capacity_provider_strategy`.\n"]
    pub fn set_capacity_provider_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcsTaskExecution_CapacityProviderStrategy>>,
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

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcsTaskExecution_NetworkConfiguration>>,
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

    #[doc = "Set the field `overrides`.\n"]
    pub fn set_overrides(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcsTaskExecution_Overrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.overrides = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_constraints`.\n"]
    pub fn set_placement_constraints(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcsTaskExecution_PlacementConstraints>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_constraints = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_constraints = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcsTaskExecution_PlacementStrategy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement_strategy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement_strategy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `started_by` after provisioning.\n"]
    pub fn started_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arns` after provisioning.\n"]
    pub fn task_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<DataEcsTaskExecution_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> terrars::ListRef<DataEcsTaskExecution_OverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> terrars::ListRef<DataEcsTaskExecution_PlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_strategy", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcsTaskExecution {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcsTaskExecution { }

impl terrars::ToListMappable for DataEcsTaskExecution {
    type O = terrars::ListRef<DataEcsTaskExecutionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcsTaskExecution_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecs_task_execution".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcsTaskExecution {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub task_definition: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcsTaskExecution {
        let out = DataEcsTaskExecution(std::rc::Rc::new(DataEcsTaskExecution_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcsTaskExecutionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                client_token: core::default::Default::default(),
                cluster: self.cluster,
                desired_count: core::default::Default::default(),
                enable_ecs_managed_tags: core::default::Default::default(),
                enable_execute_command: core::default::Default::default(),
                group: core::default::Default::default(),
                id: core::default::Default::default(),
                launch_type: core::default::Default::default(),
                platform_version: core::default::Default::default(),
                propagate_tags: core::default::Default::default(),
                reference_id: core::default::Default::default(),
                region: core::default::Default::default(),
                started_by: core::default::Default::default(),
                tags: core::default::Default::default(),
                task_definition: self.task_definition,
                capacity_provider_strategy: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                overrides: core::default::Default::default(),
                placement_constraints: core::default::Default::default(),
                placement_strategy: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcsTaskExecutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecutionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcsTaskExecutionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `client_token` after provisioning.\n"]
    pub fn client_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `group` after provisioning.\n"]
    pub fn group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `propagate_tags` after provisioning.\n"]
    pub fn propagate_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.propagate_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reference_id` after provisioning.\n"]
    pub fn reference_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reference_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `started_by` after provisioning.\n"]
    pub fn started_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_arns` after provisioning.\n"]
    pub fn task_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.task_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<DataEcsTaskExecution_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `overrides` after provisioning.\n"]
    pub fn overrides(&self) -> terrars::ListRef<DataEcsTaskExecution_OverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.overrides", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> terrars::ListRef<DataEcsTaskExecution_PlacementStrategyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_strategy", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEcsTaskExecutionDynamic {
    capacity_provider_strategy: std
    ::option
    ::Option<terrars::DynamicBlock<DataEcsTaskExecution_CapacityProviderStrategy>>,
    network_configuration: std::option::Option<terrars::DynamicBlock<DataEcsTaskExecution_NetworkConfiguration>>,
    overrides: std::option::Option<terrars::DynamicBlock<DataEcsTaskExecution_Overrides>>,
    placement_constraints: std::option::Option<terrars::DynamicBlock<DataEcsTaskExecution_PlacementConstraints>>,
    placement_strategy: std::option::Option<terrars::DynamicBlock<DataEcsTaskExecution_PlacementStrategy>>,
}

#[derive(Serialize)]
pub struct DataEcsTaskExecution_CapacityProviderStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base: std::option::Option<terrars::PrimField<f64>>,
    capacity_provider: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsTaskExecution_CapacityProviderStrategy {
    #[doc = "Set the field `base`.\n"]
    pub fn set_base(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.base = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskExecution_CapacityProviderStrategy {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_CapacityProviderStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_CapacityProviderStrategy {
    #[doc = ""]
    pub capacity_provider: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution_CapacityProviderStrategy {
    pub fn build(self) -> DataEcsTaskExecution_CapacityProviderStrategy {
        DataEcsTaskExecution_CapacityProviderStrategy {
            base: core::default::Default::default(),
            capacity_provider: self.capacity_provider,
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_CapacityProviderStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_CapacityProviderStrategyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskExecution_CapacityProviderStrategyRef {
        DataEcsTaskExecution_CapacityProviderStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_CapacityProviderStrategyRef {
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
pub struct DataEcsTaskExecution_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_public_ip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskExecution_NetworkConfiguration {
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

impl terrars::ToListMappable for DataEcsTaskExecution_NetworkConfiguration {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_NetworkConfiguration {
    #[doc = ""]
    pub subnets: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEcsTaskExecution_NetworkConfiguration {
    pub fn build(self) -> DataEcsTaskExecution_NetworkConfiguration {
        DataEcsTaskExecution_NetworkConfiguration {
            assign_public_ip: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnets: self.subnets,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_NetworkConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskExecution_NetworkConfigurationRef {
        DataEcsTaskExecution_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_NetworkConfigurationRef {
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
pub struct DataEcsTaskExecution_Overrides_ContainerOverrides_Environment {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl DataEcsTaskExecution_Overrides_ContainerOverrides_Environment { }

impl terrars::ToListMappable for DataEcsTaskExecution_Overrides_ContainerOverrides_Environment {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_Overrides_ContainerOverrides_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_Overrides_ContainerOverrides_Environment {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution_Overrides_ContainerOverrides_Environment {
    pub fn build(self) -> DataEcsTaskExecution_Overrides_ContainerOverrides_Environment {
        DataEcsTaskExecution_Overrides_ContainerOverrides_Environment {
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_Overrides_ContainerOverrides_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_Overrides_ContainerOverrides_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskExecution_Overrides_ContainerOverrides_EnvironmentRef {
        DataEcsTaskExecution_Overrides_ContainerOverrides_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_Overrides_ContainerOverrides_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements { }

impl terrars::ToListMappable for DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements {
    pub fn build(self) -> DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements {
        DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirementsRef {
        DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcsTaskExecution_Overrides_ContainerOverridesDynamic {
    environment: std
    ::option
    ::Option<terrars::DynamicBlock<DataEcsTaskExecution_Overrides_ContainerOverrides_Environment>>,
    resource_requirements: std
    ::option
    ::Option<terrars::DynamicBlock<DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements>>,
}

#[derive(Serialize)]
pub struct DataEcsTaskExecution_Overrides_ContainerOverrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_reservation: std::option::Option<terrars::PrimField<f64>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<DataEcsTaskExecution_Overrides_ContainerOverrides_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_requirements: std
    ::option
    ::Option<std::vec::Vec<DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements>>,
    dynamic: DataEcsTaskExecution_Overrides_ContainerOverridesDynamic,
}

impl DataEcsTaskExecution_Overrides_ContainerOverrides {
    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_reservation`.\n"]
    pub fn set_memory_reservation(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_reservation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataEcsTaskExecution_Overrides_ContainerOverrides_Environment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_requirements`.\n"]
    pub fn set_resource_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DataEcsTaskExecution_Overrides_ContainerOverrides_ResourceRequirements>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_requirements = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskExecution_Overrides_ContainerOverrides {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_Overrides_ContainerOverrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_Overrides_ContainerOverrides {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution_Overrides_ContainerOverrides {
    pub fn build(self) -> DataEcsTaskExecution_Overrides_ContainerOverrides {
        DataEcsTaskExecution_Overrides_ContainerOverrides {
            command: core::default::Default::default(),
            cpu: core::default::Default::default(),
            memory: core::default::Default::default(),
            memory_reservation: core::default::Default::default(),
            name: self.name,
            environment: core::default::Default::default(),
            resource_requirements: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_Overrides_ContainerOverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_Overrides_ContainerOverridesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskExecution_Overrides_ContainerOverridesRef {
        DataEcsTaskExecution_Overrides_ContainerOverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_Overrides_ContainerOverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_reservation` after provisioning.\n"]
    pub fn memory_reservation(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_reservation", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEcsTaskExecution_OverridesDynamic {
    container_overrides: std
    ::option
    ::Option<terrars::DynamicBlock<DataEcsTaskExecution_Overrides_ContainerOverrides>>,
}

#[derive(Serialize)]
pub struct DataEcsTaskExecution_Overrides {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_overrides: std::option::Option<std::vec::Vec<DataEcsTaskExecution_Overrides_ContainerOverrides>>,
    dynamic: DataEcsTaskExecution_OverridesDynamic,
}

impl DataEcsTaskExecution_Overrides {
    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.task_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_overrides`.\n"]
    pub fn set_container_overrides(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEcsTaskExecution_Overrides_ContainerOverrides>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_overrides = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_overrides = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskExecution_Overrides {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_Overrides>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_Overrides {}

impl BuildDataEcsTaskExecution_Overrides {
    pub fn build(self) -> DataEcsTaskExecution_Overrides {
        DataEcsTaskExecution_Overrides {
            cpu: core::default::Default::default(),
            execution_role_arn: core::default::Default::default(),
            memory: core::default::Default::default(),
            task_role_arn: core::default::Default::default(),
            container_overrides: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_OverridesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_OverridesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskExecution_OverridesRef {
        DataEcsTaskExecution_OverridesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_OverridesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.base))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `container_overrides` after provisioning.\n"]
    pub fn container_overrides(&self) -> terrars::ListRef<DataEcsTaskExecution_Overrides_ContainerOverridesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_overrides", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskExecution_PlacementConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataEcsTaskExecution_PlacementConstraints {
    #[doc = "Set the field `expression`.\n"]
    pub fn set_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.expression = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskExecution_PlacementConstraints {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_PlacementConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_PlacementConstraints {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution_PlacementConstraints {
    pub fn build(self) -> DataEcsTaskExecution_PlacementConstraints {
        DataEcsTaskExecution_PlacementConstraints {
            expression: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_PlacementConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_PlacementConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskExecution_PlacementConstraintsRef {
        DataEcsTaskExecution_PlacementConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_PlacementConstraintsRef {
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
pub struct DataEcsTaskExecution_PlacementStrategy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl DataEcsTaskExecution_PlacementStrategy {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskExecution_PlacementStrategy {
    type O = terrars::BlockAssignable<DataEcsTaskExecution_PlacementStrategy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskExecution_PlacementStrategy {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskExecution_PlacementStrategy {
    pub fn build(self) -> DataEcsTaskExecution_PlacementStrategy {
        DataEcsTaskExecution_PlacementStrategy {
            field: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskExecution_PlacementStrategyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskExecution_PlacementStrategyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskExecution_PlacementStrategyRef {
        DataEcsTaskExecution_PlacementStrategyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskExecution_PlacementStrategyRef {
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
