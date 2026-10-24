use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerEndpointData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_config: std::option::Option<std::vec::Vec<SagemakerEndpointDeploymentConfigEl>>,
    dynamic: SagemakerEndpointDynamic,
}

struct SagemakerEndpoint_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerEndpointData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerEndpointDeploymentConfigEl>>,
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> terrars::ListRef<SagemakerEndpointDeploymentConfigElRef> {
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
            data: std::cell::RefCell::new(SagemakerEndpointData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                endpoint_config_name: self.endpoint_config_name,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> terrars::ListRef<SagemakerEndpointDeploymentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    alarm_name: terrars::PrimField<std::string::String>,
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl { }

impl terrars::ToListMappable for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    type O = terrars::BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl { alarm_name: self.alarm_name }
    }
}

pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarm_name` after provisioning.\n"]
    pub fn alarm_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.alarm_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElDynamic {
    alarms: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<std::vec::Vec<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>>,
    dynamic: SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElDynamic,
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElAlarmsEl>,
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

impl terrars::ToListMappable for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    type O = terrars::BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {}

impl BuildSagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl {
            alarms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
        SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl { }

impl terrars
::ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
    pub fn build(
        self,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef {
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
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl { }

impl terrars
::ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    type O =
        terrars
        ::BlockAssignable<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
    pub fn build(
        self,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl {
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef {
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
struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElDynamic {
    canary_size: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl,
        >,
    >,
    linear_step_size: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    wait_interval_in_seconds: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    canary_size: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    linear_step_size: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl>,
    >,
    dynamic: SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElDynamic,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
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
                            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeEl,
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
                            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeEl,
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

impl terrars
::ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub wait_interval_in_seconds: terrars::PrimField<f64>,
}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl {
            type_: self.type_,
            wait_interval_in_seconds: self.wait_interval_in_seconds,
            canary_size: core::default::Default::default(),
            linear_step_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef {
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
    ::ListRef<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElCanarySizeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.canary_size", self.base))
    }

    #[doc = "Get a reference to the value of field `linear_step_size` after provisioning.\n"]
    pub fn linear_step_size(
        &self,
    ) -> terrars
    ::ListRef<
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElLinearStepSizeElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.linear_step_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElDynamic {
    traffic_routing_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_execution_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_wait_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    traffic_routing_configuration: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl>>,
    dynamic: SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElDynamic,
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
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
                            SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationEl,
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

impl terrars::ToListMappable for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    type O = terrars::BlockAssignable<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {}

impl BuildSagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl {
            maximum_execution_timeout_in_seconds: core::default::Default::default(),
            termination_wait_in_seconds: core::default::Default::default(),
            traffic_routing_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
        SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef {
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
    ) -> terrars::ListRef<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElTrafficRoutingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.traffic_routing_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDeploymentConfigElDynamic {
    auto_rollback_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>>,
    blue_green_update_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>>,
}

#[derive(Serialize)]
pub struct SagemakerEndpointDeploymentConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_rollback_configuration: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_update_policy: std
    ::option
    ::Option<std::vec::Vec<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>>,
    dynamic: SagemakerEndpointDeploymentConfigElDynamic,
}

impl SagemakerEndpointDeploymentConfigEl {
    #[doc = "Set the field `auto_rollback_configuration`.\n"]
    pub fn set_auto_rollback_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationEl>>,
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
                    ::Into<terrars::BlockAssignable<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyEl>>,
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
}

impl terrars::ToListMappable for SagemakerEndpointDeploymentConfigEl {
    type O = terrars::BlockAssignable<SagemakerEndpointDeploymentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerEndpointDeploymentConfigEl {}

impl BuildSagemakerEndpointDeploymentConfigEl {
    pub fn build(self) -> SagemakerEndpointDeploymentConfigEl {
        SagemakerEndpointDeploymentConfigEl {
            auto_rollback_configuration: core::default::Default::default(),
            blue_green_update_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SagemakerEndpointDeploymentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerEndpointDeploymentConfigElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerEndpointDeploymentConfigElRef {
        SagemakerEndpointDeploymentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerEndpointDeploymentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointDeploymentConfigElAutoRollbackConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `blue_green_update_policy` after provisioning.\n"]
    pub fn blue_green_update_policy(
        &self,
    ) -> terrars::ListRef<SagemakerEndpointDeploymentConfigElBlueGreenUpdatePolicyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_update_policy", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerEndpointDynamic {
    deployment_config: std::option::Option<terrars::DynamicBlock<SagemakerEndpointDeploymentConfigEl>>,
}
