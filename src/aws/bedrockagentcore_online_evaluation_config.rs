#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreOnlineEvaluationConfigData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    enable_on_create: terrars::PrimField<bool>,
    evaluation_execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_status: std::option::Option<terrars::PrimField<std::string::String>>,
    online_evaluation_config_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_config: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluator: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_Evaluator>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreOnlineEvaluationConfig_Timeouts>,
    dynamic: BedrockagentcoreOnlineEvaluationConfigDynamic,
}

struct BedrockagentcoreOnlineEvaluationConfig_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreOnlineEvaluationConfigData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreOnlineEvaluationConfig(std::rc::Rc<BedrockagentcoreOnlineEvaluationConfig_>);

impl BedrockagentcoreOnlineEvaluationConfig {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_status`.\n"]
    pub fn set_execution_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_source_config`.\n"]
    pub fn set_data_source_config(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_source_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_source_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `evaluator`.\n"]
    pub fn set_evaluator(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Evaluator>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreOnlineEvaluationConfig_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_on_create` after provisioning.\n"]
    pub fn enable_on_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_on_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_execution_role_arn` after provisioning.\n"]
    pub fn evaluation_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_status` after provisioning.\n"]
    pub fn execution_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_evaluation_config_arn` after provisioning.\n"]
    pub fn online_evaluation_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.online_evaluation_config_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_evaluation_config_id` after provisioning.\n"]
    pub fn online_evaluation_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.online_evaluation_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_evaluation_config_name` after provisioning.\n"]
    pub fn online_evaluation_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.online_evaluation_config_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_source_config` after provisioning.\n"]
    pub fn data_source_config(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
        BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for BedrockagentcoreOnlineEvaluationConfig {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreOnlineEvaluationConfig { }

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig {
    type O = terrars::ListRef<BedrockagentcoreOnlineEvaluationConfigRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreOnlineEvaluationConfig_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_online_evaluation_config".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub enable_on_create: terrars::PrimField<bool>,
    #[doc = ""]
    pub evaluation_execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub online_evaluation_config_name: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOnlineEvaluationConfig {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreOnlineEvaluationConfig {
        let out = BedrockagentcoreOnlineEvaluationConfig(std::rc::Rc::new(BedrockagentcoreOnlineEvaluationConfig_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreOnlineEvaluationConfigData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                enable_on_create: self.enable_on_create,
                evaluation_execution_role_arn: self.evaluation_execution_role_arn,
                execution_status: core::default::Default::default(),
                online_evaluation_config_name: self.online_evaluation_config_name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                data_source_config: core::default::Default::default(),
                evaluator: core::default::Default::default(),
                rule: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreOnlineEvaluationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfigRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_on_create` after provisioning.\n"]
    pub fn enable_on_create(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_on_create", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluation_execution_role_arn` after provisioning.\n"]
    pub fn evaluation_execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.evaluation_execution_role_arn", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `execution_status` after provisioning.\n"]
    pub fn execution_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_evaluation_config_arn` after provisioning.\n"]
    pub fn online_evaluation_config_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.online_evaluation_config_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_evaluation_config_id` after provisioning.\n"]
    pub fn online_evaluation_config_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.online_evaluation_config_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `online_evaluation_config_name` after provisioning.\n"]
    pub fn online_evaluation_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.online_evaluation_config_name", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `data_source_config` after provisioning.\n"]
    pub fn data_source_config(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_source_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
        BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOnlineEvaluationConfigDynamic {
    data_source_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig>>,
    evaluator: std::option::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_Evaluator>>,
    rule: std::option::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_Rule>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {
    #[doc = "Set the field `log_group_name`.\n"]
    pub fn set_log_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {}

impl BuildBedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {
        BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig {
            log_group_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfigRef {
        BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name` after provisioning.\n"]
    pub fn log_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_OutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_config: std
    ::option
    ::Option<terrars::ListField<BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig>>,
}

impl BedrockagentcoreOnlineEvaluationConfig_OutputConfig {
    #[doc = "Set the field `cloudwatch_config`.\n"]
    pub fn set_cloudwatch_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfig>>,
    ) -> Self {
        self.cloudwatch_config = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_OutputConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_OutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_OutputConfig {}

impl BuildBedrockagentcoreOnlineEvaluationConfig_OutputConfig {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_OutputConfig {
        BedrockagentcoreOnlineEvaluationConfig_OutputConfig { cloudwatch_config: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef {
        BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_OutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_config` after provisioning.\n"]
    pub fn cloudwatch_config(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_OutputConfig_CloudwatchConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs {
    log_group_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    service_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs { }

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs {
    #[doc = ""]
    pub log_group_names: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub service_names: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs {
        BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs {
            log_group_names: self.log_group_names,
            service_names: self.service_names,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogsRef {
        BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_names` after provisioning.\n"]
    pub fn log_group_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_group_names", self.base))
    }

    #[doc = "Get a reference to the value of field `service_names` after provisioning.\n"]
    pub fn service_names(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.service_names", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigDynamic {
    cloudwatch_logs: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch_logs: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs>>,
    dynamic: BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigDynamic,
}

impl BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {
    #[doc = "Set the field `cloudwatch_logs`.\n"]
    pub fn set_cloudwatch_logs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogs>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch_logs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch_logs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {}

impl BuildBedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {
        BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig {
            cloudwatch_logs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef {
        BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_DataSourceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch_logs` after provisioning.\n"]
    pub fn cloudwatch_logs(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_DataSourceConfig_CloudwatchLogsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch_logs", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Evaluator {
    evaluator_id: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreOnlineEvaluationConfig_Evaluator { }

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Evaluator {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Evaluator>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Evaluator {
    #[doc = ""]
    pub evaluator_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Evaluator {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Evaluator {
        BedrockagentcoreOnlineEvaluationConfig_Evaluator { evaluator_id: self.evaluator_id }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_EvaluatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_EvaluatorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_EvaluatorRef {
        BedrockagentcoreOnlineEvaluationConfig_EvaluatorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_EvaluatorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `evaluator_id` after provisioning.\n"]
    pub fn evaluator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_id", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_value: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    double_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {
    #[doc = "Set the field `boolean_value`.\n"]
    pub fn set_boolean_value(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.boolean_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `double_value`.\n"]
    pub fn set_double_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.double_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {
        BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value {
            boolean_value: core::default::Default::default(),
            double_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_ValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_ValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_ValueRef {
        BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_ValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_ValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boolean_value` after provisioning.\n"]
    pub fn boolean_value(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.boolean_value", self.base))
    }

    #[doc = "Get a reference to the value of field `double_value` after provisioning.\n"]
    pub fn double_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.double_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOnlineEvaluationConfig_Rule_FilterDynamic {
    value: std::option::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
    key: terrars::PrimField<std::string::String>,
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value>>,
    dynamic: BedrockagentcoreOnlineEvaluationConfig_Rule_FilterDynamic,
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_Value>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.value = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
        BedrockagentcoreOnlineEvaluationConfig_Rule_Filter {
            key: self.key,
            operator: self.operator,
            value: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_Rule_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_Rule_FilterRef {
        BedrockagentcoreOnlineEvaluationConfig_Rule_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter_ValueRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig {
    sampling_percentage: terrars::PrimField<f64>,
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig { }

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig {
    #[doc = ""]
    pub sampling_percentage: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig {
        BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig { sampling_percentage: self.sampling_percentage }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfigRef {
        BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sampling_percentage` after provisioning.\n"]
    pub fn sampling_percentage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sampling_percentage", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig {
    session_timeout_minutes: terrars::PrimField<f64>,
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig { }

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig {
    #[doc = ""]
    pub session_timeout_minutes: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig {
        BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig {
            session_timeout_minutes: self.session_timeout_minutes,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfigRef {
        BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `session_timeout_minutes` after provisioning.\n"]
    pub fn session_timeout_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.session_timeout_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreOnlineEvaluationConfig_RuleDynamic {
    filter: std::option::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter>>,
    sampling_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig>>,
    session_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sampling_config: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    session_config: std::option::Option<std::vec::Vec<BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig>>,
    dynamic: BedrockagentcoreOnlineEvaluationConfig_RuleDynamic,
}

impl BedrockagentcoreOnlineEvaluationConfig_Rule {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sampling_config`.\n"]
    pub fn set_sampling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.sampling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.sampling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `session_config`.\n"]
    pub fn set_session_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.session_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.session_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Rule {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Rule {}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Rule {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Rule {
        BedrockagentcoreOnlineEvaluationConfig_Rule {
            filter: core::default::Default::default(),
            sampling_config: core::default::Default::default(),
            session_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_RuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_RuleRef {
        BedrockagentcoreOnlineEvaluationConfig_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_Rule_FilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }

    #[doc = "Get a reference to the value of field `sampling_config` after provisioning.\n"]
    pub fn sampling_config(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_Rule_SamplingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sampling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `session_config` after provisioning.\n"]
    pub fn session_config(&self) -> terrars::ListRef<BedrockagentcoreOnlineEvaluationConfig_Rule_SessionConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.session_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreOnlineEvaluationConfig_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreOnlineEvaluationConfig_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreOnlineEvaluationConfig_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreOnlineEvaluationConfig_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreOnlineEvaluationConfig_Timeouts {}

impl BuildBedrockagentcoreOnlineEvaluationConfig_Timeouts {
    pub fn build(self) -> BedrockagentcoreOnlineEvaluationConfig_Timeouts {
        BedrockagentcoreOnlineEvaluationConfig_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
        BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreOnlineEvaluationConfig_TimeoutsRef {
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
