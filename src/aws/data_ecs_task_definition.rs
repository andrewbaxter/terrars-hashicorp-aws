#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEcsTaskDefinitionData_ {
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
    task_definition: terrars::PrimField<std::string::String>,
}

struct DataEcsTaskDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEcsTaskDefinitionData_>,
}

#[derive(Clone)]
pub struct DataEcsTaskDefinition(std::rc::Rc<DataEcsTaskDefinition_>);

impl DataEcsTaskDefinition {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn_without_revision` after provisioning.\n"]
    pub fn arn_without_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_without_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_fault_injection` after provisioning.\n"]
    pub fn enable_fault_injection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fault_injection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<DataEcsTaskDefinition_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipc_mode` after provisioning.\n"]
    pub fn ipc_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pid_mode` after provisioning.\n"]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> terrars::SetRef<DataEcsTaskDefinition_PlacementConstraintsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.placement_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> terrars::ListRef<DataEcsTaskDefinition_ProxyConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requires_compatibilities` after provisioning.\n"]
    pub fn requires_compatibilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.requires_compatibilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> terrars::ListRef<DataEcsTaskDefinition_RuntimePlatformRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume` after provisioning.\n"]
    pub fn volume(&self) -> terrars::SetRef<DataEcsTaskDefinition_VolumeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.volume", self.extract_ref()))
    }
}

impl terrars::Referable for DataEcsTaskDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEcsTaskDefinition { }

impl terrars::ToListMappable for DataEcsTaskDefinition {
    type O = terrars::ListRef<DataEcsTaskDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEcsTaskDefinition_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ecs_task_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEcsTaskDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub task_definition: terrars::PrimField<std::string::String>,
}

impl BuildDataEcsTaskDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEcsTaskDefinition {
        let out = DataEcsTaskDefinition(std::rc::Rc::new(DataEcsTaskDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEcsTaskDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                task_definition: self.task_definition,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEcsTaskDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEcsTaskDefinitionRef {
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

    #[doc = "Get a reference to the value of field `arn_without_revision` after provisioning.\n"]
    pub fn arn_without_revision(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn_without_revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `container_definitions` after provisioning.\n"]
    pub fn container_definitions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_definitions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_fault_injection` after provisioning.\n"]
    pub fn enable_fault_injection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_fault_injection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ephemeral_storage` after provisioning.\n"]
    pub fn ephemeral_storage(&self) -> terrars::ListRef<DataEcsTaskDefinition_EphemeralStorageRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ephemeral_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `family` after provisioning.\n"]
    pub fn family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipc_mode` after provisioning.\n"]
    pub fn ipc_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipc_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_mode` after provisioning.\n"]
    pub fn network_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pid_mode` after provisioning.\n"]
    pub fn pid_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pid_mode", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_constraints` after provisioning.\n"]
    pub fn placement_constraints(&self) -> terrars::SetRef<DataEcsTaskDefinition_PlacementConstraintsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.placement_constraints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `proxy_configuration` after provisioning.\n"]
    pub fn proxy_configuration(&self) -> terrars::ListRef<DataEcsTaskDefinition_ProxyConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.proxy_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requires_compatibilities` after provisioning.\n"]
    pub fn requires_compatibilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.requires_compatibilities", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\n"]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_platform` after provisioning.\n"]
    pub fn runtime_platform(&self) -> terrars::ListRef<DataEcsTaskDefinition_RuntimePlatformRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_definition` after provisioning.\n"]
    pub fn task_definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_definition", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume` after provisioning.\n"]
    pub fn volume(&self) -> terrars::SetRef<DataEcsTaskDefinition_VolumeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.volume", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_EphemeralStorage {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size_in_gib: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsTaskDefinition_EphemeralStorage {
    #[doc = "Set the field `size_in_gib`.\n"]
    pub fn set_size_in_gib(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size_in_gib = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_EphemeralStorage {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_EphemeralStorage>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_EphemeralStorage {}

impl BuildDataEcsTaskDefinition_EphemeralStorage {
    pub fn build(self) -> DataEcsTaskDefinition_EphemeralStorage {
        DataEcsTaskDefinition_EphemeralStorage { size_in_gib: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_EphemeralStorageRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_EphemeralStorageRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskDefinition_EphemeralStorageRef {
        DataEcsTaskDefinition_EphemeralStorageRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_EphemeralStorageRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `size_in_gib` after provisioning.\n"]
    pub fn size_in_gib(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size_in_gib", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_PlacementConstraints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_PlacementConstraints {
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

impl terrars::ToListMappable for DataEcsTaskDefinition_PlacementConstraints {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_PlacementConstraints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_PlacementConstraints {}

impl BuildDataEcsTaskDefinition_PlacementConstraints {
    pub fn build(self) -> DataEcsTaskDefinition_PlacementConstraints {
        DataEcsTaskDefinition_PlacementConstraints {
            expression: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_PlacementConstraintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_PlacementConstraintsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskDefinition_PlacementConstraintsRef {
        DataEcsTaskDefinition_PlacementConstraintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_PlacementConstraintsRef {
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
pub struct DataEcsTaskDefinition_ProxyConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_ProxyConfiguration {
    #[doc = "Set the field `container_name`.\n"]
    pub fn set_container_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.container_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_ProxyConfiguration {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_ProxyConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_ProxyConfiguration {}

impl BuildDataEcsTaskDefinition_ProxyConfiguration {
    pub fn build(self) -> DataEcsTaskDefinition_ProxyConfiguration {
        DataEcsTaskDefinition_ProxyConfiguration {
            container_name: core::default::Default::default(),
            properties: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_ProxyConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_ProxyConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskDefinition_ProxyConfigurationRef {
        DataEcsTaskDefinition_ProxyConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_ProxyConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `container_name` after provisioning.\n"]
    pub fn container_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_name", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_RuntimePlatform {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system_family: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_RuntimePlatform {
    #[doc = "Set the field `cpu_architecture`.\n"]
    pub fn set_cpu_architecture(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system_family`.\n"]
    pub fn set_operating_system_family(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.operating_system_family = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_RuntimePlatform {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_RuntimePlatform>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_RuntimePlatform {}

impl BuildDataEcsTaskDefinition_RuntimePlatform {
    pub fn build(self) -> DataEcsTaskDefinition_RuntimePlatform {
        DataEcsTaskDefinition_RuntimePlatform {
            cpu_architecture: core::default::Default::default(),
            operating_system_family: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_RuntimePlatformRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_RuntimePlatformRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskDefinition_RuntimePlatformRef {
        DataEcsTaskDefinition_RuntimePlatformRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_RuntimePlatformRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_architecture` after provisioning.\n"]
    pub fn cpu_architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `operating_system_family` after provisioning.\n"]
    pub fn operating_system_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system_family", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume_DockerVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoprovision: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    driver_opts: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    labels: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scope: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_Volume_DockerVolumeConfiguration {
    #[doc = "Set the field `autoprovision`.\n"]
    pub fn set_autoprovision(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.autoprovision = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver`.\n"]
    pub fn set_driver(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.driver = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `driver_opts`.\n"]
    pub fn set_driver_opts(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.driver_opts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `labels`.\n"]
    pub fn set_labels(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.labels = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scope`.\n"]
    pub fn set_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scope = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_Volume_DockerVolumeConfiguration {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_Volume_DockerVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume_DockerVolumeConfiguration {}

impl BuildDataEcsTaskDefinition_Volume_DockerVolumeConfiguration {
    pub fn build(self) -> DataEcsTaskDefinition_Volume_DockerVolumeConfiguration {
        DataEcsTaskDefinition_Volume_DockerVolumeConfiguration {
            autoprovision: core::default::Default::default(),
            driver: core::default::Default::default(),
            driver_opts: core::default::Default::default(),
            labels: core::default::Default::default(),
            scope: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
        DataEcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_Volume_DockerVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoprovision` after provisioning.\n"]
    pub fn autoprovision(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoprovision", self.base))
    }

    #[doc = "Get a reference to the value of field `driver` after provisioning.\n"]
    pub fn driver(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.driver", self.base))
    }

    #[doc = "Get a reference to the value of field `driver_opts` after provisioning.\n"]
    pub fn driver_opts(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.driver_opts", self.base))
    }

    #[doc = "Get a reference to the value of field `labels` after provisioning.\n"]
    pub fn labels(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.labels", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    #[doc = "Set the field `access_point_id`.\n"]
    pub fn set_access_point_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam`.\n"]
    pub fn set_iam(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.iam = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {}

impl BuildDataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
    pub fn build(self) -> DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
        DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig {
            access_point_id: core::default::Default::default(),
            iam: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
        DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_id` after provisioning.\n"]
    pub fn access_point_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_id", self.base))
    }

    #[doc = "Get a reference to the value of field `iam` after provisioning.\n"]
    pub fn iam(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume_EfsVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std
    ::option
    ::Option<terrars::ListField<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsTaskDefinition_Volume_EfsVolumeConfiguration {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfig>,
                    >,
    ) -> Self {
        self.authorization_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption`.\n"]
    pub fn set_transit_encryption(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption_port`.\n"]
    pub fn set_transit_encryption_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transit_encryption_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_Volume_EfsVolumeConfiguration {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume_EfsVolumeConfiguration {}

impl BuildDataEcsTaskDefinition_Volume_EfsVolumeConfiguration {
    pub fn build(self) -> DataEcsTaskDefinition_Volume_EfsVolumeConfiguration {
        DataEcsTaskDefinition_Volume_EfsVolumeConfiguration {
            authorization_config: core::default::Default::default(),
            file_system_id: core::default::Default::default(),
            root_directory: core::default::Default::default(),
            transit_encryption: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
        DataEcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_Volume_EfsVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> terrars::ListRef<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration_AuthorizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption` after provisioning.\n"]
    pub fn transit_encryption(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"]
    pub fn transit_encryption_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credentials_parameter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    domain: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    #[doc = "Set the field `credentials_parameter`.\n"]
    pub fn set_credentials_parameter(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.credentials_parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `domain`.\n"]
    pub fn set_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.domain = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    type O =
        terrars
        ::BlockAssignable<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {}

impl BuildDataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
    pub fn build(self) -> DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
        DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig {
            credentials_parameter: core::default::Default::default(),
            domain: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
        DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_parameter` after provisioning.\n"]
    pub fn credentials_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `domain` after provisioning.\n"]
    pub fn domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    authorization_config: std
    ::option
    ::Option<
        terrars::ListField<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    #[doc = "Set the field `authorization_config`.\n"]
    pub fn set_authorization_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfig,
                        >,
                    >,
    ) -> Self {
        self.authorization_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_id`.\n"]
    pub fn set_file_system_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_directory = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {}

impl BuildDataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
    pub fn build(self) -> DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
        DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration {
            authorization_config: core::default::Default::default(),
            file_system_id: core::default::Default::default(),
            root_directory: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
        DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `authorization_config` after provisioning.\n"]
    pub fn authorization_config(
        &self,
    ) -> terrars
    ::ListRef<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration_AuthorizationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.authorization_config", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_id` after provisioning.\n"]
    pub fn file_system_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_id", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_point_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_system_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_directory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_encryption_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    #[doc = "Set the field `access_point_arn`.\n"]
    pub fn set_access_point_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_point_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_system_arn`.\n"]
    pub fn set_file_system_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_system_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_directory`.\n"]
    pub fn set_root_directory(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.root_directory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_encryption_port`.\n"]
    pub fn set_transit_encryption_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.transit_encryption_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {}

impl BuildDataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
    pub fn build(self) -> DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
        DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration {
            access_point_arn: core::default::Default::default(),
            file_system_arn: core::default::Default::default(),
            root_directory: core::default::Default::default(),
            transit_encryption_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
        DataEcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_Volume_S3filesVolumeConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_point_arn` after provisioning.\n"]
    pub fn access_point_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_point_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `file_system_arn` after provisioning.\n"]
    pub fn file_system_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_system_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `root_directory` after provisioning.\n"]
    pub fn root_directory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_directory", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_encryption_port` after provisioning.\n"]
    pub fn transit_encryption_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_encryption_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEcsTaskDefinition_Volume {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configure_at_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    docker_volume_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsTaskDefinition_Volume_DockerVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    efs_volume_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fsx_windows_file_server_volume_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_path: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3files_volume_configuration: std
    ::option
    ::Option<terrars::ListField<DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration>>,
}

impl DataEcsTaskDefinition_Volume {
    #[doc = "Set the field `configure_at_launch`.\n"]
    pub fn set_configure_at_launch(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.configure_at_launch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `docker_volume_configuration`.\n"]
    pub fn set_docker_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsTaskDefinition_Volume_DockerVolumeConfiguration>>,
    ) -> Self {
        self.docker_volume_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `efs_volume_configuration`.\n"]
    pub fn set_efs_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsTaskDefinition_Volume_EfsVolumeConfiguration>>,
    ) -> Self {
        self.efs_volume_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `fsx_windows_file_server_volume_configuration`.\n"]
    pub fn set_fsx_windows_file_server_volume_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfiguration>>,
    ) -> Self {
        self.fsx_windows_file_server_volume_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_path`.\n"]
    pub fn set_host_path(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3files_volume_configuration`.\n"]
    pub fn set_s3files_volume_configuration(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEcsTaskDefinition_Volume_S3filesVolumeConfiguration>>,
    ) -> Self {
        self.s3files_volume_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEcsTaskDefinition_Volume {
    type O = terrars::BlockAssignable<DataEcsTaskDefinition_Volume>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEcsTaskDefinition_Volume {}

impl BuildDataEcsTaskDefinition_Volume {
    pub fn build(self) -> DataEcsTaskDefinition_Volume {
        DataEcsTaskDefinition_Volume {
            configure_at_launch: core::default::Default::default(),
            docker_volume_configuration: core::default::Default::default(),
            efs_volume_configuration: core::default::Default::default(),
            fsx_windows_file_server_volume_configuration: core::default::Default::default(),
            host_path: core::default::Default::default(),
            name: core::default::Default::default(),
            s3files_volume_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEcsTaskDefinition_VolumeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEcsTaskDefinition_VolumeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEcsTaskDefinition_VolumeRef {
        DataEcsTaskDefinition_VolumeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEcsTaskDefinition_VolumeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configure_at_launch` after provisioning.\n"]
    pub fn configure_at_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configure_at_launch", self.base))
    }

    #[doc = "Get a reference to the value of field `docker_volume_configuration` after provisioning.\n"]
    pub fn docker_volume_configuration(
        &self,
    ) -> terrars::ListRef<DataEcsTaskDefinition_Volume_DockerVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.docker_volume_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `efs_volume_configuration` after provisioning.\n"]
    pub fn efs_volume_configuration(&self) -> terrars::ListRef<DataEcsTaskDefinition_Volume_EfsVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.efs_volume_configuration", self.base))
    }

    #[doc =
        "Get a reference to the value of field `fsx_windows_file_server_volume_configuration` after provisioning.\n"]
    pub fn fsx_windows_file_server_volume_configuration(
        &self,
    ) -> terrars::ListRef<DataEcsTaskDefinition_Volume_FsxWindowsFileServerVolumeConfigurationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.fsx_windows_file_server_volume_configuration", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `host_path` after provisioning.\n"]
    pub fn host_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3files_volume_configuration` after provisioning.\n"]
    pub fn s3files_volume_configuration(
        &self,
    ) -> terrars::ListRef<DataEcsTaskDefinition_Volume_S3filesVolumeConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3files_volume_configuration", self.base))
    }
}
