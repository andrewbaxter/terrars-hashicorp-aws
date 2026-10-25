#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LambdamicrovmsMicrovmData_ {
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
    egress_network_connectors: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    image_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_network_connectors: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_duration_in_seconds: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    run_hook_payload: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_policy: std::option::Option<std::vec::Vec<LambdamicrovmsMicrovm_IdlePolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging: std::option::Option<std::vec::Vec<LambdamicrovmsMicrovm_Logging>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<LambdamicrovmsMicrovm_Timeouts>,
    dynamic: LambdamicrovmsMicrovmDynamic,
}

struct LambdamicrovmsMicrovm_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LambdamicrovmsMicrovmData_>,
}

#[derive(Clone)]
pub struct LambdamicrovmsMicrovm(std::rc::Rc<LambdamicrovmsMicrovm_>);

impl LambdamicrovmsMicrovm {
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

    #[doc = "Set the field `egress_network_connectors`.\n"]
    pub fn set_egress_network_connectors(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().egress_network_connectors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_role_arn`.\n"]
    pub fn set_execution_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().execution_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_version`.\n"]
    pub fn set_image_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_network_connectors`.\n"]
    pub fn set_ingress_network_connectors(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ingress_network_connectors = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_duration_in_seconds`.\n"]
    pub fn set_maximum_duration_in_seconds(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().maximum_duration_in_seconds = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `run_hook_payload`.\n"]
    pub fn set_run_hook_payload(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().run_hook_payload = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `idle_policy`.\n"]
    pub fn set_idle_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdamicrovmsMicrovm_IdlePolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().idle_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.idle_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging`.\n"]
    pub fn set_logging(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdamicrovmsMicrovm_Logging>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<LambdamicrovmsMicrovm_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `egress_network_connectors` after provisioning.\n"]
    pub fn egress_network_connectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_network_connectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_version` after provisioning.\n"]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_network_connectors` after provisioning.\n"]
    pub fn ingress_network_connectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_network_connectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_duration_in_seconds` after provisioning.\n"]
    pub fn maximum_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_duration_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `microvm_id` after provisioning.\n"]
    pub fn microvm_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.microvm_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_hook_payload` after provisioning.\n"]
    pub fn run_hook_payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_hook_payload", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `started_at` after provisioning.\n"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_policy` after provisioning.\n"]
    pub fn idle_policy(&self) -> terrars::ListRef<LambdamicrovmsMicrovm_IdlePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<LambdamicrovmsMicrovm_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdamicrovmsMicrovm_TimeoutsRef {
        LambdamicrovmsMicrovm_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for LambdamicrovmsMicrovm {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LambdamicrovmsMicrovm { }

impl terrars::ToListMappable for LambdamicrovmsMicrovm {
    type O = terrars::ListRef<LambdamicrovmsMicrovmRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LambdamicrovmsMicrovm_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_lambdamicrovms_microvm".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLambdamicrovmsMicrovm {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub image_arn: terrars::PrimField<std::string::String>,
}

impl BuildLambdamicrovmsMicrovm {
    pub fn build(self, stack: &mut terrars::Stack) -> LambdamicrovmsMicrovm {
        let out = LambdamicrovmsMicrovm(std::rc::Rc::new(LambdamicrovmsMicrovm_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LambdamicrovmsMicrovmData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                egress_network_connectors: core::default::Default::default(),
                execution_role_arn: core::default::Default::default(),
                image_arn: self.image_arn,
                image_version: core::default::Default::default(),
                ingress_network_connectors: core::default::Default::default(),
                maximum_duration_in_seconds: core::default::Default::default(),
                region: core::default::Default::default(),
                run_hook_payload: core::default::Default::default(),
                idle_policy: core::default::Default::default(),
                logging: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LambdamicrovmsMicrovmRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdamicrovmsMicrovmRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LambdamicrovmsMicrovmRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `egress_network_connectors` after provisioning.\n"]
    pub fn egress_network_connectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.egress_network_connectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_arn` after provisioning.\n"]
    pub fn image_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_version` after provisioning.\n"]
    pub fn image_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_network_connectors` after provisioning.\n"]
    pub fn ingress_network_connectors(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_network_connectors", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_duration_in_seconds` after provisioning.\n"]
    pub fn maximum_duration_in_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_duration_in_seconds", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `microvm_id` after provisioning.\n"]
    pub fn microvm_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.microvm_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `run_hook_payload` after provisioning.\n"]
    pub fn run_hook_payload(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.run_hook_payload", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `started_at` after provisioning.\n"]
    pub fn started_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.started_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `idle_policy` after provisioning.\n"]
    pub fn idle_policy(&self) -> terrars::ListRef<LambdamicrovmsMicrovm_IdlePolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging` after provisioning.\n"]
    pub fn logging(&self) -> terrars::ListRef<LambdamicrovmsMicrovm_LoggingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> LambdamicrovmsMicrovm_TimeoutsRef {
        LambdamicrovmsMicrovm_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LambdamicrovmsMicrovmDynamic {
    idle_policy: std::option::Option<terrars::DynamicBlock<LambdamicrovmsMicrovm_IdlePolicy>>,
    logging: std::option::Option<terrars::DynamicBlock<LambdamicrovmsMicrovm_Logging>>,
}

#[derive(Serialize)]
pub struct LambdamicrovmsMicrovm_IdlePolicy {
    auto_resume_enabled: terrars::PrimField<bool>,
    max_idle_duration_seconds: terrars::PrimField<f64>,
    suspended_duration_seconds: terrars::PrimField<f64>,
}

impl LambdamicrovmsMicrovm_IdlePolicy { }

impl terrars::ToListMappable for LambdamicrovmsMicrovm_IdlePolicy {
    type O = terrars::BlockAssignable<LambdamicrovmsMicrovm_IdlePolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdamicrovmsMicrovm_IdlePolicy {
    #[doc = ""]
    pub auto_resume_enabled: terrars::PrimField<bool>,
    #[doc = ""]
    pub max_idle_duration_seconds: terrars::PrimField<f64>,
    #[doc = ""]
    pub suspended_duration_seconds: terrars::PrimField<f64>,
}

impl BuildLambdamicrovmsMicrovm_IdlePolicy {
    pub fn build(self) -> LambdamicrovmsMicrovm_IdlePolicy {
        LambdamicrovmsMicrovm_IdlePolicy {
            auto_resume_enabled: self.auto_resume_enabled,
            max_idle_duration_seconds: self.max_idle_duration_seconds,
            suspended_duration_seconds: self.suspended_duration_seconds,
        }
    }
}

#[allow(dead_code)]
pub struct LambdamicrovmsMicrovm_IdlePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdamicrovmsMicrovm_IdlePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdamicrovmsMicrovm_IdlePolicyRef {
        LambdamicrovmsMicrovm_IdlePolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdamicrovmsMicrovm_IdlePolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_resume_enabled` after provisioning.\n"]
    pub fn auto_resume_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_resume_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_idle_duration_seconds` after provisioning.\n"]
    pub fn max_idle_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_idle_duration_seconds", self.base))
    }

    #[doc = "Get a reference to the value of field `suspended_duration_seconds` after provisioning.\n"]
    pub fn suspended_duration_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspended_duration_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdamicrovmsMicrovm_Logging_Cloudwatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdamicrovmsMicrovm_Logging_Cloudwatch {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream`.\n"]
    pub fn set_log_stream(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_stream = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LambdamicrovmsMicrovm_Logging_Cloudwatch {
    type O = terrars::BlockAssignable<LambdamicrovmsMicrovm_Logging_Cloudwatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdamicrovmsMicrovm_Logging_Cloudwatch {}

impl BuildLambdamicrovmsMicrovm_Logging_Cloudwatch {
    pub fn build(self) -> LambdamicrovmsMicrovm_Logging_Cloudwatch {
        LambdamicrovmsMicrovm_Logging_Cloudwatch {
            log_group: core::default::Default::default(),
            log_stream: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdamicrovmsMicrovm_Logging_CloudwatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdamicrovmsMicrovm_Logging_CloudwatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdamicrovmsMicrovm_Logging_CloudwatchRef {
        LambdamicrovmsMicrovm_Logging_CloudwatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdamicrovmsMicrovm_Logging_CloudwatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream` after provisioning.\n"]
    pub fn log_stream(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdamicrovmsMicrovm_Logging_Disabled {}

impl LambdamicrovmsMicrovm_Logging_Disabled { }

impl terrars::ToListMappable for LambdamicrovmsMicrovm_Logging_Disabled {
    type O = terrars::BlockAssignable<LambdamicrovmsMicrovm_Logging_Disabled>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdamicrovmsMicrovm_Logging_Disabled {}

impl BuildLambdamicrovmsMicrovm_Logging_Disabled {
    pub fn build(self) -> LambdamicrovmsMicrovm_Logging_Disabled {
        LambdamicrovmsMicrovm_Logging_Disabled {}
    }
}

#[allow(dead_code)]
pub struct LambdamicrovmsMicrovm_Logging_DisabledRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdamicrovmsMicrovm_Logging_DisabledRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdamicrovmsMicrovm_Logging_DisabledRef {
        LambdamicrovmsMicrovm_Logging_DisabledRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdamicrovmsMicrovm_Logging_DisabledRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct LambdamicrovmsMicrovm_LoggingDynamic {
    cloudwatch: std::option::Option<terrars::DynamicBlock<LambdamicrovmsMicrovm_Logging_Cloudwatch>>,
    disabled: std::option::Option<terrars::DynamicBlock<LambdamicrovmsMicrovm_Logging_Disabled>>,
}

#[derive(Serialize)]
pub struct LambdamicrovmsMicrovm_Logging {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cloudwatch: std::option::Option<std::vec::Vec<LambdamicrovmsMicrovm_Logging_Cloudwatch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disabled: std::option::Option<std::vec::Vec<LambdamicrovmsMicrovm_Logging_Disabled>>,
    dynamic: LambdamicrovmsMicrovm_LoggingDynamic,
}

impl LambdamicrovmsMicrovm_Logging {
    #[doc = "Set the field `cloudwatch`.\n"]
    pub fn set_cloudwatch(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdamicrovmsMicrovm_Logging_Cloudwatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cloudwatch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cloudwatch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `disabled`.\n"]
    pub fn set_disabled(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LambdamicrovmsMicrovm_Logging_Disabled>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.disabled = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.disabled = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LambdamicrovmsMicrovm_Logging {
    type O = terrars::BlockAssignable<LambdamicrovmsMicrovm_Logging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdamicrovmsMicrovm_Logging {}

impl BuildLambdamicrovmsMicrovm_Logging {
    pub fn build(self) -> LambdamicrovmsMicrovm_Logging {
        LambdamicrovmsMicrovm_Logging {
            cloudwatch: core::default::Default::default(),
            disabled: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdamicrovmsMicrovm_LoggingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdamicrovmsMicrovm_LoggingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdamicrovmsMicrovm_LoggingRef {
        LambdamicrovmsMicrovm_LoggingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdamicrovmsMicrovm_LoggingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cloudwatch` after provisioning.\n"]
    pub fn cloudwatch(&self) -> terrars::ListRef<LambdamicrovmsMicrovm_Logging_CloudwatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cloudwatch", self.base))
    }

    #[doc = "Get a reference to the value of field `disabled` after provisioning.\n"]
    pub fn disabled(&self) -> terrars::ListRef<LambdamicrovmsMicrovm_Logging_DisabledRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.disabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LambdamicrovmsMicrovm_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LambdamicrovmsMicrovm_Timeouts {
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
}

impl terrars::ToListMappable for LambdamicrovmsMicrovm_Timeouts {
    type O = terrars::BlockAssignable<LambdamicrovmsMicrovm_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLambdamicrovmsMicrovm_Timeouts {}

impl BuildLambdamicrovmsMicrovm_Timeouts {
    pub fn build(self) -> LambdamicrovmsMicrovm_Timeouts {
        LambdamicrovmsMicrovm_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LambdamicrovmsMicrovm_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LambdamicrovmsMicrovm_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LambdamicrovmsMicrovm_TimeoutsRef {
        LambdamicrovmsMicrovm_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LambdamicrovmsMicrovm_TimeoutsRef {
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
}
