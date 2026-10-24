use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodedeployDeploymentGroupData {
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
    service_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarm_configuration: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupAlarmConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_rollback_configuration: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupAutoRollbackConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    blue_green_deployment_config: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_style: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupDeploymentStyleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_tag_filter: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupEc2TagFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_tag_set: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupEc2TagSetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_service: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupEcsServiceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_info: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_premises_instance_tag_filter: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    trigger_configuration: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupTriggerConfigurationEl>>,
    dynamic: CodedeployDeploymentGroupDynamic,
}

struct CodedeployDeploymentGroup_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodedeployDeploymentGroupData>,
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

    #[doc = "Set the field `alarm_configuration`.\n"]
    pub fn set_alarm_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupAlarmConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupAutoRollbackConfigurationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupDeploymentStyleEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupEc2TagFilterEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupEc2TagSetEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupEcsServiceEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupTriggerConfigurationEl>>,
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

    #[doc = "Get a reference to the value of field `alarm_configuration` after provisioning.\n"]
    pub fn alarm_configuration(&self) -> terrars::ListRef<CodedeployDeploymentGroupAlarmConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupAutoRollbackConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blue_green_deployment_config` after provisioning.\n"]
    pub fn blue_green_deployment_config(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_deployment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_style` after provisioning.\n"]
    pub fn deployment_style(&self) -> terrars::ListRef<CodedeployDeploymentGroupDeploymentStyleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_service` after provisioning.\n"]
    pub fn ecs_service(&self) -> terrars::ListRef<CodedeployDeploymentGroupEcsServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_info` after provisioning.\n"]
    pub fn load_balancer_info(&self) -> terrars::ListRef<CodedeployDeploymentGroupLoadBalancerInfoElRef> {
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
            data: std::cell::RefCell::new(CodedeployDeploymentGroupData {
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
                service_role_arn: self.service_role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `alarm_configuration` after provisioning.\n"]
    pub fn alarm_configuration(&self) -> terrars::ListRef<CodedeployDeploymentGroupAlarmConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alarm_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_rollback_configuration` after provisioning.\n"]
    pub fn auto_rollback_configuration(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupAutoRollbackConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_rollback_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `blue_green_deployment_config` after provisioning.\n"]
    pub fn blue_green_deployment_config(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.blue_green_deployment_config", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `deployment_style` after provisioning.\n"]
    pub fn deployment_style(&self) -> terrars::ListRef<CodedeployDeploymentGroupDeploymentStyleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_style", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_service` after provisioning.\n"]
    pub fn ecs_service(&self) -> terrars::ListRef<CodedeployDeploymentGroupEcsServiceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ecs_service", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `load_balancer_info` after provisioning.\n"]
    pub fn load_balancer_info(&self) -> terrars::ListRef<CodedeployDeploymentGroupLoadBalancerInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_info", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupAlarmConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alarms: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ignore_poll_alarm_failure: std::option::Option<terrars::PrimField<bool>>,
}

impl CodedeployDeploymentGroupAlarmConfigurationEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupAlarmConfigurationEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupAlarmConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupAlarmConfigurationEl {}

impl BuildCodedeployDeploymentGroupAlarmConfigurationEl {
    pub fn build(self) -> CodedeployDeploymentGroupAlarmConfigurationEl {
        CodedeployDeploymentGroupAlarmConfigurationEl {
            alarms: core::default::Default::default(),
            enabled: core::default::Default::default(),
            ignore_poll_alarm_failure: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupAlarmConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupAlarmConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupAlarmConfigurationElRef {
        CodedeployDeploymentGroupAlarmConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupAlarmConfigurationElRef {
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
pub struct CodedeployDeploymentGroupAutoRollbackConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CodedeployDeploymentGroupAutoRollbackConfigurationEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupAutoRollbackConfigurationEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupAutoRollbackConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupAutoRollbackConfigurationEl {}

impl BuildCodedeployDeploymentGroupAutoRollbackConfigurationEl {
    pub fn build(self) -> CodedeployDeploymentGroupAutoRollbackConfigurationEl {
        CodedeployDeploymentGroupAutoRollbackConfigurationEl {
            enabled: core::default::Default::default(),
            events: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
        CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupAutoRollbackConfigurationElRef {
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
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_on_timeout: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_time_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
    pub fn build(self) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl {
            action_on_timeout: core::default::Default::default(),
            wait_time_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef {
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
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    type O =
        terrars::BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
    pub fn build(self) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl {
            action: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_wait_time_in_minutes: std::option::Option<terrars::PrimField<f64>>,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
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
::ToListMappable for CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
    type O =
        terrars
        ::BlockAssignable<
            CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
    pub fn build(
        self,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl {
            action: core::default::Default::default(),
            termination_wait_time_in_minutes: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef {
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
struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElDynamic {
    deployment_ready_option: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl>>,
    green_fleet_provisioning_option: std
    ::option
    ::Option<
        terrars::DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl>,
    >,
    terminate_blue_instances_on_deployment_success: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl,
        >,
    >,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_ready_option: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    green_fleet_provisioning_option: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    terminate_blue_instances_on_deployment_success: std
    ::option
    ::Option<
        std::vec::Vec<CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl>,
    >,
    dynamic: CodedeployDeploymentGroupBlueGreenDeploymentConfigElDynamic,
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    #[doc = "Set the field `deployment_ready_option`.\n"]
    pub fn set_deployment_ready_option(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionEl>,
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
                            CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionEl,
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
                            CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessEl,
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

impl terrars::ToListMappable for CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigEl {}

impl BuildCodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
    pub fn build(self) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigEl {
            deployment_ready_option: core::default::Default::default(),
            green_fleet_provisioning_option: core::default::Default::default(),
            terminate_blue_instances_on_deployment_success: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
        CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupBlueGreenDeploymentConfigElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deployment_ready_option` after provisioning.\n"]
    pub fn deployment_ready_option(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElDeploymentReadyOptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deployment_ready_option", self.base))
    }

    #[doc = "Get a reference to the value of field `green_fleet_provisioning_option` after provisioning.\n"]
    pub fn green_fleet_provisioning_option(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElGreenFleetProvisioningOptionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.green_fleet_provisioning_option", self.base))
    }

    #[doc =
        "Get a reference to the value of field `terminate_blue_instances_on_deployment_success` after provisioning.\n"]
    pub fn terminate_blue_instances_on_deployment_success(
        &self,
    ) -> terrars
    ::ListRef<CodedeployDeploymentGroupBlueGreenDeploymentConfigElTerminateBlueInstancesOnDeploymentSuccessElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.terminate_blue_instances_on_deployment_success", self.base),
        )
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupDeploymentStyleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deployment_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupDeploymentStyleEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupDeploymentStyleEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupDeploymentStyleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupDeploymentStyleEl {}

impl BuildCodedeployDeploymentGroupDeploymentStyleEl {
    pub fn build(self) -> CodedeployDeploymentGroupDeploymentStyleEl {
        CodedeployDeploymentGroupDeploymentStyleEl {
            deployment_option: core::default::Default::default(),
            deployment_type: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupDeploymentStyleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupDeploymentStyleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroupDeploymentStyleElRef {
        CodedeployDeploymentGroupDeploymentStyleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupDeploymentStyleElRef {
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
pub struct CodedeployDeploymentGroupEc2TagFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupEc2TagFilterEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupEc2TagFilterEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupEc2TagFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEc2TagFilterEl {}

impl BuildCodedeployDeploymentGroupEc2TagFilterEl {
    pub fn build(self) -> CodedeployDeploymentGroupEc2TagFilterEl {
        CodedeployDeploymentGroupEc2TagFilterEl {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupEc2TagFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupEc2TagFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroupEc2TagFilterElRef {
        CodedeployDeploymentGroupEc2TagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEc2TagFilterElRef {
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
pub struct CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {}

impl BuildCodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
    pub fn build(self) -> CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
        CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
        CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEc2TagSetElEc2TagFilterElRef {
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
struct CodedeployDeploymentGroupEc2TagSetElDynamic {
    ec2_tag_filter: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupEc2TagSetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_tag_filter: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>>,
    dynamic: CodedeployDeploymentGroupEc2TagSetElDynamic,
}

impl CodedeployDeploymentGroupEc2TagSetEl {
    #[doc = "Set the field `ec2_tag_filter`.\n"]
    pub fn set_ec2_tag_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupEc2TagSetElEc2TagFilterEl>>,
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

impl terrars::ToListMappable for CodedeployDeploymentGroupEc2TagSetEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupEc2TagSetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEc2TagSetEl {}

impl BuildCodedeployDeploymentGroupEc2TagSetEl {
    pub fn build(self) -> CodedeployDeploymentGroupEc2TagSetEl {
        CodedeployDeploymentGroupEc2TagSetEl {
            ec2_tag_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupEc2TagSetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupEc2TagSetElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroupEc2TagSetElRef {
        CodedeployDeploymentGroupEc2TagSetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEc2TagSetElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupEcsServiceEl {
    cluster_name: terrars::PrimField<std::string::String>,
    service_name: terrars::PrimField<std::string::String>,
}

impl CodedeployDeploymentGroupEcsServiceEl { }

impl terrars::ToListMappable for CodedeployDeploymentGroupEcsServiceEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupEcsServiceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupEcsServiceEl {
    #[doc = ""]
    pub cluster_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_name: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroupEcsServiceEl {
    pub fn build(self) -> CodedeployDeploymentGroupEcsServiceEl {
        CodedeployDeploymentGroupEcsServiceEl {
            cluster_name: self.cluster_name,
            service_name: self.service_name,
        }
    }
}

pub struct CodedeployDeploymentGroupEcsServiceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupEcsServiceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodedeployDeploymentGroupEcsServiceElRef {
        CodedeployDeploymentGroupEcsServiceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupEcsServiceElRef {
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
pub struct CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl { name: core::default::Default::default() }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElElbInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl { name: core::default::Default::default() }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl { }

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    type O =
        terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    #[doc = ""]
    pub listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl {
            listener_arns: self.listener_arns,
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener_arns` after provisioning.\n"]
    pub fn listener_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.listener_arns", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    name: terrars::PrimField<std::string::String>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl { }

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl { name: self.name }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl { }

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    type O =
        terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    #[doc = ""]
    pub listener_arns: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl {
            listener_arns: self.listener_arns,
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `listener_arns` after provisioning.\n"]
    pub fn listener_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.listener_arns", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElDynamic {
    prod_traffic_route: std
    ::option
    ::Option<
        terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl>,
    >,
    target_group: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl>>,
    test_traffic_route: std
    ::option
    ::Option<
        terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl>,
    >,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prod_traffic_route: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    test_traffic_route: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl>>,
    dynamic: CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElDynamic,
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
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
                            CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteEl,
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
                        ::BlockAssignable<
                            CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupEl,
                        >,
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
                            CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteEl,
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

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl {
            prod_traffic_route: core::default::Default::default(),
            target_group: core::default::Default::default(),
            test_traffic_route: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `prod_traffic_route` after provisioning.\n"]
    pub fn prod_traffic_route(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElProdTrafficRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prod_traffic_route", self.base))
    }

    #[doc = "Get a reference to the value of field `target_group` after provisioning.\n"]
    pub fn target_group(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTargetGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `test_traffic_route` after provisioning.\n"]
    pub fn test_traffic_route(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElTestTrafficRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.test_traffic_route", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupLoadBalancerInfoElDynamic {
    elb_info: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>>,
    target_group_info: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>>,
    target_group_pair_info: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>>,
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupLoadBalancerInfoEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elb_info: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_info: std::option::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_group_pair_info: std
    ::option
    ::Option<std::vec::Vec<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>>,
    dynamic: CodedeployDeploymentGroupLoadBalancerInfoElDynamic,
}

impl CodedeployDeploymentGroupLoadBalancerInfoEl {
    #[doc = "Set the field `elb_info`.\n"]
    pub fn set_elb_info(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElElbInfoEl>>,
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
                    ::Into<terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupInfoEl>>,
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
                    ::Into<
                        terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoEl>,
                    >,
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

impl terrars::ToListMappable for CodedeployDeploymentGroupLoadBalancerInfoEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupLoadBalancerInfoEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupLoadBalancerInfoEl {}

impl BuildCodedeployDeploymentGroupLoadBalancerInfoEl {
    pub fn build(self) -> CodedeployDeploymentGroupLoadBalancerInfoEl {
        CodedeployDeploymentGroupLoadBalancerInfoEl {
            elb_info: core::default::Default::default(),
            target_group_info: core::default::Default::default(),
            target_group_pair_info: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupLoadBalancerInfoElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupLoadBalancerInfoElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupLoadBalancerInfoElRef {
        CodedeployDeploymentGroupLoadBalancerInfoElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupLoadBalancerInfoElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_group_pair_info` after provisioning.\n"]
    pub fn target_group_pair_info(
        &self,
    ) -> terrars::ListRef<CodedeployDeploymentGroupLoadBalancerInfoElTargetGroupPairInfoElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_group_pair_info", self.base))
    }
}

#[derive(Serialize)]
pub struct CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
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

impl terrars::ToListMappable for CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {}

impl BuildCodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
    pub fn build(self) -> CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
        CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl {
            key: core::default::Default::default(),
            type_: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

pub struct CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
        CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupOnPremisesInstanceTagFilterElRef {
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
pub struct CodedeployDeploymentGroupTriggerConfigurationEl {
    trigger_events: terrars::SetField<terrars::PrimField<std::string::String>>,
    trigger_name: terrars::PrimField<std::string::String>,
    trigger_target_arn: terrars::PrimField<std::string::String>,
}

impl CodedeployDeploymentGroupTriggerConfigurationEl { }

impl terrars::ToListMappable for CodedeployDeploymentGroupTriggerConfigurationEl {
    type O = terrars::BlockAssignable<CodedeployDeploymentGroupTriggerConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodedeployDeploymentGroupTriggerConfigurationEl {
    #[doc = ""]
    pub trigger_events: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub trigger_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub trigger_target_arn: terrars::PrimField<std::string::String>,
}

impl BuildCodedeployDeploymentGroupTriggerConfigurationEl {
    pub fn build(self) -> CodedeployDeploymentGroupTriggerConfigurationEl {
        CodedeployDeploymentGroupTriggerConfigurationEl {
            trigger_events: self.trigger_events,
            trigger_name: self.trigger_name,
            trigger_target_arn: self.trigger_target_arn,
        }
    }
}

pub struct CodedeployDeploymentGroupTriggerConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodedeployDeploymentGroupTriggerConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CodedeployDeploymentGroupTriggerConfigurationElRef {
        CodedeployDeploymentGroupTriggerConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodedeployDeploymentGroupTriggerConfigurationElRef {
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

#[derive(Serialize, Default)]
struct CodedeployDeploymentGroupDynamic {
    alarm_configuration: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupAlarmConfigurationEl>>,
    auto_rollback_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupAutoRollbackConfigurationEl>>,
    blue_green_deployment_config: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupBlueGreenDeploymentConfigEl>>,
    deployment_style: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupDeploymentStyleEl>>,
    ec2_tag_filter: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupEc2TagFilterEl>>,
    ec2_tag_set: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupEc2TagSetEl>>,
    ecs_service: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupEcsServiceEl>>,
    load_balancer_info: std::option::Option<terrars::DynamicBlock<CodedeployDeploymentGroupLoadBalancerInfoEl>>,
    on_premises_instance_tag_filter: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupOnPremisesInstanceTagFilterEl>>,
    trigger_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<CodedeployDeploymentGroupTriggerConfigurationEl>>,
}
