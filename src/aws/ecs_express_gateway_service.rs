#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EcsExpressGatewayServiceData_ {
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
    cluster: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu: std::option::Option<terrars::PrimField<std::string::String>>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    health_check_path: std::option::Option<terrars::PrimField<std::string::String>>,
    infrastructure_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_configuration: std::option::Option<terrars::ListField<EcsExpressGatewayService_NetworkConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scaling_target: std::option::Option<terrars::ListField<EcsExpressGatewayService_ScalingTarget>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    task_role_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_steady_state: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_container: std::option::Option<std::vec::Vec<EcsExpressGatewayService_PrimaryContainer>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<EcsExpressGatewayService_Timeouts>,
    dynamic: EcsExpressGatewayServiceDynamic,
}

struct EcsExpressGatewayService_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EcsExpressGatewayServiceData_>,
}

#[derive(Clone)]
pub struct EcsExpressGatewayService(std::rc::Rc<EcsExpressGatewayService_>);

impl EcsExpressGatewayService {
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

    #[doc = "Set the field `cluster`.\n"]
    pub fn set_cluster(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cluster = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu`.\n"]
    pub fn set_cpu(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `health_check_path`.\n"]
    pub fn set_health_check_path(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().health_check_path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory`.\n"]
    pub fn set_memory(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().memory = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_configuration`.\n"]
    pub fn set_network_configuration(
        self,
        v: impl std::convert::Into<terrars::ListField<EcsExpressGatewayService_NetworkConfiguration>>,
    ) -> Self {
        self.0.data.borrow_mut().network_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scaling_target`.\n"]
    pub fn set_scaling_target(
        self,
        v: impl std::convert::Into<terrars::ListField<EcsExpressGatewayService_ScalingTarget>>,
    ) -> Self {
        self.0.data.borrow_mut().scaling_target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().service_name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `task_role_arn`.\n"]
    pub fn set_task_role_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().task_role_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_for_steady_state`.\n"]
    pub fn set_wait_for_steady_state(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_steady_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_container`.\n"]
    pub fn set_primary_container(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().primary_container = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.primary_container = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<EcsExpressGatewayService_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_deployment` after provisioning.\n"]
    pub fn current_deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_path` after provisioning.\n"]
    pub fn health_check_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_role_arn` after provisioning.\n"]
    pub fn infrastructure_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_paths` after provisioning.\n"]
    pub fn ingress_paths(&self) -> terrars::ListRef<EcsExpressGatewayService_IngressPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_paths", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsExpressGatewayService_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_target` after provisioning.\n"]
    pub fn scaling_target(&self) -> terrars::ListRef<EcsExpressGatewayService_ScalingTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_revision_arn` after provisioning.\n"]
    pub fn service_revision_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_revision_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_steady_state` after provisioning.\n"]
    pub fn wait_for_steady_state(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_steady_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> terrars::ListRef<EcsExpressGatewayService_PrimaryContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsExpressGatewayService_TimeoutsRef {
        EcsExpressGatewayService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for EcsExpressGatewayService {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EcsExpressGatewayService { }

impl terrars::ToListMappable for EcsExpressGatewayService {
    type O = terrars::ListRef<EcsExpressGatewayServiceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EcsExpressGatewayService_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ecs_express_gateway_service".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEcsExpressGatewayService {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub infrastructure_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildEcsExpressGatewayService {
    pub fn build(self, stack: &mut terrars::Stack) -> EcsExpressGatewayService {
        let out = EcsExpressGatewayService(std::rc::Rc::new(EcsExpressGatewayService_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EcsExpressGatewayServiceData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster: core::default::Default::default(),
                cpu: core::default::Default::default(),
                execution_role_arn: self.execution_role_arn,
                health_check_path: core::default::Default::default(),
                infrastructure_role_arn: self.infrastructure_role_arn,
                memory: core::default::Default::default(),
                network_configuration: core::default::Default::default(),
                region: core::default::Default::default(),
                scaling_target: core::default::Default::default(),
                service_name: core::default::Default::default(),
                tags: core::default::Default::default(),
                task_role_arn: core::default::Default::default(),
                wait_for_steady_state: core::default::Default::default(),
                primary_container: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EcsExpressGatewayServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EcsExpressGatewayServiceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster` after provisioning.\n"]
    pub fn cluster(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu` after provisioning.\n"]
    pub fn cpu(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `current_deployment` after provisioning.\n"]
    pub fn current_deployment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_deployment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `health_check_path` after provisioning.\n"]
    pub fn health_check_path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.health_check_path", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_role_arn` after provisioning.\n"]
    pub fn infrastructure_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ingress_paths` after provisioning.\n"]
    pub fn ingress_paths(&self) -> terrars::ListRef<EcsExpressGatewayService_IngressPathsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_paths", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory` after provisioning.\n"]
    pub fn memory(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_configuration` after provisioning.\n"]
    pub fn network_configuration(&self) -> terrars::ListRef<EcsExpressGatewayService_NetworkConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_configuration", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scaling_target` after provisioning.\n"]
    pub fn scaling_target(&self) -> terrars::ListRef<EcsExpressGatewayService_ScalingTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scaling_target", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_arn` after provisioning.\n"]
    pub fn service_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_revision_arn` after provisioning.\n"]
    pub fn service_revision_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_revision_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `task_role_arn` after provisioning.\n"]
    pub fn task_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.task_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_steady_state` after provisioning.\n"]
    pub fn wait_for_steady_state(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_steady_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_container` after provisioning.\n"]
    pub fn primary_container(&self) -> terrars::ListRef<EcsExpressGatewayService_PrimaryContainerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.primary_container", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> EcsExpressGatewayService_TimeoutsRef {
        EcsExpressGatewayService_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EcsExpressGatewayServiceDynamic {
    primary_container: std::option::Option<terrars::DynamicBlock<EcsExpressGatewayService_PrimaryContainer>>,
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_IngressPaths {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    access_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsExpressGatewayService_IngressPaths {
    #[doc = "Set the field `access_type`.\n"]
    pub fn set_access_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.access_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `endpoint`.\n"]
    pub fn set_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsExpressGatewayService_IngressPaths {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_IngressPaths>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_IngressPaths {}

impl BuildEcsExpressGatewayService_IngressPaths {
    pub fn build(self) -> EcsExpressGatewayService_IngressPaths {
        EcsExpressGatewayService_IngressPaths {
            access_type: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_IngressPathsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_IngressPathsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsExpressGatewayService_IngressPathsRef {
        EcsExpressGatewayService_IngressPathsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_IngressPathsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `access_type` after provisioning.\n"]
    pub fn access_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.access_type", self.base))
    }

    #[doc = "Get a reference to the value of field `endpoint` after provisioning.\n"]
    pub fn endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_NetworkConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnets: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EcsExpressGatewayService_NetworkConfiguration {
    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnets`.\n"]
    pub fn set_subnets(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnets = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsExpressGatewayService_NetworkConfiguration {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_NetworkConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_NetworkConfiguration {}

impl BuildEcsExpressGatewayService_NetworkConfiguration {
    pub fn build(self) -> EcsExpressGatewayService_NetworkConfiguration {
        EcsExpressGatewayService_NetworkConfiguration {
            security_groups: core::default::Default::default(),
            subnets: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_NetworkConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_NetworkConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsExpressGatewayService_NetworkConfigurationRef {
        EcsExpressGatewayService_NetworkConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_NetworkConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnets` after provisioning.\n"]
    pub fn subnets(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnets", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_ScalingTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_metric: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_target_value: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_task_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_task_count: std::option::Option<terrars::PrimField<f64>>,
}

impl EcsExpressGatewayService_ScalingTarget {
    #[doc = "Set the field `auto_scaling_metric`.\n"]
    pub fn set_auto_scaling_metric(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.auto_scaling_metric = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_scaling_target_value`.\n"]
    pub fn set_auto_scaling_target_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.auto_scaling_target_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_task_count`.\n"]
    pub fn set_max_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_task_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min_task_count`.\n"]
    pub fn set_min_task_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min_task_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsExpressGatewayService_ScalingTarget {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_ScalingTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_ScalingTarget {}

impl BuildEcsExpressGatewayService_ScalingTarget {
    pub fn build(self) -> EcsExpressGatewayService_ScalingTarget {
        EcsExpressGatewayService_ScalingTarget {
            auto_scaling_metric: core::default::Default::default(),
            auto_scaling_target_value: core::default::Default::default(),
            max_task_count: core::default::Default::default(),
            min_task_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_ScalingTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_ScalingTargetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsExpressGatewayService_ScalingTargetRef {
        EcsExpressGatewayService_ScalingTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_ScalingTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_scaling_metric` after provisioning.\n"]
    pub fn auto_scaling_metric(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_metric", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_target_value` after provisioning.\n"]
    pub fn auto_scaling_target_value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_target_value", self.base))
    }

    #[doc = "Get a reference to the value of field `max_task_count` after provisioning.\n"]
    pub fn max_task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_task_count", self.base))
    }

    #[doc = "Get a reference to the value of field `min_task_count` after provisioning.\n"]
    pub fn min_task_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min_task_count", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_stream_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {
    #[doc = "Set the field `log_group`.\n"]
    pub fn set_log_group(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_stream_prefix`.\n"]
    pub fn set_log_stream_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.log_stream_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {}

impl BuildEcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {
    pub fn build(self) -> EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {
        EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration {
            log_group: core::default::Default::default(),
            log_stream_prefix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_PrimaryContainer_AwsLogsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_PrimaryContainer_AwsLogsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsExpressGatewayService_PrimaryContainer_AwsLogsConfigurationRef {
        EcsExpressGatewayService_PrimaryContainer_AwsLogsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_PrimaryContainer_AwsLogsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group` after provisioning.\n"]
    pub fn log_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group", self.base))
    }

    #[doc = "Get a reference to the value of field `log_stream_prefix` after provisioning.\n"]
    pub fn log_stream_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_stream_prefix", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_PrimaryContainer_Environment {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl EcsExpressGatewayService_PrimaryContainer_Environment { }

impl terrars::ToListMappable for EcsExpressGatewayService_PrimaryContainer_Environment {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_Environment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_PrimaryContainer_Environment {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildEcsExpressGatewayService_PrimaryContainer_Environment {
    pub fn build(self) -> EcsExpressGatewayService_PrimaryContainer_Environment {
        EcsExpressGatewayService_PrimaryContainer_Environment {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_PrimaryContainer_EnvironmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_PrimaryContainer_EnvironmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsExpressGatewayService_PrimaryContainer_EnvironmentRef {
        EcsExpressGatewayService_PrimaryContainer_EnvironmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_PrimaryContainer_EnvironmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials {
    credentials_parameter: terrars::PrimField<std::string::String>,
}

impl EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials { }

impl terrars::ToListMappable for EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_PrimaryContainer_RepositoryCredentials {
    #[doc = ""]
    pub credentials_parameter: terrars::PrimField<std::string::String>,
}

impl BuildEcsExpressGatewayService_PrimaryContainer_RepositoryCredentials {
    pub fn build(self) -> EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials {
        EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials {
            credentials_parameter: self.credentials_parameter,
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_PrimaryContainer_RepositoryCredentialsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_PrimaryContainer_RepositoryCredentialsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsExpressGatewayService_PrimaryContainer_RepositoryCredentialsRef {
        EcsExpressGatewayService_PrimaryContainer_RepositoryCredentialsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_PrimaryContainer_RepositoryCredentialsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `credentials_parameter` after provisioning.\n"]
    pub fn credentials_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.credentials_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_PrimaryContainer_Secret {
    name: terrars::PrimField<std::string::String>,
    value_from: terrars::PrimField<std::string::String>,
}

impl EcsExpressGatewayService_PrimaryContainer_Secret { }

impl terrars::ToListMappable for EcsExpressGatewayService_PrimaryContainer_Secret {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_Secret>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_PrimaryContainer_Secret {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value_from: terrars::PrimField<std::string::String>,
}

impl BuildEcsExpressGatewayService_PrimaryContainer_Secret {
    pub fn build(self) -> EcsExpressGatewayService_PrimaryContainer_Secret {
        EcsExpressGatewayService_PrimaryContainer_Secret {
            name: self.name,
            value_from: self.value_from,
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_PrimaryContainer_SecretRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_PrimaryContainer_SecretRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EcsExpressGatewayService_PrimaryContainer_SecretRef {
        EcsExpressGatewayService_PrimaryContainer_SecretRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_PrimaryContainer_SecretRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value_from` after provisioning.\n"]
    pub fn value_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value_from", self.base))
    }
}

#[derive(Serialize, Default)]
struct EcsExpressGatewayService_PrimaryContainerDynamic {
    environment: std::option::Option<terrars::DynamicBlock<EcsExpressGatewayService_PrimaryContainer_Environment>>,
    repository_credentials: std
    ::option
    ::Option<terrars::DynamicBlock<EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials>>,
    secret: std::option::Option<terrars::DynamicBlock<EcsExpressGatewayService_PrimaryContainer_Secret>>,
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_PrimaryContainer {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_logs_configuration: std
    ::option
    ::Option<terrars::ListField<EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    command: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    container_port: std::option::Option<terrars::PrimField<f64>>,
    image: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    environment: std::option::Option<std::vec::Vec<EcsExpressGatewayService_PrimaryContainer_Environment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    repository_credentials: std
    ::option
    ::Option<std::vec::Vec<EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secret: std::option::Option<std::vec::Vec<EcsExpressGatewayService_PrimaryContainer_Secret>>,
    dynamic: EcsExpressGatewayService_PrimaryContainerDynamic,
}

impl EcsExpressGatewayService_PrimaryContainer {
    #[doc = "Set the field `aws_logs_configuration`.\n"]
    pub fn set_aws_logs_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<EcsExpressGatewayService_PrimaryContainer_AwsLogsConfiguration>>,
    ) -> Self {
        self.aws_logs_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `command`.\n"]
    pub fn set_command(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.command = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `container_port`.\n"]
    pub fn set_container_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.container_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `environment`.\n"]
    pub fn set_environment(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_Environment>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.environment = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.environment = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `repository_credentials`.\n"]
    pub fn set_repository_credentials(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_RepositoryCredentials>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.repository_credentials = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.repository_credentials = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secret`.\n"]
    pub fn set_secret(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer_Secret>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.secret = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.secret = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EcsExpressGatewayService_PrimaryContainer {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_PrimaryContainer>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_PrimaryContainer {
    #[doc = ""]
    pub image: terrars::PrimField<std::string::String>,
}

impl BuildEcsExpressGatewayService_PrimaryContainer {
    pub fn build(self) -> EcsExpressGatewayService_PrimaryContainer {
        EcsExpressGatewayService_PrimaryContainer {
            aws_logs_configuration: core::default::Default::default(),
            command: core::default::Default::default(),
            container_port: core::default::Default::default(),
            image: self.image,
            environment: core::default::Default::default(),
            repository_credentials: core::default::Default::default(),
            secret: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_PrimaryContainerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_PrimaryContainerRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsExpressGatewayService_PrimaryContainerRef {
        EcsExpressGatewayService_PrimaryContainerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_PrimaryContainerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `aws_logs_configuration` after provisioning.\n"]
    pub fn aws_logs_configuration(
        &self,
    ) -> terrars::ListRef<EcsExpressGatewayService_PrimaryContainer_AwsLogsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_logs_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `command` after provisioning.\n"]
    pub fn command(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.command", self.base))
    }

    #[doc = "Get a reference to the value of field `container_port` after provisioning.\n"]
    pub fn container_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.container_port", self.base))
    }

    #[doc = "Get a reference to the value of field `image` after provisioning.\n"]
    pub fn image(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image", self.base))
    }

    #[doc = "Get a reference to the value of field `environment` after provisioning.\n"]
    pub fn environment(&self) -> terrars::ListRef<EcsExpressGatewayService_PrimaryContainer_EnvironmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.environment", self.base))
    }

    #[doc = "Get a reference to the value of field `repository_credentials` after provisioning.\n"]
    pub fn repository_credentials(
        &self,
    ) -> terrars::ListRef<EcsExpressGatewayService_PrimaryContainer_RepositoryCredentialsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.repository_credentials", self.base))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::ListRef<EcsExpressGatewayService_PrimaryContainer_SecretRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secret", self.base))
    }
}

#[derive(Serialize)]
pub struct EcsExpressGatewayService_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EcsExpressGatewayService_Timeouts {
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

impl terrars::ToListMappable for EcsExpressGatewayService_Timeouts {
    type O = terrars::BlockAssignable<EcsExpressGatewayService_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEcsExpressGatewayService_Timeouts {}

impl BuildEcsExpressGatewayService_Timeouts {
    pub fn build(self) -> EcsExpressGatewayService_Timeouts {
        EcsExpressGatewayService_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EcsExpressGatewayService_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EcsExpressGatewayService_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EcsExpressGatewayService_TimeoutsRef {
        EcsExpressGatewayService_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EcsExpressGatewayService_TimeoutsRef {
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
