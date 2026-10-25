#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerInferenceComponentData_ {
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
    endpoint_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variant_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_config: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_DeploymentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    runtime_config: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_RuntimeConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specification: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    specifications: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specifications>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SagemakerInferenceComponent_Timeouts>,
    dynamic: SagemakerInferenceComponentDynamic,
}

struct SagemakerInferenceComponent_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SagemakerInferenceComponentData_>,
}

#[derive(Clone)]
pub struct SagemakerInferenceComponent(std::rc::Rc<SagemakerInferenceComponent_>);

impl SagemakerInferenceComponent {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variant_name`.\n"]
    pub fn set_variant_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().variant_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_config`.\n"]
    pub fn set_deployment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig>>,
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

    #[doc = "Set the field `runtime_config`.\n"]
    pub fn set_runtime_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerInferenceComponent_RuntimeConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().runtime_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.runtime_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `specification`.\n"]
    pub fn set_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `specifications`.\n"]
    pub fn set_specifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specifications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.specifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SagemakerInferenceComponent_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `variant_name` after provisioning.\n"]
    pub fn variant_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variant_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_DeploymentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specification` after provisioning.\n"]
    pub fn specification(&self) -> terrars::ListRef<SagemakerInferenceComponent_SpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specifications` after provisioning.\n"]
    pub fn specifications(&self) -> terrars::ListRef<SagemakerInferenceComponent_SpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerInferenceComponent_TimeoutsRef {
        SagemakerInferenceComponent_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for SagemakerInferenceComponent {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SagemakerInferenceComponent { }

impl terrars::ToListMappable for SagemakerInferenceComponent {
    type O = terrars::ListRef<SagemakerInferenceComponentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SagemakerInferenceComponent_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_sagemaker_inference_component".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerInferenceComponent {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub endpoint_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerInferenceComponent {
    pub fn build(self, stack: &mut terrars::Stack) -> SagemakerInferenceComponent {
        let out = SagemakerInferenceComponent(std::rc::Rc::new(SagemakerInferenceComponent_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SagemakerInferenceComponentData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                endpoint_name: self.endpoint_name,
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                variant_name: core::default::Default::default(),
                deployment_config: core::default::Default::default(),
                runtime_config: core::default::Default::default(),
                specification: core::default::Default::default(),
                specifications: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerInferenceComponentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerInferenceComponentRef {
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

    #[doc = "Get a reference to the value of field `creation_time` after provisioning.\n"]
    pub fn creation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.creation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_arn` after provisioning.\n"]
    pub fn endpoint_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint_name` after provisioning.\n"]
    pub fn endpoint_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `failure_reason` after provisioning.\n"]
    pub fn failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.failure_reason", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modified_time` after provisioning.\n"]
    pub fn last_modified_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modified_time", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `variant_name` after provisioning.\n"]
    pub fn variant_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.variant_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config` after provisioning.\n"]
    pub fn deployment_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_DeploymentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `runtime_config` after provisioning.\n"]
    pub fn runtime_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_RuntimeConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.runtime_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specification` after provisioning.\n"]
    pub fn specification(&self) -> terrars::ListRef<SagemakerInferenceComponent_SpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `specifications` after provisioning.\n"]
    pub fn specifications(&self) -> terrars::ListRef<SagemakerInferenceComponent_SpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SagemakerInferenceComponent_TimeoutsRef {
        SagemakerInferenceComponent_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponentDynamic {
    deployment_config: std::option::Option<terrars::DynamicBlock<SagemakerInferenceComponent_DeploymentConfig>>,
    runtime_config: std::option::Option<terrars::DynamicBlock<SagemakerInferenceComponent_RuntimeConfig>>,
    specification: std::option::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification>>,
    specifications: std::option::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specifications>>,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    alarm_name: terrars::PrimField<std::string::String>,
}

impl SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms { }

impl terrars::ToListMappable for SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    #[doc = ""]
    pub alarm_name: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms {
    pub fn build(self) -> SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms {
        SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms { alarm_name: self.alarm_name }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
        SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef {
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
struct SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationDynamic {
    alarms: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms>>,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms>>,
    dynamic: SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationDynamic,
}

impl SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_Alarms,
                        >,
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

impl terrars::ToListMappable for SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {}

impl BuildSagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {
    pub fn build(self) -> SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {
        SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration {
            alarms: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationRef {
        SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration_AlarmsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize { }

impl terrars::ToListMappable for SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    type O =
        terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
    pub fn build(self) -> SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
        SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
        SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef {
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
pub struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize { }

impl terrars
::ToListMappable for SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    type O =
        terrars
        ::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
    pub fn build(self) -> SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
        SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize {
            type_: self.type_,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
        SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef {
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
struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyDynamic {
    maximum_batch_size: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>,
    >,
    rollback_maximum_batch_size: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_execution_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    wait_interval_in_seconds: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_batch_size: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rollback_maximum_batch_size: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize>,
    >,
    dynamic: SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyDynamic,
}

impl SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
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
                        ::BlockAssignable<
                            SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSize,
                        >,
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
                            SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSize,
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

impl terrars::ToListMappable for SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
    #[doc = ""]
    pub wait_interval_in_seconds: terrars::PrimField<f64>,
}

impl BuildSagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
    pub fn build(self) -> SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
        SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy {
            maximum_execution_timeout_in_seconds: core::default::Default::default(),
            wait_interval_in_seconds: self.wait_interval_in_seconds,
            maximum_batch_size: core::default::Default::default(),
            rollback_maximum_batch_size: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyRef {
        SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyRef {
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
    ) -> terrars::ListRef<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_MaximumBatchSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maximum_batch_size", self.base))
    }

    #[doc = "Get a reference to the value of field `rollback_maximum_batch_size` after provisioning.\n"]
    pub fn rollback_maximum_batch_size(
        &self,
    ) -> terrars
    ::ListRef<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy_RollbackMaximumBatchSizeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rollback_maximum_batch_size", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponent_DeploymentConfigDynamic {
    auto_rollback_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration>>,
    rolling_update_policy: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy>>,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_DeploymentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_rollback_configuration: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rolling_update_policy: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy>>,
    dynamic: SagemakerInferenceComponent_DeploymentConfigDynamic,
}

impl SagemakerInferenceComponent_DeploymentConfig {
    #[doc = "Set the field `auto_rollback_configuration`.\n"]
    pub fn set_auto_rollback_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfiguration>,
                    >,
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

    #[doc = "Set the field `rolling_update_policy`.\n"]
    pub fn set_rolling_update_policy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicy>,
                    >,
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

impl terrars::ToListMappable for SagemakerInferenceComponent_DeploymentConfig {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_DeploymentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_DeploymentConfig {}

impl BuildSagemakerInferenceComponent_DeploymentConfig {
    pub fn build(self) -> SagemakerInferenceComponent_DeploymentConfig {
        SagemakerInferenceComponent_DeploymentConfig {
            auto_rollback_configuration: core::default::Default::default(),
            rolling_update_policy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_DeploymentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_DeploymentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_DeploymentConfigRef {
        SagemakerInferenceComponent_DeploymentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_DeploymentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_DeploymentConfig_AutoRollbackConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `rolling_update_policy` after provisioning.\n"]
    pub fn rolling_update_policy(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_DeploymentConfig_RollingUpdatePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rolling_update_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_RuntimeConfig_PlacementStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    current_copy_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerInferenceComponent_RuntimeConfig_PlacementStatus {
    #[doc = "Set the field `current_copy_count`.\n"]
    pub fn set_current_copy_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.current_copy_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_RuntimeConfig_PlacementStatus {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_RuntimeConfig_PlacementStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_RuntimeConfig_PlacementStatus {}

impl BuildSagemakerInferenceComponent_RuntimeConfig_PlacementStatus {
    pub fn build(self) -> SagemakerInferenceComponent_RuntimeConfig_PlacementStatus {
        SagemakerInferenceComponent_RuntimeConfig_PlacementStatus {
            current_copy_count: core::default::Default::default(),
            instance_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_RuntimeConfig_PlacementStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_RuntimeConfig_PlacementStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_RuntimeConfig_PlacementStatusRef {
        SagemakerInferenceComponent_RuntimeConfig_PlacementStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_RuntimeConfig_PlacementStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `current_copy_count` after provisioning.\n"]
    pub fn current_copy_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_copy_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_RuntimeConfig {
    copy_count: terrars::PrimField<f64>,
}

impl SagemakerInferenceComponent_RuntimeConfig { }

impl terrars::ToListMappable for SagemakerInferenceComponent_RuntimeConfig {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_RuntimeConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_RuntimeConfig {
    #[doc = ""]
    pub copy_count: terrars::PrimField<f64>,
}

impl BuildSagemakerInferenceComponent_RuntimeConfig {
    pub fn build(self) -> SagemakerInferenceComponent_RuntimeConfig {
        SagemakerInferenceComponent_RuntimeConfig { copy_count: self.copy_count }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_RuntimeConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_RuntimeConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerInferenceComponent_RuntimeConfigRef {
        SagemakerInferenceComponent_RuntimeConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_RuntimeConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_count` after provisioning.\n"]
    pub fn copy_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_count", self.base))
    }

    #[doc = "Get a reference to the value of field `current_copy_count` after provisioning.\n"]
    pub fn current_copy_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_copy_count", self.base))
    }

    #[doc = "Get a reference to the value of field `desired_copy_count` after provisioning.\n"]
    pub fn desired_copy_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.desired_copy_count", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_status` after provisioning.\n"]
    pub fn placement_status(&self) -> terrars::ListRef<SagemakerInferenceComponent_RuntimeConfig_PlacementStatusRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_status", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_ComputeResourceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_memory_required_in_mb: std::option::Option<terrars::PrimField<f64>>,
    min_memory_required_in_mb: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_accelerator_devices_required: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_of_cpu_cores_required: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerInferenceComponent_Specification_ComputeResourceRequirements {
    #[doc = "Set the field `max_memory_required_in_mb`.\n"]
    pub fn set_max_memory_required_in_mb(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_memory_required_in_mb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_accelerator_devices_required`.\n"]
    pub fn set_number_of_accelerator_devices_required(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.number_of_accelerator_devices_required = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `number_of_cpu_cores_required`.\n"]
    pub fn set_number_of_cpu_cores_required(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.number_of_cpu_cores_required = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_ComputeResourceRequirements {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_ComputeResourceRequirements {
    #[doc = ""]
    pub min_memory_required_in_mb: terrars::PrimField<f64>,
}

impl BuildSagemakerInferenceComponent_Specification_ComputeResourceRequirements {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_ComputeResourceRequirements {
        SagemakerInferenceComponent_Specification_ComputeResourceRequirements {
            max_memory_required_in_mb: core::default::Default::default(),
            min_memory_required_in_mb: self.min_memory_required_in_mb,
            number_of_accelerator_devices_required: core::default::Default::default(),
            number_of_cpu_cores_required: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef {
        SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_memory_required_in_mb` after provisioning.\n"]
    pub fn max_memory_required_in_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_memory_required_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `min_memory_required_in_mb` after provisioning.\n"]
    pub fn min_memory_required_in_mb(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_memory_required_in_mb", self.base))
    }

    #[doc = "Get a reference to the value of field `number_of_accelerator_devices_required` after provisioning.\n"]
    pub fn number_of_accelerator_devices_required(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.number_of_accelerator_devices_required", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `number_of_cpu_cores_required` after provisioning.\n"]
    pub fn number_of_cpu_cores_required(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.number_of_cpu_cores_required", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metric_publish_frequency_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    metrics_endpoint_path: terrars::PrimField<std::string::String>,
}

impl SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
    #[doc = "Set the field `metric_publish_frequency_in_seconds`.\n"]
    pub fn set_metric_publish_frequency_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.metric_publish_frequency_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
    type O =
        terrars
        ::BlockAssignable<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
    #[doc = ""]
    pub metrics_endpoint_path: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
        SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint {
            metric_publish_frequency_in_seconds: core::default::Default::default(),
            metrics_endpoint_path: self.metrics_endpoint_path,
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpointRef {
        SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metric_publish_frequency_in_seconds` after provisioning.\n"]
    pub fn metric_publish_frequency_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_publish_frequency_in_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `metrics_endpoint_path` after provisioning.\n"]
    pub fn metrics_endpoint_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_endpoint_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigDynamic {
    metrics_endpoint: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metrics_endpoint: std
    ::option
    ::Option<
        std::vec::Vec<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint>,
    >,
    dynamic: SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigDynamic,
}

impl SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {
    #[doc = "Set the field `metrics_endpoint`.\n"]
    pub fn set_metrics_endpoint(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpoint,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.metrics_endpoint = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.metrics_endpoint = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {}

impl BuildSagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {
        SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig {
            metrics_endpoint: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigRef {
        SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metrics_endpoint` after provisioning.\n"]
    pub fn metrics_endpoint(
        &self,
    ) -> terrars
    ::ListRef<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig_MetricsEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metrics_endpoint", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponent_Specification_ContainerDynamic {
    container_metrics_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig>>,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_Container {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    artifact_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_metrics_config: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig>>,
    dynamic: SagemakerInferenceComponent_Specification_ContainerDynamic,
}

impl SagemakerInferenceComponent_Specification_Container {
    #[doc = "Set the field `artifact_url`.\n"]
    pub fn set_artifact_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.artifact_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.environment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image`.\n"]
    pub fn set_image(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.image = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_metrics_config`.\n"]
    pub fn set_container_metrics_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfig>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container_metrics_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container_metrics_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_Container {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification_Container>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_Container {}

impl BuildSagemakerInferenceComponent_Specification_Container {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_Container {
        SagemakerInferenceComponent_Specification_Container {
            artifact_url: core::default::Default::default(),
            environment: core::default::Default::default(),
            image: core::default::Default::default(),
            container_metrics_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_ContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_ContainerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_ContainerRef {
        SagemakerInferenceComponent_Specification_ContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_ContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `artifact_url` after provisioning.\n"]
    pub fn artifact_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.artifact_url", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `resolved_image` after provisioning.\n"]
    pub fn resolved_image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved_image", self.base))
    }

    #[doc = "Get a reference to the value of field `container_metrics_config` after provisioning.\n"]
    pub fn container_metrics_config(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_Specification_Container_ContainerMetricsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container_metrics_config", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_DataCacheConfig {
    enable_caching: terrars::PrimField<bool>,
}

impl SagemakerInferenceComponent_Specification_DataCacheConfig { }

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_DataCacheConfig {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification_DataCacheConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_DataCacheConfig {
    #[doc = ""]
    pub enable_caching: terrars::PrimField<bool>,
}

impl BuildSagemakerInferenceComponent_Specification_DataCacheConfig {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_DataCacheConfig {
        SagemakerInferenceComponent_Specification_DataCacheConfig { enable_caching: self.enable_caching }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_DataCacheConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_DataCacheConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_DataCacheConfigRef {
        SagemakerInferenceComponent_Specification_DataCacheConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_DataCacheConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_caching` after provisioning.\n"]
    pub fn enable_caching(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_caching", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
    enforcement_mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_imbalance: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
    #[doc = "Set the field `max_imbalance`.\n"]
    pub fn set_max_imbalance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_imbalance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
    type O =
        terrars::BlockAssignable<SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
    #[doc = ""]
    pub enforcement_mode: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
        SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance {
            enforcement_mode: self.enforcement_mode,
            max_imbalance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalanceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalanceRef {
        SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalanceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalanceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enforcement_mode` after provisioning.\n"]
    pub fn enforcement_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enforcement_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `max_imbalance` after provisioning.\n"]
    pub fn max_imbalance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_imbalance", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponent_Specification_SchedulingConfigDynamic {
    availability_zone_balance: std
    ::option
    ::Option<
        terrars::DynamicBlock<SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance>,
    >,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_SchedulingConfig {
    placement_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_balance: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance>>,
    dynamic: SagemakerInferenceComponent_Specification_SchedulingConfigDynamic,
}

impl SagemakerInferenceComponent_Specification_SchedulingConfig {
    #[doc = "Set the field `availability_zone_balance`.\n"]
    pub fn set_availability_zone_balance(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalance,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.availability_zone_balance = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.availability_zone_balance = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_SchedulingConfig {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification_SchedulingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_SchedulingConfig {
    #[doc = ""]
    pub placement_strategy: terrars::PrimField<std::string::String>,
}

impl BuildSagemakerInferenceComponent_Specification_SchedulingConfig {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_SchedulingConfig {
        SagemakerInferenceComponent_Specification_SchedulingConfig {
            placement_strategy: self.placement_strategy,
            availability_zone_balance: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_SchedulingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_SchedulingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_SchedulingConfigRef {
        SagemakerInferenceComponent_Specification_SchedulingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_SchedulingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone_balance` after provisioning.\n"]
    pub fn availability_zone_balance(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_Specification_SchedulingConfig_AvailabilityZoneBalanceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zone_balance", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification_StartupParameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_startup_health_check_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_data_download_timeout_in_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl SagemakerInferenceComponent_Specification_StartupParameters {
    #[doc = "Set the field `container_startup_health_check_timeout_in_seconds`.\n"]
    pub fn set_container_startup_health_check_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.container_startup_health_check_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_data_download_timeout_in_seconds`.\n"]
    pub fn set_model_data_download_timeout_in_seconds(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.model_data_download_timeout_in_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification_StartupParameters {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification_StartupParameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification_StartupParameters {}

impl BuildSagemakerInferenceComponent_Specification_StartupParameters {
    pub fn build(self) -> SagemakerInferenceComponent_Specification_StartupParameters {
        SagemakerInferenceComponent_Specification_StartupParameters {
            container_startup_health_check_timeout_in_seconds: core::default::Default::default(),
            model_data_download_timeout_in_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_Specification_StartupParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_Specification_StartupParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SagemakerInferenceComponent_Specification_StartupParametersRef {
        SagemakerInferenceComponent_Specification_StartupParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_Specification_StartupParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `container_startup_health_check_timeout_in_seconds` after provisioning.\n"]
    pub fn container_startup_health_check_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.container_startup_health_check_timeout_in_seconds", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `model_data_download_timeout_in_seconds` after provisioning.\n"]
    pub fn model_data_download_timeout_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.model_data_download_timeout_in_seconds", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponent_SpecificationDynamic {
    compute_resource_requirements: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>>,
    container: std::option::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_Container>>,
    data_cache_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_DataCacheConfig>>,
    scheduling_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_SchedulingConfig>>,
    startup_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_StartupParameters>>,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_inference_component_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_resource_requirements: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_Container>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_config: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_DataCacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_config: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_SchedulingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_parameters: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_StartupParameters>>,
    dynamic: SagemakerInferenceComponent_SpecificationDynamic,
}

impl SagemakerInferenceComponent_Specification {
    #[doc = "Set the field `base_inference_component_name`.\n"]
    pub fn set_base_inference_component_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.base_inference_component_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_resource_requirements`.\n"]
    pub fn set_compute_resource_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compute_resource_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compute_resource_requirements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_DataCacheConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_cache_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_cache_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduling_config`.\n"]
    pub fn set_scheduling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_SchedulingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `startup_parameters`.\n"]
    pub fn set_startup_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_StartupParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.startup_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specification {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specification {}

impl BuildSagemakerInferenceComponent_Specification {
    pub fn build(self) -> SagemakerInferenceComponent_Specification {
        SagemakerInferenceComponent_Specification {
            base_inference_component_name: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            model_name: core::default::Default::default(),
            compute_resource_requirements: core::default::Default::default(),
            container: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            scheduling_config: core::default::Default::default(),
            startup_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_SpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_SpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerInferenceComponent_SpecificationRef {
        SagemakerInferenceComponent_SpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_SpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_inference_component_name` after provisioning.\n"]
    pub fn base_inference_component_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_inference_component_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_resource_requirements` after provisioning.\n"]
    pub fn compute_resource_requirements(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_resource_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_DataCacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_config` after provisioning.\n"]
    pub fn scheduling_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_SchedulingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_parameters` after provisioning.\n"]
    pub fn startup_parameters(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_StartupParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_parameters", self.base))
    }
}

#[derive(Serialize, Default)]
struct SagemakerInferenceComponent_SpecificationsDynamic {
    compute_resource_requirements: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>>,
    container: std::option::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_Container>>,
    data_cache_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_DataCacheConfig>>,
    scheduling_config: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_SchedulingConfig>>,
    startup_parameters: std
    ::option
    ::Option<terrars::DynamicBlock<SagemakerInferenceComponent_Specification_StartupParameters>>,
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Specifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    base_inference_component_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    model_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compute_resource_requirements: std
    ::option
    ::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_Container>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_cache_config: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_DataCacheConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheduling_config: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_SchedulingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    startup_parameters: std::option::Option<std::vec::Vec<SagemakerInferenceComponent_Specification_StartupParameters>>,
    dynamic: SagemakerInferenceComponent_SpecificationsDynamic,
}

impl SagemakerInferenceComponent_Specifications {
    #[doc = "Set the field `base_inference_component_name`.\n"]
    pub fn set_base_inference_component_name(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.base_inference_component_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `model_name`.\n"]
    pub fn set_model_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.model_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compute_resource_requirements`.\n"]
    pub fn set_compute_resource_requirements(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SagemakerInferenceComponent_Specification_ComputeResourceRequirements>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compute_resource_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compute_resource_requirements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `container`.\n"]
    pub fn set_container(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_Container>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `data_cache_config`.\n"]
    pub fn set_data_cache_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_DataCacheConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.data_cache_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.data_cache_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `scheduling_config`.\n"]
    pub fn set_scheduling_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_SchedulingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scheduling_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scheduling_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `startup_parameters`.\n"]
    pub fn set_startup_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SagemakerInferenceComponent_Specification_StartupParameters>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.startup_parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.startup_parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SagemakerInferenceComponent_Specifications {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Specifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Specifications {}

impl BuildSagemakerInferenceComponent_Specifications {
    pub fn build(self) -> SagemakerInferenceComponent_Specifications {
        SagemakerInferenceComponent_Specifications {
            base_inference_component_name: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            model_name: core::default::Default::default(),
            compute_resource_requirements: core::default::Default::default(),
            container: core::default::Default::default(),
            data_cache_config: core::default::Default::default(),
            scheduling_config: core::default::Default::default(),
            startup_parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_SpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_SpecificationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerInferenceComponent_SpecificationsRef {
        SagemakerInferenceComponent_SpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_SpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `base_inference_component_name` after provisioning.\n"]
    pub fn base_inference_component_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.base_inference_component_name", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `model_name` after provisioning.\n"]
    pub fn model_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model_name", self.base))
    }

    #[doc = "Get a reference to the value of field `compute_resource_requirements` after provisioning.\n"]
    pub fn compute_resource_requirements(
        &self,
    ) -> terrars::ListRef<SagemakerInferenceComponent_Specification_ComputeResourceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.compute_resource_requirements", self.base))
    }

    #[doc = "Get a reference to the value of field `container` after provisioning.\n"]
    pub fn container(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_ContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.container", self.base))
    }

    #[doc = "Get a reference to the value of field `data_cache_config` after provisioning.\n"]
    pub fn data_cache_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_DataCacheConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_cache_config", self.base))
    }

    #[doc = "Get a reference to the value of field `scheduling_config` after provisioning.\n"]
    pub fn scheduling_config(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_SchedulingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scheduling_config", self.base))
    }

    #[doc = "Get a reference to the value of field `startup_parameters` after provisioning.\n"]
    pub fn startup_parameters(&self) -> terrars::ListRef<SagemakerInferenceComponent_Specification_StartupParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.startup_parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct SagemakerInferenceComponent_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SagemakerInferenceComponent_Timeouts {
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

impl terrars::ToListMappable for SagemakerInferenceComponent_Timeouts {
    type O = terrars::BlockAssignable<SagemakerInferenceComponent_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSagemakerInferenceComponent_Timeouts {}

impl BuildSagemakerInferenceComponent_Timeouts {
    pub fn build(self) -> SagemakerInferenceComponent_Timeouts {
        SagemakerInferenceComponent_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SagemakerInferenceComponent_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SagemakerInferenceComponent_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SagemakerInferenceComponent_TimeoutsRef {
        SagemakerInferenceComponent_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SagemakerInferenceComponent_TimeoutsRef {
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
