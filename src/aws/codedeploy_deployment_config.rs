#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodedeployDeploymentConfigData_ {
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
    compute_platform: std::option::Option<terrars::PrimField<std::string::String>>,
    deployment_config_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_healthy_hosts: std::option::Option<std::vec::Vec<CodedeployDeploymentConfig_MinimumHealthyHosts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_routing_config: std::option::Option<std::vec::Vec<CodedeployDeploymentConfig_TrafficRoutingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zonal_config: std::option::Option<std::vec::Vec<CodedeployDeploymentConfig_ZonalConfig>>,
    dynamic: CodedeployDeploymentConfigDynamic,
}

struct CodedeployDeploymentConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodedeployDeploymentConfigData_>,
}

#[derive(Clone)]
pub struct CodedeployDeploymentConfig(std::rc::Rc<CodedeployDeploymentConfig_>);

impl CodedeployDeploymentConfig {
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

    #[doc = "Set the field `compute_platform`.\n"]
    pub fn set_compute_platform(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().compute_platform = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `minimum_healthy_hosts`.\n"]
    pub fn set_minimum_healthy_hosts(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentConfig_MinimumHealthyHosts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().minimum_healthy_hosts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.minimum_healthy_hosts = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `traffic_routing_config`.\n"]
    pub fn set_traffic_routing_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentConfig_TrafficRoutingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().traffic_routing_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.traffic_routing_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `zonal_config`.\n"]
    pub fn set_zonal_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentConfig_ZonalConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().zonal_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.zonal_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_id` after provisioning.\n"]
    pub fn deployment_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts` after provisioning.\n"]
    pub fn minimum_healthy_hosts(&self) -> terrars::ListRef<CodedeployDeploymentConfig_MinimumHealthyHostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_config` after provisioning.\n"]
    pub fn traffic_routing_config(&self) -> terrars::ListRef<CodedeployDeploymentConfig_TrafficRoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_routing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_config` after provisioning.\n"]
    pub fn zonal_config(&self) -> terrars::ListRef<CodedeployDeploymentConfig_ZonalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zonal_config", self.extract_ref()))
    }
}

impl terrars::Referable for CodedeployDeploymentConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodedeployDeploymentConfig { }

impl terrars::ToListMappable for CodedeployDeploymentConfig {
    type O = terrars::ListRef<CodedeployDeploymentConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodedeployDeploymentConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codedeploy_deployment_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodedeployDeploymentConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub deployment_config_name: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> CodedeployDeploymentConfig {
        let out = CodedeployDeploymentConfig(std::rc::Rc::new(CodedeployDeploymentConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodedeployDeploymentConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                compute_platform: core::default::Default::default(),
                deployment_config_name: self.deployment_config_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                minimum_healthy_hosts: core::default::Default::default(),
                traffic_routing_config: core::default::Default::default(),
                zonal_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodedeployDeploymentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodedeployDeploymentConfigRef {
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

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_id` after provisioning.\n"]
    pub fn deployment_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts` after provisioning.\n"]
    pub fn minimum_healthy_hosts(&self) -> terrars::ListRef<CodedeployDeploymentConfig_MinimumHealthyHostsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_config` after provisioning.\n"]
    pub fn traffic_routing_config(&self) -> terrars::ListRef<CodedeployDeploymentConfig_TrafficRoutingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_routing_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `zonal_config` after provisioning.\n"]
    pub fn zonal_config(&self) -> terrars::ListRef<CodedeployDeploymentConfig_ZonalConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.zonal_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentConfigDynamic {
    minimum_healthy_hosts: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentConfig_MinimumHealthyHosts>>,
    traffic_routing_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfig_TrafficRoutingConfig>>,
    zonal_config: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentConfig_ZonalConfig>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfig_MinimumHealthyHosts {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfig_MinimumHealthyHosts {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentConfig_MinimumHealthyHosts {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfig_MinimumHealthyHosts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfig_MinimumHealthyHosts {}

impl BuildCodedeployDeploymentConfig_MinimumHealthyHosts {
    pub fn build(self) -> CodedeployDeploymentConfig_MinimumHealthyHosts {
        CodedeployDeploymentConfig_MinimumHealthyHosts {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentConfig_MinimumHealthyHostsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfig_MinimumHealthyHostsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfig_MinimumHealthyHostsRef {
        CodedeployDeploymentConfig_MinimumHealthyHostsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfig_MinimumHealthyHostsRef {
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
pub struct CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\n"]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {}

impl BuildCodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {
    pub fn build(self) -> CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {
        CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary {
            interval: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanaryRef {
        CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `percentage` after provisioning.\n"]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {
    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `percentage`.\n"]
    pub fn set_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {}

impl BuildCodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {
    pub fn build(self) -> CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {
        CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear {
            interval: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinearRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinearRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinearRef {
        CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinearRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinearRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `percentage` after provisioning.\n"]
    pub fn percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentConfig_TrafficRoutingConfigDynamic {
    time_based_canary: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary>>,
    time_based_linear: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfig_TrafficRoutingConfig {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_canary: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_linear: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear>>,
    dynamic: CodedeployDeploymentConfig_TrafficRoutingConfigDynamic,
}

impl CodedeployDeploymentConfig_TrafficRoutingConfig {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `time_based_canary`.\n"]
    pub fn set_time_based_canary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanary>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_based_canary = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_canary = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `time_based_linear`.\n"]
    pub fn set_time_based_linear(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinear>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_based_linear = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_based_linear = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentConfig_TrafficRoutingConfig {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfig_TrafficRoutingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfig_TrafficRoutingConfig {}

impl BuildCodedeployDeploymentConfig_TrafficRoutingConfig {
    pub fn build(self) -> CodedeployDeploymentConfig_TrafficRoutingConfig {
        CodedeployDeploymentConfig_TrafficRoutingConfig {
            type_: core::default::Default::default(),
            time_based_canary: core::default::Default::default(),
            time_based_linear: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentConfig_TrafficRoutingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfig_TrafficRoutingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfig_TrafficRoutingConfigRef {
        CodedeployDeploymentConfig_TrafficRoutingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfig_TrafficRoutingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_canary` after provisioning.\n"]
    pub fn time_based_canary(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedCanaryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_canary", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_linear` after provisioning.\n"]
    pub fn time_based_linear(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentConfig_TrafficRoutingConfig_TimeBasedLinearRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_linear", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {}

impl BuildCodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {
    pub fn build(self) -> CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {
        CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZoneRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZoneRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZoneRef {
        CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZoneRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZoneRef {
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
struct CodedeployDeploymentConfig_ZonalConfigDynamic {
    minimum_healthy_hosts_per_zone: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfig_ZonalConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_zone_monitor_duration_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitor_duration_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    minimum_healthy_hosts_per_zone: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone>>,
    dynamic: CodedeployDeploymentConfig_ZonalConfigDynamic,
}

impl CodedeployDeploymentConfig_ZonalConfig {
    #[doc = "Set the field `first_zone_monitor_duration_in_seconds`.\n"]
    pub fn set_first_zone_monitor_duration_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.first_zone_monitor_duration_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitor_duration_in_seconds`.\n"]
    pub fn set_monitor_duration_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.monitor_duration_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `minimum_healthy_hosts_per_zone`.\n"]
    pub fn set_minimum_healthy_hosts_per_zone(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZone>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.minimum_healthy_hosts_per_zone = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.minimum_healthy_hosts_per_zone = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentConfig_ZonalConfig {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfig_ZonalConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfig_ZonalConfig {}

impl BuildCodedeployDeploymentConfig_ZonalConfig {
    pub fn build(self) -> CodedeployDeploymentConfig_ZonalConfig {
        CodedeployDeploymentConfig_ZonalConfig {
            first_zone_monitor_duration_in_seconds: core::default::Default::default(),
            monitor_duration_in_seconds: core::default::Default::default(),
            minimum_healthy_hosts_per_zone: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentConfig_ZonalConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfig_ZonalConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentConfig_ZonalConfigRef {
        CodedeployDeploymentConfig_ZonalConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfig_ZonalConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `first_zone_monitor_duration_in_seconds` after provisioning.\n"]
    pub fn first_zone_monitor_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.first_zone_monitor_duration_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `monitor_duration_in_seconds` after provisioning.\n"]
    pub fn monitor_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitor_duration_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts_per_zone` after provisioning.\n"]
    pub fn minimum_healthy_hosts_per_zone(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentConfig_ZonalConfig_MinimumHealthyHostsPerZoneRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts_per_zone", self.base))
    }
}
