use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2InstanceTypeData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeouts: Option<DataEc2InstanceTypeTimeoutsEl>,
}

struct DataEc2InstanceType_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataEc2InstanceTypeData>,
}

#[derive(Clone)]
pub struct DataEc2InstanceType(Rc<DataEc2InstanceType_>);

impl DataEc2InstanceType {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl Into<DataEc2InstanceTypeTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `auto_recovery_supported` after provisioning.\n"]
    pub fn auto_recovery_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `burstable_performance_supported` after provisioning.\n"]
    pub fn burstable_performance_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.burstable_performance_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `current_generation` after provisioning.\n"]
    pub fn current_generation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_generation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dedicated_hosts_supported` after provisioning.\n"]
    pub fn dedicated_hosts_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dedicated_hosts_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_cores` after provisioning.\n"]
    pub fn default_cores(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_cores", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_threads_per_core` after provisioning.\n"]
    pub fn default_threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_threads_per_core", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_vcpus` after provisioning.\n"]
    pub fn default_vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_vcpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_encryption_support` after provisioning.\n"]
    pub fn ebs_encryption_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_nvme_support` after provisioning.\n"]
    pub fn ebs_nvme_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_nvme_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_optimized_support` after provisioning.\n"]
    pub fn ebs_optimized_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_baseline_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_baseline_bandwidth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_baseline_bandwidth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_baseline_iops` after provisioning.\n"]
    pub fn ebs_performance_baseline_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_baseline_iops", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_baseline_throughput` after provisioning.\n"]
    pub fn ebs_performance_baseline_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_baseline_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_maximum_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_maximum_bandwidth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_bandwidth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_maximum_iops` after provisioning.\n"]
    pub fn ebs_performance_maximum_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_iops", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_maximum_throughput` after provisioning.\n"]
    pub fn ebs_performance_maximum_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `efa_supported` after provisioning.\n"]
    pub fn efa_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.efa_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ena_support` after provisioning.\n"]
    pub fn ena_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ena_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_in_transit_supported` after provisioning.\n"]
    pub fn encryption_in_transit_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_in_transit_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fpgas` after provisioning.\n"]
    pub fn fpgas(&self) -> SetRef<DataEc2InstanceTypeFpgasElRef> {
        SetRef::new(self.shared().clone(), format!("{}.fpgas", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `free_tier_eligible` after provisioning.\n"]
    pub fn free_tier_eligible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.free_tier_eligible", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gpus` after provisioning.\n"]
    pub fn gpus(&self) -> SetRef<DataEc2InstanceTypeGpusElRef> {
        SetRef::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hibernation_supported` after provisioning.\n"]
    pub fn hibernation_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hibernation_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hypervisor` after provisioning.\n"]
    pub fn hypervisor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hypervisor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inference_accelerators` after provisioning.\n"]
    pub fn inference_accelerators(&self) -> SetRef<DataEc2InstanceTypeInferenceAcceleratorsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.inference_accelerators", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_disks` after provisioning.\n"]
    pub fn instance_disks(&self) -> SetRef<DataEc2InstanceTypeInstanceDisksElRef> {
        SetRef::new(self.shared().clone(), format!("{}.instance_disks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_storage_supported` after provisioning.\n"]
    pub fn instance_storage_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_storage_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_supported` after provisioning.\n"]
    pub fn ipv6_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_ipv4_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv4_addresses_per_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ipv4_addresses_per_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_ipv6_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv6_addresses_per_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ipv6_addresses_per_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_network_interfaces` after provisioning.\n"]
    pub fn maximum_network_interfaces(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_interfaces", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_performance` after provisioning.\n"]
    pub fn network_performance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_performance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_architectures` after provisioning.\n"]
    pub fn supported_architectures(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_architectures", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_placement_strategies` after provisioning.\n"]
    pub fn supported_placement_strategies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_placement_strategies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_root_device_types` after provisioning.\n"]
    pub fn supported_root_device_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_root_device_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_usages_classes` after provisioning.\n"]
    pub fn supported_usages_classes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_usages_classes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_virtualization_types` after provisioning.\n"]
    pub fn supported_virtualization_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_virtualization_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sustained_clock_speed` after provisioning.\n"]
    pub fn sustained_clock_speed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sustained_clock_speed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_fpga_memory` after provisioning.\n"]
    pub fn total_fpga_memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_fpga_memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_gpu_memory` after provisioning.\n"]
    pub fn total_gpu_memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_gpu_memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_instance_storage` after provisioning.\n"]
    pub fn total_instance_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_instance_storage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_cores` after provisioning.\n"]
    pub fn valid_cores(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.valid_cores", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_threads_per_core` after provisioning.\n"]
    pub fn valid_threads_per_core(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.valid_threads_per_core", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeTimeoutsElRef {
        DataEc2InstanceTypeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl Referable for DataEc2InstanceType {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Datasource for DataEc2InstanceType { }

impl ToListMappable for DataEc2InstanceType {
    type O = ListRef<DataEc2InstanceTypeRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataEc2InstanceType_ {
    fn extract_datasource_type(&self) -> String {
        "aws_ec2_instance_type".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2InstanceType {
    pub tf_id: String,
    #[doc= ""]
    pub instance_type: PrimField<String>,
}

impl BuildDataEc2InstanceType {
    pub fn build(self, stack: &mut Stack) -> DataEc2InstanceType {
        let out = DataEc2InstanceType(Rc::new(DataEc2InstanceType_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataEc2InstanceTypeData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
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
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2InstanceTypeRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `auto_recovery_supported` after provisioning.\n"]
    pub fn auto_recovery_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `burstable_performance_supported` after provisioning.\n"]
    pub fn burstable_performance_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.burstable_performance_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `current_generation` after provisioning.\n"]
    pub fn current_generation(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.current_generation", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `dedicated_hosts_supported` after provisioning.\n"]
    pub fn dedicated_hosts_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.dedicated_hosts_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_cores` after provisioning.\n"]
    pub fn default_cores(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_cores", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_threads_per_core` after provisioning.\n"]
    pub fn default_threads_per_core(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_threads_per_core", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_vcpus` after provisioning.\n"]
    pub fn default_vcpus(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.default_vcpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_encryption_support` after provisioning.\n"]
    pub fn ebs_encryption_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_encryption_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_nvme_support` after provisioning.\n"]
    pub fn ebs_nvme_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_nvme_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_optimized_support` after provisioning.\n"]
    pub fn ebs_optimized_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_optimized_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_baseline_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_baseline_bandwidth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_baseline_bandwidth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_baseline_iops` after provisioning.\n"]
    pub fn ebs_performance_baseline_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_baseline_iops", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_baseline_throughput` after provisioning.\n"]
    pub fn ebs_performance_baseline_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_baseline_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_maximum_bandwidth` after provisioning.\n"]
    pub fn ebs_performance_maximum_bandwidth(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_bandwidth", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_maximum_iops` after provisioning.\n"]
    pub fn ebs_performance_maximum_iops(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_iops", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ebs_performance_maximum_throughput` after provisioning.\n"]
    pub fn ebs_performance_maximum_throughput(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.ebs_performance_maximum_throughput", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `efa_supported` after provisioning.\n"]
    pub fn efa_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.efa_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ena_support` after provisioning.\n"]
    pub fn ena_support(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.ena_support", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_in_transit_supported` after provisioning.\n"]
    pub fn encryption_in_transit_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.encryption_in_transit_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `fpgas` after provisioning.\n"]
    pub fn fpgas(&self) -> SetRef<DataEc2InstanceTypeFpgasElRef> {
        SetRef::new(self.shared().clone(), format!("{}.fpgas", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `free_tier_eligible` after provisioning.\n"]
    pub fn free_tier_eligible(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.free_tier_eligible", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `gpus` after provisioning.\n"]
    pub fn gpus(&self) -> SetRef<DataEc2InstanceTypeGpusElRef> {
        SetRef::new(self.shared().clone(), format!("{}.gpus", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hibernation_supported` after provisioning.\n"]
    pub fn hibernation_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.hibernation_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `hypervisor` after provisioning.\n"]
    pub fn hypervisor(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.hypervisor", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `inference_accelerators` after provisioning.\n"]
    pub fn inference_accelerators(&self) -> SetRef<DataEc2InstanceTypeInferenceAcceleratorsElRef> {
        SetRef::new(self.shared().clone(), format!("{}.inference_accelerators", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_disks` after provisioning.\n"]
    pub fn instance_disks(&self) -> SetRef<DataEc2InstanceTypeInstanceDisksElRef> {
        SetRef::new(self.shared().clone(), format!("{}.instance_disks", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_storage_supported` after provisioning.\n"]
    pub fn instance_storage_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_storage_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `ipv6_supported` after provisioning.\n"]
    pub fn ipv6_supported(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.ipv6_supported", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_ipv4_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv4_addresses_per_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ipv4_addresses_per_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_ipv6_addresses_per_interface` after provisioning.\n"]
    pub fn maximum_ipv6_addresses_per_interface(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_ipv6_addresses_per_interface", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `maximum_network_interfaces` after provisioning.\n"]
    pub fn maximum_network_interfaces(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.maximum_network_interfaces", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `network_performance` after provisioning.\n"]
    pub fn network_performance(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.network_performance", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_architectures` after provisioning.\n"]
    pub fn supported_architectures(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_architectures", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_placement_strategies` after provisioning.\n"]
    pub fn supported_placement_strategies(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_placement_strategies", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_root_device_types` after provisioning.\n"]
    pub fn supported_root_device_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_root_device_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_usages_classes` after provisioning.\n"]
    pub fn supported_usages_classes(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_usages_classes", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `supported_virtualization_types` after provisioning.\n"]
    pub fn supported_virtualization_types(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.supported_virtualization_types", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `sustained_clock_speed` after provisioning.\n"]
    pub fn sustained_clock_speed(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.sustained_clock_speed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_fpga_memory` after provisioning.\n"]
    pub fn total_fpga_memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_fpga_memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_gpu_memory` after provisioning.\n"]
    pub fn total_gpu_memory(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_gpu_memory", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `total_instance_storage` after provisioning.\n"]
    pub fn total_instance_storage(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.total_instance_storage", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_cores` after provisioning.\n"]
    pub fn valid_cores(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.valid_cores", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `valid_threads_per_core` after provisioning.\n"]
    pub fn valid_threads_per_core(&self) -> ListRef<PrimExpr<f64>> {
        ListRef::new(self.shared().clone(), format!("{}.valid_threads_per_core", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2InstanceTypeTimeoutsElRef {
        DataEc2InstanceTypeTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeFpgasEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataEc2InstanceTypeFpgasEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2InstanceTypeFpgasEl {
    type O = BlockAssignable<DataEc2InstanceTypeFpgasEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeFpgasElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeFpgasElRef {
        DataEc2InstanceTypeFpgasElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeFpgasElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc= "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeGpusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataEc2InstanceTypeGpusEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }

    #[doc= "Set the field `memory_size`.\n"]
    pub fn set_memory_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.memory_size = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2InstanceTypeGpusEl {
    type O = BlockAssignable<DataEc2InstanceTypeGpusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeGpusElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeGpusElRef {
        DataEc2InstanceTypeGpusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeGpusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc= "Get a reference to the value of field `memory_size` after provisioning.\n"]
    pub fn memory_size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.memory_size", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeInferenceAcceleratorsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    manufacturer: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
}

impl DataEc2InstanceTypeInferenceAcceleratorsEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `manufacturer`.\n"]
    pub fn set_manufacturer(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2InstanceTypeInferenceAcceleratorsEl {
    type O = BlockAssignable<DataEc2InstanceTypeInferenceAcceleratorsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeInferenceAcceleratorsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeInferenceAcceleratorsElRef {
        DataEc2InstanceTypeInferenceAcceleratorsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeInferenceAcceleratorsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `manufacturer` after provisioning.\n"]
    pub fn manufacturer(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.manufacturer", self.base))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeInstanceDisksEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    count: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
}

impl DataEc2InstanceTypeInstanceDisksEl {
    #[doc= "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.count = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2InstanceTypeInstanceDisksEl {
    type O = BlockAssignable<DataEc2InstanceTypeInstanceDisksEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeInstanceDisksElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeInstanceDisksElRef {
        DataEc2InstanceTypeInstanceDisksElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeInstanceDisksElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2InstanceTypeTimeoutsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    read: Option<PrimField<String>>,
}

impl DataEc2InstanceTypeTimeoutsEl {
    #[doc= "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.read = Some(v.into());
        self
    }
}

impl ToListMappable for DataEc2InstanceTypeTimeoutsEl {
    type O = BlockAssignable<DataEc2InstanceTypeTimeoutsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataEc2InstanceTypeTimeoutsElRef {
    fn new(shared: StackShared, base: String) -> DataEc2InstanceTypeTimeoutsElRef {
        DataEc2InstanceTypeTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2InstanceTypeTimeoutsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
