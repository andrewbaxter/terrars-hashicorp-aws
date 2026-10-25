#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrClusterData_ {
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
    additional_info: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    applications: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configurations: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configurations_json: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_ami_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_root_volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keep_job_flow_alive_when_no_steps: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    list_steps_states: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_encryption_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_uri: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os_release_label: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_group_config: std::option::Option<terrars::ListField<EmrCluster_PlacementGroupConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    release_label: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scale_down_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_configuration: std::option::Option<terrars::PrimField<std::string::String>>,
    service_role: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step: std::option::Option<terrars::ListField<EmrCluster_Step>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    step_concurrency_level: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    termination_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unhealthy_node_replacement: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    visible_to_all_users: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_termination_policy: std::option::Option<std::vec::Vec<EmrCluster_AutoTerminationPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bootstrap_action: std::option::Option<std::vec::Vec<EmrCluster_BootstrapAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_instance_fleet: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_instance_group: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_attributes: std::option::Option<std::vec::Vec<EmrCluster_Ec2Attributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kerberos_attributes: std::option::Option<std::vec::Vec<EmrCluster_KerberosAttributes>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_instance_fleet: std::option::Option<std::vec::Vec<EmrCluster_MasterInstanceFleet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    master_instance_group: std::option::Option<std::vec::Vec<EmrCluster_MasterInstanceGroup>>,
    dynamic: EmrClusterDynamic,
}

struct EmrCluster_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrClusterData_>,
}

#[derive(Clone)]
pub struct EmrCluster(std::rc::Rc<EmrCluster_>);

impl EmrCluster {
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

    #[doc = "Set the field `additional_info`.\n"]
    pub fn set_additional_info(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().additional_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `applications`.\n"]
    pub fn set_applications(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().applications = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `autoscaling_role`.\n"]
    pub fn set_autoscaling_role(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().autoscaling_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().configurations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configurations_json`.\n"]
    pub fn set_configurations_json(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().configurations_json = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_ami_id`.\n"]
    pub fn set_custom_ami_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().custom_ami_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_root_volume_size`.\n"]
    pub fn set_ebs_root_volume_size(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ebs_root_volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keep_job_flow_alive_when_no_steps`.\n"]
    pub fn set_keep_job_flow_alive_when_no_steps(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().keep_job_flow_alive_when_no_steps = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `list_steps_states`.\n"]
    pub fn set_list_steps_states(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().list_steps_states = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_encryption_kms_key_id`.\n"]
    pub fn set_log_encryption_kms_key_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().log_encryption_kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_uri`.\n"]
    pub fn set_log_uri(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().log_uri = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os_release_label`.\n"]
    pub fn set_os_release_label(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().os_release_label = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group_config`.\n"]
    pub fn set_placement_group_config(
        self,
        v: impl std::convert::Into<terrars::ListField<EmrCluster_PlacementGroupConfig>>,
    ) -> Self {
        self.0.data.borrow_mut().placement_group_config = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scale_down_behavior`.\n"]
    pub fn set_scale_down_behavior(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().scale_down_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_configuration`.\n"]
    pub fn set_security_configuration(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().security_configuration = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step`.\n"]
    pub fn set_step(self, v: impl std::convert::Into<terrars::ListField<EmrCluster_Step>>) -> Self {
        self.0.data.borrow_mut().step = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `step_concurrency_level`.\n"]
    pub fn set_step_concurrency_level(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().step_concurrency_level = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `termination_protection`.\n"]
    pub fn set_termination_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().termination_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unhealthy_node_replacement`.\n"]
    pub fn set_unhealthy_node_replacement(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().unhealthy_node_replacement = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `visible_to_all_users`.\n"]
    pub fn set_visible_to_all_users(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().visible_to_all_users = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_termination_policy`.\n"]
    pub fn set_auto_termination_policy(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_AutoTerminationPolicy>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().auto_termination_policy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.auto_termination_policy = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bootstrap_action`.\n"]
    pub fn set_bootstrap_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_BootstrapAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bootstrap_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bootstrap_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_instance_fleet`.\n"]
    pub fn set_core_instance_fleet(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().core_instance_fleet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.core_instance_fleet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `core_instance_group`.\n"]
    pub fn set_core_instance_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().core_instance_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.core_instance_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_attributes`.\n"]
    pub fn set_ec2_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_Ec2Attributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ec2_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ec2_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `kerberos_attributes`.\n"]
    pub fn set_kerberos_attributes(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_KerberosAttributes>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().kerberos_attributes = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.kerberos_attributes = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_instance_fleet`.\n"]
    pub fn set_master_instance_fleet(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_MasterInstanceFleet>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().master_instance_fleet = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.master_instance_fleet = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `master_instance_group`.\n"]
    pub fn set_master_instance_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_MasterInstanceGroup>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().master_instance_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.master_instance_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `additional_info` after provisioning.\n"]
    pub fn additional_info(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_role` after provisioning.\n"]
    pub fn autoscaling_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations` after provisioning.\n"]
    pub fn configurations(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations_json` after provisioning.\n"]
    pub fn configurations_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configurations_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_ami_id` after provisioning.\n"]
    pub fn custom_ami_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_ami_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_root_volume_size` after provisioning.\n"]
    pub fn ebs_root_volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_root_volume_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keep_job_flow_alive_when_no_steps` after provisioning.\n"]
    pub fn keep_job_flow_alive_when_no_steps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.keep_job_flow_alive_when_no_steps", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `list_steps_states` after provisioning.\n"]
    pub fn list_steps_states(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.list_steps_states", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_encryption_kms_key_id` after provisioning.\n"]
    pub fn log_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_encryption_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_public_dns` after provisioning.\n"]
    pub fn master_public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_release_label` after provisioning.\n"]
    pub fn os_release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_config` after provisioning.\n"]
    pub fn placement_group_config(&self) -> terrars::ListRef<EmrCluster_PlacementGroupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_group_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale_down_behavior` after provisioning.\n"]
    pub fn scale_down_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_down_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<EmrCluster_StepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step_concurrency_level` after provisioning.\n"]
    pub fn step_concurrency_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_concurrency_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection` after provisioning.\n"]
    pub fn termination_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `unhealthy_node_replacement` after provisioning.\n"]
    pub fn unhealthy_node_replacement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_node_replacement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visible_to_all_users` after provisioning.\n"]
    pub fn visible_to_all_users(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible_to_all_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_termination_policy` after provisioning.\n"]
    pub fn auto_termination_policy(&self) -> terrars::ListRef<EmrCluster_AutoTerminationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_termination_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_action` after provisioning.\n"]
    pub fn bootstrap_action(&self) -> terrars::ListRef<EmrCluster_BootstrapActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bootstrap_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_fleet` after provisioning.\n"]
    pub fn core_instance_fleet(&self) -> terrars::ListRef<EmrCluster_CoreInstanceFleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_group` after provisioning.\n"]
    pub fn core_instance_group(&self) -> terrars::ListRef<EmrCluster_CoreInstanceGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_instance_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_attributes` after provisioning.\n"]
    pub fn ec2_attributes(&self) -> terrars::ListRef<EmrCluster_Ec2AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_attributes` after provisioning.\n"]
    pub fn kerberos_attributes(&self) -> terrars::ListRef<EmrCluster_KerberosAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_fleet` after provisioning.\n"]
    pub fn master_instance_fleet(&self) -> terrars::ListRef<EmrCluster_MasterInstanceFleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_group` after provisioning.\n"]
    pub fn master_instance_group(&self) -> terrars::ListRef<EmrCluster_MasterInstanceGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_instance_group", self.extract_ref()))
    }
}

impl terrars::Referable for EmrCluster {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrCluster { }

impl terrars::ToListMappable for EmrCluster {
    type O = terrars::ListRef<EmrClusterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrCluster_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emr_cluster".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrCluster {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub release_label: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub service_role: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrCluster {
        let out = EmrCluster(std::rc::Rc::new(EmrCluster_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrClusterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                additional_info: core::default::Default::default(),
                applications: core::default::Default::default(),
                autoscaling_role: core::default::Default::default(),
                configurations: core::default::Default::default(),
                configurations_json: core::default::Default::default(),
                custom_ami_id: core::default::Default::default(),
                ebs_root_volume_size: core::default::Default::default(),
                id: core::default::Default::default(),
                keep_job_flow_alive_when_no_steps: core::default::Default::default(),
                list_steps_states: core::default::Default::default(),
                log_encryption_kms_key_id: core::default::Default::default(),
                log_uri: core::default::Default::default(),
                name: self.name,
                os_release_label: core::default::Default::default(),
                placement_group_config: core::default::Default::default(),
                region: core::default::Default::default(),
                release_label: self.release_label,
                scale_down_behavior: core::default::Default::default(),
                security_configuration: core::default::Default::default(),
                service_role: self.service_role,
                step: core::default::Default::default(),
                step_concurrency_level: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                termination_protection: core::default::Default::default(),
                unhealthy_node_replacement: core::default::Default::default(),
                visible_to_all_users: core::default::Default::default(),
                auto_termination_policy: core::default::Default::default(),
                bootstrap_action: core::default::Default::default(),
                core_instance_fleet: core::default::Default::default(),
                core_instance_group: core::default::Default::default(),
                ec2_attributes: core::default::Default::default(),
                kerberos_attributes: core::default::Default::default(),
                master_instance_fleet: core::default::Default::default(),
                master_instance_group: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrClusterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrClusterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrClusterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_info` after provisioning.\n"]
    pub fn additional_info(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `applications` after provisioning.\n"]
    pub fn applications(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.applications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `autoscaling_role` after provisioning.\n"]
    pub fn autoscaling_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cluster_state` after provisioning.\n"]
    pub fn cluster_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations` after provisioning.\n"]
    pub fn configurations(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configurations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `configurations_json` after provisioning.\n"]
    pub fn configurations_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configurations_json", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `custom_ami_id` after provisioning.\n"]
    pub fn custom_ami_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.custom_ami_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_root_volume_size` after provisioning.\n"]
    pub fn ebs_root_volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_root_volume_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `keep_job_flow_alive_when_no_steps` after provisioning.\n"]
    pub fn keep_job_flow_alive_when_no_steps(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.keep_job_flow_alive_when_no_steps", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `list_steps_states` after provisioning.\n"]
    pub fn list_steps_states(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.list_steps_states", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_encryption_kms_key_id` after provisioning.\n"]
    pub fn log_encryption_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_encryption_kms_key_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `log_uri` after provisioning.\n"]
    pub fn log_uri(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_uri", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_public_dns` after provisioning.\n"]
    pub fn master_public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.master_public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os_release_label` after provisioning.\n"]
    pub fn os_release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os_release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group_config` after provisioning.\n"]
    pub fn placement_group_config(&self) -> terrars::ListRef<EmrCluster_PlacementGroupConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement_group_config", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_label` after provisioning.\n"]
    pub fn release_label(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_label", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scale_down_behavior` after provisioning.\n"]
    pub fn scale_down_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scale_down_behavior", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_configuration` after provisioning.\n"]
    pub fn security_configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `service_role` after provisioning.\n"]
    pub fn service_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_role", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step` after provisioning.\n"]
    pub fn step(&self) -> terrars::ListRef<EmrCluster_StepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.step", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `step_concurrency_level` after provisioning.\n"]
    pub fn step_concurrency_level(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.step_concurrency_level", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `termination_protection` after provisioning.\n"]
    pub fn termination_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.termination_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `unhealthy_node_replacement` after provisioning.\n"]
    pub fn unhealthy_node_replacement(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unhealthy_node_replacement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `visible_to_all_users` after provisioning.\n"]
    pub fn visible_to_all_users(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.visible_to_all_users", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_termination_policy` after provisioning.\n"]
    pub fn auto_termination_policy(&self) -> terrars::ListRef<EmrCluster_AutoTerminationPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_termination_policy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bootstrap_action` after provisioning.\n"]
    pub fn bootstrap_action(&self) -> terrars::ListRef<EmrCluster_BootstrapActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bootstrap_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_fleet` after provisioning.\n"]
    pub fn core_instance_fleet(&self) -> terrars::ListRef<EmrCluster_CoreInstanceFleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_instance_group` after provisioning.\n"]
    pub fn core_instance_group(&self) -> terrars::ListRef<EmrCluster_CoreInstanceGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.core_instance_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_attributes` after provisioning.\n"]
    pub fn ec2_attributes(&self) -> terrars::ListRef<EmrCluster_Ec2AttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ec2_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kerberos_attributes` after provisioning.\n"]
    pub fn kerberos_attributes(&self) -> terrars::ListRef<EmrCluster_KerberosAttributesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.kerberos_attributes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_fleet` after provisioning.\n"]
    pub fn master_instance_fleet(&self) -> terrars::ListRef<EmrCluster_MasterInstanceFleetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_instance_fleet", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `master_instance_group` after provisioning.\n"]
    pub fn master_instance_group(&self) -> terrars::ListRef<EmrCluster_MasterInstanceGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.master_instance_group", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EmrClusterDynamic {
    auto_termination_policy: std::option::Option<terrars::DynamicBlock<EmrCluster_AutoTerminationPolicy>>,
    bootstrap_action: std::option::Option<terrars::DynamicBlock<EmrCluster_BootstrapAction>>,
    core_instance_fleet: std::option::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet>>,
    core_instance_group: std::option::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceGroup>>,
    ec2_attributes: std::option::Option<terrars::DynamicBlock<EmrCluster_Ec2Attributes>>,
    kerberos_attributes: std::option::Option<terrars::DynamicBlock<EmrCluster_KerberosAttributes>>,
    master_instance_fleet: std::option::Option<terrars::DynamicBlock<EmrCluster_MasterInstanceFleet>>,
    master_instance_group: std::option::Option<terrars::DynamicBlock<EmrCluster_MasterInstanceGroup>>,
}

#[derive(Serialize)]
pub struct EmrCluster_PlacementGroupConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_role: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrCluster_PlacementGroupConfig {
    #[doc = "Set the field `instance_role`.\n"]
    pub fn set_instance_role(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_role = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_strategy`.\n"]
    pub fn set_placement_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.placement_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_PlacementGroupConfig {
    type O = terrars::BlockAssignable<EmrCluster_PlacementGroupConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_PlacementGroupConfig {}

impl BuildEmrCluster_PlacementGroupConfig {
    pub fn build(self) -> EmrCluster_PlacementGroupConfig {
        EmrCluster_PlacementGroupConfig {
            instance_role: core::default::Default::default(),
            placement_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_PlacementGroupConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_PlacementGroupConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_PlacementGroupConfigRef {
        EmrCluster_PlacementGroupConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_PlacementGroupConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_role` after provisioning.\n"]
    pub fn instance_role(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_role", self.base))
    }

    #[doc = "Get a reference to the value of field `placement_strategy` after provisioning.\n"]
    pub fn placement_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_Step_HadoopJarStep {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    jar: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    main_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EmrCluster_Step_HadoopJarStep {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `jar`.\n"]
    pub fn set_jar(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.jar = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `main_class`.\n"]
    pub fn set_main_class(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.main_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_Step_HadoopJarStep {
    type O = terrars::BlockAssignable<EmrCluster_Step_HadoopJarStep>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_Step_HadoopJarStep {}

impl BuildEmrCluster_Step_HadoopJarStep {
    pub fn build(self) -> EmrCluster_Step_HadoopJarStep {
        EmrCluster_Step_HadoopJarStep {
            args: core::default::Default::default(),
            jar: core::default::Default::default(),
            main_class: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_Step_HadoopJarStepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_Step_HadoopJarStepRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_Step_HadoopJarStepRef {
        EmrCluster_Step_HadoopJarStepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_Step_HadoopJarStepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `jar` after provisioning.\n"]
    pub fn jar(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.jar", self.base))
    }

    #[doc = "Get a reference to the value of field `main_class` after provisioning.\n"]
    pub fn main_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_class", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_Step {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_on_failure: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hadoop_jar_step: std::option::Option<terrars::ListField<EmrCluster_Step_HadoopJarStep>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl EmrCluster_Step {
    #[doc = "Set the field `action_on_failure`.\n"]
    pub fn set_action_on_failure(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action_on_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hadoop_jar_step`.\n"]
    pub fn set_hadoop_jar_step(
        mut self,
        v: impl std::convert::Into<terrars::ListField<EmrCluster_Step_HadoopJarStep>>,
    ) -> Self {
        self.hadoop_jar_step = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_Step {
    type O = terrars::BlockAssignable<EmrCluster_Step>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_Step {}

impl BuildEmrCluster_Step {
    pub fn build(self) -> EmrCluster_Step {
        EmrCluster_Step {
            action_on_failure: core::default::Default::default(),
            hadoop_jar_step: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_StepRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_StepRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_StepRef {
        EmrCluster_StepRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_StepRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_on_failure` after provisioning.\n"]
    pub fn action_on_failure(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_on_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `hadoop_jar_step` after provisioning.\n"]
    pub fn hadoop_jar_step(&self) -> terrars::ListRef<EmrCluster_Step_HadoopJarStepRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hadoop_jar_step", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_AutoTerminationPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle_timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrCluster_AutoTerminationPolicy {
    #[doc = "Set the field `idle_timeout`.\n"]
    pub fn set_idle_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.idle_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_AutoTerminationPolicy {
    type O = terrars::BlockAssignable<EmrCluster_AutoTerminationPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_AutoTerminationPolicy {}

impl BuildEmrCluster_AutoTerminationPolicy {
    pub fn build(self) -> EmrCluster_AutoTerminationPolicy {
        EmrCluster_AutoTerminationPolicy { idle_timeout: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_AutoTerminationPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_AutoTerminationPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_AutoTerminationPolicyRef {
        EmrCluster_AutoTerminationPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_AutoTerminationPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle_timeout` after provisioning.\n"]
    pub fn idle_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.idle_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_BootstrapAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    name: terrars::PrimField<std::string::String>,
    path: terrars::PrimField<std::string::String>,
}

impl EmrCluster_BootstrapAction {
    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.args = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_BootstrapAction {
    type O = terrars::BlockAssignable<EmrCluster_BootstrapAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_BootstrapAction {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub path: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_BootstrapAction {
    pub fn build(self) -> EmrCluster_BootstrapAction {
        EmrCluster_BootstrapAction {
            args: core::default::Default::default(),
            name: self.name,
            path: self.path,
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_BootstrapActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_BootstrapActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_BootstrapActionRef {
        EmrCluster_BootstrapActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_BootstrapActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {
    #[doc = "Set the field `classification`.\n"]
    pub fn set_classification(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.classification = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `properties`.\n"]
    pub fn set_properties(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.properties = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {}

impl BuildEmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {
        EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
        EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `classification` after provisioning.\n"]
    pub fn classification(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.classification", self.base))
    }

    #[doc = "Get a reference to the value of field `properties` after provisioning.\n"]
    pub fn properties(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.properties", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    size: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes_per_instance: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volumes_per_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
        EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig {
            iops: core::default::Default::default(),
            size: self.size,
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
        EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigsDynamic {
    configurations: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations>>,
    ebs_config: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig>>,
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_price_as_percentage_of_on_demand_price: std::option::Option<terrars::PrimField<f64>>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configurations: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_config: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig>>,
    dynamic: EmrCluster_CoreInstanceFleet_InstanceTypeConfigsDynamic,
}

impl EmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bid_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bid_price_as_percentage_of_on_demand_price`.\n"]
    pub fn set_bid_price_as_percentage_of_on_demand_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.bid_price_as_percentage_of_on_demand_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weighted_capacity`.\n"]
    pub fn set_weighted_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weighted_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configurations`.\n"]
    pub fn set_configurations(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_Configurations>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.configurations = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.configurations = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs_EbsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
        EmrCluster_CoreInstanceFleet_InstanceTypeConfigs {
            bid_price: core::default::Default::default(),
            bid_price_as_percentage_of_on_demand_price: core::default::Default::default(),
            instance_type: self.instance_type,
            weighted_capacity: core::default::Default::default(),
            configurations: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleet_InstanceTypeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleet_InstanceTypeConfigsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrCluster_CoreInstanceFleet_InstanceTypeConfigsRef {
        EmrCluster_CoreInstanceFleet_InstanceTypeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleet_InstanceTypeConfigsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_price_as_percentage_of_on_demand_price` after provisioning.\n"]
    pub fn bid_price_as_percentage_of_on_demand_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.bid_price_as_percentage_of_on_demand_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `weighted_capacity` after provisioning.\n"]
    pub fn weighted_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weighted_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    allocation_strategy: terrars::PrimField<std::string::String>,
}

impl EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification { }

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    #[doc = ""]
    pub allocation_strategy: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification {
        EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification {
            allocation_strategy: self.allocation_strategy,
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
        EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
    allocation_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_duration_minutes: std::option::Option<terrars::PrimField<f64>>,
    timeout_action: terrars::PrimField<std::string::String>,
    timeout_duration_minutes: terrars::PrimField<f64>,
}

impl EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.block_duration_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
    #[doc = ""]
    pub allocation_strategy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub timeout_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub timeout_duration_minutes: terrars::PrimField<f64>,
}

impl BuildEmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
        EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification {
            allocation_strategy: self.allocation_strategy,
            block_duration_minutes: core::default::Default::default(),
            timeout_action: self.timeout_action,
            timeout_duration_minutes: self.timeout_duration_minutes,
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
        EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_strategy` after provisioning.\n"]
    pub fn allocation_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_action` after provisioning.\n"]
    pub fn timeout_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_action", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout_duration_minutes` after provisioning.\n"]
    pub fn timeout_duration_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.timeout_duration_minutes", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrCluster_CoreInstanceFleet_LaunchSpecificationsDynamic {
    on_demand_specification: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification>>,
    spot_specification: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification>>,
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet_LaunchSpecifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_specification: std
    ::option
    ::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_specification: std
    ::option
    ::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification>>,
    dynamic: EmrCluster_CoreInstanceFleet_LaunchSpecificationsDynamic,
}

impl EmrCluster_CoreInstanceFleet_LaunchSpecifications {
    #[doc = "Set the field `on_demand_specification`.\n"]
    pub fn set_on_demand_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.on_demand_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.on_demand_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `spot_specification`.\n"]
    pub fn set_spot_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spot_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spot_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet_LaunchSpecifications {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet_LaunchSpecifications {}

impl BuildEmrCluster_CoreInstanceFleet_LaunchSpecifications {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet_LaunchSpecifications {
        EmrCluster_CoreInstanceFleet_LaunchSpecifications {
            on_demand_specification: core::default::Default::default(),
            spot_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef {
        EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_specification` after provisioning.\n"]
    pub fn on_demand_specification(
        &self,
    ) -> terrars::ListRef<EmrCluster_CoreInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_specification` after provisioning.\n"]
    pub fn spot_specification(
        &self,
    ) -> terrars::ListRef<EmrCluster_CoreInstanceFleet_LaunchSpecifications_SpotSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrCluster_CoreInstanceFleetDynamic {
    instance_type_configs: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>>,
    launch_specifications: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_LaunchSpecifications>>,
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceFleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_on_demand_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_spot_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type_configs: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_specifications: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_LaunchSpecifications>>,
    dynamic: EmrCluster_CoreInstanceFleetDynamic,
}

impl EmrCluster_CoreInstanceFleet {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_on_demand_capacity`.\n"]
    pub fn set_target_on_demand_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_on_demand_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_spot_capacity`.\n"]
    pub fn set_target_spot_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_spot_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type_configs`.\n"]
    pub fn set_instance_type_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_type_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_type_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_specifications`.\n"]
    pub fn set_launch_specifications(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_specifications = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceFleet {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceFleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceFleet {}

impl BuildEmrCluster_CoreInstanceFleet {
    pub fn build(self) -> EmrCluster_CoreInstanceFleet {
        EmrCluster_CoreInstanceFleet {
            name: core::default::Default::default(),
            target_on_demand_capacity: core::default::Default::default(),
            target_spot_capacity: core::default::Default::default(),
            instance_type_configs: core::default::Default::default(),
            launch_specifications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_CoreInstanceFleetRef {
        EmrCluster_CoreInstanceFleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceFleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> terrars::ListRef<EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceGroup_EbsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    size: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes_per_instance: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrCluster_CoreInstanceGroup_EbsConfig {
    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volumes_per_instance`.\n"]
    pub fn set_volumes_per_instance(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volumes_per_instance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceGroup_EbsConfig {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceGroup_EbsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceGroup_EbsConfig {
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_CoreInstanceGroup_EbsConfig {
    pub fn build(self) -> EmrCluster_CoreInstanceGroup_EbsConfig {
        EmrCluster_CoreInstanceGroup_EbsConfig {
            iops: core::default::Default::default(),
            size: self.size,
            throughput: core::default::Default::default(),
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceGroup_EbsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceGroup_EbsConfigRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_CoreInstanceGroup_EbsConfigRef {
        EmrCluster_CoreInstanceGroup_EbsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceGroup_EbsConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `volumes_per_instance` after provisioning.\n"]
    pub fn volumes_per_instance(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volumes_per_instance", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrCluster_CoreInstanceGroupDynamic {
    ebs_config: std::option::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceGroup_EbsConfig>>,
}

#[derive(Serialize)]
pub struct EmrCluster_CoreInstanceGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    autoscaling_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_config: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceGroup_EbsConfig>>,
    dynamic: EmrCluster_CoreInstanceGroupDynamic,
}

impl EmrCluster_CoreInstanceGroup {
    #[doc = "Set the field `autoscaling_policy`.\n"]
    pub fn set_autoscaling_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.autoscaling_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bid_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceGroup_EbsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrCluster_CoreInstanceGroup {
    type O = terrars::BlockAssignable<EmrCluster_CoreInstanceGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_CoreInstanceGroup {
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_CoreInstanceGroup {
    pub fn build(self) -> EmrCluster_CoreInstanceGroup {
        EmrCluster_CoreInstanceGroup {
            autoscaling_policy: core::default::Default::default(),
            bid_price: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: self.instance_type,
            name: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_CoreInstanceGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_CoreInstanceGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_CoreInstanceGroupRef {
        EmrCluster_CoreInstanceGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_CoreInstanceGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `autoscaling_policy` after provisioning.\n"]
    pub fn autoscaling_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.autoscaling_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_Ec2Attributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_master_security_groups: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_slave_security_groups: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emr_managed_master_security_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    emr_managed_slave_security_group: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_profile: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_access_security_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl EmrCluster_Ec2Attributes {
    #[doc = "Set the field `additional_master_security_groups`.\n"]
    pub fn set_additional_master_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_master_security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_slave_security_groups`.\n"]
    pub fn set_additional_slave_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_slave_security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `emr_managed_master_security_group`.\n"]
    pub fn set_emr_managed_master_security_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.emr_managed_master_security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `emr_managed_slave_security_group`.\n"]
    pub fn set_emr_managed_slave_security_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.emr_managed_slave_security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_access_security_group`.\n"]
    pub fn set_service_access_security_group(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.service_access_security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_ids`.\n"]
    pub fn set_subnet_ids(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.subnet_ids = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_Ec2Attributes {
    type O = terrars::BlockAssignable<EmrCluster_Ec2Attributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_Ec2Attributes {
    #[doc = ""]
    pub instance_profile: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_Ec2Attributes {
    pub fn build(self) -> EmrCluster_Ec2Attributes {
        EmrCluster_Ec2Attributes {
            additional_master_security_groups: core::default::Default::default(),
            additional_slave_security_groups: core::default::Default::default(),
            emr_managed_master_security_group: core::default::Default::default(),
            emr_managed_slave_security_group: core::default::Default::default(),
            instance_profile: self.instance_profile,
            key_name: core::default::Default::default(),
            service_access_security_group: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            subnet_ids: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_Ec2AttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_Ec2AttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_Ec2AttributesRef {
        EmrCluster_Ec2AttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_Ec2AttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_master_security_groups` after provisioning.\n"]
    pub fn additional_master_security_groups(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_master_security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_slave_security_groups` after provisioning.\n"]
    pub fn additional_slave_security_groups(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_slave_security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_managed_master_security_group` after provisioning.\n"]
    pub fn emr_managed_master_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.emr_managed_master_security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `emr_managed_slave_security_group` after provisioning.\n"]
    pub fn emr_managed_slave_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.emr_managed_slave_security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_profile` after provisioning.\n"]
    pub fn instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile", self.base))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.base))
    }

    #[doc = "Get a reference to the value of field `service_access_security_group` after provisioning.\n"]
    pub fn service_access_security_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_access_security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_ids` after provisioning.\n"]
    pub fn subnet_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_ids", self.base))
    }
}

#[derive(Serialize)]
pub struct EmrCluster_KerberosAttributes {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ad_domain_join_password: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ad_domain_join_user: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_realm_trust_principal_password: std::option::Option<terrars::PrimField<std::string::String>>,
    kdc_admin_password: terrars::PrimField<std::string::String>,
    realm: terrars::PrimField<std::string::String>,
}

impl EmrCluster_KerberosAttributes {
    #[doc = "Set the field `ad_domain_join_password`.\n"]
    pub fn set_ad_domain_join_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ad_domain_join_password = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ad_domain_join_user`.\n"]
    pub fn set_ad_domain_join_user(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.ad_domain_join_user = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cross_realm_trust_principal_password`.\n"]
    pub fn set_cross_realm_trust_principal_password(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.cross_realm_trust_principal_password = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrCluster_KerberosAttributes {
    type O = terrars::BlockAssignable<EmrCluster_KerberosAttributes>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_KerberosAttributes {
    #[doc = ""]
    pub kdc_admin_password: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub realm: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_KerberosAttributes {
    pub fn build(self) -> EmrCluster_KerberosAttributes {
        EmrCluster_KerberosAttributes {
            ad_domain_join_password: core::default::Default::default(),
            ad_domain_join_user: core::default::Default::default(),
            cross_realm_trust_principal_password: core::default::Default::default(),
            kdc_admin_password: self.kdc_admin_password,
            realm: self.realm,
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_KerberosAttributesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_KerberosAttributesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_KerberosAttributesRef {
        EmrCluster_KerberosAttributesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_KerberosAttributesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ad_domain_join_password` after provisioning.\n"]
    pub fn ad_domain_join_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ad_domain_join_password", self.base))
    }

    #[doc = "Get a reference to the value of field `ad_domain_join_user` after provisioning.\n"]
    pub fn ad_domain_join_user(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ad_domain_join_user", self.base))
    }

    #[doc = "Get a reference to the value of field `cross_realm_trust_principal_password` after provisioning.\n"]
    pub fn cross_realm_trust_principal_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cross_realm_trust_principal_password", self.base))
    }

    #[doc = "Get a reference to the value of field `kdc_admin_password` after provisioning.\n"]
    pub fn kdc_admin_password(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kdc_admin_password", self.base))
    }

    #[doc = "Get a reference to the value of field `realm` after provisioning.\n"]
    pub fn realm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.realm", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrCluster_MasterInstanceFleetDynamic {
    instance_type_configs: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>>,
    launch_specifications: std
    ::option
    ::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceFleet_LaunchSpecifications>>,
}

#[derive(Serialize)]
pub struct EmrCluster_MasterInstanceFleet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_on_demand_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_spot_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type_configs: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_specifications: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceFleet_LaunchSpecifications>>,
    dynamic: EmrCluster_MasterInstanceFleetDynamic,
}

impl EmrCluster_MasterInstanceFleet {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_on_demand_capacity`.\n"]
    pub fn set_target_on_demand_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_on_demand_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_spot_capacity`.\n"]
    pub fn set_target_spot_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.target_spot_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type_configs`.\n"]
    pub fn set_instance_type_configs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_InstanceTypeConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.instance_type_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.instance_type_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_specifications`.\n"]
    pub fn set_launch_specifications(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceFleet_LaunchSpecifications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.launch_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.launch_specifications = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrCluster_MasterInstanceFleet {
    type O = terrars::BlockAssignable<EmrCluster_MasterInstanceFleet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_MasterInstanceFleet {}

impl BuildEmrCluster_MasterInstanceFleet {
    pub fn build(self) -> EmrCluster_MasterInstanceFleet {
        EmrCluster_MasterInstanceFleet {
            name: core::default::Default::default(),
            target_on_demand_capacity: core::default::Default::default(),
            target_spot_capacity: core::default::Default::default(),
            instance_type_configs: core::default::Default::default(),
            launch_specifications: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_MasterInstanceFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_MasterInstanceFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_MasterInstanceFleetRef {
        EmrCluster_MasterInstanceFleetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_MasterInstanceFleetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> terrars::ListRef<EmrCluster_CoreInstanceFleet_LaunchSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.base))
    }
}

#[derive(Serialize, Default)]
struct EmrCluster_MasterInstanceGroupDynamic {
    ebs_config: std::option::Option<terrars::DynamicBlock<EmrCluster_CoreInstanceGroup_EbsConfig>>,
}

#[derive(Serialize)]
pub struct EmrCluster_MasterInstanceGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_count: std::option::Option<terrars::PrimField<f64>>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_config: std::option::Option<std::vec::Vec<EmrCluster_CoreInstanceGroup_EbsConfig>>,
    dynamic: EmrCluster_MasterInstanceGroupDynamic,
}

impl EmrCluster_MasterInstanceGroup {
    #[doc = "Set the field `bid_price`.\n"]
    pub fn set_bid_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bid_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_count`.\n"]
    pub fn set_instance_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_config`.\n"]
    pub fn set_ebs_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrCluster_CoreInstanceGroup_EbsConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs_config = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for EmrCluster_MasterInstanceGroup {
    type O = terrars::BlockAssignable<EmrCluster_MasterInstanceGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrCluster_MasterInstanceGroup {
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrCluster_MasterInstanceGroup {
    pub fn build(self) -> EmrCluster_MasterInstanceGroup {
        EmrCluster_MasterInstanceGroup {
            bid_price: core::default::Default::default(),
            instance_count: core::default::Default::default(),
            instance_type: self.instance_type,
            name: core::default::Default::default(),
            ebs_config: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrCluster_MasterInstanceGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrCluster_MasterInstanceGroupRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrCluster_MasterInstanceGroupRef {
        EmrCluster_MasterInstanceGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrCluster_MasterInstanceGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bid_price` after provisioning.\n"]
    pub fn bid_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bid_price", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_count` after provisioning.\n"]
    pub fn instance_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_count", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}
