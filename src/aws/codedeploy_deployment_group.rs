#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodedeployDeploymentGroupData_ {
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
    app_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_config_name: std::option::Option<terrars::PrimField<std::string::String>>,
    deployment_group_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outdated_instances_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    service_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_hook_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarm_configuration: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_AlarmConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_rollback_configuration: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_AutoRollbackConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_deployment_config: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_BlueGreenDeploymentConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_style: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_DeploymentStyle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_tag_filter: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_Ec2TagFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_tag_set: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_Ec2TagSet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_service: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_EcsService>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_info: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_premises_instance_tag_filter: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_OnPremisesInstanceTagFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_configuration: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_TriggerConfiguration>>,
    dynamic: CodedeployDeploymentGroupDynamic,
}

struct CodedeployDeploymentGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodedeployDeploymentGroupData_>,
}

#[derive(Clone)]
pub struct CodedeployDeploymentGroup(std::rc::Rc<CodedeployDeploymentGroup_>);

impl CodedeployDeploymentGroup {
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

    #[doc = "Set the field `autoscaling_groups`.\n"]
    pub fn set_autoscaling_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().autoscaling_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_config_name`.\n"]
    pub fn set_deployment_config_name(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().deployment_config_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outdated_instances_strategy`.\n"]
    pub fn set_outdated_instances_strategy(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().outdated_instances_strategy = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `termination_hook_enabled`.\n"]
    pub fn set_termination_hook_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().termination_hook_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `alarm_configuration`.\n"]
    pub fn set_alarm_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_AlarmConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().alarm_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.alarm_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `auto_rollback_configuration`.\n"]
    pub fn set_auto_rollback_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_AutoRollbackConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_rollback_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_rollback_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `blue_green_deployment_config`.\n"]
    pub fn set_blue_green_deployment_config(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_BlueGreenDeploymentConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().blue_green_deployment_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.blue_green_deployment_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deployment_style`.\n"]
    pub fn set_deployment_style(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_DeploymentStyle>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().deployment_style = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.deployment_style = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_tag_filter`.\n"]
    pub fn set_ec2_tag_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_Ec2TagFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_tag_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_tag_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_tag_set`.\n"]
    pub fn set_ec2_tag_set(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_Ec2TagSet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_tag_set = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_tag_set = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecs_service`.\n"]
    pub fn set_ecs_service(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_EcsService>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ecs_service = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ecs_service = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `load_balancer_info`.\n"]
    pub fn set_load_balancer_info(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().load_balancer_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.load_balancer_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `on_premises_instance_tag_filter`.\n"]
    pub fn set_on_premises_instance_tag_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_OnPremisesInstanceTagFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().on_premises_instance_tag_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.on_premises_instance_tag_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `trigger_configuration`.\n"]
    pub fn set_trigger_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_TriggerConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().trigger_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.trigger_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `app_name` after provisioning.\n"]
    pub fn app_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_group_id` after provisioning.\n"]
    pub fn deployment_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_group_name` after provisioning.\n"]
    pub fn deployment_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outdated_instances_strategy` after provisioning.\n"]
    pub fn outdated_instances_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outdated_instances_strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_hook_enabled` after provisioning.\n"]
    pub fn termination_hook_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_hook_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_configuration` after provisioning.\n"]
    pub fn alarm_configuration(&self) -> terrars::ListRef<CodedeployDeploymentGroup_AlarmConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_AutoRollbackConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blue_green_deployment_config` after provisioning.\n"]
    pub fn blue_green_deployment_config(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_deployment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_style` after provisioning.\n"]
    pub fn deployment_style(&self) -> terrars::ListRef<CodedeployDeploymentGroup_DeploymentStyleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_service` after provisioning.\n"]
    pub fn ecs_service(&self) -> terrars::ListRef<CodedeployDeploymentGroup_EcsServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_info` after provisioning.\n"]
    pub fn load_balancer_info(&self) -> terrars::ListRef<CodedeployDeploymentGroup_LoadBalancerInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_info", self.extract_ref()))
    }
}

impl terrars::Referable for CodedeployDeploymentGroup {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodedeployDeploymentGroup { }

impl terrars::ToListMappable for CodedeployDeploymentGroup {
    type O = terrars::ListRef<CodedeployDeploymentGroupRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodedeployDeploymentGroup_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codedeploy_deployment_group".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodedeployDeploymentGroup {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub app_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub deployment_group_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroup {
    pub fn build(self, stack: &mut terrars::Stack) -> CodedeployDeploymentGroup {
        let out = CodedeployDeploymentGroup(std::rc::Rc::new(CodedeployDeploymentGroup_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodedeployDeploymentGroupData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                app_name: self.app_name,
                autoscaling_groups: core::default::Default::default(),
                deployment_config_name: core::default::Default::default(),
                deployment_group_name: self.deployment_group_name,
                id: core::default::Default::default(),
                outdated_instances_strategy: core::default::Default::default(),
                region: core::default::Default::default(),
                service_role_arn: self.service_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                termination_hook_enabled: core::default::Default::default(),
                alarm_configuration: core::default::Default::default(),
                auto_rollback_configuration: core::default::Default::default(),
                blue_green_deployment_config: core::default::Default::default(),
                deployment_style: core::default::Default::default(),
                ec2_tag_filter: core::default::Default::default(),
                ec2_tag_set: core::default::Default::default(),
                ecs_service: core::default::Default::default(),
                load_balancer_info: core::default::Default::default(),
                on_premises_instance_tag_filter: core::default::Default::default(),
                trigger_configuration: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodedeployDeploymentGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodedeployDeploymentGroupRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `app_name` after provisioning.\n"]
    pub fn app_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.app_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_groups` after provisioning.\n"]
    pub fn autoscaling_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.autoscaling_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `compute_platform` after provisioning.\n"]
    pub fn compute_platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compute_platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_config_name` after provisioning.\n"]
    pub fn deployment_config_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_config_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_group_id` after provisioning.\n"]
    pub fn deployment_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_group_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_group_name` after provisioning.\n"]
    pub fn deployment_group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_group_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outdated_instances_strategy` after provisioning.\n"]
    pub fn outdated_instances_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outdated_instances_strategy", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role_arn` after provisioning.\n"]
    pub fn service_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_hook_enabled` after provisioning.\n"]
    pub fn termination_hook_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_hook_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `alarm_configuration` after provisioning.\n"]
    pub fn alarm_configuration(&self) -> terrars::ListRef<CodedeployDeploymentGroup_AlarmConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_AutoRollbackConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blue_green_deployment_config` after provisioning.\n"]
    pub fn blue_green_deployment_config(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_deployment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_style` after provisioning.\n"]
    pub fn deployment_style(&self) -> terrars::ListRef<CodedeployDeploymentGroup_DeploymentStyleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_service` after provisioning.\n"]
    pub fn ecs_service(&self) -> terrars::ListRef<CodedeployDeploymentGroup_EcsServiceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_info` after provisioning.\n"]
    pub fn load_balancer_info(&self) -> terrars::ListRef<CodedeployDeploymentGroup_LoadBalancerInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_info", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupDynamic {
    alarm_configuration: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_AlarmConfiguration>>,
    auto_rollback_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_AutoRollbackConfiguration>>,
    blue_green_deployment_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_BlueGreenDeploymentConfig>>,
    deployment_style: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_DeploymentStyle>>,
    ec2_tag_filter: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_Ec2TagFilter>>,
    ec2_tag_set: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_Ec2TagSet>>,
    ecs_service: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_EcsService>>,
    load_balancer_info: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo>>,
    on_premises_instance_tag_filter: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_OnPremisesInstanceTagFilter>>,
    trigger_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_TriggerConfiguration>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_AlarmConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_poll_alarm_failure: std::option::Option<terrars::PrimField<bool>>,
}

impl CodedeployDeploymentGroup_AlarmConfiguration {
    #[doc = "Set the field `alarms`.\n"]
    pub fn set_alarms(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.alarms = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ignore_poll_alarm_failure`.\n"]
    pub fn set_ignore_poll_alarm_failure(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ignore_poll_alarm_failure = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_AlarmConfiguration {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_AlarmConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_AlarmConfiguration {}

impl BuildCodedeployDeploymentGroup_AlarmConfiguration {
    pub fn build(self) -> CodedeployDeploymentGroup_AlarmConfiguration {
        CodedeployDeploymentGroup_AlarmConfiguration {
            alarms: core::default::Default::default(),
            enabled: core::default::Default::default(),
            ignore_poll_alarm_failure: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_AlarmConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_AlarmConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_AlarmConfigurationRef {
        CodedeployDeploymentGroup_AlarmConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_AlarmConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alarms` after provisioning.\n"]
    pub fn alarms(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.alarms", self.base))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ignore_poll_alarm_failure` after provisioning.\n"]
    pub fn ignore_poll_alarm_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ignore_poll_alarm_failure", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_AutoRollbackConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CodedeployDeploymentGroup_AutoRollbackConfiguration {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `events`.\n"]
    pub fn set_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.events = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_AutoRollbackConfiguration {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_AutoRollbackConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_AutoRollbackConfiguration {}

impl BuildCodedeployDeploymentGroup_AutoRollbackConfiguration {
    pub fn build(self) -> CodedeployDeploymentGroup_AutoRollbackConfiguration {
        CodedeployDeploymentGroup_AutoRollbackConfiguration {
            enabled: core::default::Default::default(),
            events: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_AutoRollbackConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_AutoRollbackConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_AutoRollbackConfigurationRef {
        CodedeployDeploymentGroup_AutoRollbackConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_AutoRollbackConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `events` after provisioning.\n"]
    pub fn events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.events", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_on_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_time_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {
    #[doc = "Set the field `action_on_timeout`.\n"]
    pub fn set_action_on_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action_on_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `wait_time_in_minutes`.\n"]
    pub fn set_wait_time_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.wait_time_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {}

impl BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {
    pub fn build(self) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption {
            action_on_timeout: core::default::Default::default(),
            wait_time_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOptionRef {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_on_timeout` after provisioning.\n"]
    pub fn action_on_timeout(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_on_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `wait_time_in_minutes` after provisioning.\n"]
    pub fn wait_time_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_time_in_minutes", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {
    type O =
        terrars::BlockAssignable<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {}

impl BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {
    pub fn build(self) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption {
            action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOptionRef {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_wait_time_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `termination_wait_time_in_minutes`.\n"]
    pub fn set_termination_wait_time_in_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.termination_wait_time_in_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {
    type O =
        terrars
        ::BlockAssignable<
            CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {}

impl BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess {
            action: core::default::Default::default(),
            termination_wait_time_in_minutes: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccessRef {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `termination_wait_time_in_minutes` after provisioning.\n"]
    pub fn termination_wait_time_in_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_wait_time_in_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroup_BlueGreenDeploymentConfigDynamic {
    deployment_ready_option: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption>>,
    green_fleet_provisioning_option: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption>>,
    terminate_blue_instances_on_deployment_success: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess>,
    >,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_ready_option: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    green_fleet_provisioning_option: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_blue_instances_on_deployment_success: std
    ::option
    ::Option<
        std::vec::Vec<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess>,
    >,
    dynamic: CodedeployDeploymentGroup_BlueGreenDeploymentConfigDynamic,
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfig {
    #[doc = "Set the field `deployment_ready_option`.\n"]
    pub fn set_deployment_ready_option(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOption>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deployment_ready_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deployment_ready_option = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `green_fleet_provisioning_option`.\n"]
    pub fn set_green_fleet_provisioning_option(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOption,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.green_fleet_provisioning_option = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.green_fleet_provisioning_option = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `terminate_blue_instances_on_deployment_success`.\n"]
    pub fn set_terminate_blue_instances_on_deployment_success(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccess,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.terminate_blue_instances_on_deployment_success = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.terminate_blue_instances_on_deployment_success = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_BlueGreenDeploymentConfig {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_BlueGreenDeploymentConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig {}

impl BuildCodedeployDeploymentGroup_BlueGreenDeploymentConfig {
    pub fn build(self) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfig {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfig {
            deployment_ready_option: core::default::Default::default(),
            green_fleet_provisioning_option: core::default::Default::default(),
            terminate_blue_instances_on_deployment_success: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef {
        CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_BlueGreenDeploymentConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_ready_option` after provisioning.\n"]
    pub fn deployment_ready_option(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_DeploymentReadyOptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_ready_option", self.base))
    }

    #[doc = "Get a reference to the value of field `green_fleet_provisioning_option` after provisioning.\n"]
    pub fn green_fleet_provisioning_option(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_GreenFleetProvisioningOptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.green_fleet_provisioning_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminate_blue_instances_on_deployment_success` after provisioning.\n"]
    pub fn terminate_blue_instances_on_deployment_success(
        &self,
    ) -> terrars
    ::ListRef<CodedeployDeploymentGroup_BlueGreenDeploymentConfig_TerminateBlueInstancesOnDeploymentSuccessRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.terminate_blue_instances_on_deployment_success", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_DeploymentStyle {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_DeploymentStyle {
    #[doc = "Set the field `deployment_option`.\n"]
    pub fn set_deployment_option(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deployment_type`.\n"]
    pub fn set_deployment_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.deployment_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_DeploymentStyle {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_DeploymentStyle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_DeploymentStyle {}

impl BuildCodedeployDeploymentGroup_DeploymentStyle {
    pub fn build(self) -> CodedeployDeploymentGroup_DeploymentStyle {
        CodedeployDeploymentGroup_DeploymentStyle {
            deployment_option: core::default::Default::default(),
            deployment_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_DeploymentStyleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_DeploymentStyleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroup_DeploymentStyleRef {
        CodedeployDeploymentGroup_DeploymentStyleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_DeploymentStyleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_option` after provisioning.\n"]
    pub fn deployment_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_option", self.base))
    }

    #[doc = "Get a reference to the value of field `deployment_type` after provisioning.\n"]
    pub fn deployment_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deployment_type", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_Ec2TagFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_Ec2TagFilter {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_Ec2TagFilter {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_Ec2TagFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_Ec2TagFilter {}

impl BuildCodedeployDeploymentGroup_Ec2TagFilter {
    pub fn build(self) -> CodedeployDeploymentGroup_Ec2TagFilter {
        CodedeployDeploymentGroup_Ec2TagFilter {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_Ec2TagFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_Ec2TagFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroup_Ec2TagFilterRef {
        CodedeployDeploymentGroup_Ec2TagFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_Ec2TagFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroup_Ec2TagSetDynamic {
    ec2_tag_filter: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_Ec2TagFilter>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_Ec2TagSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_tag_filter: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_Ec2TagFilter>>,
    dynamic: CodedeployDeploymentGroup_Ec2TagSetDynamic,
}

impl CodedeployDeploymentGroup_Ec2TagSet {
    #[doc = "Set the field `ec2_tag_filter`.\n"]
    pub fn set_ec2_tag_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_Ec2TagFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_tag_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_tag_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_Ec2TagSet {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_Ec2TagSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_Ec2TagSet {}

impl BuildCodedeployDeploymentGroup_Ec2TagSet {
    pub fn build(self) -> CodedeployDeploymentGroup_Ec2TagSet {
        CodedeployDeploymentGroup_Ec2TagSet {
            ec2_tag_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_Ec2TagSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_Ec2TagSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroup_Ec2TagSetRef {
        CodedeployDeploymentGroup_Ec2TagSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_Ec2TagSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_EcsService {
    cluster_name: terrars::PrimField<std::string::String>,
    service_name: terrars::PrimField<std::string::String>,
}

impl CodedeployDeploymentGroup_EcsService { }

impl terrars::ToListMappable for CodedeployDeploymentGroup_EcsService {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_EcsService>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_EcsService {
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroup_EcsService {
    pub fn build(self) -> CodedeployDeploymentGroup_EcsService {
        CodedeployDeploymentGroup_EcsService {
            cluster_name: self.cluster_name,
            service_name: self.service_name,
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_EcsServiceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_EcsServiceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroup_EcsServiceRef {
        CodedeployDeploymentGroup_EcsServiceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_EcsServiceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_name` after provisioning.\n"]
    pub fn cluster_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo {}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo {
        CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfoRef {
        CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo {}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo { name: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfoRef {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute {
    listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute { }

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute {
    type O =
        terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute {
    #[doc = ""]
    pub listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute {
            listener_arns: self.listener_arns,
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRouteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRouteRef {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener_arns` after provisioning.\n"]
    pub fn listener_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.listener_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup {
    name: terrars::PrimField<std::string::String>,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup { }

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup { name: self.name }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroupRef {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute {
    listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute { }

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute {
    type O =
        terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute {
    #[doc = ""]
    pub listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute {
            listener_arns: self.listener_arns,
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRouteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRouteRef {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener_arns` after provisioning.\n"]
    pub fn listener_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.listener_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoDynamic {
    prod_traffic_route: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute>>,
    target_group: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup>>,
    test_traffic_route: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prod_traffic_route: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_traffic_route: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute>>,
    dynamic: CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoDynamic,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {
    #[doc = "Set the field `prod_traffic_route`.\n"]
    pub fn set_prod_traffic_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRoute,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.prod_traffic_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.prod_traffic_route = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_group`.\n"]
    pub fn set_target_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroup>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `test_traffic_route`.\n"]
    pub fn set_test_traffic_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRoute,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.test_traffic_route = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.test_traffic_route = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo {
            prod_traffic_route: core::default::Default::default(),
            target_group: core::default::Default::default(),
            test_traffic_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoRef {
        CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prod_traffic_route` after provisioning.\n"]
    pub fn prod_traffic_route(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_ProdTrafficRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prod_traffic_route", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group` after provisioning.\n"]
    pub fn target_group(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TargetGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `test_traffic_route` after provisioning.\n"]
    pub fn test_traffic_route(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo_TestTrafficRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_traffic_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroup_LoadBalancerInfoDynamic {
    elb_info: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo>>,
    target_group_info: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo>>,
    target_group_pair_info: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elb_info: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_info: std::option::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_pair_info: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo>>,
    dynamic: CodedeployDeploymentGroup_LoadBalancerInfoDynamic,
}

impl CodedeployDeploymentGroup_LoadBalancerInfo {
    #[doc = "Set the field `elb_info`.\n"]
    pub fn set_elb_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_ElbInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.elb_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.elb_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_group_info`.\n"]
    pub fn set_target_group_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_group_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group_info = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `target_group_pair_info`.\n"]
    pub fn set_target_group_pair_info(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfo>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_group_pair_info = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_group_pair_info = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_LoadBalancerInfo {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_LoadBalancerInfo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_LoadBalancerInfo {}

impl BuildCodedeployDeploymentGroup_LoadBalancerInfo {
    pub fn build(self) -> CodedeployDeploymentGroup_LoadBalancerInfo {
        CodedeployDeploymentGroup_LoadBalancerInfo {
            elb_info: core::default::Default::default(),
            target_group_info: core::default::Default::default(),
            target_group_pair_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_LoadBalancerInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_LoadBalancerInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroup_LoadBalancerInfoRef {
        CodedeployDeploymentGroup_LoadBalancerInfoRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_LoadBalancerInfoRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_group_pair_info` after provisioning.\n"]
    pub fn target_group_pair_info(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroup_LoadBalancerInfo_TargetGroupPairInfoRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_group_pair_info", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_OnPremisesInstanceTagFilter {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroup_OnPremisesInstanceTagFilter {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroup_OnPremisesInstanceTagFilter {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_OnPremisesInstanceTagFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_OnPremisesInstanceTagFilter {}

impl BuildCodedeployDeploymentGroup_OnPremisesInstanceTagFilter {
    pub fn build(self) -> CodedeployDeploymentGroup_OnPremisesInstanceTagFilter {
        CodedeployDeploymentGroup_OnPremisesInstanceTagFilter {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_OnPremisesInstanceTagFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_OnPremisesInstanceTagFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_OnPremisesInstanceTagFilterRef {
        CodedeployDeploymentGroup_OnPremisesInstanceTagFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_OnPremisesInstanceTagFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroup_TriggerConfiguration {
    trigger_events: terrars::SetField<terrars::PrimField<std::string::String>>,
    trigger_name: terrars::PrimField<std::string::String>,
    trigger_target_arn: terrars::PrimField<std::string::String>,
}

impl CodedeployDeploymentGroup_TriggerConfiguration { }

impl terrars::ToListMappable for CodedeployDeploymentGroup_TriggerConfiguration {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroup_TriggerConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroup_TriggerConfiguration {
    #[doc = ""]
    pub trigger_events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub trigger_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub trigger_target_arn: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroup_TriggerConfiguration {
    pub fn build(self) -> CodedeployDeploymentGroup_TriggerConfiguration {
        CodedeployDeploymentGroup_TriggerConfiguration {
            trigger_events: self.trigger_events,
            trigger_name: self.trigger_name,
            trigger_target_arn: self.trigger_target_arn,
        }
    }
}

#[allow(dead_code)]
pub struct CodedeployDeploymentGroup_TriggerConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroup_TriggerConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroup_TriggerConfigurationRef {
        CodedeployDeploymentGroup_TriggerConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroup_TriggerConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `trigger_events` after provisioning.\n"]
    pub fn trigger_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.trigger_events", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_name` after provisioning.\n"]
    pub fn trigger_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_name", self.base))
    }

    #[doc = "Get a reference to the value of field `trigger_target_arn` after provisioning.\n"]
    pub fn trigger_target_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.trigger_target_arn", self.base))
    }
}
