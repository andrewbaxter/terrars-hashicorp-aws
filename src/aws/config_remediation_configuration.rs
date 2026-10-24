use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConfigRemediationConfigurationData {
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
    automatic: std::option::Option<terrars::PrimField<bool>>,
    config_rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_automatic_attempts: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_attempt_seconds: std::option::Option<terrars::PrimField<f64>>,
    target_id: terrars::PrimField<std::string::String>,
    target_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_controls: std::option::Option<std::vec::Vec<ConfigRemediationConfigurationExecutionControlsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter: std::option::Option<std::vec::Vec<ConfigRemediationConfigurationParameterEl>>,
    dynamic: ConfigRemediationConfigurationDynamic,
}

struct ConfigRemediationConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ConfigRemediationConfigurationData>,
}

#[derive(Clone)]
pub struct ConfigRemediationConfiguration(std::rc::Rc<ConfigRemediationConfiguration_>);

impl ConfigRemediationConfiguration {
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

    #[doc = "Set the field `automatic`.\n"]
    pub fn set_automatic(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().automatic = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_automatic_attempts`.\n"]
    pub fn set_maximum_automatic_attempts(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_automatic_attempts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_attempt_seconds`.\n"]
    pub fn set_retry_attempt_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().retry_attempt_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_version`.\n"]
    pub fn set_target_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().target_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_controls`.\n"]
    pub fn set_execution_controls(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConfigRemediationConfigurationExecutionControlsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().execution_controls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.execution_controls = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameter`.\n"]
    pub fn set_parameter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<ConfigRemediationConfigurationParameterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `automatic` after provisioning.\n"]
    pub fn automatic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_rule_name` after provisioning.\n"]
    pub fn config_rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_automatic_attempts` after provisioning.\n"]
    pub fn maximum_automatic_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_automatic_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_attempt_seconds` after provisioning.\n"]
    pub fn retry_attempt_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_attempt_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_controls` after provisioning.\n"]
    pub fn execution_controls(&self) -> terrars::ListRef<ConfigRemediationConfigurationExecutionControlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_controls", self.extract_ref()))
    }
}

impl terrars::Referable for ConfigRemediationConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ConfigRemediationConfiguration { }

impl terrars::ToListMappable for ConfigRemediationConfiguration {
    type O = terrars::ListRef<ConfigRemediationConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ConfigRemediationConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_config_remediation_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConfigRemediationConfiguration {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub config_rule_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target_type: terrars::PrimField<std::string::String>,
}

impl BuildConfigRemediationConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> ConfigRemediationConfiguration {
        let out = ConfigRemediationConfiguration(std::rc::Rc::new(ConfigRemediationConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(ConfigRemediationConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                automatic: core::default::Default::default(),
                config_rule_name: self.config_rule_name,
                id: core::default::Default::default(),
                maximum_automatic_attempts: core::default::Default::default(),
                resource_type: core::default::Default::default(),
                retry_attempt_seconds: core::default::Default::default(),
                target_id: self.target_id,
                target_type: self.target_type,
                target_version: core::default::Default::default(),
                execution_controls: core::default::Default::default(),
                parameter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConfigRemediationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigRemediationConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConfigRemediationConfigurationRef {
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

    #[doc = "Get a reference to the value of field `automatic` after provisioning.\n"]
    pub fn automatic(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.automatic", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `config_rule_name` after provisioning.\n"]
    pub fn config_rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.config_rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_automatic_attempts` after provisioning.\n"]
    pub fn maximum_automatic_attempts(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_automatic_attempts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `retry_attempt_seconds` after provisioning.\n"]
    pub fn retry_attempt_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_attempt_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_id` after provisioning.\n"]
    pub fn target_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_version` after provisioning.\n"]
    pub fn target_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_controls` after provisioning.\n"]
    pub fn execution_controls(&self) -> terrars::ListRef<ConfigRemediationConfigurationExecutionControlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.execution_controls", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    concurrent_execution_rate_percentage: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    error_percentage: std::option::Option<terrars::PrimField<f64>>,
}

impl ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    #[doc = "Set the field `concurrent_execution_rate_percentage`.\n"]
    pub fn set_concurrent_execution_rate_percentage(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.concurrent_execution_rate_percentage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `error_percentage`.\n"]
    pub fn set_error_percentage(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.error_percentage = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    type O = terrars::BlockAssignable<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigRemediationConfigurationExecutionControlsElSsmControlsEl {}

impl BuildConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
    pub fn build(self) -> ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
        ConfigRemediationConfigurationExecutionControlsElSsmControlsEl {
            concurrent_execution_rate_percentage: core::default::Default::default(),
            error_percentage: core::default::Default::default(),
        }
    }
}

pub struct ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
        ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `concurrent_execution_rate_percentage` after provisioning.\n"]
    pub fn concurrent_execution_rate_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.concurrent_execution_rate_percentage", self.base))
    }

    #[doc = "Get a reference to the value of field `error_percentage` after provisioning.\n"]
    pub fn error_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.error_percentage", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigRemediationConfigurationExecutionControlsElDynamic {
    ssm_controls: std
    ::option
    ::Option<terrars::DynamicBlock<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>>,
}

#[derive(Serialize)]
pub struct ConfigRemediationConfigurationExecutionControlsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssm_controls: std::option::Option<std::vec::Vec<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>>,
    dynamic: ConfigRemediationConfigurationExecutionControlsElDynamic,
}

impl ConfigRemediationConfigurationExecutionControlsEl {
    #[doc = "Set the field `ssm_controls`.\n"]
    pub fn set_ssm_controls(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ConfigRemediationConfigurationExecutionControlsElSsmControlsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ssm_controls = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ssm_controls = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ConfigRemediationConfigurationExecutionControlsEl {
    type O = terrars::BlockAssignable<ConfigRemediationConfigurationExecutionControlsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigRemediationConfigurationExecutionControlsEl {}

impl BuildConfigRemediationConfigurationExecutionControlsEl {
    pub fn build(self) -> ConfigRemediationConfigurationExecutionControlsEl {
        ConfigRemediationConfigurationExecutionControlsEl {
            ssm_controls: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct ConfigRemediationConfigurationExecutionControlsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigRemediationConfigurationExecutionControlsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ConfigRemediationConfigurationExecutionControlsElRef {
        ConfigRemediationConfigurationExecutionControlsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigRemediationConfigurationExecutionControlsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ssm_controls` after provisioning.\n"]
    pub fn ssm_controls(&self) -> terrars::ListRef<ConfigRemediationConfigurationExecutionControlsElSsmControlsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ssm_controls", self.base))
    }
}

#[derive(Serialize)]
pub struct ConfigRemediationConfigurationParameterEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    static_values: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl ConfigRemediationConfigurationParameterEl {
    #[doc = "Set the field `resource_value`.\n"]
    pub fn set_resource_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_value`.\n"]
    pub fn set_static_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.static_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `static_values`.\n"]
    pub fn set_static_values(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.static_values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ConfigRemediationConfigurationParameterEl {
    type O = terrars::BlockAssignable<ConfigRemediationConfigurationParameterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildConfigRemediationConfigurationParameterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildConfigRemediationConfigurationParameterEl {
    pub fn build(self) -> ConfigRemediationConfigurationParameterEl {
        ConfigRemediationConfigurationParameterEl {
            name: self.name,
            resource_value: core::default::Default::default(),
            static_value: core::default::Default::default(),
            static_values: core::default::Default::default(),
        }
    }
}

pub struct ConfigRemediationConfigurationParameterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ConfigRemediationConfigurationParameterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> ConfigRemediationConfigurationParameterElRef {
        ConfigRemediationConfigurationParameterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ConfigRemediationConfigurationParameterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_value` after provisioning.\n"]
    pub fn resource_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_value", self.base))
    }

    #[doc = "Get a reference to the value of field `static_value` after provisioning.\n"]
    pub fn static_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.static_value", self.base))
    }

    #[doc = "Get a reference to the value of field `static_values` after provisioning.\n"]
    pub fn static_values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.static_values", self.base))
    }
}

#[derive(Serialize, Default)]
struct ConfigRemediationConfigurationDynamic {
    execution_controls: std::option::Option<terrars::DynamicBlock<ConfigRemediationConfigurationExecutionControlsEl>>,
    parameter: std::option::Option<terrars::DynamicBlock<ConfigRemediationConfigurationParameterEl>>,
}
