#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdaCapacityProviderData_ {
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
    capacity_provider_scaling_config: std
    ::option
    ::Option<terrars::ListField<LambdaCapacityProvider_CapacityProviderScalingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std::option::Option<terrars::ListField<LambdaCapacityProvider_InstanceRequirements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    permissions_config: std::option::Option<std::vec::Vec<LambdaCapacityProvider_PermissionsConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdaCapacityProvider_Timeouts>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_config: std::option::Option<std::vec::Vec<LambdaCapacityProvider_VpcConfig>>,
    dynamic: LambdaCapacityProviderDynamic,
}

struct LambdaCapacityProvider_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdaCapacityProviderData_>,
}

#[derive(Clone)]
pub struct LambdaCapacityProvider(std::rc::Rc<LambdaCapacityProvider_>);

impl LambdaCapacityProvider {
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

    #[doc = "Set the field `capacity_provider_scaling_config`.\n"]
    pub fn set_capacity_provider_scaling_config(
        self,
        v: impl std::convert::Into<terrars::ListField<LambdaCapacityProvider_CapacityProviderScalingConfig>>,
    ) -> Self {
        self.0.data.borrow_mut().capacity_provider_scaling_config = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        self,
        v: impl std::convert::Into<terrars::ListField<LambdaCapacityProvider_InstanceRequirements>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_requirements = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `permissions_config`.\n"]
    pub fn set_permissions_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaCapacityProvider_PermissionsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().permissions_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.permissions_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdaCapacityProvider_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_config`.\n"]
    pub fn set_vpc_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdaCapacityProvider_VpcConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().vpc_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.vpc_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_provider_scaling_config` after provisioning.\n"]
    pub fn capacity_provider_scaling_config(
        &self,
    ) -> terrars::ListRef<LambdaCapacityProvider_CapacityProviderScalingConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_provider_scaling_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<LambdaCapacityProvider_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `permissions_config` after provisioning.\n"]
    pub fn permissions_config(&self) -> terrars::ListRef<LambdaCapacityProvider_PermissionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaCapacityProvider_TimeoutsRef {
        LambdaCapacityProvider_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<LambdaCapacityProvider_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

impl terrars::Referable for LambdaCapacityProvider {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdaCapacityProvider { }

impl terrars::ToListMappable for LambdaCapacityProvider {
    type O = terrars::ListRef<LambdaCapacityProviderRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdaCapacityProvider_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambda_capacity_provider".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdaCapacityProvider {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildLambdaCapacityProvider {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdaCapacityProvider {
        let out = LambdaCapacityProvider(std::rc::Rc::new(LambdaCapacityProvider_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdaCapacityProviderData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                capacity_provider_scaling_config: core::default::Default::default(),
                instance_requirements: core::default::Default::default(),
                kms_key_arn: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                permissions_config: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                vpc_config: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdaCapacityProviderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProviderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdaCapacityProviderRef {
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

    #[doc = "Get a reference to the value of field `capacity_provider_scaling_config` after provisioning.\n"]
    pub fn capacity_provider_scaling_config(
        &self,
    ) -> terrars::ListRef<LambdaCapacityProvider_CapacityProviderScalingConfigRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_provider_scaling_config", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<LambdaCapacityProvider_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `permissions_config` after provisioning.\n"]
    pub fn permissions_config(&self) -> terrars::ListRef<LambdaCapacityProvider_PermissionsConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.permissions_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdaCapacityProvider_TimeoutsRef {
        LambdaCapacityProvider_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_config` after provisioning.\n"]
    pub fn vpc_config(&self) -> terrars::ListRef<LambdaCapacityProvider_VpcConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_config", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LambdaCapacityProviderDynamic {
    permissions_config: std::option::Option<terrars::DynamicBlock<LambdaCapacityProvider_PermissionsConfig>>,
    vpc_config: std::option::Option<terrars::DynamicBlock<LambdaCapacityProvider_VpcConfig>>,
}

#[derive(Serialize)]
pub struct LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    predefined_metric_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_value: std::option::Option<terrars::PrimField<f64>>,
}

impl LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {
    #[doc = "Set the field `predefined_metric_type`.\n"]
    pub fn set_predefined_metric_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.predefined_metric_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_value`.\n"]
    pub fn set_target_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {
    type O = terrars::BlockAssignable<LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {}

impl BuildLambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {
    pub fn build(self) -> LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {
        LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies {
            predefined_metric_type: core::default::Default::default(),
            target_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPoliciesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPoliciesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPoliciesRef {
        LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPoliciesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPoliciesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `predefined_metric_type` after provisioning.\n"]
    pub fn predefined_metric_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.predefined_metric_type", self.base))
    }

    #[doc = "Get a reference to the value of field `target_value` after provisioning.\n"]
    pub fn target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_value", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaCapacityProvider_CapacityProviderScalingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_vcpu_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_mode: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_policies: std
    ::option
    ::Option<terrars::ListField<LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies>>,
}

impl LambdaCapacityProvider_CapacityProviderScalingConfig {
    #[doc = "Set the field `max_vcpu_count`.\n"]
    pub fn set_max_vcpu_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_vcpu_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_mode`.\n"]
    pub fn set_scaling_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scaling_mode = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_policies`.\n"]
    pub fn set_scaling_policies(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPolicies>>,
    ) -> Self {
        self.scaling_policies = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaCapacityProvider_CapacityProviderScalingConfig {
    type O = terrars::BlockAssignable<LambdaCapacityProvider_CapacityProviderScalingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCapacityProvider_CapacityProviderScalingConfig {}

impl BuildLambdaCapacityProvider_CapacityProviderScalingConfig {
    pub fn build(self) -> LambdaCapacityProvider_CapacityProviderScalingConfig {
        LambdaCapacityProvider_CapacityProviderScalingConfig {
            max_vcpu_count: core::default::Default::default(),
            scaling_mode: core::default::Default::default(),
            scaling_policies: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaCapacityProvider_CapacityProviderScalingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProvider_CapacityProviderScalingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaCapacityProvider_CapacityProviderScalingConfigRef {
        LambdaCapacityProvider_CapacityProviderScalingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCapacityProvider_CapacityProviderScalingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max_vcpu_count` after provisioning.\n"]
    pub fn max_vcpu_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_vcpu_count", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_mode` after provisioning.\n"]
    pub fn scaling_mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scaling_mode", self.base))
    }

    #[doc = "Get a reference to the value of field `scaling_policies` after provisioning.\n"]
    pub fn scaling_policies(
        &self,
    ) -> terrars::ListRef<LambdaCapacityProvider_CapacityProviderScalingConfig_ScalingPoliciesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_policies", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaCapacityProvider_InstanceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_instance_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architectures: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_instance_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl LambdaCapacityProvider_InstanceRequirements {
    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `architectures`.\n"]
    pub fn set_architectures(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.architectures = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_instance_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdaCapacityProvider_InstanceRequirements {
    type O = terrars::BlockAssignable<LambdaCapacityProvider_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCapacityProvider_InstanceRequirements {}

impl BuildLambdaCapacityProvider_InstanceRequirements {
    pub fn build(self) -> LambdaCapacityProvider_InstanceRequirements {
        LambdaCapacityProvider_InstanceRequirements {
            allowed_instance_types: core::default::Default::default(),
            architectures: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaCapacityProvider_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProvider_InstanceRequirementsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LambdaCapacityProvider_InstanceRequirementsRef {
        LambdaCapacityProvider_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCapacityProvider_InstanceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `architectures` after provisioning.\n"]
    pub fn architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.architectures", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.excluded_instance_types", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaCapacityProvider_PermissionsConfig {
    capacity_provider_operator_role_arn: terrars::PrimField<std::string::String>,
}

impl LambdaCapacityProvider_PermissionsConfig { }

impl terrars::ToListMappable for LambdaCapacityProvider_PermissionsConfig {
    type O = terrars::BlockAssignable<LambdaCapacityProvider_PermissionsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCapacityProvider_PermissionsConfig {
    #[doc = ""]
    pub capacity_provider_operator_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdaCapacityProvider_PermissionsConfig {
    pub fn build(self) -> LambdaCapacityProvider_PermissionsConfig {
        LambdaCapacityProvider_PermissionsConfig {
            capacity_provider_operator_role_arn: self.capacity_provider_operator_role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct LambdaCapacityProvider_PermissionsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProvider_PermissionsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaCapacityProvider_PermissionsConfigRef {
        LambdaCapacityProvider_PermissionsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCapacityProvider_PermissionsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_provider_operator_role_arn` after provisioning.\n"]
    pub fn capacity_provider_operator_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_provider_operator_role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdaCapacityProvider_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdaCapacityProvider_Timeouts {
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

impl terrars::ToListMappable for LambdaCapacityProvider_Timeouts {
    type O = terrars::BlockAssignable<LambdaCapacityProvider_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCapacityProvider_Timeouts {}

impl BuildLambdaCapacityProvider_Timeouts {
    pub fn build(self) -> LambdaCapacityProvider_Timeouts {
        LambdaCapacityProvider_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdaCapacityProvider_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProvider_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaCapacityProvider_TimeoutsRef {
        LambdaCapacityProvider_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCapacityProvider_TimeoutsRef {
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

#[derive(Serialize)]
pub struct LambdaCapacityProvider_VpcConfig {
    security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl LambdaCapacityProvider_VpcConfig { }

impl terrars::ToListMappable for LambdaCapacityProvider_VpcConfig {
    type O = terrars::BlockAssignable<LambdaCapacityProvider_VpcConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdaCapacityProvider_VpcConfig {
    #[doc = ""]
    pub security_group_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub subnet_ids: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildLambdaCapacityProvider_VpcConfig {
    pub fn build(self) -> LambdaCapacityProvider_VpcConfig {
        LambdaCapacityProvider_VpcConfig {
            security_group_ids: self.security_group_ids,
            subnet_ids: self.subnet_ids,
        }
    }
}

#[allow(dead_code)]
pub struct LambdaCapacityProvider_VpcConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdaCapacityProvider_VpcConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdaCapacityProvider_VpcConfigRef {
        LambdaCapacityProvider_VpcConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdaCapacityProvider_VpcConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_ids", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}
