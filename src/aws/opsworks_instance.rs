use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct OpsworksInstanceData {
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
    agent_version: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ami_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_scaling_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_ebs: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_eip: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_optimized: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecs_cluster_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    infrastructure_class: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    install_updates_on_boot: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_profile_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    layer_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    os: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_device_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_ids: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ssh_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    stack_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtualization_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_block_device: std::option::Option<std::vec::Vec<OpsworksInstanceEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_block_device: std::option::Option<std::vec::Vec<OpsworksInstanceEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_block_device: std::option::Option<std::vec::Vec<OpsworksInstanceRootBlockDeviceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<OpsworksInstanceTimeoutsEl>,
    dynamic: OpsworksInstanceDynamic,
}

struct OpsworksInstance_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<OpsworksInstanceData>,
}

#[derive(Clone)]
pub struct OpsworksInstance(std::rc::Rc<OpsworksInstance_>);

impl OpsworksInstance {
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

    #[doc = "Set the field `agent_version`.\n"]
    pub fn set_agent_version(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().agent_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ami_id`.\n"]
    pub fn set_ami_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ami_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().architecture = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `auto_scaling_type`.\n"]
    pub fn set_auto_scaling_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().auto_scaling_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().created_at = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_ebs`.\n"]
    pub fn set_delete_ebs(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_ebs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_eip`.\n"]
    pub fn set_delete_eip(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_eip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized`.\n"]
    pub fn set_ebs_optimized(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ebs_optimized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ecs_cluster_arn`.\n"]
    pub fn set_ecs_cluster_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ecs_cluster_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_ip`.\n"]
    pub fn set_elastic_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().elastic_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname`.\n"]
    pub fn set_hostname(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().hostname = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `infrastructure_class`.\n"]
    pub fn set_infrastructure_class(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().infrastructure_class = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `install_updates_on_boot`.\n"]
    pub fn set_install_updates_on_boot(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().install_updates_on_boot = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_profile_arn`.\n"]
    pub fn set_instance_profile_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_profile_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `os`.\n"]
    pub fn set_os(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().os = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `root_device_type`.\n"]
    pub fn set_root_device_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().root_device_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_ids`.\n"]
    pub fn set_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ssh_key_name`.\n"]
    pub fn set_ssh_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ssh_key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tenancy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtualization_type`.\n"]
    pub fn set_virtualization_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().virtualization_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksInstanceEbsBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ebs_block_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ebs_block_device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksInstanceEphemeralBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().ephemeral_block_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.ephemeral_block_device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<OpsworksInstanceRootBlockDeviceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().root_block_device = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.root_block_device = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<OpsworksInstanceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ami_id` after provisioning.\n"]
    pub fn ami_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_type` after provisioning.\n"]
    pub fn auto_scaling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_ebs` after provisioning.\n"]
    pub fn delete_ebs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_ebs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_eip` after provisioning.\n"]
    pub fn delete_eip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_eip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_class` after provisioning.\n"]
    pub fn infrastructure_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_arn` after provisioning.\n"]
    pub fn instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_service_error_id` after provisioning.\n"]
    pub fn last_service_error_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_service_error_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `layer_ids` after provisioning.\n"]
    pub fn layer_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os` after provisioning.\n"]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registered_by` after provisioning.\n"]
    pub fn registered_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registered_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_agent_version` after provisioning.\n"]
    pub fn reported_agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_os_family` after provisioning.\n"]
    pub fn reported_os_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_os_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_os_name` after provisioning.\n"]
    pub fn reported_os_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_os_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_os_version` after provisioning.\n"]
    pub fn reported_os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_os_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_device_type` after provisioning.\n"]
    pub fn root_device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_device_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_device_volume_id` after provisioning.\n"]
    pub fn root_device_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_device_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_host_dsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_dsa_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_dsa_key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_host_rsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_rsa_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_rsa_key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_key_name` after provisioning.\n"]
    pub fn ssh_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtualization_type` after provisioning.\n"]
    pub fn virtualization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtualization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksInstanceTimeoutsElRef {
        OpsworksInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for OpsworksInstance {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for OpsworksInstance { }

impl terrars::ToListMappable for OpsworksInstance {
    type O = terrars::ListRef<OpsworksInstanceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for OpsworksInstance_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_opsworks_instance".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildOpsworksInstance {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub layer_ids: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub stack_id: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksInstance {
    pub fn build(self, stack: &mut terrars::Stack) -> OpsworksInstance {
        let out = OpsworksInstance(std::rc::Rc::new(OpsworksInstance_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(OpsworksInstanceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                agent_version: core::default::Default::default(),
                ami_id: core::default::Default::default(),
                architecture: core::default::Default::default(),
                auto_scaling_type: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                created_at: core::default::Default::default(),
                delete_ebs: core::default::Default::default(),
                delete_eip: core::default::Default::default(),
                ebs_optimized: core::default::Default::default(),
                ecs_cluster_arn: core::default::Default::default(),
                elastic_ip: core::default::Default::default(),
                hostname: core::default::Default::default(),
                id: core::default::Default::default(),
                infrastructure_class: core::default::Default::default(),
                install_updates_on_boot: core::default::Default::default(),
                instance_profile_arn: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                layer_ids: self.layer_ids,
                os: core::default::Default::default(),
                root_device_type: core::default::Default::default(),
                security_group_ids: core::default::Default::default(),
                ssh_key_name: core::default::Default::default(),
                stack_id: self.stack_id,
                state: core::default::Default::default(),
                status: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tenancy: core::default::Default::default(),
                virtualization_type: core::default::Default::default(),
                ebs_block_device: core::default::Default::default(),
                ephemeral_block_device: core::default::Default::default(),
                root_block_device: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct OpsworksInstanceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksInstanceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl OpsworksInstanceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `agent_version` after provisioning.\n"]
    pub fn agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ami_id` after provisioning.\n"]
    pub fn ami_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.architecture", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_scaling_type` after provisioning.\n"]
    pub fn auto_scaling_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_scaling_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_ebs` after provisioning.\n"]
    pub fn delete_ebs(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_ebs", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_eip` after provisioning.\n"]
    pub fn delete_eip(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_eip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ec2_instance_id` after provisioning.\n"]
    pub fn ec2_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ec2_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ecs_cluster_arn` after provisioning.\n"]
    pub fn ecs_cluster_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ecs_cluster_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_ip` after provisioning.\n"]
    pub fn elastic_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.elastic_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hostname` after provisioning.\n"]
    pub fn hostname(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `infrastructure_class` after provisioning.\n"]
    pub fn infrastructure_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.infrastructure_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `install_updates_on_boot` after provisioning.\n"]
    pub fn install_updates_on_boot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.install_updates_on_boot", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_profile_arn` after provisioning.\n"]
    pub fn instance_profile_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_profile_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_service_error_id` after provisioning.\n"]
    pub fn last_service_error_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_service_error_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `layer_ids` after provisioning.\n"]
    pub fn layer_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.layer_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `os` after provisioning.\n"]
    pub fn os(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.os", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `platform` after provisioning.\n"]
    pub fn platform(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.platform", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns` after provisioning.\n"]
    pub fn private_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_dns` after provisioning.\n"]
    pub fn public_dns(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_dns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `registered_by` after provisioning.\n"]
    pub fn registered_by(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.registered_by", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_agent_version` after provisioning.\n"]
    pub fn reported_agent_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_agent_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_os_family` after provisioning.\n"]
    pub fn reported_os_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_os_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_os_name` after provisioning.\n"]
    pub fn reported_os_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_os_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reported_os_version` after provisioning.\n"]
    pub fn reported_os_version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reported_os_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_device_type` after provisioning.\n"]
    pub fn root_device_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_device_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_device_volume_id` after provisioning.\n"]
    pub fn root_device_volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.root_device_volume_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_ids` after provisioning.\n"]
    pub fn security_group_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_host_dsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_dsa_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_dsa_key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_host_rsa_key_fingerprint` after provisioning.\n"]
    pub fn ssh_host_rsa_key_fingerprint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_host_rsa_key_fingerprint", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ssh_key_name` after provisioning.\n"]
    pub fn ssh_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ssh_key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `stack_id` after provisioning.\n"]
    pub fn stack_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stack_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtualization_type` after provisioning.\n"]
    pub fn virtualization_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtualization_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> OpsworksInstanceTimeoutsElRef {
        OpsworksInstanceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    device_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksInstanceEbsBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksInstanceEbsBlockDeviceEl {
    type O = terrars::BlockAssignable<OpsworksInstanceEbsBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceEbsBlockDeviceEl {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksInstanceEbsBlockDeviceEl {
    pub fn build(self) -> OpsworksInstanceEbsBlockDeviceEl {
        OpsworksInstanceEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            iops: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct OpsworksInstanceEbsBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksInstanceEbsBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksInstanceEbsBlockDeviceElRef {
        OpsworksInstanceEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceEbsBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceEphemeralBlockDeviceEl {
    device_name: terrars::PrimField<std::string::String>,
    virtual_name: terrars::PrimField<std::string::String>,
}

impl OpsworksInstanceEphemeralBlockDeviceEl { }

impl terrars::ToListMappable for OpsworksInstanceEphemeralBlockDeviceEl {
    type O = terrars::BlockAssignable<OpsworksInstanceEphemeralBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceEphemeralBlockDeviceEl {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_name: terrars::PrimField<std::string::String>,
}

impl BuildOpsworksInstanceEphemeralBlockDeviceEl {
    pub fn build(self) -> OpsworksInstanceEphemeralBlockDeviceEl {
        OpsworksInstanceEphemeralBlockDeviceEl {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
        }
    }
}

pub struct OpsworksInstanceEphemeralBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksInstanceEphemeralBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksInstanceEphemeralBlockDeviceElRef {
        OpsworksInstanceEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceEphemeralBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceRootBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksInstanceRootBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_size`.\n"]
    pub fn set_volume_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_type`.\n"]
    pub fn set_volume_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.volume_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksInstanceRootBlockDeviceEl {
    type O = terrars::BlockAssignable<OpsworksInstanceRootBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceRootBlockDeviceEl {}

impl BuildOpsworksInstanceRootBlockDeviceEl {
    pub fn build(self) -> OpsworksInstanceRootBlockDeviceEl {
        OpsworksInstanceRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            iops: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct OpsworksInstanceRootBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksInstanceRootBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksInstanceRootBlockDeviceElRef {
        OpsworksInstanceRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceRootBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_size` after provisioning.\n"]
    pub fn volume_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_size", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_type` after provisioning.\n"]
    pub fn volume_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_type", self.base))
    }
}

#[derive(Serialize)]
pub struct OpsworksInstanceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl OpsworksInstanceTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for OpsworksInstanceTimeoutsEl {
    type O = terrars::BlockAssignable<OpsworksInstanceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildOpsworksInstanceTimeoutsEl {}

impl BuildOpsworksInstanceTimeoutsEl {
    pub fn build(self) -> OpsworksInstanceTimeoutsEl {
        OpsworksInstanceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

pub struct OpsworksInstanceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for OpsworksInstanceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> OpsworksInstanceTimeoutsElRef {
        OpsworksInstanceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl OpsworksInstanceTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}

#[derive(Serialize, Default)]
struct OpsworksInstanceDynamic {
    ebs_block_device: std::option::Option<terrars::DynamicBlock<OpsworksInstanceEbsBlockDeviceEl>>,
    ephemeral_block_device: std::option::Option<terrars::DynamicBlock<OpsworksInstanceEphemeralBlockDeviceEl>>,
    root_block_device: std::option::Option<terrars::DynamicBlock<OpsworksInstanceRootBlockDeviceEl>>,
}
