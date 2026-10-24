use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SpotInstanceRequestData {
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
    ami: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    associate_public_ip_address: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_duration_minutes: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_threads_per_core: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_api_stop: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    disable_api_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_optimized: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    get_password_data: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hibernation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_resource_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iam_instance_profile: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_initiated_shutdown_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_interruption_behavior: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_address_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    monitoring: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_group: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    placement_partition_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    secondary_private_ips: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_dest_check: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tenancy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data_base64: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_data_replace_on_change: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_from: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    valid_until: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_security_group_ids: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_fulfillment: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_specification: std
    ::option
    ::Option<std::vec::Vec<SpotInstanceRequestCapacityReservationSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    credit_specification: std::option::Option<std::vec::Vec<SpotInstanceRequestCreditSpecificationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_block_device: std::option::Option<std::vec::Vec<SpotInstanceRequestEbsBlockDeviceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enclave_options: std::option::Option<std::vec::Vec<SpotInstanceRequestEnclaveOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ephemeral_block_device: std::option::Option<std::vec::Vec<SpotInstanceRequestEphemeralBlockDeviceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_template: std::option::Option<std::vec::Vec<SpotInstanceRequestLaunchTemplateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maintenance_options: std::option::Option<std::vec::Vec<SpotInstanceRequestMaintenanceOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata_options: std::option::Option<std::vec::Vec<SpotInstanceRequestMetadataOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std::option::Option<std::vec::Vec<SpotInstanceRequestNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_dns_name_options: std::option::Option<std::vec::Vec<SpotInstanceRequestPrivateDnsNameOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    root_block_device: std::option::Option<std::vec::Vec<SpotInstanceRequestRootBlockDeviceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SpotInstanceRequestTimeoutsEl>,
    dynamic: SpotInstanceRequestDynamic,
}

struct SpotInstanceRequest_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SpotInstanceRequestData>,
}

#[derive(Clone)]
pub struct SpotInstanceRequest(std::rc::Rc<SpotInstanceRequest_>);

impl SpotInstanceRequest {
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

    #[doc = "Set the field `ami`.\n"]
    pub fn set_ami(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ami = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `associate_public_ip_address`.\n"]
    pub fn set_associate_public_ip_address(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().associate_public_ip_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().block_duration_minutes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_core_count`.\n"]
    pub fn set_cpu_core_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cpu_threads_per_core`.\n"]
    pub fn set_cpu_threads_per_core(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().cpu_threads_per_core = std::option::Option::Some(v.into());
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
    pub fn set_ebs_optimized(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ebs_optimized = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `get_password_data`.\n"]
    pub fn set_get_password_data(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().get_password_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hibernation`.\n"]
    pub fn set_hibernation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().hibernation = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_resource_group_arn`.\n"]
    pub fn set_host_resource_group_arn(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().host_resource_group_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `iam_instance_profile`.\n"]
    pub fn set_iam_instance_profile(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().iam_instance_profile = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `instance_interruption_behavior`.\n"]
    pub fn set_instance_interruption_behavior(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().instance_interruption_behavior = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_address_count`.\n"]
    pub fn set_ipv6_address_count(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().ipv6_address_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_addresses`.\n"]
    pub fn set_ipv6_addresses(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().ipv6_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key_name`.\n"]
    pub fn set_key_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().key_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `launch_group`.\n"]
    pub fn set_launch_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().launch_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `monitoring`.\n"]
    pub fn set_monitoring(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().monitoring = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_group`.\n"]
    pub fn set_placement_group(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().placement_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `placement_partition_number`.\n"]
    pub fn set_placement_partition_number(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().placement_partition_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_ip`.\n"]
    pub fn set_private_ip(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().private_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `secondary_private_ips`.\n"]
    pub fn set_secondary_private_ips(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().secondary_private_ips = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_dest_check`.\n"]
    pub fn set_source_dest_check(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().source_dest_check = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_price`.\n"]
    pub fn set_spot_price(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().spot_price = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_type`.\n"]
    pub fn set_spot_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().spot_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().subnet_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tenancy`.\n"]
    pub fn set_tenancy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tenancy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data`.\n"]
    pub fn set_user_data(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_data = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_base64`.\n"]
    pub fn set_user_data_base64(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().user_data_base64 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `user_data_replace_on_change`.\n"]
    pub fn set_user_data_replace_on_change(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().user_data_replace_on_change = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_from`.\n"]
    pub fn set_valid_from(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().valid_from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `valid_until`.\n"]
    pub fn set_valid_until(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().valid_until = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `volume_tags`.\n"]
    pub fn set_volume_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().volume_tags = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `wait_for_fulfillment`.\n"]
    pub fn set_wait_for_fulfillment(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_fulfillment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capacity_reservation_specification`.\n"]
    pub fn set_capacity_reservation_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestCapacityReservationSpecificationEl>>,
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

    #[doc = "Set the field `credit_specification`.\n"]
    pub fn set_credit_specification(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestCreditSpecificationEl>>,
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

    #[doc = "Set the field `ebs_block_device`.\n"]
    pub fn set_ebs_block_device(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestEbsBlockDeviceEl>>,
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

    #[doc = "Set the field `enclave_options`.\n"]
    pub fn set_enclave_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestEnclaveOptionsEl>>,
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

    #[doc = "Set the field `ephemeral_block_device`.\n"]
    pub fn set_ephemeral_block_device(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestEphemeralBlockDeviceEl>>,
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

    #[doc = "Set the field `launch_template`.\n"]
    pub fn set_launch_template(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestLaunchTemplateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_template = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_template = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `maintenance_options`.\n"]
    pub fn set_maintenance_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestMaintenanceOptionsEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestMetadataOptionsEl>>,
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

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestNetworkInterfaceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().network_interface = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.network_interface = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `private_dns_name_options`.\n"]
    pub fn set_private_dns_name_options(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestPrivateDnsNameOptionsEl>>,
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

    #[doc = "Set the field `root_block_device`.\n"]
    pub fn set_root_block_device(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SpotInstanceRequestRootBlockDeviceEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<SpotInstanceRequestTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_threads_per_core` after provisioning.\n"]
    pub fn cpu_threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_threads_per_core", self.extract_ref()))
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
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.get_password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation` after provisioning.\n"]
    pub fn hibernation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hibernation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_initiated_shutdown_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_group` after provisioning.\n"]
    pub fn launch_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_partition_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_network_interface_id` after provisioning.\n"]
    pub fn primary_network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_network_interface_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_bid_status` after provisioning.\n"]
    pub fn spot_bid_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_bid_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_instance_id` after provisioning.\n"]
    pub fn spot_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_request_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_type` after provisioning.\n"]
    pub fn spot_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_replace_on_change` after provisioning.\n"]
    pub fn user_data_replace_on_change(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_replace_on_change", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_tags` after provisioning.\n"]
    pub fn volume_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.volume_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<SpotInstanceRequestCapacityReservationSpecificationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<SpotInstanceRequestCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<SpotInstanceRequestEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<SpotInstanceRequestLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<SpotInstanceRequestMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<SpotInstanceRequestMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<SpotInstanceRequestPrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> terrars::ListRef<SpotInstanceRequestRootBlockDeviceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotInstanceRequestTimeoutsElRef {
        SpotInstanceRequestTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for SpotInstanceRequest {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SpotInstanceRequest { }

impl terrars::ToListMappable for SpotInstanceRequest {
    type O = terrars::ListRef<SpotInstanceRequestRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SpotInstanceRequest_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_spot_instance_request".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSpotInstanceRequest {
    pub tf_id: std::string::String,
}

impl BuildSpotInstanceRequest {
    pub fn build(self, stack: &mut terrars::Stack) -> SpotInstanceRequest {
        let out = SpotInstanceRequest(std::rc::Rc::new(SpotInstanceRequest_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SpotInstanceRequestData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                ami: core::default::Default::default(),
                associate_public_ip_address: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                block_duration_minutes: core::default::Default::default(),
                cpu_core_count: core::default::Default::default(),
                cpu_threads_per_core: core::default::Default::default(),
                disable_api_stop: core::default::Default::default(),
                disable_api_termination: core::default::Default::default(),
                ebs_optimized: core::default::Default::default(),
                get_password_data: core::default::Default::default(),
                hibernation: core::default::Default::default(),
                host_id: core::default::Default::default(),
                host_resource_group_arn: core::default::Default::default(),
                iam_instance_profile: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_initiated_shutdown_behavior: core::default::Default::default(),
                instance_interruption_behavior: core::default::Default::default(),
                instance_type: core::default::Default::default(),
                ipv6_address_count: core::default::Default::default(),
                ipv6_addresses: core::default::Default::default(),
                key_name: core::default::Default::default(),
                launch_group: core::default::Default::default(),
                monitoring: core::default::Default::default(),
                placement_group: core::default::Default::default(),
                placement_partition_number: core::default::Default::default(),
                private_ip: core::default::Default::default(),
                secondary_private_ips: core::default::Default::default(),
                security_groups: core::default::Default::default(),
                source_dest_check: core::default::Default::default(),
                spot_price: core::default::Default::default(),
                spot_type: core::default::Default::default(),
                subnet_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                tenancy: core::default::Default::default(),
                user_data: core::default::Default::default(),
                user_data_base64: core::default::Default::default(),
                user_data_replace_on_change: core::default::Default::default(),
                valid_from: core::default::Default::default(),
                valid_until: core::default::Default::default(),
                volume_tags: core::default::Default::default(),
                vpc_security_group_ids: core::default::Default::default(),
                wait_for_fulfillment: core::default::Default::default(),
                capacity_reservation_specification: core::default::Default::default(),
                credit_specification: core::default::Default::default(),
                ebs_block_device: core::default::Default::default(),
                enclave_options: core::default::Default::default(),
                ephemeral_block_device: core::default::Default::default(),
                launch_template: core::default::Default::default(),
                maintenance_options: core::default::Default::default(),
                metadata_options: core::default::Default::default(),
                network_interface: core::default::Default::default(),
                private_dns_name_options: core::default::Default::default(),
                root_block_device: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SpotInstanceRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SpotInstanceRequestRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ami` after provisioning.\n"]
    pub fn ami(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ami", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `associate_public_ip_address` after provisioning.\n"]
    pub fn associate_public_ip_address(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.associate_public_ip_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_duration_minutes` after provisioning.\n"]
    pub fn block_duration_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.block_duration_minutes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_core_count` after provisioning.\n"]
    pub fn cpu_core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_core_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cpu_threads_per_core` after provisioning.\n"]
    pub fn cpu_threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_threads_per_core", self.extract_ref()))
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
    pub fn ebs_optimized(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `get_password_data` after provisioning.\n"]
    pub fn get_password_data(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.get_password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation` after provisioning.\n"]
    pub fn hibernation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hibernation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_resource_group_arn` after provisioning.\n"]
    pub fn host_resource_group_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_resource_group_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_initiated_shutdown_behavior` after provisioning.\n"]
    pub fn instance_initiated_shutdown_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_initiated_shutdown_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_interruption_behavior` after provisioning.\n"]
    pub fn instance_interruption_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.instance_interruption_behavior", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `instance_state` after provisioning.\n"]
    pub fn instance_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_address_count` after provisioning.\n"]
    pub fn ipv6_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_group` after provisioning.\n"]
    pub fn launch_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.launch_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `password_data` after provisioning.\n"]
    pub fn password_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_group` after provisioning.\n"]
    pub fn placement_group(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_group", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement_partition_number` after provisioning.\n"]
    pub fn placement_partition_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.placement_partition_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `primary_network_interface_id` after provisioning.\n"]
    pub fn primary_network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.primary_network_interface_id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `secondary_private_ips` after provisioning.\n"]
    pub fn secondary_private_ips(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.secondary_private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_dest_check` after provisioning.\n"]
    pub fn source_dest_check(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_dest_check", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_bid_status` after provisioning.\n"]
    pub fn spot_bid_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_bid_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_instance_id` after provisioning.\n"]
    pub fn spot_instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_instance_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_price` after provisioning.\n"]
    pub fn spot_price(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_price", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_request_state` after provisioning.\n"]
    pub fn spot_request_state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_request_state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spot_type` after provisioning.\n"]
    pub fn spot_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.spot_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tenancy` after provisioning.\n"]
    pub fn tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_base64` after provisioning.\n"]
    pub fn user_data_base64(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_base64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data_replace_on_change` after provisioning.\n"]
    pub fn user_data_replace_on_change(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data_replace_on_change", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_from` after provisioning.\n"]
    pub fn valid_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_from", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_until` after provisioning.\n"]
    pub fn valid_until(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.valid_until", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `volume_tags` after provisioning.\n"]
    pub fn volume_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.volume_tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_fulfillment` after provisioning.\n"]
    pub fn wait_for_fulfillment(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_fulfillment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<SpotInstanceRequestCapacityReservationSpecificationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<SpotInstanceRequestCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<SpotInstanceRequestEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_template` after provisioning.\n"]
    pub fn launch_template(&self) -> terrars::ListRef<SpotInstanceRequestLaunchTemplateElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_template", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<SpotInstanceRequestMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<SpotInstanceRequestMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<SpotInstanceRequestPrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `root_block_device` after provisioning.\n"]
    pub fn root_block_device(&self) -> terrars::ListRef<SpotInstanceRequestRootBlockDeviceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.root_block_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SpotInstanceRequestTimeoutsElRef {
        SpotInstanceRequestTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_resource_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
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

impl terrars::ToListMappable for SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    type O =
        terrars::BlockAssignable<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildSpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(self) -> SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
        SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
        SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef {
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
struct SpotInstanceRequestCapacityReservationSpecificationElDynamic {
    capacity_reservation_target: std
    ::option
    ::Option<
        terrars::DynamicBlock<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl>,
    >,
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_target: std
    ::option
    ::Option<std::vec::Vec<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl>>,
    dynamic: SpotInstanceRequestCapacityReservationSpecificationElDynamic,
}

impl SpotInstanceRequestCapacityReservationSpecificationEl {
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
                        ::BlockAssignable<
                            SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetEl,
                        >,
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

impl terrars::ToListMappable for SpotInstanceRequestCapacityReservationSpecificationEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestCapacityReservationSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCapacityReservationSpecificationEl {}

impl BuildSpotInstanceRequestCapacityReservationSpecificationEl {
    pub fn build(self) -> SpotInstanceRequestCapacityReservationSpecificationEl {
        SpotInstanceRequestCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestCapacityReservationSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestCapacityReservationSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SpotInstanceRequestCapacityReservationSpecificationElRef {
        SpotInstanceRequestCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCapacityReservationSpecificationElRef {
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
    ) -> terrars::ListRef<SpotInstanceRequestCapacityReservationSpecificationElCapacityReservationTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_target", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestCreditSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_credits: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_credits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotInstanceRequestCreditSpecificationEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestCreditSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestCreditSpecificationEl {}

impl BuildSpotInstanceRequestCreditSpecificationEl {
    pub fn build(self) -> SpotInstanceRequestCreditSpecificationEl {
        SpotInstanceRequestCreditSpecificationEl { cpu_credits: core::default::Default::default() }
    }
}

pub struct SpotInstanceRequestCreditSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestCreditSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestCreditSpecificationElRef {
        SpotInstanceRequestCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestCreditSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestEbsBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    device_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    snapshot_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestEbsBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for SpotInstanceRequestEbsBlockDeviceEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestEbsBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestEbsBlockDeviceEl {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildSpotInstanceRequestEbsBlockDeviceEl {
    pub fn build(self) -> SpotInstanceRequestEbsBlockDeviceEl {
        SpotInstanceRequestEbsBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            device_name: self.device_name,
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            snapshot_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestEbsBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestEbsBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestEbsBlockDeviceElRef {
        SpotInstanceRequestEbsBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestEbsBlockDeviceElRef {
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

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
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
pub struct SpotInstanceRequestEnclaveOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl SpotInstanceRequestEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotInstanceRequestEnclaveOptionsEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestEnclaveOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestEnclaveOptionsEl {}

impl BuildSpotInstanceRequestEnclaveOptionsEl {
    pub fn build(self) -> SpotInstanceRequestEnclaveOptionsEl {
        SpotInstanceRequestEnclaveOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct SpotInstanceRequestEnclaveOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestEnclaveOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestEnclaveOptionsElRef {
        SpotInstanceRequestEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestEnclaveOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestEphemeralBlockDeviceEl {
    device_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestEphemeralBlockDeviceEl {
    #[doc = "Set the field `no_device`.\n"]
    pub fn set_no_device(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_device = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_name`.\n"]
    pub fn set_virtual_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotInstanceRequestEphemeralBlockDeviceEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestEphemeralBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestEphemeralBlockDeviceEl {
    #[doc = ""]
    pub device_name: terrars::PrimField<std::string::String>,
}

impl BuildSpotInstanceRequestEphemeralBlockDeviceEl {
    pub fn build(self) -> SpotInstanceRequestEphemeralBlockDeviceEl {
        SpotInstanceRequestEphemeralBlockDeviceEl {
            device_name: self.device_name,
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestEphemeralBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestEphemeralBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestEphemeralBlockDeviceElRef {
        SpotInstanceRequestEphemeralBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestEphemeralBlockDeviceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestLaunchTemplateEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestLaunchTemplateEl {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.version = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotInstanceRequestLaunchTemplateEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestLaunchTemplateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestLaunchTemplateEl {}

impl BuildSpotInstanceRequestLaunchTemplateEl {
    pub fn build(self) -> SpotInstanceRequestLaunchTemplateEl {
        SpotInstanceRequestLaunchTemplateEl {
            id: core::default::Default::default(),
            name: core::default::Default::default(),
            version: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestLaunchTemplateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestLaunchTemplateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestLaunchTemplateElRef {
        SpotInstanceRequestLaunchTemplateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestLaunchTemplateElRef {
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

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_recovery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_recovery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotInstanceRequestMaintenanceOptionsEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestMaintenanceOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestMaintenanceOptionsEl {}

impl BuildSpotInstanceRequestMaintenanceOptionsEl {
    pub fn build(self) -> SpotInstanceRequestMaintenanceOptionsEl {
        SpotInstanceRequestMaintenanceOptionsEl { auto_recovery: core::default::Default::default() }
    }
}

pub struct SpotInstanceRequestMaintenanceOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestMaintenanceOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestMaintenanceOptionsElRef {
        SpotInstanceRequestMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestMaintenanceOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestMetadataOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_put_response_hop_limit: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_tokens: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_metadata_tags: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestMetadataOptionsEl {
    #[doc = "Set the field `http_endpoint`.\n"]
    pub fn set_http_endpoint(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.http_endpoint = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for SpotInstanceRequestMetadataOptionsEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestMetadataOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestMetadataOptionsEl {}

impl BuildSpotInstanceRequestMetadataOptionsEl {
    pub fn build(self) -> SpotInstanceRequestMetadataOptionsEl {
        SpotInstanceRequestMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestMetadataOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestMetadataOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestMetadataOptionsElRef {
        SpotInstanceRequestMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestMetadataOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_endpoint` after provisioning.\n"]
    pub fn http_endpoint(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.http_endpoint", self.base))
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
pub struct SpotInstanceRequestNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    device_index: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_card_index: std::option::Option<terrars::PrimField<f64>>,
    network_interface_id: terrars::PrimField<std::string::String>,
}

impl SpotInstanceRequestNetworkInterfaceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_card_index`.\n"]
    pub fn set_network_card_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.network_card_index = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SpotInstanceRequestNetworkInterfaceEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestNetworkInterfaceEl {
    #[doc = ""]
    pub device_index: terrars::PrimField<f64>,
    #[doc = ""]
    pub network_interface_id: terrars::PrimField<std::string::String>,
}

impl BuildSpotInstanceRequestNetworkInterfaceEl {
    pub fn build(self) -> SpotInstanceRequestNetworkInterfaceEl {
        SpotInstanceRequestNetworkInterfaceEl {
            delete_on_termination: core::default::Default::default(),
            device_index: self.device_index,
            network_card_index: core::default::Default::default(),
            network_interface_id: self.network_interface_id,
        }
    }
}

pub struct SpotInstanceRequestNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestNetworkInterfaceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestNetworkInterfaceElRef {
        SpotInstanceRequestNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestNetworkInterfaceElRef {
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

    #[doc = "Get a reference to the value of field `network_card_index` after provisioning.\n"]
    pub fn network_card_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_card_index", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }
}

#[derive(Serialize)]
pub struct SpotInstanceRequestPrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_a_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_aaaa_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestPrivateDnsNameOptionsEl {
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

impl terrars::ToListMappable for SpotInstanceRequestPrivateDnsNameOptionsEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestPrivateDnsNameOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestPrivateDnsNameOptionsEl {}

impl BuildSpotInstanceRequestPrivateDnsNameOptionsEl {
    pub fn build(self) -> SpotInstanceRequestPrivateDnsNameOptionsEl {
        SpotInstanceRequestPrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestPrivateDnsNameOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestPrivateDnsNameOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestPrivateDnsNameOptionsElRef {
        SpotInstanceRequestPrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestPrivateDnsNameOptionsElRef {
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
pub struct SpotInstanceRequestRootBlockDeviceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_on_termination: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    throughput: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volume_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestRootBlockDeviceEl {
    #[doc = "Set the field `delete_on_termination`.\n"]
    pub fn set_delete_on_termination(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.delete_on_termination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags = std::option::Option::Some(v.into());
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

impl terrars::ToListMappable for SpotInstanceRequestRootBlockDeviceEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestRootBlockDeviceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestRootBlockDeviceEl {}

impl BuildSpotInstanceRequestRootBlockDeviceEl {
    pub fn build(self) -> SpotInstanceRequestRootBlockDeviceEl {
        SpotInstanceRequestRootBlockDeviceEl {
            delete_on_termination: core::default::Default::default(),
            encrypted: core::default::Default::default(),
            iops: core::default::Default::default(),
            kms_key_id: core::default::Default::default(),
            tags: core::default::Default::default(),
            throughput: core::default::Default::default(),
            volume_size: core::default::Default::default(),
            volume_type: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestRootBlockDeviceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestRootBlockDeviceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestRootBlockDeviceElRef {
        SpotInstanceRequestRootBlockDeviceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestRootBlockDeviceElRef {
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

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `throughput` after provisioning.\n"]
    pub fn throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.throughput", self.base))
    }

    #[doc = "Get a reference to the value of field `volume_id` after provisioning.\n"]
    pub fn volume_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.volume_id", self.base))
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
pub struct SpotInstanceRequestTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SpotInstanceRequestTimeoutsEl {
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
}

impl terrars::ToListMappable for SpotInstanceRequestTimeoutsEl {
    type O = terrars::BlockAssignable<SpotInstanceRequestTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSpotInstanceRequestTimeoutsEl {}

impl BuildSpotInstanceRequestTimeoutsEl {
    pub fn build(self) -> SpotInstanceRequestTimeoutsEl {
        SpotInstanceRequestTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct SpotInstanceRequestTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SpotInstanceRequestTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SpotInstanceRequestTimeoutsElRef {
        SpotInstanceRequestTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SpotInstanceRequestTimeoutsElRef {
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
}

#[derive(Serialize, Default)]
struct SpotInstanceRequestDynamic {
    capacity_reservation_specification: std
    ::option
    ::Option<terrars::DynamicBlock<SpotInstanceRequestCapacityReservationSpecificationEl>>,
    credit_specification: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestCreditSpecificationEl>>,
    ebs_block_device: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestEbsBlockDeviceEl>>,
    enclave_options: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestEnclaveOptionsEl>>,
    ephemeral_block_device: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestEphemeralBlockDeviceEl>>,
    launch_template: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestLaunchTemplateEl>>,
    maintenance_options: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestMaintenanceOptionsEl>>,
    metadata_options: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestMetadataOptionsEl>>,
    network_interface: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestNetworkInterfaceEl>>,
    private_dns_name_options: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestPrivateDnsNameOptionsEl>>,
    root_block_device: std::option::Option<terrars::DynamicBlock<SpotInstanceRequestRootBlockDeviceEl>>,
}
