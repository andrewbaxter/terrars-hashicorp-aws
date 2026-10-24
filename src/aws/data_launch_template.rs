use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataLaunchTemplateData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataLaunchTemplateFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataLaunchTemplateTimeoutsEl>,
    dynamic: DataLaunchTemplateDynamic,
}

struct DataLaunchTemplate_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataLaunchTemplateData>,
}

#[derive(Clone)]
pub struct DataLaunchTemplate(std::rc::Rc<DataLaunchTemplate_>);

impl DataLaunchTemplate {
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

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataLaunchTemplateFilterEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataLaunchTemplateTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_device_mappings` after provisioning.\n"]
    pub fn block_device_mappings(&self) -> terrars::ListRef<DataLaunchTemplateBlockDeviceMappingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_device_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<DataLaunchTemplateCapacityReservationSpecificationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> terrars::ListRef<DataLaunchTemplateCpuOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<DataLaunchTemplateCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `elastic_gpu_specifications` after provisioning.\n"]
    pub fn elastic_gpu_specifications(&self) -> terrars::ListRef<DataLaunchTemplateElasticGpuSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_gpu_specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_inference_accelerator` after provisioning.\n"]
    pub fn elastic_inference_accelerator(&self) -> terrars::ListRef<DataLaunchTemplateElasticInferenceAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_inference_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<DataLaunchTemplateEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> terrars::ListRef<DataLaunchTemplateHibernationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hibernation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::ListRef<DataLaunchTemplateIamInstanceProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> terrars::ListRef<DataLaunchTemplateInstanceMarketOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_market_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `license_specification` after provisioning.\n"]
    pub fn license_specification(&self) -> terrars::ListRef<DataLaunchTemplateLicenseSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.license_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<DataLaunchTemplateMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<DataLaunchTemplateMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<DataLaunchTemplateMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<DataLaunchTemplateNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataLaunchTemplatePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<DataLaunchTemplatePrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_disk_id` after provisioning.\n"]
    pub fn ram_disk_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ram_disk_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> terrars::ListRef<DataLaunchTemplateTagSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLaunchTemplateTimeoutsElRef {
        DataLaunchTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataLaunchTemplate {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataLaunchTemplate { }

impl terrars::ToListMappable for DataLaunchTemplate {
    type O = terrars::ListRef<DataLaunchTemplateRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataLaunchTemplate_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_launch_template".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataLaunchTemplate {
    pub tf_id: std::string::String,
}

impl BuildDataLaunchTemplate {
    pub fn build(self, stack: &mut terrars::Stack) -> DataLaunchTemplate {
        let out = DataLaunchTemplate(std::rc::Rc::new(DataLaunchTemplate_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataLaunchTemplateData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataLaunchTemplateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataLaunchTemplateRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `block_device_mappings` after provisioning.\n"]
    pub fn block_device_mappings(&self) -> terrars::ListRef<DataLaunchTemplateBlockDeviceMappingsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.block_device_mappings", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `capacity_reservation_specification` after provisioning.\n"]
    pub fn capacity_reservation_specification(
        &self,
    ) -> terrars::ListRef<DataLaunchTemplateCapacityReservationSpecificationElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.capacity_reservation_specification", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `cpu_options` after provisioning.\n"]
    pub fn cpu_options(&self) -> terrars::ListRef<DataLaunchTemplateCpuOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cpu_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `credit_specification` after provisioning.\n"]
    pub fn credit_specification(&self) -> terrars::ListRef<DataLaunchTemplateCreditSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.credit_specification", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `elastic_gpu_specifications` after provisioning.\n"]
    pub fn elastic_gpu_specifications(&self) -> terrars::ListRef<DataLaunchTemplateElasticGpuSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_gpu_specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `elastic_inference_accelerator` after provisioning.\n"]
    pub fn elastic_inference_accelerator(&self) -> terrars::ListRef<DataLaunchTemplateElasticInferenceAcceleratorElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_inference_accelerator", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enclave_options` after provisioning.\n"]
    pub fn enclave_options(&self) -> terrars::ListRef<DataLaunchTemplateEnclaveOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.enclave_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `hibernation_options` after provisioning.\n"]
    pub fn hibernation_options(&self) -> terrars::ListRef<DataLaunchTemplateHibernationOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.hibernation_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `iam_instance_profile` after provisioning.\n"]
    pub fn iam_instance_profile(&self) -> terrars::ListRef<DataLaunchTemplateIamInstanceProfileElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.iam_instance_profile", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `instance_market_options` after provisioning.\n"]
    pub fn instance_market_options(&self) -> terrars::ListRef<DataLaunchTemplateInstanceMarketOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_market_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_requirements` after provisioning.\n"]
    pub fn instance_requirements(&self) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instance_requirements", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `license_specification` after provisioning.\n"]
    pub fn license_specification(&self) -> terrars::ListRef<DataLaunchTemplateLicenseSpecificationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.license_specification", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `maintenance_options` after provisioning.\n"]
    pub fn maintenance_options(&self) -> terrars::ListRef<DataLaunchTemplateMaintenanceOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.maintenance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metadata_options` after provisioning.\n"]
    pub fn metadata_options(&self) -> terrars::ListRef<DataLaunchTemplateMetadataOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.metadata_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitoring` after provisioning.\n"]
    pub fn monitoring(&self) -> terrars::ListRef<DataLaunchTemplateMonitoringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitoring", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_interfaces` after provisioning.\n"]
    pub fn network_interfaces(&self) -> terrars::ListRef<DataLaunchTemplateNetworkInterfacesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interfaces", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `placement` after provisioning.\n"]
    pub fn placement(&self) -> terrars::ListRef<DataLaunchTemplatePlacementElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name_options` after provisioning.\n"]
    pub fn private_dns_name_options(&self) -> terrars::ListRef<DataLaunchTemplatePrivateDnsNameOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_dns_name_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ram_disk_id` after provisioning.\n"]
    pub fn ram_disk_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ram_disk_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_group_names` after provisioning.\n"]
    pub fn security_group_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_group_names", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_specifications` after provisioning.\n"]
    pub fn tag_specifications(&self) -> terrars::ListRef<DataLaunchTemplateTagSpecificationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_specifications", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `user_data` after provisioning.\n"]
    pub fn user_data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.user_data", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_security_group_ids` after provisioning.\n"]
    pub fn vpc_security_group_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.vpc_security_group_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataLaunchTemplateTimeoutsElRef {
        DataLaunchTemplateTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateBlockDeviceMappingsElEbsEl {
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

impl DataLaunchTemplateBlockDeviceMappingsElEbsEl {
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

impl terrars::ToListMappable for DataLaunchTemplateBlockDeviceMappingsElEbsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateBlockDeviceMappingsElEbsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateBlockDeviceMappingsElEbsEl {}

impl BuildDataLaunchTemplateBlockDeviceMappingsElEbsEl {
    pub fn build(self) -> DataLaunchTemplateBlockDeviceMappingsElEbsEl {
        DataLaunchTemplateBlockDeviceMappingsElEbsEl {
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

pub struct DataLaunchTemplateBlockDeviceMappingsElEbsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateBlockDeviceMappingsElEbsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateBlockDeviceMappingsElEbsElRef {
        DataLaunchTemplateBlockDeviceMappingsElEbsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateBlockDeviceMappingsElEbsElRef {
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

#[derive(Serialize)]
pub struct DataLaunchTemplateBlockDeviceMappingsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs: std::option::Option<terrars::ListField<DataLaunchTemplateBlockDeviceMappingsElEbsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_device: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateBlockDeviceMappingsEl {
    #[doc = "Set the field `device_name`.\n"]
    pub fn set_device_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.device_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ebs`.\n"]
    pub fn set_ebs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLaunchTemplateBlockDeviceMappingsElEbsEl>>,
    ) -> Self {
        self.ebs = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataLaunchTemplateBlockDeviceMappingsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateBlockDeviceMappingsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateBlockDeviceMappingsEl {}

impl BuildDataLaunchTemplateBlockDeviceMappingsEl {
    pub fn build(self) -> DataLaunchTemplateBlockDeviceMappingsEl {
        DataLaunchTemplateBlockDeviceMappingsEl {
            device_name: core::default::Default::default(),
            ebs: core::default::Default::default(),
            no_device: core::default::Default::default(),
            virtual_name: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateBlockDeviceMappingsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateBlockDeviceMappingsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateBlockDeviceMappingsElRef {
        DataLaunchTemplateBlockDeviceMappingsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateBlockDeviceMappingsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `device_name` after provisioning.\n"]
    pub fn device_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_name", self.base))
    }

    #[doc = "Get a reference to the value of field `ebs` after provisioning.\n"]
    pub fn ebs(&self) -> terrars::ListRef<DataLaunchTemplateBlockDeviceMappingsElEbsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ebs", self.base))
    }

    #[doc = "Get a reference to the value of field `no_device` after provisioning.\n"]
    pub fn no_device(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_device", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_name` after provisioning.\n"]
    pub fn virtual_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_resource_group_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
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

impl terrars::ToListMappable for DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    type O =
        terrars::BlockAssignable<DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {}

impl BuildDataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
    pub fn build(self) -> DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
        DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl {
            capacity_reservation_id: core::default::Default::default(),
            capacity_reservation_resource_group_arn: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
        DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef {
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

#[derive(Serialize)]
pub struct DataLaunchTemplateCapacityReservationSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_preference: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_reservation_target: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>>,
}

impl DataLaunchTemplateCapacityReservationSpecificationEl {
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
                        ::ListField<DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetEl>,
                    >,
    ) -> Self {
        self.capacity_reservation_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateCapacityReservationSpecificationEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateCapacityReservationSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateCapacityReservationSpecificationEl {}

impl BuildDataLaunchTemplateCapacityReservationSpecificationEl {
    pub fn build(self) -> DataLaunchTemplateCapacityReservationSpecificationEl {
        DataLaunchTemplateCapacityReservationSpecificationEl {
            capacity_reservation_preference: core::default::Default::default(),
            capacity_reservation_target: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateCapacityReservationSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateCapacityReservationSpecificationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateCapacityReservationSpecificationElRef {
        DataLaunchTemplateCapacityReservationSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateCapacityReservationSpecificationElRef {
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
    ) -> terrars::ListRef<DataLaunchTemplateCapacityReservationSpecificationElCapacityReservationTargetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.capacity_reservation_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateCpuOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    core_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threads_per_core: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateCpuOptionsEl {
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

impl terrars::ToListMappable for DataLaunchTemplateCpuOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateCpuOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateCpuOptionsEl {}

impl BuildDataLaunchTemplateCpuOptionsEl {
    pub fn build(self) -> DataLaunchTemplateCpuOptionsEl {
        DataLaunchTemplateCpuOptionsEl {
            core_count: core::default::Default::default(),
            threads_per_core: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateCpuOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateCpuOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateCpuOptionsElRef {
        DataLaunchTemplateCpuOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateCpuOptionsElRef {
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
pub struct DataLaunchTemplateCreditSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cpu_credits: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateCreditSpecificationEl {
    #[doc = "Set the field `cpu_credits`.\n"]
    pub fn set_cpu_credits(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cpu_credits = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateCreditSpecificationEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateCreditSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateCreditSpecificationEl {}

impl BuildDataLaunchTemplateCreditSpecificationEl {
    pub fn build(self) -> DataLaunchTemplateCreditSpecificationEl {
        DataLaunchTemplateCreditSpecificationEl { cpu_credits: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateCreditSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateCreditSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateCreditSpecificationElRef {
        DataLaunchTemplateCreditSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateCreditSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cpu_credits` after provisioning.\n"]
    pub fn cpu_credits(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cpu_credits", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateElasticGpuSpecificationsEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateElasticGpuSpecificationsEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateElasticGpuSpecificationsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateElasticGpuSpecificationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateElasticGpuSpecificationsEl {}

impl BuildDataLaunchTemplateElasticGpuSpecificationsEl {
    pub fn build(self) -> DataLaunchTemplateElasticGpuSpecificationsEl {
        DataLaunchTemplateElasticGpuSpecificationsEl { type_: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateElasticGpuSpecificationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateElasticGpuSpecificationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateElasticGpuSpecificationsElRef {
        DataLaunchTemplateElasticGpuSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateElasticGpuSpecificationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateElasticInferenceAcceleratorEl {
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateElasticInferenceAcceleratorEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateElasticInferenceAcceleratorEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateElasticInferenceAcceleratorEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateElasticInferenceAcceleratorEl {}

impl BuildDataLaunchTemplateElasticInferenceAcceleratorEl {
    pub fn build(self) -> DataLaunchTemplateElasticInferenceAcceleratorEl {
        DataLaunchTemplateElasticInferenceAcceleratorEl { type_: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateElasticInferenceAcceleratorElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateElasticInferenceAcceleratorElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateElasticInferenceAcceleratorElRef {
        DataLaunchTemplateElasticInferenceAcceleratorElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateElasticInferenceAcceleratorElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateEnclaveOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLaunchTemplateEnclaveOptionsEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateEnclaveOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateEnclaveOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateEnclaveOptionsEl {}

impl BuildDataLaunchTemplateEnclaveOptionsEl {
    pub fn build(self) -> DataLaunchTemplateEnclaveOptionsEl {
        DataLaunchTemplateEnclaveOptionsEl { enabled: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateEnclaveOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateEnclaveOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateEnclaveOptionsElRef {
        DataLaunchTemplateEnclaveOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateEnclaveOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateHibernationOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configured: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLaunchTemplateHibernationOptionsEl {
    #[doc = "Set the field `configured`.\n"]
    pub fn set_configured(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.configured = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateHibernationOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateHibernationOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateHibernationOptionsEl {}

impl BuildDataLaunchTemplateHibernationOptionsEl {
    pub fn build(self) -> DataLaunchTemplateHibernationOptionsEl {
        DataLaunchTemplateHibernationOptionsEl { configured: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateHibernationOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateHibernationOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateHibernationOptionsElRef {
        DataLaunchTemplateHibernationOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateHibernationOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configured` after provisioning.\n"]
    pub fn configured(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configured", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateIamInstanceProfileEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateIamInstanceProfileEl {
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

impl terrars::ToListMappable for DataLaunchTemplateIamInstanceProfileEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateIamInstanceProfileEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateIamInstanceProfileEl {}

impl BuildDataLaunchTemplateIamInstanceProfileEl {
    pub fn build(self) -> DataLaunchTemplateIamInstanceProfileEl {
        DataLaunchTemplateIamInstanceProfileEl {
            arn: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateIamInstanceProfileElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateIamInstanceProfileElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateIamInstanceProfileElRef {
        DataLaunchTemplateIamInstanceProfileElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateIamInstanceProfileElRef {
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
pub struct DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
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

impl DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {}

impl BuildDataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
    pub fn build(self) -> DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
        DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl {
            block_duration_minutes: core::default::Default::default(),
            instance_interruption_behavior: core::default::Default::default(),
            max_price: core::default::Default::default(),
            spot_instance_type: core::default::Default::default(),
            valid_until: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
        DataLaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceMarketOptionsElSpotOptionsElRef {
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

#[derive(Serialize)]
pub struct DataLaunchTemplateInstanceMarketOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    market_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_options: std::option::Option<terrars::ListField<DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
}

impl DataLaunchTemplateInstanceMarketOptionsEl {
    #[doc = "Set the field `market_type`.\n"]
    pub fn set_market_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.market_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `spot_options`.\n"]
    pub fn set_spot_options(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLaunchTemplateInstanceMarketOptionsElSpotOptionsEl>>,
    ) -> Self {
        self.spot_options = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateInstanceMarketOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceMarketOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceMarketOptionsEl {}

impl BuildDataLaunchTemplateInstanceMarketOptionsEl {
    pub fn build(self) -> DataLaunchTemplateInstanceMarketOptionsEl {
        DataLaunchTemplateInstanceMarketOptionsEl {
            market_type: core::default::Default::default(),
            spot_options: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceMarketOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceMarketOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateInstanceMarketOptionsElRef {
        DataLaunchTemplateInstanceMarketOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceMarketOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `market_type` after provisioning.\n"]
    pub fn market_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.market_type", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_options` after provisioning.\n"]
    pub fn spot_options(&self) -> terrars::ListRef<DataLaunchTemplateInstanceMarketOptionsElSpotOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_options", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
        DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
        DataLaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElAcceleratorCountElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
        DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
        DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
        DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
        DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
        DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
        DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElMemoryMibEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElMemoryMibEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElMemoryMibEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElMemoryMibEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElMemoryMibEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElMemoryMibEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElMemoryMibEl {
        DataLaunchTemplateInstanceRequirementsElMemoryMibEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElMemoryMibElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElMemoryMibElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElMemoryMibElRef {
        DataLaunchTemplateInstanceRequirementsElMemoryMibElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElMemoryMibElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
        DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
        DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
        DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
        DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsElVcpuCountEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min: std::option::Option<terrars::PrimField<f64>>,
}

impl DataLaunchTemplateInstanceRequirementsElVcpuCountEl {
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

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsElVcpuCountEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsElVcpuCountEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsElVcpuCountEl {}

impl BuildDataLaunchTemplateInstanceRequirementsElVcpuCountEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsElVcpuCountEl {
        DataLaunchTemplateInstanceRequirementsElVcpuCountEl {
            max: core::default::Default::default(),
            min: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElVcpuCountElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElVcpuCountElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataLaunchTemplateInstanceRequirementsElVcpuCountElRef {
        DataLaunchTemplateInstanceRequirementsElVcpuCountElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElVcpuCountElRef {
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
pub struct DataLaunchTemplateInstanceRequirementsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_count: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_manufacturers: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_names: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_total_memory_mib: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    accelerator_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bare_metal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    baseline_ebs_bandwidth_mbps: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
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
    memory_gib_per_vcpu: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    memory_mib: std::option::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_count: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    require_hibernate_support: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_max_price_percentage_over_lowest_price: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_local_storage_gb: std
    ::option
    ::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vcpu_count: std::option::Option<terrars::ListField<DataLaunchTemplateInstanceRequirementsElVcpuCountEl>>,
}

impl DataLaunchTemplateInstanceRequirementsEl {
    #[doc = "Set the field `accelerator_count`.\n"]
    pub fn set_accelerator_count(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElAcceleratorCountEl>>,
    ) -> Self {
        self.accelerator_count = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `accelerator_total_memory_mib`.\n"]
    pub fn set_accelerator_total_memory_mib(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibEl>>,
    ) -> Self {
        self.accelerator_total_memory_mib = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `baseline_ebs_bandwidth_mbps`.\n"]
    pub fn set_baseline_ebs_bandwidth_mbps(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsEl>>,
    ) -> Self {
        self.baseline_ebs_bandwidth_mbps = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `memory_gib_per_vcpu`.\n"]
    pub fn set_memory_gib_per_vcpu(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuEl>>,
    ) -> Self {
        self.memory_gib_per_vcpu = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `memory_mib`.\n"]
    pub fn set_memory_mib(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElMemoryMibEl>>,
    ) -> Self {
        self.memory_mib = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_count`.\n"]
    pub fn set_network_interface_count(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountEl>>,
    ) -> Self {
        self.network_interface_count = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `total_local_storage_gb`.\n"]
    pub fn set_total_local_storage_gb(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbEl>>,
    ) -> Self {
        self.total_local_storage_gb = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vcpu_count`.\n"]
    pub fn set_vcpu_count(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataLaunchTemplateInstanceRequirementsElVcpuCountEl>>,
    ) -> Self {
        self.vcpu_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateInstanceRequirementsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateInstanceRequirementsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateInstanceRequirementsEl {}

impl BuildDataLaunchTemplateInstanceRequirementsEl {
    pub fn build(self) -> DataLaunchTemplateInstanceRequirementsEl {
        DataLaunchTemplateInstanceRequirementsEl {
            accelerator_count: core::default::Default::default(),
            accelerator_manufacturers: core::default::Default::default(),
            accelerator_names: core::default::Default::default(),
            accelerator_total_memory_mib: core::default::Default::default(),
            accelerator_types: core::default::Default::default(),
            bare_metal: core::default::Default::default(),
            baseline_ebs_bandwidth_mbps: core::default::Default::default(),
            burstable_performance: core::default::Default::default(),
            cpu_manufacturers: core::default::Default::default(),
            excluded_instance_types: core::default::Default::default(),
            instance_generations: core::default::Default::default(),
            local_storage: core::default::Default::default(),
            local_storage_types: core::default::Default::default(),
            memory_gib_per_vcpu: core::default::Default::default(),
            memory_mib: core::default::Default::default(),
            network_interface_count: core::default::Default::default(),
            on_demand_max_price_percentage_over_lowest_price: core::default::Default::default(),
            require_hibernate_support: core::default::Default::default(),
            spot_max_price_percentage_over_lowest_price: core::default::Default::default(),
            total_local_storage_gb: core::default::Default::default(),
            vcpu_count: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateInstanceRequirementsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateInstanceRequirementsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateInstanceRequirementsElRef {
        DataLaunchTemplateInstanceRequirementsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateInstanceRequirementsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `accelerator_count` after provisioning.\n"]
    pub fn accelerator_count(&self) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElAcceleratorCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_count", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_manufacturers` after provisioning.\n"]
    pub fn accelerator_manufacturers(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_manufacturers", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_names` after provisioning.\n"]
    pub fn accelerator_names(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_names", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_total_memory_mib` after provisioning.\n"]
    pub fn accelerator_total_memory_mib(
        &self,
    ) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElAcceleratorTotalMemoryMibElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.accelerator_total_memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `accelerator_types` after provisioning.\n"]
    pub fn accelerator_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.accelerator_types", self.base))
    }

    #[doc = "Get a reference to the value of field `bare_metal` after provisioning.\n"]
    pub fn bare_metal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bare_metal", self.base))
    }

    #[doc = "Get a reference to the value of field `baseline_ebs_bandwidth_mbps` after provisioning.\n"]
    pub fn baseline_ebs_bandwidth_mbps(
        &self,
    ) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElBaselineEbsBandwidthMbpsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.baseline_ebs_bandwidth_mbps", self.base))
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

    #[doc = "Get a reference to the value of field `memory_gib_per_vcpu` after provisioning.\n"]
    pub fn memory_gib_per_vcpu(&self) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElMemoryGibPerVcpuElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_gib_per_vcpu", self.base))
    }

    #[doc = "Get a reference to the value of field `memory_mib` after provisioning.\n"]
    pub fn memory_mib(&self) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElMemoryMibElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.memory_mib", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_count` after provisioning.\n"]
    pub fn network_interface_count(
        &self,
    ) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElNetworkInterfaceCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface_count", self.base))
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

    #[doc = "Get a reference to the value of field `total_local_storage_gb` after provisioning.\n"]
    pub fn total_local_storage_gb(
        &self,
    ) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElTotalLocalStorageGbElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.total_local_storage_gb", self.base))
    }

    #[doc = "Get a reference to the value of field `vcpu_count` after provisioning.\n"]
    pub fn vcpu_count(&self) -> terrars::ListRef<DataLaunchTemplateInstanceRequirementsElVcpuCountElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vcpu_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateLicenseSpecificationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    license_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateLicenseSpecificationEl {
    #[doc = "Set the field `license_configuration_arn`.\n"]
    pub fn set_license_configuration_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.license_configuration_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateLicenseSpecificationEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateLicenseSpecificationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateLicenseSpecificationEl {}

impl BuildDataLaunchTemplateLicenseSpecificationEl {
    pub fn build(self) -> DataLaunchTemplateLicenseSpecificationEl {
        DataLaunchTemplateLicenseSpecificationEl { license_configuration_arn: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateLicenseSpecificationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateLicenseSpecificationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateLicenseSpecificationElRef {
        DataLaunchTemplateLicenseSpecificationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateLicenseSpecificationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `license_configuration_arn` after provisioning.\n"]
    pub fn license_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.license_configuration_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateMaintenanceOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_recovery: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateMaintenanceOptionsEl {
    #[doc = "Set the field `auto_recovery`.\n"]
    pub fn set_auto_recovery(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.auto_recovery = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateMaintenanceOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateMaintenanceOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateMaintenanceOptionsEl {}

impl BuildDataLaunchTemplateMaintenanceOptionsEl {
    pub fn build(self) -> DataLaunchTemplateMaintenanceOptionsEl {
        DataLaunchTemplateMaintenanceOptionsEl { auto_recovery: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateMaintenanceOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateMaintenanceOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateMaintenanceOptionsElRef {
        DataLaunchTemplateMaintenanceOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateMaintenanceOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `auto_recovery` after provisioning.\n"]
    pub fn auto_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_recovery", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateMetadataOptionsEl {
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

impl DataLaunchTemplateMetadataOptionsEl {
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

impl terrars::ToListMappable for DataLaunchTemplateMetadataOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateMetadataOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateMetadataOptionsEl {}

impl BuildDataLaunchTemplateMetadataOptionsEl {
    pub fn build(self) -> DataLaunchTemplateMetadataOptionsEl {
        DataLaunchTemplateMetadataOptionsEl {
            http_endpoint: core::default::Default::default(),
            http_protocol_ipv6: core::default::Default::default(),
            http_put_response_hop_limit: core::default::Default::default(),
            http_tokens: core::default::Default::default(),
            instance_metadata_tags: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateMetadataOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateMetadataOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateMetadataOptionsElRef {
        DataLaunchTemplateMetadataOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateMetadataOptionsElRef {
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
pub struct DataLaunchTemplateMonitoringEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
}

impl DataLaunchTemplateMonitoringEl {
    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enabled = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateMonitoringEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateMonitoringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateMonitoringEl {}

impl BuildDataLaunchTemplateMonitoringEl {
    pub fn build(self) -> DataLaunchTemplateMonitoringEl {
        DataLaunchTemplateMonitoringEl { enabled: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateMonitoringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateMonitoringElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateMonitoringElRef {
        DataLaunchTemplateMonitoringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateMonitoringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateNetworkInterfacesEl {
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

impl DataLaunchTemplateNetworkInterfacesEl {
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

impl terrars::ToListMappable for DataLaunchTemplateNetworkInterfacesEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateNetworkInterfacesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateNetworkInterfacesEl {}

impl BuildDataLaunchTemplateNetworkInterfacesEl {
    pub fn build(self) -> DataLaunchTemplateNetworkInterfacesEl {
        DataLaunchTemplateNetworkInterfacesEl {
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

pub struct DataLaunchTemplateNetworkInterfacesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateNetworkInterfacesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateNetworkInterfacesElRef {
        DataLaunchTemplateNetworkInterfacesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateNetworkInterfacesElRef {
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
pub struct DataLaunchTemplatePlacementEl {
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

impl DataLaunchTemplatePlacementEl {
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

impl terrars::ToListMappable for DataLaunchTemplatePlacementEl {
    type O = terrars::BlockAssignable<DataLaunchTemplatePlacementEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplatePlacementEl {}

impl BuildDataLaunchTemplatePlacementEl {
    pub fn build(self) -> DataLaunchTemplatePlacementEl {
        DataLaunchTemplatePlacementEl {
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

pub struct DataLaunchTemplatePlacementElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplatePlacementElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplatePlacementElRef {
        DataLaunchTemplatePlacementElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplatePlacementElRef {
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
pub struct DataLaunchTemplatePrivateDnsNameOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_a_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_aaaa_record: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hostname_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplatePrivateDnsNameOptionsEl {
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

impl terrars::ToListMappable for DataLaunchTemplatePrivateDnsNameOptionsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplatePrivateDnsNameOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplatePrivateDnsNameOptionsEl {}

impl BuildDataLaunchTemplatePrivateDnsNameOptionsEl {
    pub fn build(self) -> DataLaunchTemplatePrivateDnsNameOptionsEl {
        DataLaunchTemplatePrivateDnsNameOptionsEl {
            enable_resource_name_dns_a_record: core::default::Default::default(),
            enable_resource_name_dns_aaaa_record: core::default::Default::default(),
            hostname_type: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplatePrivateDnsNameOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplatePrivateDnsNameOptionsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplatePrivateDnsNameOptionsElRef {
        DataLaunchTemplatePrivateDnsNameOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplatePrivateDnsNameOptionsElRef {
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
pub struct DataLaunchTemplateTagSpecificationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl DataLaunchTemplateTagSpecificationsEl {
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

impl terrars::ToListMappable for DataLaunchTemplateTagSpecificationsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateTagSpecificationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateTagSpecificationsEl {}

impl BuildDataLaunchTemplateTagSpecificationsEl {
    pub fn build(self) -> DataLaunchTemplateTagSpecificationsEl {
        DataLaunchTemplateTagSpecificationsEl {
            resource_type: core::default::Default::default(),
            tags: core::default::Default::default(),
        }
    }
}

pub struct DataLaunchTemplateTagSpecificationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateTagSpecificationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateTagSpecificationsElRef {
        DataLaunchTemplateTagSpecificationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateTagSpecificationsElRef {
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

#[derive(Serialize)]
pub struct DataLaunchTemplateFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateFilterEl { }

impl terrars::ToListMappable for DataLaunchTemplateFilterEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataLaunchTemplateFilterEl {
    pub fn build(self) -> DataLaunchTemplateFilterEl {
        DataLaunchTemplateFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataLaunchTemplateFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateFilterElRef {
        DataLaunchTemplateFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataLaunchTemplateTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataLaunchTemplateTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataLaunchTemplateTimeoutsEl {
    type O = terrars::BlockAssignable<DataLaunchTemplateTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataLaunchTemplateTimeoutsEl {}

impl BuildDataLaunchTemplateTimeoutsEl {
    pub fn build(self) -> DataLaunchTemplateTimeoutsEl {
        DataLaunchTemplateTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataLaunchTemplateTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataLaunchTemplateTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataLaunchTemplateTimeoutsElRef {
        DataLaunchTemplateTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataLaunchTemplateTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataLaunchTemplateDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataLaunchTemplateFilterEl>>,
}
