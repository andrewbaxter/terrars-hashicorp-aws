#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BedrockagentcoreEvaluatorData_ {
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
    evaluator_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    level: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluator_config: std::option::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BedrockagentcoreEvaluator_Timeouts>,
    dynamic: BedrockagentcoreEvaluatorDynamic,
}

struct BedrockagentcoreEvaluator_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BedrockagentcoreEvaluatorData_>,
}

#[derive(Clone)]
pub struct BedrockagentcoreEvaluator(std::rc::Rc<BedrockagentcoreEvaluator_>);

impl BedrockagentcoreEvaluator {
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

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kms_key_arn = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `evaluator_config`.\n"]
    pub fn set_evaluator_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().evaluator_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.evaluator_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BedrockagentcoreEvaluator_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_arn` after provisioning.\n"]
    pub fn evaluator_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_id` after provisioning.\n"]
    pub fn evaluator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_name` after provisioning.\n"]
    pub fn evaluator_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locked_for_modification` after provisioning.\n"]
    pub fn locked_for_modification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked_for_modification", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_config` after provisioning.\n"]
    pub fn evaluator_config(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluator_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreEvaluator_TimeoutsRef {
        BedrockagentcoreEvaluator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BedrockagentcoreEvaluator {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BedrockagentcoreEvaluator { }

impl terrars::ToListMappable for BedrockagentcoreEvaluator {
    type O = terrars::ListRef<BedrockagentcoreEvaluatorRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BedrockagentcoreEvaluator_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_bedrockagentcore_evaluator".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBedrockagentcoreEvaluator {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub evaluator_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub level: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreEvaluator {
    pub fn build(self, stack: &mut terrars::Stack) -> BedrockagentcoreEvaluator {
        let out = BedrockagentcoreEvaluator(std::rc::Rc::new(BedrockagentcoreEvaluator_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BedrockagentcoreEvaluatorData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                evaluator_name: self.evaluator_name,
                kms_key_arn: core::default::Default::default(),
                level: self.level,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                evaluator_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BedrockagentcoreEvaluatorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluatorRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BedrockagentcoreEvaluatorRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_arn` after provisioning.\n"]
    pub fn evaluator_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_id` after provisioning.\n"]
    pub fn evaluator_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_name` after provisioning.\n"]
    pub fn evaluator_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.evaluator_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `locked_for_modification` after provisioning.\n"]
    pub fn locked_for_modification(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.locked_for_modification", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `evaluator_config` after provisioning.\n"]
    pub fn evaluator_config(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluator_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BedrockagentcoreEvaluator_TimeoutsRef {
        BedrockagentcoreEvaluator_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluatorDynamic {
    evaluator_config: std::option::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
    lambda_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
    #[doc = "Set the field `lambda_timeout_in_seconds`.\n"]
    pub fn set_lambda_timeout_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.lambda_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
    #[doc = ""]
    pub lambda_arn: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
        BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig {
            lambda_arn: self.lambda_arn,
            lambda_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfigRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_arn` after provisioning.\n"]
    pub fn lambda_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `lambda_timeout_in_seconds` after provisioning.\n"]
    pub fn lambda_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lambda_timeout_in_seconds", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedDynamic {
    lambda_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_config: std::option::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig>>,
    dynamic: BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedDynamic,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {
    #[doc = "Set the field `lambda_config`.\n"]
    pub fn set_lambda_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {
        BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased {
            lambda_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lambda_config` after provisioning.\n"]
    pub fn lambda_config(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased_LambdaConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lambda_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_tokens: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_sequences: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    temperature: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    top_p: std::option::Option<terrars::PrimField<f64>>,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {
    #[doc = "Set the field `max_tokens`.\n"]
    pub fn set_max_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stop_sequences`.\n"]
    pub fn set_stop_sequences(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stop_sequences = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `temperature`.\n"]
    pub fn set_temperature(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.temperature = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `top_p`.\n"]
    pub fn set_top_p(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.top_p = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig {
            max_tokens: core::default::Default::default(),
            stop_sequences: core::default::Default::default(),
            temperature: core::default::Default::default(),
            top_p: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfigRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_tokens` after provisioning.\n"]
    pub fn max_tokens(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `stop_sequences` after provisioning.\n"]
    pub fn stop_sequences(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stop_sequences", self.base))
    }

    #[doc = "Get a reference to the value of field `temperature` after provisioning.\n"]
    pub fn temperature(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.temperature", self.base))
    }

    #[doc = "Get a reference to the value of field `top_p` after provisioning.\n"]
    pub fn top_p(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.top_p", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigDynamic {
    inference_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig,
        >,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_model_request_fields: std::option::Option<terrars::PrimField<std::string::String>>,
    model_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inference_config: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig,
        >,
    >,
    dynamic: BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigDynamic,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
    #[doc = "Set the field `additional_model_request_fields`.\n"]
    pub fn set_additional_model_request_fields(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_model_request_fields = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inference_config`.\n"]
    pub fn set_inference_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inference_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inference_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
    type O =
        terrars
        ::BlockAssignable<
            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
    #[doc = ""]
    pub model_id: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
    pub fn build(
        self,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig {
            additional_model_request_fields: core::default::Default::default(),
            model_id: self.model_id,
            inference_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_model_request_fields` after provisioning.\n"]
    pub fn additional_model_request_fields(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_model_request_fields", self.base))
    }

    #[doc = "Get a reference to the value of field `model_id` after provisioning.\n"]
    pub fn model_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_id", self.base))
    }

    #[doc = "Get a reference to the value of field `inference_config` after provisioning.\n"]
    pub fn inference_config(
        &self,
    ) -> terrars
    ::ListRef<
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig_InferenceConfigRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inference_config", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigDynamic {
    bedrock_evaluator_model_config: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig>,
    >,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bedrock_evaluator_model_config: std
    ::option
    ::Option<
        std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig>,
    >,
    dynamic: BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigDynamic,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {
    #[doc = "Set the field `bedrock_evaluator_model_config`.\n"]
    pub fn set_bedrock_evaluator_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bedrock_evaluator_model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bedrock_evaluator_model_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig {
            bedrock_evaluator_model_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bedrock_evaluator_model_config` after provisioning.\n"]
    pub fn bedrock_evaluator_model_config(
        &self,
    ) -> terrars
    ::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig_BedrockEvaluatorModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bedrock_evaluator_model_config", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical {
    definition: terrars::PrimField<std::string::String>,
    label: terrars::PrimField<std::string::String>,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical { }

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical {
    #[doc = ""]
    pub definition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub label: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical {
            definition: self.definition,
            label: self.label,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_CategoricalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_CategoricalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_CategoricalRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_CategoricalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_CategoricalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical {
    definition: terrars::PrimField<std::string::String>,
    label: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical { }

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical {
    #[doc = ""]
    pub definition: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub label: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical {
            definition: self.definition,
            label: self.label,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_NumericalRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_NumericalRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_NumericalRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_NumericalRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_NumericalRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.definition", self.base))
    }

    #[doc = "Get a reference to the value of field `label` after provisioning.\n"]
    pub fn label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleDynamic {
    categorical: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical>>,
    numerical: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    categorical: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    numerical: std
    ::option
    ::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical>>,
    dynamic: BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleDynamic,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {
    #[doc = "Set the field `categorical`.\n"]
    pub fn set_categorical(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Categorical,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.categorical = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.categorical = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `numerical`.\n"]
    pub fn set_numerical(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_Numerical>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.numerical = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.numerical = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale {
            categorical: core::default::Default::default(),
            numerical: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `categorical` after provisioning.\n"]
    pub fn categorical(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_CategoricalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.categorical", self.base))
    }

    #[doc = "Get a reference to the value of field `numerical` after provisioning.\n"]
    pub fn numerical(
        &self,
    ) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale_NumericalRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.numerical", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeDynamic {
    model_config: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig>>,
    rating_scale: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
    instructions: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_config: std::option::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rating_scale: std::option::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale>>,
    dynamic: BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeDynamic,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
    #[doc = "Set the field `model_config`.\n"]
    pub fn set_model_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.model_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.model_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rating_scale`.\n"]
    pub fn set_rating_scale(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScale>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.rating_scale = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.rating_scale = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
    #[doc = ""]
    pub instructions: terrars::PrimField<std::string::String>,
}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge {
            instructions: self.instructions,
            model_config: core::default::Default::default(),
            rating_scale: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeRef {
        BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instructions` after provisioning.\n"]
    pub fn instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instructions", self.base))
    }

    #[doc = "Get a reference to the value of field `model_config` after provisioning.\n"]
    pub fn model_config(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_ModelConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.model_config", self.base))
    }

    #[doc = "Get a reference to the value of field `rating_scale` after provisioning.\n"]
    pub fn rating_scale(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge_RatingScaleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rating_scale", self.base))
    }
}

#[derive(Serialize, Default)]
struct BedrockagentcoreEvaluator_EvaluatorConfigDynamic {
    code_based: std::option::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased>>,
    llm_as_a_judge: std
    ::option
    ::Option<terrars::DynamicBlock<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge>>,
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_based: std::option::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    llm_as_a_judge: std::option::Option<std::vec::Vec<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge>>,
    dynamic: BedrockagentcoreEvaluator_EvaluatorConfigDynamic,
}

impl BedrockagentcoreEvaluator_EvaluatorConfig {
    #[doc = "Set the field `code_based`.\n"]
    pub fn set_code_based(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBased>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_based = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_based = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `llm_as_a_judge`.\n"]
    pub fn set_llm_as_a_judge(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudge>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.llm_as_a_judge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.llm_as_a_judge = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BedrockagentcoreEvaluator_EvaluatorConfig {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_EvaluatorConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_EvaluatorConfig {}

impl BuildBedrockagentcoreEvaluator_EvaluatorConfig {
    pub fn build(self) -> BedrockagentcoreEvaluator_EvaluatorConfig {
        BedrockagentcoreEvaluator_EvaluatorConfig {
            code_based: core::default::Default::default(),
            llm_as_a_judge: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_EvaluatorConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_EvaluatorConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreEvaluator_EvaluatorConfigRef {
        BedrockagentcoreEvaluator_EvaluatorConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_EvaluatorConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `code_based` after provisioning.\n"]
    pub fn code_based(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_CodeBasedRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.code_based", self.base))
    }

    #[doc = "Get a reference to the value of field `llm_as_a_judge` after provisioning.\n"]
    pub fn llm_as_a_judge(&self) -> terrars::ListRef<BedrockagentcoreEvaluator_EvaluatorConfig_LlmAsAJudgeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.llm_as_a_judge", self.base))
    }
}

#[derive(Serialize)]
pub struct BedrockagentcoreEvaluator_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BedrockagentcoreEvaluator_Timeouts {
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

impl terrars::ToListMappable for BedrockagentcoreEvaluator_Timeouts {
    type O = terrars::BlockAssignable<BedrockagentcoreEvaluator_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBedrockagentcoreEvaluator_Timeouts {}

impl BuildBedrockagentcoreEvaluator_Timeouts {
    pub fn build(self) -> BedrockagentcoreEvaluator_Timeouts {
        BedrockagentcoreEvaluator_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BedrockagentcoreEvaluator_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BedrockagentcoreEvaluator_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BedrockagentcoreEvaluator_TimeoutsRef {
        BedrockagentcoreEvaluator_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BedrockagentcoreEvaluator_TimeoutsRef {
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
