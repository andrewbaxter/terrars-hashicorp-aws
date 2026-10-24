use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct LaunchTemplateData {
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
    block_device_mappings: std::option::Option<std::vec::Vec<LaunchTemplateBlockDeviceMappingsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_specification: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplateCapacityReservationSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_options: std::option::Option<std::vec::Vec<LaunchTemplateCpuOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credit_specification: std::option::Option<std::vec::Vec<LaunchTemplateCreditSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_gpu_specifications: std::option::Option<std::vec::Vec<LaunchTemplateElasticGpuSpecificationsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_inference_accelerator: std::option::Option<std::vec::Vec<LaunchTemplateElasticInferenceAcceleratorEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enclave_options: std::option::Option<std::vec::Vec<LaunchTemplateEnclaveOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hibernation_options: std::option::Option<std::vec::Vec<LaunchTemplateHibernationOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_instance_profile: std::option::Option<std::vec::Vec<LaunchTemplateIamInstanceProfileEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_market_options: std::option::Option<std::vec::Vec<LaunchTemplateInstanceMarketOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_requirements: std::option::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_specification: std::option::Option<std::vec::Vec<LaunchTemplateLicenseSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_options: std::option::Option<std::vec::Vec<LaunchTemplateMaintenanceOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_options: std::option::Option<std::vec::Vec<LaunchTemplateMetadataOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<std::vec::Vec<LaunchTemplateMonitoringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interfaces: std::option::Option<std::vec::Vec<LaunchTemplateNetworkInterfacesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement: std::option::Option<std::vec::Vec<LaunchTemplatePlacementEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_dns_name_options: std::option::Option<std::vec::Vec<LaunchTemplatePrivateDnsNameOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_specifications: std::option::Option<std::vec::Vec<LaunchTemplateTagSpecificationsEl>>,
    dynamic: LaunchTemplateDynamic,
}

struct LaunchTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<LaunchTemplateData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateBlockDeviceMappingsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateCapacityReservationSpecificationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateCpuOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateCreditSpecificationEl>>,
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

    #[doc = "Set the field `elastic_gpu_specifications`.\n"]
    pub fn set_elastic_gpu_specifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateElasticGpuSpecificationsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elastic_gpu_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elastic_gpu_specifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `elastic_inference_accelerator`.\n"]
    pub fn set_elastic_inference_accelerator(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateElasticInferenceAcceleratorEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().elastic_inference_accelerator = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.elastic_inference_accelerator = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `enclave_options`.\n"]
    pub fn set_enclave_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateEnclaveOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateHibernationOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateIamInstanceProfileEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceMarketOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateLicenseSpecificationEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateMaintenanceOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateMetadataOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateMonitoringEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateNetworkInterfacesEl>>,
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

    #[doc = "Set the field `placement`.\n"]
    pub fn set_placement(self, v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplatePlacementEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplatePrivateDnsNameOptionsEl>>,
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

    #[doc = "Set the field `tag_specifications`.\n"]
    pub fn set_tag_specifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateTagSpecificationsEl>>,
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
    pub fn block_device_mappings(&self) -> terrars::ListRef<LaunchTemplateBlockDeviceMappingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_device_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<LaunchTemplateCapacityReservationSpecificationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> terrars::ListRef<LaunchTemplateCpuOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<LaunchTemplateCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_gpu_specifications` after provisioning.\n"]
    pub fn elastic_gpu_specifications(&self) -> terrars::ListRef<LaunchTemplateElasticGpuSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_gpu_specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_inference_accelerator` after provisioning.\n"]
    pub fn elastic_inference_accelerator(&self) -> terrars::ListRef<LaunchTemplateElasticInferenceAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_inference_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<LaunchTemplateEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> terrars::ListRef<LaunchTemplateHibernationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hibernation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::ListRef<LaunchTemplateIamInstanceProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> terrars::ListRef<LaunchTemplateInstanceMarketOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_market_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<LaunchTemplateMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<LaunchTemplateMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<LaunchTemplateMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<LaunchTemplateNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<LaunchTemplatePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<LaunchTemplatePrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> terrars::ListRef<LaunchTemplateTagSpecificationsElRef> {
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
            data: std::cell::RefCell::new(LaunchTemplateData {
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
                elastic_gpu_specifications: core::default::Default::default(),
                elastic_inference_accelerator: core::default::Default::default(),
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
                placement: core::default::Default::default(),
                private_dns_name_options: core::default::Default::default(),
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
    pub fn block_device_mappings(&self) -> terrars::ListRef<LaunchTemplateBlockDeviceMappingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_device_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<LaunchTemplateCapacityReservationSpecificationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> terrars::ListRef<LaunchTemplateCpuOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<LaunchTemplateCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_gpu_specifications` after provisioning.\n"]
    pub fn elastic_gpu_specifications(&self) -> terrars::ListRef<LaunchTemplateElasticGpuSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_gpu_specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_inference_accelerator` after provisioning.\n"]
    pub fn elastic_inference_accelerator(&self) -> terrars::ListRef<LaunchTemplateElasticInferenceAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_inference_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<LaunchTemplateEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> terrars::ListRef<LaunchTemplateHibernationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hibernation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::ListRef<LaunchTemplateIamInstanceProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> terrars::ListRef<LaunchTemplateInstanceMarketOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_market_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<LaunchTemplateMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<LaunchTemplateMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<LaunchTemplateMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<LaunchTemplateNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<LaunchTemplatePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<LaunchTemplatePrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> terrars::ListRef<LaunchTemplateTagSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_specifications", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateBlockDeviceMappingsElEbsEl {
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
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplateBlockDeviceMappingsElEbsEl {
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

impl terrars::ToListMappable for LaunchTemplateBlockDeviceMappingsElEbsEl {
    type O = terrars::BlockAssignable<LaunchTemplateBlockDeviceMappingsElEbsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateBlockDeviceMappingsElEbsEl {}

impl BuildLaunchTemplateBlockDeviceMappingsElEbsEl {
    pub fn build(self) -> LaunchTemplateBlockDeviceMappingsElEbsEl {
        LaunchTemplateBlockDeviceMappingsElEbsEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateBlockDeviceMappingsElEbsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateBlockDeviceMappingsElEbsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateBlockDeviceMappingsElEbsElRef {
        LaunchTemplateBlockDeviceMappingsElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateBlockDeviceMappingsElEbsElRef {
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
struct LaunchTemplateBlockDeviceMappingsElDynamic {
    ebs: std::option::Option<terrars::DynamicBlock<LaunchTemplateBlockDeviceMappingsElEbsEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateBlockDeviceMappingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs: std::option::Option<std::vec::Vec<LaunchTemplateBlockDeviceMappingsElEbsEl>>,
    dynamic: LaunchTemplateBlockDeviceMappingsElDynamic,
}

impl LaunchTemplateBlockDeviceMappingsEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateBlockDeviceMappingsElEbsEl>>,
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

impl terrars::ToListMappable for LaunchTemplateBlockDeviceMappingsEl {
    type O = terrars::BlockAssignable<LaunchTemplateBlockDeviceMappingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateBlockDeviceMappingsEl {}

impl BuildLaunchTemplateBlockDeviceMappingsEl {
    pub fn build(self) -> LaunchTemplateBlockDeviceMappingsEl {
        LaunchTemplateBlockDeviceMappingsEl {
            device_name: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateBlockDeviceMappingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateBlockDeviceMappingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateBlockDeviceMappingsElRef {
        LaunchTemplateBlockDeviceMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateBlockDeviceMappingsElRef {
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
    pub fn ebs(&self) -> terrars::ListRef<LaunchTemplateBlockDeviceMappingsElEbsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_resource_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
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

impl terrars::ToListMappable for LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    type O = terrars::BlockAssignable<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(self) -> LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
        LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
        LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
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
struct LaunchTemplateCapacityReservationSpecificationElDynamic {
    capacity_reservation_target: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_target: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>>,
    dynamic: LaunchTemplateCapacityReservationSpecificationElDynamic,
}

impl LaunchTemplateCapacityReservationSpecificationEl {
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
                        ::BlockAssignable<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>,
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

impl terrars::ToListMappable for LaunchTemplateCapacityReservationSpecificationEl {
    type O = terrars::BlockAssignable<LaunchTemplateCapacityReservationSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCapacityReservationSpecificationEl {}

impl BuildLaunchTemplateCapacityReservationSpecificationEl {
    pub fn build(self) -> LaunchTemplateCapacityReservationSpecificationEl {
        LaunchTemplateCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateCapacityReservationSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateCapacityReservationSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateCapacityReservationSpecificationElRef {
        LaunchTemplateCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCapacityReservationSpecificationElRef {
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
    ) -> terrars::ListRef<LaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_target", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateCpuOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateCpuOptionsEl {
    #[doc = "Set the field `core_count`.\n"]
    pub fn set_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `threads_per_core`.\n"]
    pub fn set_threads_per_core(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.threads_per_core = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateCpuOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateCpuOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCpuOptionsEl {}

impl BuildLaunchTemplateCpuOptionsEl {
    pub fn build(self) -> LaunchTemplateCpuOptionsEl {
        LaunchTemplateCpuOptionsEl {
            core_count: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateCpuOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateCpuOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateCpuOptionsElRef {
        LaunchTemplateCpuOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCpuOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `core_count` after provisioning.\n"]
    pub fn core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `threads_per_core` after provisioning.\n"]
    pub fn threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threads_per_core", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateCreditSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_credits: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplateCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_credits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateCreditSpecificationEl {
    type O = terrars::BlockAssignable<LaunchTemplateCreditSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateCreditSpecificationEl {}

impl BuildLaunchTemplateCreditSpecificationEl {
    pub fn build(self) -> LaunchTemplateCreditSpecificationEl {
        LaunchTemplateCreditSpecificationEl { cpu_credits: core::default::Default::default() }
    }
}

pub struct LaunchTemplateCreditSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateCreditSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateCreditSpecificationElRef {
        LaunchTemplateCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateCreditSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateElasticGpuSpecificationsEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl LaunchTemplateElasticGpuSpecificationsEl { }

impl terrars::ToListMappable for LaunchTemplateElasticGpuSpecificationsEl {
    type O = terrars::BlockAssignable<LaunchTemplateElasticGpuSpecificationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateElasticGpuSpecificationsEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLaunchTemplateElasticGpuSpecificationsEl {
    pub fn build(self) -> LaunchTemplateElasticGpuSpecificationsEl {
        LaunchTemplateElasticGpuSpecificationsEl { type_: self.type_ }
    }
}

pub struct LaunchTemplateElasticGpuSpecificationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateElasticGpuSpecificationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateElasticGpuSpecificationsElRef {
        LaunchTemplateElasticGpuSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateElasticGpuSpecificationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateElasticInferenceAcceleratorEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl LaunchTemplateElasticInferenceAcceleratorEl { }

impl terrars::ToListMappable for LaunchTemplateElasticInferenceAcceleratorEl {
    type O = terrars::BlockAssignable<LaunchTemplateElasticInferenceAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateElasticInferenceAcceleratorEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildLaunchTemplateElasticInferenceAcceleratorEl {
    pub fn build(self) -> LaunchTemplateElasticInferenceAcceleratorEl {
        LaunchTemplateElasticInferenceAcceleratorEl { type_: self.type_ }
    }
}

pub struct LaunchTemplateElasticInferenceAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateElasticInferenceAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateElasticInferenceAcceleratorElRef {
        LaunchTemplateElasticInferenceAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateElasticInferenceAcceleratorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateEnclaveOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LaunchTemplateEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateEnclaveOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateEnclaveOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateEnclaveOptionsEl {}

impl BuildLaunchTemplateEnclaveOptionsEl {
    pub fn build(self) -> LaunchTemplateEnclaveOptionsEl {
        LaunchTemplateEnclaveOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct LaunchTemplateEnclaveOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateEnclaveOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateEnclaveOptionsElRef {
        LaunchTemplateEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateEnclaveOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateHibernationOptionsEl {
    configured: terrars::PrimField<bool>,
}

impl LaunchTemplateHibernationOptionsEl { }

impl terrars::ToListMappable for LaunchTemplateHibernationOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateHibernationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateHibernationOptionsEl {
    #[doc = ""]
    pub configured: terrars::PrimField<bool>,
}

impl BuildLaunchTemplateHibernationOptionsEl {
    pub fn build(self) -> LaunchTemplateHibernationOptionsEl {
        LaunchTemplateHibernationOptionsEl { configured: self.configured }
    }
}

pub struct LaunchTemplateHibernationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateHibernationOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateHibernationOptionsElRef {
        LaunchTemplateHibernationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateHibernationOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configured` after provisioning.\n"]
    pub fn configured(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configured", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateIamInstanceProfileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplateIamInstanceProfileEl {
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

impl terrars::ToListMappable for LaunchTemplateIamInstanceProfileEl {
    type O = terrars::BlockAssignable<LaunchTemplateIamInstanceProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateIamInstanceProfileEl {}

impl BuildLaunchTemplateIamInstanceProfileEl {
    pub fn build(self) -> LaunchTemplateIamInstanceProfileEl {
        LaunchTemplateIamInstanceProfileEl {
            arn: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateIamInstanceProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateIamInstanceProfileElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateIamInstanceProfileElRef {
        LaunchTemplateIamInstanceProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateIamInstanceProfileElRef {
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
pub struct LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
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

impl LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {}

impl BuildLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    pub fn build(self) -> LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
        LaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
            block_duration_minutes: core::default::Default::default(),
            instance_interruption_behavior: core::default::Default::default(),
            max_price: core::default::Default::default(),
            spot_instance_type: core::default::Default::default(),
            valid_until: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
        LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
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
struct LaunchTemplateInstanceMarketOptionsElDynamic {
    spot_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceMarketOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    market_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_options: std::option::Option<std::vec::Vec<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
    dynamic: LaunchTemplateInstanceMarketOptionsElDynamic,
}

impl LaunchTemplateInstanceMarketOptionsEl {
    #[doc = "Set the field `market_type`.\n"]
    pub fn set_market_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.market_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
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

impl terrars::ToListMappable for LaunchTemplateInstanceMarketOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceMarketOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceMarketOptionsEl {}

impl BuildLaunchTemplateInstanceMarketOptionsEl {
    pub fn build(self) -> LaunchTemplateInstanceMarketOptionsEl {
        LaunchTemplateInstanceMarketOptionsEl {
            market_type: core::default::Default::default(),
            spot_options: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceMarketOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceMarketOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateInstanceMarketOptionsElRef {
        LaunchTemplateInstanceMarketOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceMarketOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `market_type` after provisioning.\n"]
    pub fn market_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.market_type", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> terrars::ListRef<LaunchTemplateInstanceMarketOptionsElSpotOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_options", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElAcceleratorCountEl {}

impl BuildLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
        LaunchTemplateInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
        LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
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
pub struct LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {}

impl BuildLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
        LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
        LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
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
pub struct LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {}

impl BuildLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
        LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
        LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
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
pub struct LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {}

impl BuildLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
        LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
        LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
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
pub struct LaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl LaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElMemoryMibEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryMibEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildLaunchTemplateInstanceRequirementsElMemoryMibEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElMemoryMibEl {
        LaunchTemplateInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElMemoryMibElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElMemoryMibElRef {
        LaunchTemplateInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElMemoryMibElRef {
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
pub struct LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {}

impl BuildLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
        LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
        LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
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
pub struct LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {}

impl BuildLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
        LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
        LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
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
pub struct LaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    min: terrars::PrimField<f64>,
}

impl LaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[doc = "Set the field `max`.\n"]
    pub fn set_max(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsElVcpuCountEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElVcpuCountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[doc = ""]
    pub min: terrars::PrimField<f64>,
}

impl BuildLaunchTemplateInstanceRequirementsElVcpuCountEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsElVcpuCountEl {
        LaunchTemplateInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: self.min,
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElVcpuCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> LaunchTemplateInstanceRequirementsElVcpuCountElRef {
        LaunchTemplateInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElVcpuCountElRef {
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
struct LaunchTemplateInstanceRequirementsElDynamic {
    accelerator_count: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    accelerator_total_memory_mib: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
    memory_gib_per_vcpu: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    memory_mib: std::option::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    network_interface_count: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    total_local_storage_gb: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    vcpu_count: std::option::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsElVcpuCountEl>>,
}

#[derive(Serialize)]
pub struct LaunchTemplateInstanceRequirementsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
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
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std::option::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_gib_per_vcpu: std::option::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std::option::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std::option::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std::option::Option<std::vec::Vec<LaunchTemplateInstanceRequirementsElVcpuCountEl>>,
    dynamic: LaunchTemplateInstanceRequirementsElDynamic,
}

impl LaunchTemplateInstanceRequirementsEl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
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
                    ::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
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
                    ::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElMemoryMibEl>>,
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

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
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
                    ::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<LaunchTemplateInstanceRequirementsElVcpuCountEl>>,
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

impl terrars::ToListMappable for LaunchTemplateInstanceRequirementsEl {
    type O = terrars::BlockAssignable<LaunchTemplateInstanceRequirementsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateInstanceRequirementsEl {}

impl BuildLaunchTemplateInstanceRequirementsEl {
    pub fn build(self) -> LaunchTemplateInstanceRequirementsEl {
        LaunchTemplateInstanceRequirementsEl {
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            accelerator_count: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateInstanceRequirementsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateInstanceRequirementsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateInstanceRequirementsElRef {
        LaunchTemplateInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateInstanceRequirementsElRef {
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
    pub fn accelerator_count(&self) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElAcceleratorCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(&self) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(&self) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElMemoryMibElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
    }

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(&self) -> terrars::ListRef<LaunchTemplateInstanceRequirementsElVcpuCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateLicenseSpecificationEl {
    license_configuration_arn: terrars::PrimField<std::string::String>,
}

impl LaunchTemplateLicenseSpecificationEl { }

impl terrars::ToListMappable for LaunchTemplateLicenseSpecificationEl {
    type O = terrars::BlockAssignable<LaunchTemplateLicenseSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateLicenseSpecificationEl {
    #[doc = ""]
    pub license_configuration_arn: terrars::PrimField<std::string::String>,
}

impl BuildLaunchTemplateLicenseSpecificationEl {
    pub fn build(self) -> LaunchTemplateLicenseSpecificationEl {
        LaunchTemplateLicenseSpecificationEl { license_configuration_arn: self.license_configuration_arn }
    }
}

pub struct LaunchTemplateLicenseSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateLicenseSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateLicenseSpecificationElRef {
        LaunchTemplateLicenseSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateLicenseSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_configuration_arn` after provisioning.\n"]
    pub fn license_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_configuration_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_recovery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplateMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_recovery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateMaintenanceOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateMaintenanceOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateMaintenanceOptionsEl {}

impl BuildLaunchTemplateMaintenanceOptionsEl {
    pub fn build(self) -> LaunchTemplateMaintenanceOptionsEl {
        LaunchTemplateMaintenanceOptionsEl { auto_recovery: core::default::Default::default() }
    }
}

pub struct LaunchTemplateMaintenanceOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateMaintenanceOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateMaintenanceOptionsElRef {
        LaunchTemplateMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateMaintenanceOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateMetadataOptionsEl {
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

impl LaunchTemplateMetadataOptionsEl {
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

impl terrars::ToListMappable for LaunchTemplateMetadataOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplateMetadataOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateMetadataOptionsEl {}

impl BuildLaunchTemplateMetadataOptionsEl {
    pub fn build(self) -> LaunchTemplateMetadataOptionsEl {
        LaunchTemplateMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_protocol_ipv6: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateMetadataOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateMetadataOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateMetadataOptionsElRef {
        LaunchTemplateMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateMetadataOptionsElRef {
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
pub struct LaunchTemplateMonitoringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl LaunchTemplateMonitoringEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for LaunchTemplateMonitoringEl {
    type O = terrars::BlockAssignable<LaunchTemplateMonitoringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateMonitoringEl {}

impl BuildLaunchTemplateMonitoringEl {
    pub fn build(self) -> LaunchTemplateMonitoringEl {
        LaunchTemplateMonitoringEl { enabled: core::default::Default::default() }
    }
}

pub struct LaunchTemplateMonitoringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateMonitoringElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateMonitoringElRef {
        LaunchTemplateMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateMonitoringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct LaunchTemplateNetworkInterfacesEl {
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
    private_ip_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplateNetworkInterfacesEl {
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
}

impl terrars::ToListMappable for LaunchTemplateNetworkInterfacesEl {
    type O = terrars::BlockAssignable<LaunchTemplateNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateNetworkInterfacesEl {}

impl BuildLaunchTemplateNetworkInterfacesEl {
    pub fn build(self) -> LaunchTemplateNetworkInterfacesEl {
        LaunchTemplateNetworkInterfacesEl {
            associate_carrier_ip_address: core::default::Default::default(),
            associate_public_ip_address: core::default::Default::default(),
            delete_on_termination: core::default::Default::default(),
            description: core::default::Default::default(),
            device_index: core::default::Default::default(),
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
            private_ip_address: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateNetworkInterfacesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateNetworkInterfacesElRef {
        LaunchTemplateNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateNetworkInterfacesElRef {
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
}

#[derive(Serialize)]
pub struct LaunchTemplatePlacementEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    affinity: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
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

impl LaunchTemplatePlacementEl {
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

impl terrars::ToListMappable for LaunchTemplatePlacementEl {
    type O = terrars::BlockAssignable<LaunchTemplatePlacementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplatePlacementEl {}

impl BuildLaunchTemplatePlacementEl {
    pub fn build(self) -> LaunchTemplatePlacementEl {
        LaunchTemplatePlacementEl {
            affinity: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
            group_name: core::default::Default::default(),
            host_id: core::default::Default::default(),
            host_resource_group_arn: core::default::Default::default(),
            partition_number: core::default::Default::default(),
            spread_domain: core::default::Default::default(),
            tenancy: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplatePlacementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplatePlacementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplatePlacementElRef {
        LaunchTemplatePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplatePlacementElRef {
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
pub struct LaunchTemplatePrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_a_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_aaaa_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl LaunchTemplatePrivateDnsNameOptionsEl {
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

impl terrars::ToListMappable for LaunchTemplatePrivateDnsNameOptionsEl {
    type O = terrars::BlockAssignable<LaunchTemplatePrivateDnsNameOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplatePrivateDnsNameOptionsEl {}

impl BuildLaunchTemplatePrivateDnsNameOptionsEl {
    pub fn build(self) -> LaunchTemplatePrivateDnsNameOptionsEl {
        LaunchTemplatePrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplatePrivateDnsNameOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplatePrivateDnsNameOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplatePrivateDnsNameOptionsElRef {
        LaunchTemplatePrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplatePrivateDnsNameOptionsElRef {
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
pub struct LaunchTemplateTagSpecificationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl LaunchTemplateTagSpecificationsEl {
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

impl terrars::ToListMappable for LaunchTemplateTagSpecificationsEl {
    type O = terrars::BlockAssignable<LaunchTemplateTagSpecificationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildLaunchTemplateTagSpecificationsEl {}

impl BuildLaunchTemplateTagSpecificationsEl {
    pub fn build(self) -> LaunchTemplateTagSpecificationsEl {
        LaunchTemplateTagSpecificationsEl {
            resource_type: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct LaunchTemplateTagSpecificationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for LaunchTemplateTagSpecificationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> LaunchTemplateTagSpecificationsElRef {
        LaunchTemplateTagSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl LaunchTemplateTagSpecificationsElRef {
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

#[derive(Serialize, Default)]
struct LaunchTemplateDynamic {
    block_device_mappings: std::option::Option<terrars::DynamicBlock<LaunchTemplateBlockDeviceMappingsEl>>,
    capacity_reservation_specification: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateCapacityReservationSpecificationEl>>,
    cpu_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateCpuOptionsEl>>,
    credit_specification: std::option::Option<terrars::DynamicBlock<LaunchTemplateCreditSpecificationEl>>,
    elastic_gpu_specifications: std::option::Option<terrars::DynamicBlock<LaunchTemplateElasticGpuSpecificationsEl>>,
    elastic_inference_accelerator: std
    ::option
    ::Option<terrars::DynamicBlock<LaunchTemplateElasticInferenceAcceleratorEl>>,
    enclave_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateEnclaveOptionsEl>>,
    hibernation_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateHibernationOptionsEl>>,
    iam_instance_profile: std::option::Option<terrars::DynamicBlock<LaunchTemplateIamInstanceProfileEl>>,
    instance_market_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateInstanceMarketOptionsEl>>,
    instance_requirements: std::option::Option<terrars::DynamicBlock<LaunchTemplateInstanceRequirementsEl>>,
    license_specification: std::option::Option<terrars::DynamicBlock<LaunchTemplateLicenseSpecificationEl>>,
    maintenance_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateMaintenanceOptionsEl>>,
    metadata_options: std::option::Option<terrars::DynamicBlock<LaunchTemplateMetadataOptionsEl>>,
    monitoring: std::option::Option<terrars::DynamicBlock<LaunchTemplateMonitoringEl>>,
    network_interfaces: std::option::Option<terrars::DynamicBlock<LaunchTemplateNetworkInterfacesEl>>,
    placement: std::option::Option<terrars::DynamicBlock<LaunchTemplatePlacementEl>>,
    private_dns_name_options: std::option::Option<terrars::DynamicBlock<LaunchTemplatePrivateDnsNameOptionsEl>>,
    tag_specifications: std::option::Option<terrars::DynamicBlock<LaunchTemplateTagSpecificationsEl>>,
}
