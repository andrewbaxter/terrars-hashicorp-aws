#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerFlowDefinitionData_ {
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
    flow_definition_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_loop_activation_config: std::option::Option<std::vec::Vec<SagemakerFlowDefinition_HumanLoopActivationConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_loop_config: std::option::Option<std::vec::Vec<SagemakerFlowDefinition_HumanLoopConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_loop_request_source: std::option::Option<std::vec::Vec<SagemakerFlowDefinition_HumanLoopRequestSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    output_config: std::option::Option<std::vec::Vec<SagemakerFlowDefinition_OutputConfig>>,
    dynamic: SagemakerFlowDefinitionDynamic,
}

struct SagemakerFlowDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerFlowDefinitionData_>,
}

#[derive(Clone)]
pub struct SagemakerFlowDefinition(std::rc::Rc<SagemakerFlowDefinition_>);

impl SagemakerFlowDefinition {
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

    #[doc = "Set the field `human_loop_activation_config`.\n"]
    pub fn set_human_loop_activation_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopActivationConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_loop_activation_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_loop_activation_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human_loop_config`.\n"]
    pub fn set_human_loop_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_loop_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_loop_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `human_loop_request_source`.\n"]
    pub fn set_human_loop_request_source(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopRequestSource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().human_loop_request_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.human_loop_request_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `output_config`.\n"]
    pub fn set_output_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerFlowDefinition_OutputConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().output_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.output_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `flow_definition_name` after provisioning.\n"]
    pub fn flow_definition_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_definition_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_loop_activation_config` after provisioning.\n"]
    pub fn human_loop_activation_config(&self) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopActivationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_loop_activation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_loop_config` after provisioning.\n"]
    pub fn human_loop_config(&self) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_loop_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_loop_request_source` after provisioning.\n"]
    pub fn human_loop_request_source(&self) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopRequestSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_loop_request_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerFlowDefinition_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
    }
}

impl terrars::Referable for SagemakerFlowDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerFlowDefinition { }

impl terrars::ToListMappable for SagemakerFlowDefinition {
    type O = terrars::ListRef<SagemakerFlowDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerFlowDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_flow_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerFlowDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub flow_definition_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFlowDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerFlowDefinition {
        let out = SagemakerFlowDefinition(std::rc::Rc::new(SagemakerFlowDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerFlowDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                flow_definition_name: self.flow_definition_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                human_loop_activation_config: core::default::Default::default(),
                human_loop_config: core::default::Default::default(),
                human_loop_request_source: core::default::Default::default(),
                output_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerFlowDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerFlowDefinitionRef {
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

    #[doc = "Get a reference to the value of field `flow_definition_name` after provisioning.\n"]
    pub fn flow_definition_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_definition_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_loop_activation_config` after provisioning.\n"]
    pub fn human_loop_activation_config(&self) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopActivationConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_loop_activation_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_loop_config` after provisioning.\n"]
    pub fn human_loop_config(&self) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_loop_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `human_loop_request_source` after provisioning.\n"]
    pub fn human_loop_request_source(&self) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopRequestSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.human_loop_request_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `output_config` after provisioning.\n"]
    pub fn output_config(&self) -> terrars::ListRef<SagemakerFlowDefinition_OutputConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.output_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinitionDynamic {
    human_loop_activation_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFlowDefinition_HumanLoopActivationConfig>>,
    human_loop_config: std::option::Option<terrars::DynamicBlock<SagemakerFlowDefinition_HumanLoopConfig>>,
    human_loop_request_source: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFlowDefinition_HumanLoopRequestSource>>,
    output_config: std::option::Option<terrars::DynamicBlock<SagemakerFlowDefinition_OutputConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig {
    human_loop_activation_conditions: terrars::PrimField<std::string::String>,
}

impl SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig { }

impl terrars
::ToListMappable for SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig {
    type O =
        terrars
        ::BlockAssignable<SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig {
    #[doc = ""]
    pub human_loop_activation_conditions: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig {
    pub fn build(self) -> SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig {
        SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig {
            human_loop_activation_conditions: self.human_loop_activation_conditions,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfigRef {
        SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_loop_activation_conditions` after provisioning.\n"]
    pub fn human_loop_activation_conditions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_loop_activation_conditions", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinition_HumanLoopActivationConfigDynamic {
    human_loop_activation_conditions_config: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_HumanLoopActivationConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    human_loop_activation_conditions_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig>>,
    dynamic: SagemakerFlowDefinition_HumanLoopActivationConfigDynamic,
}

impl SagemakerFlowDefinition_HumanLoopActivationConfig {
    #[doc = "Set the field `human_loop_activation_conditions_config`.\n"]
    pub fn set_human_loop_activation_conditions_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfig,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.human_loop_activation_conditions_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.human_loop_activation_conditions_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFlowDefinition_HumanLoopActivationConfig {
    type O = terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopActivationConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_HumanLoopActivationConfig {}

impl BuildSagemakerFlowDefinition_HumanLoopActivationConfig {
    pub fn build(self) -> SagemakerFlowDefinition_HumanLoopActivationConfig {
        SagemakerFlowDefinition_HumanLoopActivationConfig {
            human_loop_activation_conditions_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_HumanLoopActivationConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_HumanLoopActivationConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFlowDefinition_HumanLoopActivationConfigRef {
        SagemakerFlowDefinition_HumanLoopActivationConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_HumanLoopActivationConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_loop_activation_conditions_config` after provisioning.\n"]
    pub fn human_loop_activation_conditions_config(
        &self,
    ) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopActivationConfig_HumanLoopActivationConditionsConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.human_loop_activation_conditions_config", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cents: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dollars: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenth_fractions_of_a_cent: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {
    #[doc = "Set the field `cents`.\n"]
    pub fn set_cents(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.cents = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dollars`.\n"]
    pub fn set_dollars(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.dollars = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenth_fractions_of_a_cent`.\n"]
    pub fn set_tenth_fractions_of_a_cent(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tenth_fractions_of_a_cent = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {
    type O = terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {}

impl BuildSagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {
    pub fn build(self) -> SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {
        SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd {
            cents: core::default::Default::default(),
            dollars: core::default::Default::default(),
            tenth_fractions_of_a_cent: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
        SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsdRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cents` after provisioning.\n"]
    pub fn cents(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cents", self.base))
    }

    #[doc = "Get a reference to the value of field `dollars` after provisioning.\n"]
    pub fn dollars(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dollars", self.base))
    }

    #[doc = "Get a reference to the value of field `tenth_fractions_of_a_cent` after provisioning.\n"]
    pub fn tenth_fractions_of_a_cent(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenth_fractions_of_a_cent", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceDynamic {
    amount_in_usd: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd>>,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amount_in_usd: std
    ::option
    ::Option<std::vec::Vec<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd>>,
    dynamic: SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceDynamic,
}

impl SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {
    #[doc = "Set the field `amount_in_usd`.\n"]
    pub fn set_amount_in_usd(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsd,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.amount_in_usd = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.amount_in_usd = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {
    type O = terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {}

impl BuildSagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {
    pub fn build(self) -> SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {
        SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice {
            amount_in_usd: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceRef {
        SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amount_in_usd` after provisioning.\n"]
    pub fn amount_in_usd(
        &self,
    ) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice_AmountInUsdRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.amount_in_usd", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerFlowDefinition_HumanLoopConfigDynamic {
    public_workforce_task_price: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice>>,
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_HumanLoopConfig {
    human_task_ui_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_availability_lifetime_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    task_count: terrars::PrimField<f64>,
    task_description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_keywords: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_time_limit_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    task_title: terrars::PrimField<std::string::String>,
    workteam_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_workforce_task_price: std
    ::option
    ::Option<std::vec::Vec<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice>>,
    dynamic: SagemakerFlowDefinition_HumanLoopConfigDynamic,
}

impl SagemakerFlowDefinition_HumanLoopConfig {
    #[doc = "Set the field `task_availability_lifetime_in_seconds`.\n"]
    pub fn set_task_availability_lifetime_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.task_availability_lifetime_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_keywords`.\n"]
    pub fn set_task_keywords(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.task_keywords = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `task_time_limit_in_seconds`.\n"]
    pub fn set_task_time_limit_in_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.task_time_limit_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_workforce_task_price`.\n"]
    pub fn set_public_workforce_task_price(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPrice>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.public_workforce_task_price = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.public_workforce_task_price = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerFlowDefinition_HumanLoopConfig {
    type O = terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_HumanLoopConfig {
    #[doc = ""]
    pub human_task_ui_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub task_count: terrars::PrimField<f64>,
    #[doc = ""]
    pub task_description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub task_title: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub workteam_arn: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFlowDefinition_HumanLoopConfig {
    pub fn build(self) -> SagemakerFlowDefinition_HumanLoopConfig {
        SagemakerFlowDefinition_HumanLoopConfig {
            human_task_ui_arn: self.human_task_ui_arn,
            task_availability_lifetime_in_seconds: core::default::Default::default(),
            task_count: self.task_count,
            task_description: self.task_description,
            task_keywords: core::default::Default::default(),
            task_time_limit_in_seconds: core::default::Default::default(),
            task_title: self.task_title,
            workteam_arn: self.workteam_arn,
            public_workforce_task_price: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_HumanLoopConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_HumanLoopConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFlowDefinition_HumanLoopConfigRef {
        SagemakerFlowDefinition_HumanLoopConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_HumanLoopConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `human_task_ui_arn` after provisioning.\n"]
    pub fn human_task_ui_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.human_task_ui_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `task_availability_lifetime_in_seconds` after provisioning.\n"]
    pub fn task_availability_lifetime_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_availability_lifetime_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `task_count` after provisioning.\n"]
    pub fn task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `task_description` after provisioning.\n"]
    pub fn task_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_description", self.base))
    }

    #[doc = "Get a reference to the value of field `task_keywords` after provisioning.\n"]
    pub fn task_keywords(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.task_keywords", self.base))
    }

    #[doc = "Get a reference to the value of field `task_time_limit_in_seconds` after provisioning.\n"]
    pub fn task_time_limit_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_time_limit_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `task_title` after provisioning.\n"]
    pub fn task_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_title", self.base))
    }

    #[doc = "Get a reference to the value of field `workteam_arn` after provisioning.\n"]
    pub fn workteam_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.workteam_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `public_workforce_task_price` after provisioning.\n"]
    pub fn public_workforce_task_price(
        &self,
    ) -> terrars::ListRef<SagemakerFlowDefinition_HumanLoopConfig_PublicWorkforceTaskPriceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.public_workforce_task_price", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_HumanLoopRequestSource {
    aws_managed_human_loop_request_source: terrars::PrimField<std::string::String>,
}

impl SagemakerFlowDefinition_HumanLoopRequestSource { }

impl terrars::ToListMappable for SagemakerFlowDefinition_HumanLoopRequestSource {
    type O = terrars::BlockAssignable<SagemakerFlowDefinition_HumanLoopRequestSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_HumanLoopRequestSource {
    #[doc = ""]
    pub aws_managed_human_loop_request_source: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFlowDefinition_HumanLoopRequestSource {
    pub fn build(self) -> SagemakerFlowDefinition_HumanLoopRequestSource {
        SagemakerFlowDefinition_HumanLoopRequestSource {
            aws_managed_human_loop_request_source: self.aws_managed_human_loop_request_source,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_HumanLoopRequestSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_HumanLoopRequestSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerFlowDefinition_HumanLoopRequestSourceRef {
        SagemakerFlowDefinition_HumanLoopRequestSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_HumanLoopRequestSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_managed_human_loop_request_source` after provisioning.\n"]
    pub fn aws_managed_human_loop_request_source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_managed_human_loop_request_source", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerFlowDefinition_OutputConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    s3_output_path: terrars::PrimField<std::string::String>,
}

impl SagemakerFlowDefinition_OutputConfig {
    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerFlowDefinition_OutputConfig {
    type O = terrars::BlockAssignable<SagemakerFlowDefinition_OutputConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerFlowDefinition_OutputConfig {
    #[doc = ""]
    pub s3_output_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerFlowDefinition_OutputConfig {
    pub fn build(self) -> SagemakerFlowDefinition_OutputConfig {
        SagemakerFlowDefinition_OutputConfig {
            kms_key_id: core::default::Default::default(),
            s3_output_path: self.s3_output_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerFlowDefinition_OutputConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerFlowDefinition_OutputConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerFlowDefinition_OutputConfigRef {
        SagemakerFlowDefinition_OutputConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerFlowDefinition_OutputConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_output_path` after provisioning.\n"]
    pub fn s3_output_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_output_path", self.base))
    }
}
