#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerEndpointData_ {
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
    endpoint_config_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_config: std::option::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig>>,
    dynamic: SagemakerEndpointDynamic,
}

struct SagemakerEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerEndpointData_>,
}

#[derive(Clone)]
pub struct SagemakerEndpoint(std::rc::Rc<SagemakerEndpoint_>);

impl SagemakerEndpoint {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_config`.\n"]
    pub fn set_deployment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_config_name` after provisioning.\n"]
    pub fn endpoint_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_config_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerEndpoint {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerEndpoint { }

impl terrars::ToListMappable for SagemakerEndpoint {
    type O = terrars::ListRef<SagemakerEndpointRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerEndpoint_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_endpoint".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerEndpoint {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub endpoint_config_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpoint {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerEndpoint {
        let out = SagemakerEndpoint(std::rc::Rc::new(SagemakerEndpoint_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerEndpointData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                endpoint_config_name: self.endpoint_config_name,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                deployment_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerEndpointRef {
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

    #[doc = "Get a reference to the value of field `endpoint_config_name` after provisioning.\n"]
    pub fn endpoint_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_config_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDynamic {
    deployment_config: std::option::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    alarm_name: terrars::PrimField<std::string::String>,
}

impl SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms { }

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    type O = terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms {
        SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms { alarm_name: self.alarm_name }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
        SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationDynamic {
    alarms: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms>>,
    dynamic: SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationDynamic,
}

impl SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration_Alarms>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.alarms = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.alarms = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {
    type O = terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {}

impl BuildSagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {
        SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration {
            alarms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationRef {
        SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize { }

impl terrars
::ToListMappable for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize {
    pub fn build(
        self,
    ) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySizeRef {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize { }

impl terrars
::ToListMappable for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize {
    pub fn build(
        self,
    ) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSizeRef {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationDynamic {
    canary_size: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize,
        >,
    >,
    linear_step_size: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    wait_interval_in_seconds: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_size: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linear_step_size: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize>,
    >,
    dynamic: SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationDynamic,
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
    #[doc = "Set the field `canary_size`.\n"]
    pub fn set_canary_size(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySize,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.canary_size = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.canary_size = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `linear_step_size`.\n"]
    pub fn set_linear_step_size(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSize,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.linear_step_size = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.linear_step_size = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
    type O =
        terrars
        ::BlockAssignable<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub wait_interval_in_seconds: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration {
            type_: self.type_,
            wait_interval_in_seconds: self.wait_interval_in_seconds,
            canary_size: core::default::Default::default(),
            linear_step_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationRef {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_interval_in_seconds` after provisioning.\n"]
    pub fn wait_interval_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_interval_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `canary_size` after provisioning.\n"]
    pub fn canary_size(
        &self,
    ) -> terrars
    ::ListRef<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_CanarySizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_size", self.base))
    }

    #[doc = "Get a reference to the value of field `linear_step_size` after provisioning.\n"]
    pub fn linear_step_size(
        &self,
    ) -> terrars
    ::ListRef<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration_LinearStepSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linear_step_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyDynamic {
    traffic_routing_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_execution_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_wait_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_routing_configuration: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration>>,
    dynamic: SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyDynamic,
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {
    #[doc = "Set the field `maximum_execution_timeout_in_seconds`.\n"]
    pub fn set_maximum_execution_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_execution_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `termination_wait_in_seconds`.\n"]
    pub fn set_termination_wait_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.termination_wait_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `traffic_routing_configuration`.\n"]
    pub fn set_traffic_routing_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.traffic_routing_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.traffic_routing_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {
    type O = terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {}

impl BuildSagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy {
            maximum_execution_timeout_in_seconds: core::default::Default::default(),
            termination_wait_in_seconds: core::default::Default::default(),
            traffic_routing_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyRef {
        SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_execution_timeout_in_seconds` after provisioning.\n"]
    pub fn maximum_execution_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `termination_wait_in_seconds` after provisioning.\n"]
    pub fn termination_wait_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_wait_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_configuration` after provisioning.\n"]
    pub fn traffic_routing_configuration(
        &self,
    ) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy_TrafficRoutingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_routing_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize { }

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    type O = terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
        SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
        SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize { }

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    type O =
        terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
        SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
        SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyDynamic {
    maximum_batch_size: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>>,
    rollback_maximum_batch_size: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_execution_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    wait_interval_in_seconds: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batch_size: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_maximum_batch_size: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize>>,
    dynamic: SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyDynamic,
}

impl SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
    #[doc = "Set the field `maximum_execution_timeout_in_seconds`.\n"]
    pub fn set_maximum_execution_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.maximum_execution_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_batch_size`.\n"]
    pub fn set_maximum_batch_size(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.maximum_batch_size = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.maximum_batch_size = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rollback_maximum_batch_size`.\n"]
    pub fn set_rollback_maximum_batch_size(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rollback_maximum_batch_size = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rollback_maximum_batch_size = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
    type O = terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
    #[doc = ""]
    pub wait_interval_in_seconds: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
        SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy {
            maximum_execution_timeout_in_seconds: core::default::Default::default(),
            wait_interval_in_seconds: self.wait_interval_in_seconds,
            maximum_batch_size: core::default::Default::default(),
            rollback_maximum_batch_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyRef {
        SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `maximum_execution_timeout_in_seconds` after provisioning.\n"]
    pub fn maximum_execution_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_execution_timeout_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_interval_in_seconds` after provisioning.\n"]
    pub fn wait_interval_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_interval_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_batch_size` after provisioning.\n"]
    pub fn maximum_batch_size(
        &self,
    ) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback_maximum_batch_size` after provisioning.\n"]
    pub fn rollback_maximum_batch_size(
        &self,
    ) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_maximum_batch_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpoint_DeploymentConfigDynamic {
    auto_rollback_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration>>,
    blue_green_update_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy>>,
    rolling_update_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpoint_DeploymentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_rollback_configuration: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_update_policy: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rolling_update_policy: std::option::Option<std::vec::Vec<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy>>,
    dynamic: SagemakerEndpoint_DeploymentConfigDynamic,
}

impl SagemakerEndpoint_DeploymentConfig {
    #[doc = "Set the field `auto_rollback_configuration`.\n"]
    pub fn set_auto_rollback_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_rollback_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_rollback_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blue_green_update_policy`.\n"]
    pub fn set_blue_green_update_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.blue_green_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.blue_green_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rolling_update_policy`.\n"]
    pub fn set_rolling_update_policy(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rolling_update_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rolling_update_policy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerEndpoint_DeploymentConfig {
    type O = terrars::BlockAssignable<SagemakerEndpoint_DeploymentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpoint_DeploymentConfig {}

impl BuildSagemakerEndpoint_DeploymentConfig {
    pub fn build(self) -> SagemakerEndpoint_DeploymentConfig {
        SagemakerEndpoint_DeploymentConfig {
            auto_rollback_configuration: core::default::Default::default(),
            blue_green_update_policy: core::default::Default::default(),
            rolling_update_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerEndpoint_DeploymentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpoint_DeploymentConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerEndpoint_DeploymentConfigRef {
        SagemakerEndpoint_DeploymentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpoint_DeploymentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfig_AutoRollbackConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `blue_green_update_policy` after provisioning.\n"]
    pub fn blue_green_update_policy(
        &self,
    ) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfig_BlueGreenUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_update_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `rolling_update_policy` after provisioning.\n"]
    pub fn rolling_update_policy(&self) -> terrars::ListRef<SagemakerEndpoint_DeploymentConfig_RollingUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rolling_update_policy", self.base))
    }
}
