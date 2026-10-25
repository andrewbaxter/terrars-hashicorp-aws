#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2InstanceTypeData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2InstanceType_Timeouts>,
}

struct DataEc2InstanceType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2InstanceTypeData_>,
}

#[derive(Clone)]
pub struct DataEc2InstanceType(std::rc::Rc<DataEc2InstanceType_>);

impl DataEc2InstanceType {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2InstanceType_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `auto_recovery_supported` after provisioning.\n"]
    pub fn auto_recovery_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth_weightings` after provisioning.\n"]
    pub fn bandwidth_weightings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.bandwidth_weightings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_modes` after provisioning.\n"]
    pub fn boot_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.boot_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `burstable_performance_supported` after provisioning.\n"]
    pub fn burstable_performance_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.burstable_performance_supported", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `current_generation` after provisioning.\n"]
    pub fn current_generation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_hosts_supported` after provisioning.\n"]
    pub fn dedicated_hosts_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_hosts_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cores` after provisioning.\n"]
    pub fn default_cores(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cores", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_network_card_index` after provisioning.\n"]
    pub fn default_network_card_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_network_card_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_threads_per_core` after provisioning.\n"]
    pub fn default_threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_threads_per_core", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_vcpus` after provisioning.\n"]
    pub fn default_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_vcpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption_support` after provisioning.\n"]
    pub fn ebs_encryption_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_nvme_support` after provisioning.\n"]
    pub fn ebs_nvme_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_nvme_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized_support` after provisioning.\n"]
    pub fn ebs_optimized_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_performance_baseline_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_baseline_bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_baseline_bandwidth", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_baseline_iops` after provisioning.\n"]
    pub fn ebs_performance_baseline_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_baseline_iops", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_baseline_throughput` after provisioning.\n"]
    pub fn ebs_performance_baseline_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_baseline_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_maximum_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_maximum_bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_maximum_bandwidth", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_maximum_iops` after provisioning.\n"]
    pub fn ebs_performance_maximum_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_performance_maximum_throughput` after provisioning.\n"]
    pub fn ebs_performance_maximum_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_maximum_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `efa_maximum_interfaces` after provisioning.\n"]
    pub fn efa_maximum_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_maximum_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efa_supported` after provisioning.\n"]
    pub fn efa_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ena_srd_supported` after provisioning.\n"]
    pub fn ena_srd_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_srd_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ena_support` after provisioning.\n"]
    pub fn ena_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_in_transit_supported` after provisioning.\n"]
    pub fn encryption_in_transit_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_in_transit_supported", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fpgas` after provisioning.\n"]
    pub fn fpgas(&self) -> terrars::SetRef<DataEc2InstanceType_FpgasRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fpgas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `free_tier_eligible` after provisioning.\n"]
    pub fn free_tier_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free_tier_eligible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gpus` after provisioning.\n"]
    pub fn gpus(&self) -> terrars::SetRef<DataEc2InstanceType_GpusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_supported` after provisioning.\n"]
    pub fn hibernation_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hibernation_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hypervisor` after provisioning.\n"]
    pub fn hypervisor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hypervisor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_accelerators` after provisioning.\n"]
    pub fn inference_accelerators(&self) -> terrars::SetRef<DataEc2InstanceType_InferenceAcceleratorsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inference_accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_disks` after provisioning.\n"]
    pub fn instance_disks(&self) -> terrars::SetRef<DataEc2InstanceType_InstanceDisksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_storage_supported` after provisioning.\n"]
    pub fn instance_storage_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_storage_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_supported` after provisioning.\n"]
    pub fn ipv6_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_ipv4_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv4_addresses_per_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_ipv4_addresses_per_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_ipv6_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv6_addresses_per_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_ipv6_addresses_per_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_network_cards` after provisioning.\n"]
    pub fn maximum_network_cards(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_cards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_network_interfaces` after provisioning.\n"]
    pub fn maximum_network_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_accelerators` after provisioning.\n"]
    pub fn media_accelerators(&self) -> terrars::SetRef<DataEc2InstanceType_MediaAcceleratorsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.media_accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_cards` after provisioning.\n"]
    pub fn network_cards(&self) -> terrars::SetRef<DataEc2InstanceType_NetworkCardsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_cards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance` after provisioning.\n"]
    pub fn network_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_performance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `neuron_devices` after provisioning.\n"]
    pub fn neuron_devices(&self) -> terrars::SetRef<DataEc2InstanceType_NeuronDevicesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.neuron_devices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nitro_enclaves_support` after provisioning.\n"]
    pub fn nitro_enclaves_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nitro_enclaves_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nitro_tpm_support` after provisioning.\n"]
    pub fn nitro_tpm_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nitro_tpm_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nitro_tpm_supported_versions` after provisioning.\n"]
    pub fn nitro_tpm_supported_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nitro_tpm_supported_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phc_support` after provisioning.\n"]
    pub fn phc_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phc_support", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_architectures` after provisioning.\n"]
    pub fn supported_architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_architectures", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_cpu_features` after provisioning.\n"]
    pub fn supported_cpu_features(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_cpu_features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_placement_strategies` after provisioning.\n"]
    pub fn supported_placement_strategies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_placement_strategies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_root_device_types` after provisioning.\n"]
    pub fn supported_root_device_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_root_device_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_usages_classes` after provisioning.\n"]
    pub fn supported_usages_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_usages_classes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_virtualization_types` after provisioning.\n"]
    pub fn supported_virtualization_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_virtualization_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sustained_clock_speed` after provisioning.\n"]
    pub fn sustained_clock_speed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sustained_clock_speed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_fpga_memory` after provisioning.\n"]
    pub fn total_fpga_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_fpga_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_gpu_memory` after provisioning.\n"]
    pub fn total_gpu_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_gpu_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_inference_memory` after provisioning.\n"]
    pub fn total_inference_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_inference_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_instance_storage` after provisioning.\n"]
    pub fn total_instance_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_instance_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_media_memory` after provisioning.\n"]
    pub fn total_media_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_media_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_neuron_device_memory` after provisioning.\n"]
    pub fn total_neuron_device_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_neuron_device_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_cores` after provisioning.\n"]
    pub fn valid_cores(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.valid_cores", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_threads_per_core` after provisioning.\n"]
    pub fn valid_threads_per_core(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.valid_threads_per_core", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceType_TimeoutsRef {
        DataEc2InstanceType_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2InstanceType {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2InstanceType { }

impl terrars::ToListMappable for DataEc2InstanceType {
    type O = terrars::ListRef<DataEc2InstanceTypeRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2InstanceType_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_instance_type".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2InstanceType {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildDataEc2InstanceType {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2InstanceType {
        let out = DataEc2InstanceType(std::rc::Rc::new(DataEc2InstanceType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2InstanceTypeData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_type: self.instance_type,
                region: core::default::Default::default(),
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2InstanceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2InstanceTypeRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `auto_recovery_supported` after provisioning.\n"]
    pub fn auto_recovery_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bandwidth_weightings` after provisioning.\n"]
    pub fn bandwidth_weightings(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.bandwidth_weightings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `boot_modes` after provisioning.\n"]
    pub fn boot_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.boot_modes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `burstable_performance_supported` after provisioning.\n"]
    pub fn burstable_performance_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.burstable_performance_supported", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `current_generation` after provisioning.\n"]
    pub fn current_generation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.current_generation", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dedicated_hosts_supported` after provisioning.\n"]
    pub fn dedicated_hosts_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dedicated_hosts_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_cores` after provisioning.\n"]
    pub fn default_cores(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_cores", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_network_card_index` after provisioning.\n"]
    pub fn default_network_card_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_network_card_index", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_threads_per_core` after provisioning.\n"]
    pub fn default_threads_per_core(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_threads_per_core", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_vcpus` after provisioning.\n"]
    pub fn default_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_vcpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_encryption_support` after provisioning.\n"]
    pub fn ebs_encryption_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_nvme_support` after provisioning.\n"]
    pub fn ebs_nvme_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_nvme_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_optimized_support` after provisioning.\n"]
    pub fn ebs_optimized_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_performance_baseline_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_baseline_bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_baseline_bandwidth", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_baseline_iops` after provisioning.\n"]
    pub fn ebs_performance_baseline_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_baseline_iops", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_baseline_throughput` after provisioning.\n"]
    pub fn ebs_performance_baseline_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_baseline_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_maximum_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_maximum_bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_maximum_bandwidth", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ebs_performance_maximum_iops` after provisioning.\n"]
    pub fn ebs_performance_maximum_iops(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_iops", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ebs_performance_maximum_throughput` after provisioning.\n"]
    pub fn ebs_performance_maximum_throughput(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ebs_performance_maximum_throughput", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `efa_maximum_interfaces` after provisioning.\n"]
    pub fn efa_maximum_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_maximum_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `efa_supported` after provisioning.\n"]
    pub fn efa_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ena_srd_supported` after provisioning.\n"]
    pub fn ena_srd_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_srd_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ena_support` after provisioning.\n"]
    pub fn ena_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ena_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_in_transit_supported` after provisioning.\n"]
    pub fn encryption_in_transit_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_in_transit_supported", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `fpgas` after provisioning.\n"]
    pub fn fpgas(&self) -> terrars::SetRef<DataEc2InstanceType_FpgasRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fpgas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `free_tier_eligible` after provisioning.\n"]
    pub fn free_tier_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free_tier_eligible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gpus` after provisioning.\n"]
    pub fn gpus(&self) -> terrars::SetRef<DataEc2InstanceType_GpusRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_supported` after provisioning.\n"]
    pub fn hibernation_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hibernation_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hypervisor` after provisioning.\n"]
    pub fn hypervisor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hypervisor", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `inference_accelerators` after provisioning.\n"]
    pub fn inference_accelerators(&self) -> terrars::SetRef<DataEc2InstanceType_InferenceAcceleratorsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inference_accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_disks` after provisioning.\n"]
    pub fn instance_disks(&self) -> terrars::SetRef<DataEc2InstanceType_InstanceDisksRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.instance_disks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_storage_supported` after provisioning.\n"]
    pub fn instance_storage_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_storage_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_supported` after provisioning.\n"]
    pub fn ipv6_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_ipv4_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv4_addresses_per_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_ipv4_addresses_per_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_ipv6_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv6_addresses_per_interface(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.maximum_ipv6_addresses_per_interface", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `maximum_network_cards` after provisioning.\n"]
    pub fn maximum_network_cards(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_cards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maximum_network_interfaces` after provisioning.\n"]
    pub fn maximum_network_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_accelerators` after provisioning.\n"]
    pub fn media_accelerators(&self) -> terrars::SetRef<DataEc2InstanceType_MediaAcceleratorsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.media_accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_cards` after provisioning.\n"]
    pub fn network_cards(&self) -> terrars::SetRef<DataEc2InstanceType_NetworkCardsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.network_cards", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance` after provisioning.\n"]
    pub fn network_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_performance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `neuron_devices` after provisioning.\n"]
    pub fn neuron_devices(&self) -> terrars::SetRef<DataEc2InstanceType_NeuronDevicesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.neuron_devices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nitro_enclaves_support` after provisioning.\n"]
    pub fn nitro_enclaves_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nitro_enclaves_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nitro_tpm_support` after provisioning.\n"]
    pub fn nitro_tpm_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nitro_tpm_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `nitro_tpm_supported_versions` after provisioning.\n"]
    pub fn nitro_tpm_supported_versions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.nitro_tpm_supported_versions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `phc_support` after provisioning.\n"]
    pub fn phc_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.phc_support", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_architectures` after provisioning.\n"]
    pub fn supported_architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_architectures", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_cpu_features` after provisioning.\n"]
    pub fn supported_cpu_features(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_cpu_features", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_placement_strategies` after provisioning.\n"]
    pub fn supported_placement_strategies(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_placement_strategies", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `supported_root_device_types` after provisioning.\n"]
    pub fn supported_root_device_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_root_device_types", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_usages_classes` after provisioning.\n"]
    pub fn supported_usages_classes(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_usages_classes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_virtualization_types` after provisioning.\n"]
    pub fn supported_virtualization_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.supported_virtualization_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `sustained_clock_speed` after provisioning.\n"]
    pub fn sustained_clock_speed(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sustained_clock_speed", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_fpga_memory` after provisioning.\n"]
    pub fn total_fpga_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_fpga_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_gpu_memory` after provisioning.\n"]
    pub fn total_gpu_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_gpu_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_inference_memory` after provisioning.\n"]
    pub fn total_inference_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_inference_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_instance_storage` after provisioning.\n"]
    pub fn total_instance_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_instance_storage", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_media_memory` after provisioning.\n"]
    pub fn total_media_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_media_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_neuron_device_memory` after provisioning.\n"]
    pub fn total_neuron_device_memory(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_neuron_device_memory", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_cores` after provisioning.\n"]
    pub fn valid_cores(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.valid_cores", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `valid_threads_per_core` after provisioning.\n"]
    pub fn valid_threads_per_core(&self) -> terrars::ListRef<terrars::PrimExpr<f64>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.valid_threads_per_core", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceType_TimeoutsRef {
        DataEc2InstanceType_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_Fpgas {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_Fpgas {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manufacturer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_Fpgas {
    type O = terrars::BlockAssignable<DataEc2InstanceType_Fpgas>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_Fpgas {}

impl BuildDataEc2InstanceType_Fpgas {
    pub fn build(self) -> DataEc2InstanceType_Fpgas {
        DataEc2InstanceType_Fpgas {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_FpgasRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_FpgasRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_FpgasRef {
        DataEc2InstanceType_FpgasRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_FpgasRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_Gpus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_Gpus {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manufacturer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_Gpus {
    type O = terrars::BlockAssignable<DataEc2InstanceType_Gpus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_Gpus {}

impl BuildDataEc2InstanceType_Gpus {
    pub fn build(self) -> DataEc2InstanceType_Gpus {
        DataEc2InstanceType_Gpus {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_GpusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_GpusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_GpusRef {
        DataEc2InstanceType_GpusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_GpusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_InferenceAccelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_InferenceAccelerators {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manufacturer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_InferenceAccelerators {
    type O = terrars::BlockAssignable<DataEc2InstanceType_InferenceAccelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_InferenceAccelerators {}

impl BuildDataEc2InstanceType_InferenceAccelerators {
    pub fn build(self) -> DataEc2InstanceType_InferenceAccelerators {
        DataEc2InstanceType_InferenceAccelerators {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_InferenceAcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_InferenceAcceleratorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_InferenceAcceleratorsRef {
        DataEc2InstanceType_InferenceAcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_InferenceAcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_InstanceDisks {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_InstanceDisks {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_InstanceDisks {
    type O = terrars::BlockAssignable<DataEc2InstanceType_InstanceDisks>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_InstanceDisks {}

impl BuildDataEc2InstanceType_InstanceDisks {
    pub fn build(self) -> DataEc2InstanceType_InstanceDisks {
        DataEc2InstanceType_InstanceDisks {
            count: core::default::Default::default(),
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_InstanceDisksRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_InstanceDisksRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_InstanceDisksRef {
        DataEc2InstanceType_InstanceDisksRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_InstanceDisksRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_MediaAccelerators {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_MediaAccelerators {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.manufacturer = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_MediaAccelerators {
    type O = terrars::BlockAssignable<DataEc2InstanceType_MediaAccelerators>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_MediaAccelerators {}

impl BuildDataEc2InstanceType_MediaAccelerators {
    pub fn build(self) -> DataEc2InstanceType_MediaAccelerators {
        DataEc2InstanceType_MediaAccelerators {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_MediaAcceleratorsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_MediaAcceleratorsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_MediaAcceleratorsRef {
        DataEc2InstanceType_MediaAcceleratorsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_MediaAcceleratorsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_NetworkCards {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_bandwidth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_interfaces: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peak_bandwidth: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    performance: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_NetworkCards {
    #[doc = "Set the field `baseline_bandwidth`.\n"]
    pub fn set_baseline_bandwidth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.baseline_bandwidth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `index`.\n"]
    pub fn set_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_interfaces`.\n"]
    pub fn set_maximum_interfaces(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_interfaces = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peak_bandwidth`.\n"]
    pub fn set_peak_bandwidth(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.peak_bandwidth = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `performance`.\n"]
    pub fn set_performance(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.performance = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_NetworkCards {
    type O = terrars::BlockAssignable<DataEc2InstanceType_NetworkCards>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_NetworkCards {}

impl BuildDataEc2InstanceType_NetworkCards {
    pub fn build(self) -> DataEc2InstanceType_NetworkCards {
        DataEc2InstanceType_NetworkCards {
            baseline_bandwidth: core::default::Default::default(),
            index: core::default::Default::default(),
            maximum_interfaces: core::default::Default::default(),
            peak_bandwidth: core::default::Default::default(),
            performance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_NetworkCardsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_NetworkCardsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_NetworkCardsRef {
        DataEc2InstanceType_NetworkCardsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_NetworkCardsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `baseline_bandwidth` after provisioning.\n"]
    pub fn baseline_bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.baseline_bandwidth", self.base))
    }

    #[doc = "Get a reference to the value of field `index` after provisioning.\n"]
    pub fn index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_interfaces` after provisioning.\n"]
    pub fn maximum_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_interfaces", self.base))
    }

    #[doc = "Get a reference to the value of field `peak_bandwidth` after provisioning.\n"]
    pub fn peak_bandwidth(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peak_bandwidth", self.base))
    }

    #[doc = "Get a reference to the value of field `performance` after provisioning.\n"]
    pub fn performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.performance", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_NeuronDevices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_version: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_NeuronDevices {
    #[doc = "Set the field `core_count`.\n"]
    pub fn set_core_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.core_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `core_version`.\n"]
    pub fn set_core_version(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.core_version = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.memory_size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_NeuronDevices {
    type O = terrars::BlockAssignable<DataEc2InstanceType_NeuronDevices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_NeuronDevices {}

impl BuildDataEc2InstanceType_NeuronDevices {
    pub fn build(self) -> DataEc2InstanceType_NeuronDevices {
        DataEc2InstanceType_NeuronDevices {
            core_count: core::default::Default::default(),
            core_version: core::default::Default::default(),
            count: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_NeuronDevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_NeuronDevicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_NeuronDevicesRef {
        DataEc2InstanceType_NeuronDevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_NeuronDevicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `core_count` after provisioning.\n"]
    pub fn core_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_count", self.base))
    }

    #[doc = "Get a reference to the value of field `core_version` after provisioning.\n"]
    pub fn core_version(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_version", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceType_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceType_Timeouts {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceType_Timeouts {
    type O = terrars::BlockAssignable<DataEc2InstanceType_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceType_Timeouts {}

impl BuildDataEc2InstanceType_Timeouts {
    pub fn build(self) -> DataEc2InstanceType_Timeouts {
        DataEc2InstanceType_Timeouts { read: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEc2InstanceType_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceType_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceType_TimeoutsRef {
        DataEc2InstanceType_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceType_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
