#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LaunchTemplateData_ {
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
    default_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_api_stop: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_api_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_optimized: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    image_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_initiated_shutdown_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kernel_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ram_disk_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update_default_version: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_device_mappings: std::option::Option<std::vec::Vec<LaunchTemplate_BlockDeviceMappings>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_specification: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_CapacityReservationSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_options: std::option::Option<std::vec::Vec<LaunchTemplate_CpuOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credit_specification: std::option::Option<std::vec::Vec<LaunchTemplate_CreditSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enclave_options: std::option::Option<std::vec::Vec<LaunchTemplate_EnclaveOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hibernation_options: std::option::Option<std::vec::Vec<LaunchTemplate_HibernationOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_instance_profile: std::option::Option<std::vec::Vec<LaunchTemplate_IamInstanceProfile>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_market_options: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceMarketOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_specification: std::option::Option<std::vec::Vec<LaunchTemplate_LicenseSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_options: std::option::Option<std::vec::Vec<LaunchTemplate_MaintenanceOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_options: std::option::Option<std::vec::Vec<LaunchTemplate_MetadataOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<std::vec::Vec<LaunchTemplate_Monitoring>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std::option::Option<std::vec::Vec<LaunchTemplate_NetworkInterfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_performance_options: std::option::Option<std::vec::Vec<LaunchTemplate_NetworkPerformanceOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<LaunchTemplate_Placement>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_dns_name_options: std::option::Option<std::vec::Vec<LaunchTemplate_PrivateDnsNameOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_interfaces: std::option::Option<std::vec::Vec<LaunchTemplate_SecondaryInterfaces>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_specifications: std::option::Option<std::vec::Vec<LaunchTemplate_TagSpecifications>>,
    dynamic: LaunchTemplateDynamic,
}

struct LaunchTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LaunchTemplateData_>,
}

#[derive(Clone)]
pub struct LaunchTemplate(std::rc::Rc<LaunchTemplate_>);

impl LaunchTemplate {
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

    #[doc = "Set the field `default_version`.\n"]
    pub fn set_default_version(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().default_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_api_stop`.\n"]
    pub fn set_disable_api_stop(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_api_stop = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `disable_api_termination`.\n"]
    pub fn set_disable_api_termination(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().disable_api_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs_optimized`.\n"]
    pub fn set_ebs_optimized(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ebs_optimized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `image_id`.\n"]
    pub fn set_image_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().image_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_initiated_shutdown_behavior`.\n"]
    pub fn set_instance_initiated_shutdown_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_initiated_shutdown_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kernel_id`.\n"]
    pub fn set_kernel_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().kernel_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ram_disk_id`.\n"]
    pub fn set_ram_disk_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ram_disk_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_names`.\n"]
    pub fn set_security_group_names(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_group_names = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `update_default_version`.\n"]
    pub fn set_update_default_version(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().update_default_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_security_group_ids`.\n"]
    pub fn set_vpc_security_group_ids(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().vpc_security_group_ids = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_device_mappings`.\n"]
    pub fn set_block_device_mappings(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_BlockDeviceMappings>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().block_device_mappings = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.block_device_mappings = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_CapacityReservationSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().capacity_reservation_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.capacity_reservation_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cpu_options`.\n"]
    pub fn set_cpu_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_CpuOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cpu_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cpu_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `credit_specification`.\n"]
    pub fn set_credit_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_CreditSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().credit_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.credit_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enclave_options`.\n"]
    pub fn set_enclave_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_EnclaveOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().enclave_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.enclave_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `hibernation_options`.\n"]
    pub fn set_hibernation_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_HibernationOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().hibernation_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.hibernation_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `iam_instance_profile`.\n"]
    pub fn set_iam_instance_profile(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_IamInstanceProfile>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().iam_instance_profile = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.iam_instance_profile = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_market_options`.\n"]
    pub fn set_instance_market_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceMarketOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_market_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_market_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `instance_requirements`.\n"]
    pub fn set_instance_requirements(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_requirements = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_requirements = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `license_specification`.\n"]
    pub fn set_license_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_LicenseSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().license_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.license_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_options`.\n"]
    pub fn set_maintenance_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_MaintenanceOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().maintenance_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.maintenance_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `metadata_options`.\n"]
    pub fn set_metadata_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_MetadataOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().metadata_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.metadata_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_Monitoring>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().monitoring = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.monitoring = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interfaces`.\n"]
    pub fn set_network_interfaces(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interfaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_performance_options`.\n"]
    pub fn set_network_performance_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_NetworkPerformanceOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_performance_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_performance_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(self, v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_Placement>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().placement = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.placement = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_dns_name_options`.\n"]
    pub fn set_private_dns_name_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_PrivateDnsNameOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().private_dns_name_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.private_dns_name_options = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `secondary_interfaces`.\n"]
    pub fn set_secondary_interfaces(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_SecondaryInterfaces>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().secondary_interfaces = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.secondary_interfaces = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_specifications`.\n"]
    pub fn set_tag_specifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_TagSpecifications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tag_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tag_specifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_version` after provisioning.\n"]
    pub fn default_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_initiated_shutdown_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_id` after provisioning.\n"]
    pub fn kernel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kernel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest_version` after provisioning.\n"]
    pub fn latest_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_disk_id` after provisioning.\n"]
    pub fn ram_disk_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ram_disk_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_default_version` after provisioning.\n"]
    pub fn update_default_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_device_mappings` after provisioning.\n"]
    pub fn block_device_mappings(&self) -> terrars::ListRef<LaunchTemplate_BlockDeviceMappingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_device_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_CapacityReservationSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> terrars::ListRef<LaunchTemplate_CpuOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<LaunchTemplate_CreditSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<LaunchTemplate_EnclaveOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> terrars::ListRef<LaunchTemplate_HibernationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hibernation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::ListRef<LaunchTemplate_IamInstanceProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> terrars::ListRef<LaunchTemplate_InstanceMarketOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_market_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<LaunchTemplate_MaintenanceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<LaunchTemplate_MetadataOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<LaunchTemplate_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<LaunchTemplate_NetworkInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_options` after provisioning.\n"]
    pub fn network_performance_options(&self) -> terrars::ListRef<LaunchTemplate_NetworkPerformanceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<LaunchTemplate_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<LaunchTemplate_PrivateDnsNameOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_interfaces` after provisioning.\n"]
    pub fn secondary_interfaces(&self) -> terrars::ListRef<LaunchTemplate_SecondaryInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> terrars::ListRef<LaunchTemplate_TagSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_specifications", self.extract_ref()))
    }
}

impl terrars::Referable for LaunchTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for LaunchTemplate { }

impl terrars::ToListMappable for LaunchTemplate {
    type O = terrars::ListRef<LaunchTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for LaunchTemplate_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_launch_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildLaunchTemplate {
    pub tf_id: std::string::String,
}

impl BuildLaunchTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> LaunchTemplate {
        let out = LaunchTemplate(std::rc::Rc::new(LaunchTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(LaunchTemplateData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_version: core::default::Default::default(),
                description: core::default::Default::default(),
                disable_api_stop: core::default::Default::default(),
                disable_api_termination: core::default::Default::default(),
                ebs_optimized: core::default::Default::default(),
                id: core::default::Default::default(),
                image_id: core::default::Default::default(),
                instance_initiated_shutdown_behavior: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                kernel_id: core::default::Default::default(),
                key_name: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                ram_disk_id: core::default::Default::default(),
                region: core::default::Default::default(),
                security_group_names: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                update_default_version: core::default::Default::default(),
                user_data: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                block_device_mappings: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                cpu_options: core::default::Default::default(),
                credit_specification: core::default::Default::default(),
                enclave_options: core::default::Default::default(),
                hibernation_options: core::default::Default::default(),
                iam_instance_profile: core::default::Default::default(),
                instance_market_options: core::default::Default::default(),
                instance_requirements: core::default::Default::default(),
                license_specification: core::default::Default::default(),
                maintenance_options: core::default::Default::default(),
                metadata_options: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                network_interfaces: core::default::Default::default(),
                network_performance_options: core::default::Default::default(),
                placement: core::default::Default::default(),
                private_dns_name_options: core::default::Default::default(),
                secondary_interfaces: core::default::Default::default(),
                tag_specifications: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct LaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl LaunchTemplateRef {
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

    #[doc = "Get a reference to the value of field `default_version` after provisioning.\n"]
    pub fn default_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_stop` after provisioning.\n"]
    pub fn disable_api_stop(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_stop", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `disable_api_termination` after provisioning.\n"]
    pub fn disable_api_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.disable_api_termination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized` after provisioning.\n"]
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `image_id` after provisioning.\n"]
    pub fn image_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.image_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_initiated_shutdown_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `kernel_id` after provisioning.\n"]
    pub fn kernel_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kernel_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `latest_version` after provisioning.\n"]
    pub fn latest_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latest_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_disk_id` after provisioning.\n"]
    pub fn ram_disk_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ram_disk_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_default_version` after provisioning.\n"]
    pub fn update_default_version(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_default_version", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_device_mappings` after provisioning.\n"]
    pub fn block_device_mappings(&self) -> terrars::ListRef<LaunchTemplate_BlockDeviceMappingsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_device_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_CapacityReservationSpecificationRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> terrars::ListRef<LaunchTemplate_CpuOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<LaunchTemplate_CreditSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<LaunchTemplate_EnclaveOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> terrars::ListRef<LaunchTemplate_HibernationOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hibernation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::ListRef<LaunchTemplate_IamInstanceProfileRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> terrars::ListRef<LaunchTemplate_InstanceMarketOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_market_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirementsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<LaunchTemplate_MaintenanceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<LaunchTemplate_MetadataOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<LaunchTemplate_MonitoringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<LaunchTemplate_NetworkInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance_options` after provisioning.\n"]
    pub fn network_performance_options(&self) -> terrars::ListRef<LaunchTemplate_NetworkPerformanceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_performance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<LaunchTemplate_PlacementRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<LaunchTemplate_PrivateDnsNameOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secondary_interfaces` after provisioning.\n"]
    pub fn secondary_interfaces(&self) -> terrars::ListRef<LaunchTemplate_SecondaryInterfacesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.secondary_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> terrars::ListRef<LaunchTemplate_TagSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_specifications", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplateDynamic {
    block_device_mappings: std::option::Option<terrars::DynamicBlock<LaunchTemplate_BlockDeviceMappings>>,
    capacity_reservation_specification: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_CapacityReservationSpecification>>,
    cpu_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_CpuOptions>>,
    credit_specification: std::option::Option<terrars::DynamicBlock<LaunchTemplate_CreditSpecification>>,
    enclave_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_EnclaveOptions>>,
    hibernation_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_HibernationOptions>>,
    iam_instance_profile: std::option::Option<terrars::DynamicBlock<LaunchTemplate_IamInstanceProfile>>,
    instance_market_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_InstanceMarketOptions>>,
    instance_requirements: std::option::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements>>,
    license_specification: std::option::Option<terrars::DynamicBlock<LaunchTemplate_LicenseSpecification>>,
    maintenance_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_MaintenanceOptions>>,
    metadata_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_MetadataOptions>>,
    monitoring: std::option::Option<terrars::DynamicBlock<LaunchTemplate_Monitoring>>,
    network_interfaces: std::option::Option<terrars::DynamicBlock<LaunchTemplate_NetworkInterfaces>>,
    network_performance_options: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_NetworkPerformanceOptions>>,
    placement: std::option::Option<terrars::DynamicBlock<LaunchTemplate_Placement>>,
    private_dns_name_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_PrivateDnsNameOptions>>,
    secondary_interfaces: std::option::Option<terrars::DynamicBlock<LaunchTemplate_SecondaryInterfaces>>,
    tag_specifications: std::option::Option<terrars::DynamicBlock<LaunchTemplate_TagSpecifications>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_BlockDeviceMappings_Ebs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_initialization_rate: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_BlockDeviceMappings_Ebs {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iops`.\n"]
    pub fn set_iops(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.iops = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_id`.\n"]
    pub fn set_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `snapshot_id`.\n"]
    pub fn set_snapshot_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.snapshot_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `throughput`.\n"]
    pub fn set_throughput(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.throughput = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_initialization_rate`.\n"]
    pub fn set_volume_initialization_rate(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.volume_initialization_rate = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for LaunchTemplate_BlockDeviceMappings_Ebs {
    type O = terrars::BlockAssignable<LaunchTemplate_BlockDeviceMappings_Ebs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_BlockDeviceMappings_Ebs {}

impl BuildLaunchTemplate_BlockDeviceMappings_Ebs {
    pub fn build(self) -> LaunchTemplate_BlockDeviceMappings_Ebs {
        LaunchTemplate_BlockDeviceMappings_Ebs {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_initialization_rate: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_BlockDeviceMappings_EbsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_BlockDeviceMappings_EbsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_BlockDeviceMappings_EbsRef {
        LaunchTemplate_BlockDeviceMappings_EbsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_BlockDeviceMappings_EbsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `iops` after provisioning.\n"]
    pub fn iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iops", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_id` after provisioning.\n"]
    pub fn kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_id` after provisioning.\n"]
    pub fn snapshot_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.snapshot_id", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_initialization_rate` after provisioning.\n"]
    pub fn volume_initialization_rate(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_initialization_rate", self.base))
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

#[derive(Serialize, Default)]
struct LaunchTemplate_BlockDeviceMappingsDynamic {
    ebs: std::option::Option<terrars::DynamicBlock<LaunchTemplate_BlockDeviceMappings_Ebs>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_BlockDeviceMappings {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs: std::option::Option<std::vec::Vec<LaunchTemplate_BlockDeviceMappings_Ebs>>,
    dynamic: LaunchTemplate_BlockDeviceMappingsDynamic,
}

impl LaunchTemplate_BlockDeviceMappings {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.no_device = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_BlockDeviceMappings_Ebs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ebs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ebs = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_BlockDeviceMappings {
    type O = terrars::BlockAssignable<LaunchTemplate_BlockDeviceMappings>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_BlockDeviceMappings {}

impl BuildLaunchTemplate_BlockDeviceMappings {
    pub fn build(self) -> LaunchTemplate_BlockDeviceMappings {
        LaunchTemplate_BlockDeviceMappings {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_BlockDeviceMappingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_BlockDeviceMappingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_BlockDeviceMappingsRef {
        LaunchTemplate_BlockDeviceMappingsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_BlockDeviceMappingsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(&self) -> terrars::ListRef<LaunchTemplate_BlockDeviceMappings_EbsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_resource_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {
    #[doc = "Set the field `capacity_reservation_id`.\n"]
    pub fn set_capacity_reservation_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_reservation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_resource_group_arn`.\n"]
    pub fn set_capacity_reservation_resource_group_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_reservation_resource_group_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {
    type O = terrars::BlockAssignable<LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {}

impl BuildLaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {
    pub fn build(self) -> LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {
        LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_CapacityReservationSpecification_CapacityReservationTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_CapacityReservationSpecification_CapacityReservationTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_CapacityReservationSpecification_CapacityReservationTargetRef {
        LaunchTemplate_CapacityReservationSpecification_CapacityReservationTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_CapacityReservationSpecification_CapacityReservationTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_id` after provisioning.\n"]
    pub fn capacity_reservation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_resource_group_arn` after provisioning.\n"]
    pub fn capacity_reservation_resource_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_resource_group_arn", self.base),
        )
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplate_CapacityReservationSpecificationDynamic {
    capacity_reservation_target: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_CapacityReservationSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_target: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget>>,
    dynamic: LaunchTemplate_CapacityReservationSpecificationDynamic,
}

impl LaunchTemplate_CapacityReservationSpecification {
    #[doc = "Set the field `capacity_reservation_preference`.\n"]
    pub fn set_capacity_reservation_preference(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.capacity_reservation_preference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_target`.\n"]
    pub fn set_capacity_reservation_target(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<LaunchTemplate_CapacityReservationSpecification_CapacityReservationTarget>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.capacity_reservation_target = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.capacity_reservation_target = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_CapacityReservationSpecification {
    type O = terrars::BlockAssignable<LaunchTemplate_CapacityReservationSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_CapacityReservationSpecification {}

impl BuildLaunchTemplate_CapacityReservationSpecification {
    pub fn build(self) -> LaunchTemplate_CapacityReservationSpecification {
        LaunchTemplate_CapacityReservationSpecification {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_CapacityReservationSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_CapacityReservationSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_CapacityReservationSpecificationRef {
        LaunchTemplate_CapacityReservationSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_CapacityReservationSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_preference` after provisioning.\n"]
    pub fn capacity_reservation_preference(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.capacity_reservation_preference", self.base))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_target` after provisioning.\n"]
    pub fn capacity_reservation_target(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_CapacityReservationSpecification_CapacityReservationTargetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_target", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_CpuOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amd_sev_snp: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nested_virtualization: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_CpuOptions {
    #[doc = "Set the field `amd_sev_snp`.\n"]
    pub fn set_amd_sev_snp(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.amd_sev_snp = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `core_count`.\n"]
    pub fn set_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nested_virtualization`.\n"]
    pub fn set_nested_virtualization(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.nested_virtualization = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_CpuOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_CpuOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_CpuOptions {}

impl BuildLaunchTemplate_CpuOptions {
    pub fn build(self) -> LaunchTemplate_CpuOptions {
        LaunchTemplate_CpuOptions {
            amd_sev_snp: core::default::Default::default(),
            core_count: core::default::Default::default(),
            nested_virtualization: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_CpuOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_CpuOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_CpuOptionsRef {
        LaunchTemplate_CpuOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_CpuOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `amd_sev_snp` after provisioning.\n"]
    pub fn amd_sev_snp(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amd_sev_snp", self.base))
    }

    #[doc = "Get a reference to the value of field `core_count` after provisioning.\n"]
    pub fn core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `nested_virtualization` after provisioning.\n"]
    pub fn nested_virtualization(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nested_virtualization", self.base))
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_CreditSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_credits: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_CreditSpecification {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_credits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_CreditSpecification {
    type O = terrars::BlockAssignable<LaunchTemplate_CreditSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_CreditSpecification {}

impl BuildLaunchTemplate_CreditSpecification {
    pub fn build(self) -> LaunchTemplate_CreditSpecification {
        LaunchTemplate_CreditSpecification { cpu_credits: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_CreditSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_CreditSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_CreditSpecificationRef {
        LaunchTemplate_CreditSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_CreditSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_EnclaveOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LaunchTemplate_EnclaveOptions {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_EnclaveOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_EnclaveOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_EnclaveOptions {}

impl BuildLaunchTemplate_EnclaveOptions {
    pub fn build(self) -> LaunchTemplate_EnclaveOptions {
        LaunchTemplate_EnclaveOptions { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_EnclaveOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_EnclaveOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_EnclaveOptionsRef {
        LaunchTemplate_EnclaveOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_EnclaveOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_HibernationOptions {
    configured: terrars::PrimField<bool>,
}

impl LaunchTemplate_HibernationOptions { }

impl terrars::ToListMappable for LaunchTemplate_HibernationOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_HibernationOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_HibernationOptions {
    #[doc = ""]
    pub configured: terrars::PrimField<bool>,
}

impl BuildLaunchTemplate_HibernationOptions {
    pub fn build(self) -> LaunchTemplate_HibernationOptions {
        LaunchTemplate_HibernationOptions { configured: self.configured }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_HibernationOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_HibernationOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_HibernationOptionsRef {
        LaunchTemplate_HibernationOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_HibernationOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configured` after provisioning.\n"]
    pub fn configured(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configured", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_IamInstanceProfile {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_IamInstanceProfile {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_IamInstanceProfile {
    type O = terrars::BlockAssignable<LaunchTemplate_IamInstanceProfile>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_IamInstanceProfile {}

impl BuildLaunchTemplate_IamInstanceProfile {
    pub fn build(self) -> LaunchTemplate_IamInstanceProfile {
        LaunchTemplate_IamInstanceProfile {
            arn: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_IamInstanceProfileRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_IamInstanceProfileRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_IamInstanceProfileRef {
        LaunchTemplate_IamInstanceProfileRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_IamInstanceProfileRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceMarketOptions_SpotOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_duration_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_interruption_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_until: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_InstanceMarketOptions_SpotOptions {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.block_duration_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_interruption_behavior`.\n"]
    pub fn set_instance_interruption_behavior(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_interruption_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_price`.\n"]
    pub fn set_max_price(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.max_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_instance_type`.\n"]
    pub fn set_spot_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.spot_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_until`.\n"]
    pub fn set_valid_until(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.valid_until = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceMarketOptions_SpotOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceMarketOptions_SpotOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceMarketOptions_SpotOptions {}

impl BuildLaunchTemplate_InstanceMarketOptions_SpotOptions {
    pub fn build(self) -> LaunchTemplate_InstanceMarketOptions_SpotOptions {
        LaunchTemplate_InstanceMarketOptions_SpotOptions {
            block_duration_minutes: core::default::Default::default(),
            instance_interruption_behavior: core::default::Default::default(),
            max_price: core::default::Default::default(),
            spot_instance_type: core::default::Default::default(),
            valid_until: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceMarketOptions_SpotOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceMarketOptions_SpotOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceMarketOptions_SpotOptionsRef {
        LaunchTemplate_InstanceMarketOptions_SpotOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceMarketOptions_SpotOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_interruption_behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `max_price` after provisioning.\n"]
    pub fn max_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_price", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_instance_type` after provisioning.\n"]
    pub fn spot_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.base))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplate_InstanceMarketOptionsDynamic {
    spot_options: std::option::Option<terrars::DynamicBlock<LaunchTemplate_InstanceMarketOptions_SpotOptions>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceMarketOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    market_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_options: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceMarketOptions_SpotOptions>>,
    dynamic: LaunchTemplate_InstanceMarketOptionsDynamic,
}

impl LaunchTemplate_InstanceMarketOptions {
    #[doc = "Set the field `market_type`.\n"]
    pub fn set_market_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.market_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceMarketOptions_SpotOptions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.spot_options = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.spot_options = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceMarketOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceMarketOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceMarketOptions {}

impl BuildLaunchTemplate_InstanceMarketOptions {
    pub fn build(self) -> LaunchTemplate_InstanceMarketOptions {
        LaunchTemplate_InstanceMarketOptions {
            market_type: core::default::Default::default(),
            spot_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceMarketOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceMarketOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_InstanceMarketOptionsRef {
        LaunchTemplate_InstanceMarketOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceMarketOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `market_type` after provisioning.\n"]
    pub fn market_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.market_type", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> terrars::ListRef<LaunchTemplate_InstanceMarketOptions_SpotOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_options", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_AcceleratorCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_AcceleratorCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_AcceleratorCount {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_AcceleratorCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_AcceleratorCount {}

impl BuildLaunchTemplate_InstanceRequirements_AcceleratorCount {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_AcceleratorCount {
        LaunchTemplate_InstanceRequirements_AcceleratorCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_AcceleratorCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_AcceleratorCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_AcceleratorCountRef {
        LaunchTemplate_InstanceRequirements_AcceleratorCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_AcceleratorCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {}

impl BuildLaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
        LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
        LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {}

impl BuildLaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
        LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
        LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {}

impl BuildLaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
        LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
        LaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_MemoryMib {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl LaunchTemplate_InstanceRequirements_MemoryMib {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_MemoryMib {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_MemoryMib>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_MemoryMib {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildLaunchTemplate_InstanceRequirements_MemoryMib {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_MemoryMib {
        LaunchTemplate_InstanceRequirements_MemoryMib {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_MemoryMibRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_MemoryMibRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_MemoryMibRef {
        LaunchTemplate_InstanceRequirements_MemoryMibRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_MemoryMibRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {}

impl BuildLaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
        LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
        LaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_NetworkInterfaceCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {}

impl BuildLaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
        LaunchTemplate_InstanceRequirements_NetworkInterfaceCount {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
        LaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `min`.\n"]
    pub fn set_min(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.min = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_TotalLocalStorageGb>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {}

impl BuildLaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
        LaunchTemplate_InstanceRequirements_TotalLocalStorageGb {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
        LaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements_VcpuCount {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl LaunchTemplate_InstanceRequirements_VcpuCount {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements_VcpuCount {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_VcpuCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements_VcpuCount {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildLaunchTemplate_InstanceRequirements_VcpuCount {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements_VcpuCount {
        LaunchTemplate_InstanceRequirements_VcpuCount {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirements_VcpuCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirements_VcpuCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_InstanceRequirements_VcpuCountRef {
        LaunchTemplate_InstanceRequirements_VcpuCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirements_VcpuCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `max` after provisioning.\n"]
    pub fn max(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max", self.base))
    }

    #[doc = "Get a reference to the value of field `min` after provisioning.\n"]
    pub fn min(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.min", self.base))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplate_InstanceRequirementsDynamic {
    accelerator_count: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_AcceleratorCount>>,
    accelerator_total_memory_mib: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib>>,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps>>,
    memory_gib_per_vcpu: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu>>,
    memory_mib: std::option::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_MemoryMib>>,
    network_bandwidth_gbps: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps>>,
    network_interface_count: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_NetworkInterfaceCount>>,
    total_local_storage_gb: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_TotalLocalStorageGb>>,
    vcpu_count: std::option::Option<terrars::DynamicBlock<LaunchTemplate_InstanceRequirements_VcpuCount>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_InstanceRequirements {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bare_metal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    burstable_performance: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    excluded_instance_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_generations: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    local_storage_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_spot_price_as_percentage_of_optimal_on_demand_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_AcceleratorCount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_MemoryMib>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_bandwidth_gbps: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_NetworkInterfaceCount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_TotalLocalStorageGb>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std::option::Option<std::vec::Vec<LaunchTemplate_InstanceRequirements_VcpuCount>>,
    dynamic: LaunchTemplate_InstanceRequirementsDynamic,
}

impl LaunchTemplate_InstanceRequirements {
    #[doc = "Set the field `accelerator_manufacturers`.\n"]
    pub fn set_accelerator_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_names`.\n"]
    pub fn set_accelerator_names(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_names = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_types`.\n"]
    pub fn set_accelerator_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.accelerator_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `allowed_instance_types`.\n"]
    pub fn set_allowed_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bare_metal`.\n"]
    pub fn set_bare_metal(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bare_metal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `burstable_performance`.\n"]
    pub fn set_burstable_performance(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.burstable_performance = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_manufacturers`.\n"]
    pub fn set_cpu_manufacturers(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cpu_manufacturers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `excluded_instance_types`.\n"]
    pub fn set_excluded_instance_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.excluded_instance_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_generations`.\n"]
    pub fn set_instance_generations(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.instance_generations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage`.\n"]
    pub fn set_local_storage(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.local_storage = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `local_storage_types`.\n"]
    pub fn set_local_storage_types(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.local_storage_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_spot_price_as_percentage_of_optimal_on_demand_price`.\n"]
    pub fn set_max_spot_price_as_percentage_of_optimal_on_demand_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_spot_price_as_percentage_of_optimal_on_demand_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `on_demand_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_on_demand_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.on_demand_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `require_hibernate_support`.\n"]
    pub fn set_require_hibernate_support(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.require_hibernate_support = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_max_price_percentage_over_lowest_price`.\n"]
    pub fn set_spot_max_price_percentage_over_lowest_price(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.spot_max_price_percentage_over_lowest_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_AcceleratorCount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMib>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.accelerator_total_memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.accelerator_total_memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.baseline_ebs_bandwidth_mbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.baseline_ebs_bandwidth_mbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_MemoryGibPerVcpu>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_gib_per_vcpu = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_gib_per_vcpu = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_MemoryMib>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.memory_mib = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.memory_mib = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_bandwidth_gbps`.\n"]
    pub fn set_network_bandwidth_gbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_NetworkBandwidthGbps>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_bandwidth_gbps = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_bandwidth_gbps = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_NetworkInterfaceCount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_interface_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_interface_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_TotalLocalStorageGb>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.total_local_storage_gb = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.total_local_storage_gb = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_InstanceRequirements_VcpuCount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vcpu_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vcpu_count = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_InstanceRequirements {
    type O = terrars::BlockAssignable<LaunchTemplate_InstanceRequirements>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_InstanceRequirements {}

impl BuildLaunchTemplate_InstanceRequirements {
    pub fn build(self) -> LaunchTemplate_InstanceRequirements {
        LaunchTemplate_InstanceRequirements {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            allowed_instance_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            max_spot_price_as_percentage_of_optimal_on_demand_price: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_bandwidth_gbps: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_InstanceRequirementsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_InstanceRequirementsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_InstanceRequirementsRef {
        LaunchTemplate_InstanceRequirementsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_InstanceRequirementsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_names", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_types", self.base))
    }

    #[doc = "Get a reference to the value of field `allowed_instance_types` after provisioning.\n"]
    pub fn allowed_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.allowed_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `burstable_performance` after provisioning.\n"]
    pub fn burstable_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.burstable_performance", self.base))
    }

    #[doc = "Get a reference to the value of field `cpu_manufacturers` after provisioning.\n"]
    pub fn cpu_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cpu_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `excluded_instance_types` after provisioning.\n"]
    pub fn excluded_instance_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.excluded_instance_types", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_generations` after provisioning.\n"]
    pub fn instance_generations(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_generations", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage` after provisioning.\n"]
    pub fn local_storage(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_storage", self.base))
    }

    #[doc = "Get a reference to the value of field `local_storage_types` after provisioning.\n"]
    pub fn local_storage_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.local_storage_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_spot_price_as_percentage_of_optimal_on_demand_price` after provisioning.\n"]
    pub fn max_spot_price_as_percentage_of_optimal_on_demand_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.max_spot_price_as_percentage_of_optimal_on_demand_price", self.base),
        )
    }

    #[doc =
        "Get a reference to the value of field `on_demand_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn on_demand_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.on_demand_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `require_hibernate_support` after provisioning.\n"]
    pub fn require_hibernate_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.require_hibernate_support", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_max_price_percentage_over_lowest_price` after provisioning.\n"]
    pub fn spot_max_price_percentage_over_lowest_price(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.spot_max_price_percentage_over_lowest_price", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_AcceleratorCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_AcceleratorTotalMemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_BaselineEbsBandwidthMbpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_MemoryGibPerVcpuRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_MemoryMibRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_bandwidth_gbps` after provisioning.\n"]
    pub fn network_bandwidth_gbps(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_NetworkBandwidthGbpsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_bandwidth_gbps", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_NetworkInterfaceCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_TotalLocalStorageGbRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(&self) -> terrars::ListRef<LaunchTemplate_InstanceRequirements_VcpuCountRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_LicenseSpecification {
    license_configuration_arn: terrars::PrimField<std::string::String>,
}

impl LaunchTemplate_LicenseSpecification { }

impl terrars::ToListMappable for LaunchTemplate_LicenseSpecification {
    type O = terrars::BlockAssignable<LaunchTemplate_LicenseSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_LicenseSpecification {
    #[doc = ""]
    pub license_configuration_arn: terrars::PrimField<std::string::String>,
}

impl BuildLaunchTemplate_LicenseSpecification {
    pub fn build(self) -> LaunchTemplate_LicenseSpecification {
        LaunchTemplate_LicenseSpecification { license_configuration_arn: self.license_configuration_arn }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_LicenseSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_LicenseSpecificationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_LicenseSpecificationRef {
        LaunchTemplate_LicenseSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_LicenseSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_configuration_arn` after provisioning.\n"]
    pub fn license_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_configuration_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_MaintenanceOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_recovery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_MaintenanceOptions {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_recovery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_MaintenanceOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_MaintenanceOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_MaintenanceOptions {}

impl BuildLaunchTemplate_MaintenanceOptions {
    pub fn build(self) -> LaunchTemplate_MaintenanceOptions {
        LaunchTemplate_MaintenanceOptions { auto_recovery: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_MaintenanceOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_MaintenanceOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_MaintenanceOptionsRef {
        LaunchTemplate_MaintenanceOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_MaintenanceOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_MetadataOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_protocol_ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_put_response_hop_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_tokens: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_metadata_tags: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_MetadataOptions {
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_protocol_ipv6`.\n"]
    pub fn set_http_protocol_ipv6(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.http_protocol_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_put_response_hop_limit`.\n"]
    pub fn set_http_put_response_hop_limit(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.http_put_response_hop_limit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_tokens`.\n"]
    pub fn set_http_tokens(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_tokens = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_metadata_tags`.\n"]
    pub fn set_instance_metadata_tags(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.instance_metadata_tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_MetadataOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_MetadataOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_MetadataOptions {}

impl BuildLaunchTemplate_MetadataOptions {
    pub fn build(self) -> LaunchTemplate_MetadataOptions {
        LaunchTemplate_MetadataOptions {
            http_endpoint: core::default::Default::default(),
            http_protocol_ipv6: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_MetadataOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_MetadataOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_MetadataOptionsRef {
        LaunchTemplate_MetadataOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_MetadataOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `http_protocol_ipv6` after provisioning.\n"]
    pub fn http_protocol_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_protocol_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `http_put_response_hop_limit` after provisioning.\n"]
    pub fn http_put_response_hop_limit(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_put_response_hop_limit", self.base))
    }

    #[doc = "Get a reference to the value of field `http_tokens` after provisioning.\n"]
    pub fn http_tokens(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_tokens", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_metadata_tags` after provisioning.\n"]
    pub fn instance_metadata_tags(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_metadata_tags", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_Monitoring {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LaunchTemplate_Monitoring {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_Monitoring {
    type O = terrars::BlockAssignable<LaunchTemplate_Monitoring>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_Monitoring {}

impl BuildLaunchTemplate_Monitoring {
    pub fn build(self) -> LaunchTemplate_Monitoring {
        LaunchTemplate_Monitoring { enabled: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_MonitoringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_MonitoringRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_MonitoringRef {
        LaunchTemplate_MonitoringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_MonitoringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_established_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_stream_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    udp_timeout: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {
    #[doc = "Set the field `tcp_established_timeout`.\n"]
    pub fn set_tcp_established_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_established_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udp_stream_timeout`.\n"]
    pub fn set_udp_stream_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.udp_stream_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `udp_timeout`.\n"]
    pub fn set_udp_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.udp_timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {
    type O = terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {}

impl BuildLaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {
    pub fn build(self) -> LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {
        LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification {
            tcp_established_timeout: core::default::Default::default(),
            udp_stream_timeout: core::default::Default::default(),
            udp_timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecificationRef {
        LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tcp_established_timeout` after provisioning.\n"]
    pub fn tcp_established_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_established_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_stream_timeout` after provisioning.\n"]
    pub fn udp_stream_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_stream_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `udp_timeout` after provisioning.\n"]
    pub fn udp_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.udp_timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_udp_enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {
    #[doc = "Set the field `ena_srd_udp_enabled`.\n"]
    pub fn set_ena_srd_udp_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ena_srd_udp_enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {
    type O = terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {}

impl BuildLaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {
    pub fn build(self) -> LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {
        LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification {
            ena_srd_udp_enabled: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
        LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ena_srd_udp_enabled` after provisioning.\n"]
    pub fn ena_srd_udp_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_srd_udp_enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationDynamic {
    ena_srd_udp_specification: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_NetworkInterfaces_EnaSrdSpecification {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_udp_specification: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification>>,
    dynamic: LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationDynamic,
}

impl LaunchTemplate_NetworkInterfaces_EnaSrdSpecification {
    #[doc = "Set the field `ena_srd_enabled`.\n"]
    pub fn set_ena_srd_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.ena_srd_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ena_srd_udp_specification`.\n"]
    pub fn set_ena_srd_udp_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ena_srd_udp_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ena_srd_udp_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_NetworkInterfaces_EnaSrdSpecification {
    type O = terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_NetworkInterfaces_EnaSrdSpecification {}

impl BuildLaunchTemplate_NetworkInterfaces_EnaSrdSpecification {
    pub fn build(self) -> LaunchTemplate_NetworkInterfaces_EnaSrdSpecification {
        LaunchTemplate_NetworkInterfaces_EnaSrdSpecification {
            ena_srd_enabled: core::default::Default::default(),
            ena_srd_udp_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationRef {
        LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ena_srd_enabled` after provisioning.\n"]
    pub fn ena_srd_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_srd_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `ena_srd_udp_specification` after provisioning.\n"]
    pub fn ena_srd_udp_specification(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification_EnaSrdUdpSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ena_srd_udp_specification", self.base))
    }
}

#[derive(Serialize, Default)]
struct LaunchTemplate_NetworkInterfacesDynamic {
    connection_tracking_specification: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification>>,
    ena_srd_specification: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification>>,
}

#[derive(Serialize)]
pub struct LaunchTemplate_NetworkInterfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associate_carrier_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associate_public_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_queue_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_prefix_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv4_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_prefix_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_prefixes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_card_index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    primary_ipv6: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    connection_tracking_specification: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ena_srd_specification: std::option::Option<std::vec::Vec<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification>>,
    dynamic: LaunchTemplate_NetworkInterfacesDynamic,
}

impl LaunchTemplate_NetworkInterfaces {
    #[doc = "Set the field `associate_carrier_ip_address`.\n"]
    pub fn set_associate_carrier_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.associate_carrier_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `associate_public_ip_address`.\n"]
    pub fn set_associate_public_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.associate_public_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_index`.\n"]
    pub fn set_device_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.device_index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ena_queue_count`.\n"]
    pub fn set_ena_queue_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ena_queue_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_type`.\n"]
    pub fn set_interface_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_address_count`.\n"]
    pub fn set_ipv4_address_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ipv4_address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_addresses`.\n"]
    pub fn set_ipv4_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_prefix_count`.\n"]
    pub fn set_ipv4_prefix_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ipv4_prefix_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv4_prefixes`.\n"]
    pub fn set_ipv4_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv4_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_count`.\n"]
    pub fn set_ipv6_address_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ipv6_address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_addresses`.\n"]
    pub fn set_ipv6_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv6_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_prefix_count`.\n"]
    pub fn set_ipv6_prefix_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.ipv6_prefix_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_prefixes`.\n"]
    pub fn set_ipv6_prefixes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.ipv6_prefixes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.network_card_index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_interface_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `primary_ipv6`.\n"]
    pub fn set_primary_ipv6(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.primary_ipv6 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address`.\n"]
    pub fn set_private_ip_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.private_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `connection_tracking_specification`.\n"]
    pub fn set_connection_tracking_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecification>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.connection_tracking_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.connection_tracking_specification = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ena_srd_specification`.\n"]
    pub fn set_ena_srd_specification(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces_EnaSrdSpecification>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ena_srd_specification = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ena_srd_specification = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_NetworkInterfaces {
    type O = terrars::BlockAssignable<LaunchTemplate_NetworkInterfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_NetworkInterfaces {}

impl BuildLaunchTemplate_NetworkInterfaces {
    pub fn build(self) -> LaunchTemplate_NetworkInterfaces {
        LaunchTemplate_NetworkInterfaces {
            associate_carrier_ip_address: core::default::Default::default(),
            associate_public_ip_address: core::default::Default::default(),
            delete_on_termination: core::default::Default::default(),
            description: core::default::Default::default(),
            device_index: core::default::Default::default(),
            ena_queue_count: core::default::Default::default(),
            interface_type: core::default::Default::default(),
            ipv4_address_count: core::default::Default::default(),
            ipv4_addresses: core::default::Default::default(),
            ipv4_prefix_count: core::default::Default::default(),
            ipv4_prefixes: core::default::Default::default(),
            ipv6_address_count: core::default::Default::default(),
            ipv6_addresses: core::default::Default::default(),
            ipv6_prefix_count: core::default::Default::default(),
            ipv6_prefixes: core::default::Default::default(),
            network_card_index: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            primary_ipv6: core::default::Default::default(),
            private_ip_address: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
            connection_tracking_specification: core::default::Default::default(),
            ena_srd_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_NetworkInterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_NetworkInterfacesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_NetworkInterfacesRef {
        LaunchTemplate_NetworkInterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_NetworkInterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `associate_carrier_ip_address` after provisioning.\n"]
    pub fn associate_carrier_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_carrier_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.base))
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `ena_queue_count` after provisioning.\n"]
    pub fn ena_queue_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_queue_count", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_type", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_address_count` after provisioning.\n"]
    pub fn ipv4_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_address_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_addresses` after provisioning.\n"]
    pub fn ipv4_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv4_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_prefix_count` after provisioning.\n"]
    pub fn ipv4_prefix_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv4_prefix_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv4_prefixes` after provisioning.\n"]
    pub fn ipv4_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv4_prefixes", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_prefix_count` after provisioning.\n"]
    pub fn ipv6_prefix_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_prefix_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ipv6_prefixes` after provisioning.\n"]
    pub fn ipv6_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_prefixes", self.base))
    }

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_card_index", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `primary_ipv6` after provisioning.\n"]
    pub fn primary_ipv6(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_ipv6", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address` after provisioning.\n"]
    pub fn private_ip_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }

    #[doc = "Get a reference to the value of field `connection_tracking_specification` after provisioning.\n"]
    pub fn connection_tracking_specification(
        &self,
    ) -> terrars::ListRef<LaunchTemplate_NetworkInterfaces_ConnectionTrackingSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.connection_tracking_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `ena_srd_specification` after provisioning.\n"]
    pub fn ena_srd_specification(&self) -> terrars::ListRef<LaunchTemplate_NetworkInterfaces_EnaSrdSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ena_srd_specification", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_NetworkPerformanceOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bandwidth_weighting: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_NetworkPerformanceOptions {
    #[doc = "Set the field `bandwidth_weighting`.\n"]
    pub fn set_bandwidth_weighting(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.bandwidth_weighting = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_NetworkPerformanceOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_NetworkPerformanceOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_NetworkPerformanceOptions {}

impl BuildLaunchTemplate_NetworkPerformanceOptions {
    pub fn build(self) -> LaunchTemplate_NetworkPerformanceOptions {
        LaunchTemplate_NetworkPerformanceOptions { bandwidth_weighting: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_NetworkPerformanceOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_NetworkPerformanceOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_NetworkPerformanceOptionsRef {
        LaunchTemplate_NetworkPerformanceOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_NetworkPerformanceOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bandwidth_weighting` after provisioning.\n"]
    pub fn bandwidth_weighting(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bandwidth_weighting", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_Placement {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    group_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_resource_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    partition_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spread_domain: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_Placement {
    #[doc = "Set the field `affinity`.\n"]
    pub fn set_affinity(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.affinity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_id`.\n"]
    pub fn set_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `group_name`.\n"]
    pub fn set_group_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.group_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.host_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_resource_group_arn`.\n"]
    pub fn set_host_resource_group_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.host_resource_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `partition_number`.\n"]
    pub fn set_partition_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.partition_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spread_domain`.\n"]
    pub fn set_spread_domain(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.spread_domain = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.tenancy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_Placement {
    type O = terrars::BlockAssignable<LaunchTemplate_Placement>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_Placement {}

impl BuildLaunchTemplate_Placement {
    pub fn build(self) -> LaunchTemplate_Placement {
        LaunchTemplate_Placement {
            affinity: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            group_id: core::default::Default::default(),
            group_name: core::default::Default::default(),
            host_id: core::default::Default::default(),
            host_resource_group_arn: core::default::Default::default(),
            partition_number: core::default::Default::default(),
            spread_domain: core::default::Default::default(),
            tenancy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_PlacementRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_PlacementRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_PlacementRef {
        LaunchTemplate_PlacementRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_PlacementRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `affinity` after provisioning.\n"]
    pub fn affinity(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.affinity", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `group_id` after provisioning.\n"]
    pub fn group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_id", self.base))
    }

    #[doc = "Get a reference to the value of field `group_name` after provisioning.\n"]
    pub fn group_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_name", self.base))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.base))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `partition_number` after provisioning.\n"]
    pub fn partition_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.partition_number", self.base))
    }

    #[doc = "Get a reference to the value of field `spread_domain` after provisioning.\n"]
    pub fn spread_domain(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spread_domain", self.base))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_PrivateDnsNameOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_a_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_aaaa_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_PrivateDnsNameOptions {
    #[doc = "Set the field `enable_resource_name_dns_a_record`.\n"]
    pub fn set_enable_resource_name_dns_a_record(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_resource_name_dns_a_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_aaaa_record`.\n"]
    pub fn set_enable_resource_name_dns_aaaa_record(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.enable_resource_name_dns_aaaa_record = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hostname_type`.\n"]
    pub fn set_hostname_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.hostname_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_PrivateDnsNameOptions {
    type O = terrars::BlockAssignable<LaunchTemplate_PrivateDnsNameOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_PrivateDnsNameOptions {}

impl BuildLaunchTemplate_PrivateDnsNameOptions {
    pub fn build(self) -> LaunchTemplate_PrivateDnsNameOptions {
        LaunchTemplate_PrivateDnsNameOptions {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_PrivateDnsNameOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_PrivateDnsNameOptionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_PrivateDnsNameOptionsRef {
        LaunchTemplate_PrivateDnsNameOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_PrivateDnsNameOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_name_dns_a_record", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_aaaa_record` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_resource_name_dns_aaaa_record", self.base))
    }

    #[doc = "Get a reference to the value of field `hostname_type` after provisioning.\n"]
    pub fn hostname_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hostname_type", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_SecondaryInterfaces {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interface_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_card_index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip_addresses: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplate_SecondaryInterfaces {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_index`.\n"]
    pub fn set_device_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.device_index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interface_type`.\n"]
    pub fn set_interface_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interface_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.network_card_index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_address_count`.\n"]
    pub fn set_private_ip_address_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.private_ip_address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip_addresses`.\n"]
    pub fn set_private_ip_addresses(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.private_ip_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_subnet_id`.\n"]
    pub fn set_secondary_subnet_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.secondary_subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_SecondaryInterfaces {
    type O = terrars::BlockAssignable<LaunchTemplate_SecondaryInterfaces>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_SecondaryInterfaces {}

impl BuildLaunchTemplate_SecondaryInterfaces {
    pub fn build(self) -> LaunchTemplate_SecondaryInterfaces {
        LaunchTemplate_SecondaryInterfaces {
            delete_on_termination: core::default::Default::default(),
            device_index: core::default::Default::default(),
            interface_type: core::default::Default::default(),
            network_card_index: core::default::Default::default(),
            private_ip_address_count: core::default::Default::default(),
            private_ip_addresses: core::default::Default::default(),
            secondary_subnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_SecondaryInterfacesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_SecondaryInterfacesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_SecondaryInterfacesRef {
        LaunchTemplate_SecondaryInterfacesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_SecondaryInterfacesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `delete_on_termination` after provisioning.\n"]
    pub fn delete_on_termination(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_on_termination", self.base))
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_type", self.base))
    }

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_card_index", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_address_count` after provisioning.\n"]
    pub fn private_ip_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip_address_count", self.base))
    }

    #[doc = "Get a reference to the value of field `private_ip_addresses` after provisioning.\n"]
    pub fn private_ip_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.private_ip_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `secondary_subnet_id` after provisioning.\n"]
    pub fn secondary_subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secondary_subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplate_TagSpecifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl LaunchTemplate_TagSpecifications {
    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplate_TagSpecifications {
    type O = terrars::BlockAssignable<LaunchTemplate_TagSpecifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplate_TagSpecifications {}

impl BuildLaunchTemplate_TagSpecifications {
    pub fn build(self) -> LaunchTemplate_TagSpecifications {
        LaunchTemplate_TagSpecifications {
            resource_type: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct LaunchTemplate_TagSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplate_TagSpecificationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplate_TagSpecificationsRef {
        LaunchTemplate_TagSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplate_TagSpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
