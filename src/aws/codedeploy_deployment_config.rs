use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodedeployDeploymentConfigData {
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
    minimum_healthy_hosts: std::option::Option<std::vec::Vec<CodedeployDeploymentConfigMinimumHealthyHostsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_routing_config: std::option::Option<std::vec::Vec<CodedeployDeploymentConfigTrafficRoutingConfigEl>>,
    dynamic: CodedeployDeploymentConfigDynamic,
}

struct CodedeployDeploymentConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodedeployDeploymentConfigData>,
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

    #[doc = "Set the field `minimum_healthy_hosts`.\n"]
    pub fn set_minimum_healthy_hosts(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentConfigMinimumHealthyHostsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigEl>>,
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

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts` after provisioning.\n"]
    pub fn minimum_healthy_hosts(&self) -> terrars::ListRef<CodedeployDeploymentConfigMinimumHealthyHostsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_config` after provisioning.\n"]
    pub fn traffic_routing_config(&self) -> terrars::ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_routing_config", self.extract_ref()))
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
            data: std::cell::RefCell::new(CodedeployDeploymentConfigData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                compute_platform: core::default::Default::default(),
                deployment_config_name: self.deployment_config_name,
                id: core::default::Default::default(),
                minimum_healthy_hosts: core::default::Default::default(),
                traffic_routing_config: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `minimum_healthy_hosts` after provisioning.\n"]
    pub fn minimum_healthy_hosts(&self) -> terrars::ListRef<CodedeployDeploymentConfigMinimumHealthyHostsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.minimum_healthy_hosts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `traffic_routing_config` after provisioning.\n"]
    pub fn traffic_routing_config(&self) -> terrars::ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_routing_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigMinimumHealthyHostsEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfigMinimumHealthyHostsEl {
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

impl terrars::ToListMappable for CodedeployDeploymentConfigMinimumHealthyHostsEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfigMinimumHealthyHostsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigMinimumHealthyHostsEl {}

impl BuildCodedeployDeploymentConfigMinimumHealthyHostsEl {
    pub fn build(self) -> CodedeployDeploymentConfigMinimumHealthyHostsEl {
        CodedeployDeploymentConfigMinimumHealthyHostsEl {
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigMinimumHealthyHostsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfigMinimumHealthyHostsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfigMinimumHealthyHostsElRef {
        CodedeployDeploymentConfigMinimumHealthyHostsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigMinimumHealthyHostsElRef {
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
pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
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

impl terrars::ToListMappable for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {}

impl BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
    pub fn build(self) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl {
            interval: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef {
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
pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
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

impl terrars::ToListMappable for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {}

impl BuildCodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
    pub fn build(self) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl {
            interval: core::default::Default::default(),
            percentage: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
        CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef {
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
struct CodedeployDeploymentConfigTrafficRoutingConfigElDynamic {
    time_based_canary: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>>,
    time_based_linear: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentConfigTrafficRoutingConfigEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_canary: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_based_linear: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>>,
    dynamic: CodedeployDeploymentConfigTrafficRoutingConfigElDynamic,
}

impl CodedeployDeploymentConfigTrafficRoutingConfigEl {
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
                    ::Into<
                        terrars::BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryEl>,
                    >,
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
                    ::Into<
                        terrars::BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearEl>,
                    >,
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

impl terrars::ToListMappable for CodedeployDeploymentConfigTrafficRoutingConfigEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentConfigTrafficRoutingConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentConfigTrafficRoutingConfigEl {}

impl BuildCodedeployDeploymentConfigTrafficRoutingConfigEl {
    pub fn build(self) -> CodedeployDeploymentConfigTrafficRoutingConfigEl {
        CodedeployDeploymentConfigTrafficRoutingConfigEl {
            type_: core::default::Default::default(),
            time_based_canary: core::default::Default::default(),
            time_based_linear: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentConfigTrafficRoutingConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentConfigTrafficRoutingConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentConfigTrafficRoutingConfigElRef {
        CodedeployDeploymentConfigTrafficRoutingConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentConfigTrafficRoutingConfigElRef {
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
    ) -> terrars::ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedCanaryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_canary", self.base))
    }

    #[doc = "Get a reference to the value of field `time_based_linear` after provisioning.\n"]
    pub fn time_based_linear(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentConfigTrafficRoutingConfigElTimeBasedLinearElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_based_linear", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentConfigDynamic {
    minimum_healthy_hosts: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfigMinimumHealthyHostsEl>>,
    traffic_routing_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentConfigTrafficRoutingConfigEl>>,
}
