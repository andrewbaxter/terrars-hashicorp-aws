#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaFunctionScalingConfigData_ {
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
    function_name: terrars::PrimField<std::string::String>,
    qualifier: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    function_scaling_config: std::option::Option<std::vec::Vec<LambdaFunctionScalingConfig_FunctionScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdaFunctionScalingConfig_Timeouts>,
    dynamic: LambdaFunctionScalingConfigDynamic,
}

struct LambdaFunctionScalingConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaFunctionScalingConfigData_>,
}

#[derive(Clone)]
pub struct LambdaFunctionScalingConfig(std::rc::Rc<LambdaFunctionScalingConfig_>);

impl LambdaFunctionScalingConfig {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `function_scaling_config`.\n"]
    pub fn set_function_scaling_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaFunctionScalingConfig_FunctionScalingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().function_scaling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.function_scaling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdaFunctionScalingConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\nARN of the Lambda function."]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `function_name` after provisioning.\nName or ARN of the Lambda function."]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `function_state` after provisioning.\nState of the function after applying the scaling configuration."]
    pub fn function_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `qualifier` after provisioning.\nQualifier for the scaling configuration. Valid values: $LATEST.PUBLISHED or a numeric version number."]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_scaling_config` after provisioning.\n"]
    pub fn function_scaling_config(&self) -> terrars::ListRef<LambdaFunctionScalingConfig_FunctionScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionScalingConfig_TimeoutsRef {
        LambdaFunctionScalingConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for LambdaFunctionScalingConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaFunctionScalingConfig { }

impl terrars::ToListMappable for LambdaFunctionScalingConfig {
    type O = terrars::ListRef<LambdaFunctionScalingConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaFunctionScalingConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_function_scaling_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaFunctionScalingConfig {
    pub tf_id: std::string::String,
    #[doc = "Name or ARN of the Lambda function."]
    pub function_name: terrars::PrimField<std::string::String>,
    #[doc = "Qualifier for the scaling configuration. Valid values: $LATEST.PUBLISHED or a numeric version number."]
    pub qualifier: terrars::PrimField<std::string::String>,
}

impl BuildLambdaFunctionScalingConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaFunctionScalingConfig {
        let out = LambdaFunctionScalingConfig(std::rc::Rc::new(LambdaFunctionScalingConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaFunctionScalingConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                function_name: self.function_name,
                qualifier: self.qualifier,
                region: core::default::Default::default(),
                function_scaling_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaFunctionScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionScalingConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaFunctionScalingConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\nARN of the Lambda function."]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `function_name` after provisioning.\nName or ARN of the Lambda function."]
    pub fn function_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `function_state` after provisioning.\nState of the function after applying the scaling configuration."]
    pub fn function_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_state", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `qualifier` after provisioning.\nQualifier for the scaling configuration. Valid values: $LATEST.PUBLISHED or a numeric version number."]
    pub fn qualifier(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.qualifier", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `function_scaling_config` after provisioning.\n"]
    pub fn function_scaling_config(&self) -> terrars::ListRef<LambdaFunctionScalingConfig_FunctionScalingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.function_scaling_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaFunctionScalingConfig_TimeoutsRef {
        LambdaFunctionScalingConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct LambdaFunctionScalingConfigDynamic {
    function_scaling_config: std
    ::option
    ::Option<terrars::DynamicBlock<LambdaFunctionScalingConfig_FunctionScalingConfig>>,
}

#[derive(Serialize)]
pub struct LambdaFunctionScalingConfig_FunctionScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_execution_environments: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_execution_environments: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaFunctionScalingConfig_FunctionScalingConfig {
    #[doc =
        "Set the field `max_execution_environments`.\nMaximum number of execution environments that can be provisioned for the function."]
    pub fn set_max_execution_environments(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_execution_environments = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `min_execution_environments`.\nMinimum number of execution environments to maintain for the function."]
    pub fn set_min_execution_environments(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_execution_environments = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionScalingConfig_FunctionScalingConfig {
    type O = terrars::BlockAssignable<LambdaFunctionScalingConfig_FunctionScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionScalingConfig_FunctionScalingConfig {}

impl BuildLambdaFunctionScalingConfig_FunctionScalingConfig {
    pub fn build(self) -> LambdaFunctionScalingConfig_FunctionScalingConfig {
        LambdaFunctionScalingConfig_FunctionScalingConfig {
            max_execution_environments: core::default::Default::default(),
            min_execution_environments: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunctionScalingConfig_FunctionScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionScalingConfig_FunctionScalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaFunctionScalingConfig_FunctionScalingConfigRef {
        LambdaFunctionScalingConfig_FunctionScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionScalingConfig_FunctionScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `max_execution_environments` after provisioning.\nMaximum number of execution environments that can be provisioned for the function."]
    pub fn max_execution_environments(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_execution_environments", self.base))
    }

    #[doc =
        "Get a reference to the value of field `min_execution_environments` after provisioning.\nMinimum number of execution environments to maintain for the function."]
    pub fn min_execution_environments(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_execution_environments", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaFunctionScalingConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaFunctionScalingConfig_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaFunctionScalingConfig_Timeouts {
    type O = terrars::BlockAssignable<LambdaFunctionScalingConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaFunctionScalingConfig_Timeouts {}

impl BuildLambdaFunctionScalingConfig_Timeouts {
    pub fn build(self) -> LambdaFunctionScalingConfig_Timeouts {
        LambdaFunctionScalingConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaFunctionScalingConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaFunctionScalingConfig_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaFunctionScalingConfig_TimeoutsRef {
        LambdaFunctionScalingConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaFunctionScalingConfig_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
