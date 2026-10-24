use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2InstanceTypeData {
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
    timeouts: std::option::Option<DataEc2InstanceTypeTimeoutsEl>,
}

struct DataEc2InstanceType_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2InstanceTypeData>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2InstanceTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `auto_recovery_supported` after provisioning.\n"]
    pub fn auto_recovery_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery_supported", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `efa_supported` after provisioning.\n"]
    pub fn efa_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_supported", self.extract_ref()))
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
    pub fn fpgas(&self) -> terrars::SetRef<DataEc2InstanceTypeFpgasElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fpgas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `free_tier_eligible` after provisioning.\n"]
    pub fn free_tier_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free_tier_eligible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gpus` after provisioning.\n"]
    pub fn gpus(&self) -> terrars::SetRef<DataEc2InstanceTypeGpusElRef> {
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
    pub fn inference_accelerators(&self) -> terrars::SetRef<DataEc2InstanceTypeInferenceAcceleratorsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inference_accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_disks` after provisioning.\n"]
    pub fn instance_disks(&self) -> terrars::SetRef<DataEc2InstanceTypeInstanceDisksElRef> {
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

    #[doc = "Get a reference to the value of field `maximum_network_interfaces` after provisioning.\n"]
    pub fn maximum_network_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance` after provisioning.\n"]
    pub fn network_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_performance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_architectures` after provisioning.\n"]
    pub fn supported_architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_architectures", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `total_instance_storage` after provisioning.\n"]
    pub fn total_instance_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_instance_storage", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataEc2InstanceTypeTimeoutsElRef {
        DataEc2InstanceTypeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataEc2InstanceTypeData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                instance_type: self.instance_type,
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

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `efa_supported` after provisioning.\n"]
    pub fn efa_supported(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.efa_supported", self.extract_ref()))
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
    pub fn fpgas(&self) -> terrars::SetRef<DataEc2InstanceTypeFpgasElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.fpgas", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `free_tier_eligible` after provisioning.\n"]
    pub fn free_tier_eligible(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.free_tier_eligible", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `gpus` after provisioning.\n"]
    pub fn gpus(&self) -> terrars::SetRef<DataEc2InstanceTypeGpusElRef> {
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
    pub fn inference_accelerators(&self) -> terrars::SetRef<DataEc2InstanceTypeInferenceAcceleratorsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.inference_accelerators", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_disks` after provisioning.\n"]
    pub fn instance_disks(&self) -> terrars::SetRef<DataEc2InstanceTypeInstanceDisksElRef> {
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

    #[doc = "Get a reference to the value of field `maximum_network_interfaces` after provisioning.\n"]
    pub fn maximum_network_interfaces(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_performance` after provisioning.\n"]
    pub fn network_performance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_performance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `supported_architectures` after provisioning.\n"]
    pub fn supported_architectures(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_architectures", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `total_instance_storage` after provisioning.\n"]
    pub fn total_instance_storage(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_instance_storage", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataEc2InstanceTypeTimeoutsElRef {
        DataEc2InstanceTypeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeFpgasEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeFpgasEl {
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

impl terrars::ToListMappable for DataEc2InstanceTypeFpgasEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeFpgasEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeFpgasEl {}

impl BuildDataEc2InstanceTypeFpgasEl {
    pub fn build(self) -> DataEc2InstanceTypeFpgasEl {
        DataEc2InstanceTypeFpgasEl {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2InstanceTypeFpgasElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeFpgasElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeFpgasElRef {
        DataEc2InstanceTypeFpgasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeFpgasElRef {
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
pub struct DataEc2InstanceTypeGpusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeGpusEl {
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

impl terrars::ToListMappable for DataEc2InstanceTypeGpusEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeGpusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeGpusEl {}

impl BuildDataEc2InstanceTypeGpusEl {
    pub fn build(self) -> DataEc2InstanceTypeGpusEl {
        DataEc2InstanceTypeGpusEl {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            memory_size: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2InstanceTypeGpusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeGpusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeGpusElRef {
        DataEc2InstanceTypeGpusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeGpusElRef {
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
pub struct DataEc2InstanceTypeInferenceAcceleratorsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    manufacturer: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeInferenceAcceleratorsEl {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceTypeInferenceAcceleratorsEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeInferenceAcceleratorsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeInferenceAcceleratorsEl {}

impl BuildDataEc2InstanceTypeInferenceAcceleratorsEl {
    pub fn build(self) -> DataEc2InstanceTypeInferenceAcceleratorsEl {
        DataEc2InstanceTypeInferenceAcceleratorsEl {
            count: core::default::Default::default(),
            manufacturer: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2InstanceTypeInferenceAcceleratorsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeInferenceAcceleratorsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeInferenceAcceleratorsElRef {
        DataEc2InstanceTypeInferenceAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeInferenceAcceleratorsElRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeInstanceDisksEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeInstanceDisksEl {
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

impl terrars::ToListMappable for DataEc2InstanceTypeInstanceDisksEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeInstanceDisksEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeInstanceDisksEl {}

impl BuildDataEc2InstanceTypeInstanceDisksEl {
    pub fn build(self) -> DataEc2InstanceTypeInstanceDisksEl {
        DataEc2InstanceTypeInstanceDisksEl {
            count: core::default::Default::default(),
            size: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataEc2InstanceTypeInstanceDisksElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeInstanceDisksElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeInstanceDisksElRef {
        DataEc2InstanceTypeInstanceDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeInstanceDisksElRef {
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
pub struct DataEc2InstanceTypeTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2InstanceTypeTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2InstanceTypeTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2InstanceTypeTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2InstanceTypeTimeoutsEl {}

impl BuildDataEc2InstanceTypeTimeoutsEl {
    pub fn build(self) -> DataEc2InstanceTypeTimeoutsEl {
        DataEc2InstanceTypeTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2InstanceTypeTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2InstanceTypeTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2InstanceTypeTimeoutsElRef {
        DataEc2InstanceTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
