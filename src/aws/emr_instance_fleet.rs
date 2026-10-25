#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct EmrInstanceFleetData_ {
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
    cluster_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_on_demand_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_spot_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type_configs: std::option::Option<std::vec::Vec<EmrInstanceFleet_InstanceTypeConfigs>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    launch_specifications: std::option::Option<std::vec::Vec<EmrInstanceFleet_LaunchSpecifications>>,
    dynamic: EmrInstanceFleetDynamic,
}

struct EmrInstanceFleet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<EmrInstanceFleetData_>,
}

#[derive(Clone)]
pub struct EmrInstanceFleet(std::rc::Rc<EmrInstanceFleet_>);

impl EmrInstanceFleet {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_on_demand_capacity`.\n"]
    pub fn set_target_on_demand_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_on_demand_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_spot_capacity`.\n"]
    pub fn set_target_spot_capacity(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().target_spot_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type_configs`.\n"]
    pub fn set_instance_type_configs(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrInstanceFleet_InstanceTypeConfigs>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().instance_type_configs = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.instance_type_configs = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `launch_specifications`.\n"]
    pub fn set_launch_specifications(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<EmrInstanceFleet_LaunchSpecifications>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().launch_specifications = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.launch_specifications = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_on_demand_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> terrars::ListRef<EmrInstanceFleet_LaunchSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.extract_ref()))
    }
}

impl terrars::Referable for EmrInstanceFleet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for EmrInstanceFleet { }

impl terrars::ToListMappable for EmrInstanceFleet {
    type O = terrars::ListRef<EmrInstanceFleetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for EmrInstanceFleet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_emr_instance_fleet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEmrInstanceFleet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub cluster_id: terrars::PrimField<std::string::String>,
}

impl BuildEmrInstanceFleet {
    pub fn build(self, stack: &mut terrars::Stack) -> EmrInstanceFleet {
        let out = EmrInstanceFleet(std::rc::Rc::new(EmrInstanceFleet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(EmrInstanceFleetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                cluster_id: self.cluster_id,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
                target_on_demand_capacity: core::default::Default::default(),
                target_spot_capacity: core::default::Default::default(),
                instance_type_configs: core::default::Default::default(),
                launch_specifications: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct EmrInstanceFleetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl EmrInstanceFleetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cluster_id` after provisioning.\n"]
    pub fn cluster_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cluster_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `provisioned_on_demand_capacity` after provisioning.\n"]
    pub fn provisioned_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.provisioned_on_demand_capacity", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `provisioned_spot_capacity` after provisioning.\n"]
    pub fn provisioned_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.provisioned_spot_capacity", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_on_demand_capacity` after provisioning.\n"]
    pub fn target_on_demand_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_on_demand_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `target_spot_capacity` after provisioning.\n"]
    pub fn target_spot_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_spot_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `launch_specifications` after provisioning.\n"]
    pub fn launch_specifications(&self) -> terrars::ListRef<EmrInstanceFleet_LaunchSpecificationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.launch_specifications", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct EmrInstanceFleetDynamic {
    instance_type_configs: std::option::Option<terrars::DynamicBlock<EmrInstanceFleet_InstanceTypeConfigs>>,
    launch_specifications: std::option::Option<terrars::DynamicBlock<EmrInstanceFleet_LaunchSpecifications>>,
}

#[derive(Serialize)]
pub struct EmrInstanceFleet_InstanceTypeConfigs_Configurations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classification: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    properties: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

impl EmrInstanceFleet_InstanceTypeConfigs_Configurations {
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

impl terrars::ToListMappable for EmrInstanceFleet_InstanceTypeConfigs_Configurations {
    type O = terrars::BlockAssignable<EmrInstanceFleet_InstanceTypeConfigs_Configurations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleet_InstanceTypeConfigs_Configurations {}

impl BuildEmrInstanceFleet_InstanceTypeConfigs_Configurations {
    pub fn build(self) -> EmrInstanceFleet_InstanceTypeConfigs_Configurations {
        EmrInstanceFleet_InstanceTypeConfigs_Configurations {
            classification: core::default::Default::default(),
            properties: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
        EmrInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleet_InstanceTypeConfigs_ConfigurationsRef {
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
pub struct EmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    iops: std::option::Option<terrars::PrimField<f64>>,
    size: terrars::PrimField<f64>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    volumes_per_instance: std::option::Option<terrars::PrimField<f64>>,
}

impl EmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
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

impl terrars::ToListMappable for EmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
    type O = terrars::BlockAssignable<EmrInstanceFleet_InstanceTypeConfigs_EbsConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
    #[doc = ""]
    pub size: terrars::PrimField<f64>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildEmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
    pub fn build(self) -> EmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
        EmrInstanceFleet_InstanceTypeConfigs_EbsConfig {
            iops: core::default::Default::default(),
            size: self.size,
            type_: self.type_,
            volumes_per_instance: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
        EmrInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleet_InstanceTypeConfigs_EbsConfigRef {
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
struct EmrInstanceFleet_InstanceTypeConfigsDynamic {
    configurations: std::option::Option<terrars::DynamicBlock<EmrInstanceFleet_InstanceTypeConfigs_Configurations>>,
    ebs_config: std::option::Option<terrars::DynamicBlock<EmrInstanceFleet_InstanceTypeConfigs_EbsConfig>>,
}

#[derive(Serialize)]
pub struct EmrInstanceFleet_InstanceTypeConfigs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_price: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bid_price_as_percentage_of_on_demand_price: std::option::Option<terrars::PrimField<f64>>,
    instance_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configurations: std::option::Option<std::vec::Vec<EmrInstanceFleet_InstanceTypeConfigs_Configurations>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ebs_config: std::option::Option<std::vec::Vec<EmrInstanceFleet_InstanceTypeConfigs_EbsConfig>>,
    dynamic: EmrInstanceFleet_InstanceTypeConfigsDynamic,
}

impl EmrInstanceFleet_InstanceTypeConfigs {
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
        v: impl std::convert::Into<terrars::BlockAssignable<EmrInstanceFleet_InstanceTypeConfigs_Configurations>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<EmrInstanceFleet_InstanceTypeConfigs_EbsConfig>>,
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

impl terrars::ToListMappable for EmrInstanceFleet_InstanceTypeConfigs {
    type O = terrars::BlockAssignable<EmrInstanceFleet_InstanceTypeConfigs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleet_InstanceTypeConfigs {
    #[doc = ""]
    pub instance_type: terrars::PrimField<std::string::String>,
}

impl BuildEmrInstanceFleet_InstanceTypeConfigs {
    pub fn build(self) -> EmrInstanceFleet_InstanceTypeConfigs {
        EmrInstanceFleet_InstanceTypeConfigs {
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
pub struct EmrInstanceFleet_InstanceTypeConfigsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleet_InstanceTypeConfigsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrInstanceFleet_InstanceTypeConfigsRef {
        EmrInstanceFleet_InstanceTypeConfigsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleet_InstanceTypeConfigsRef {
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
pub struct EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    allocation_strategy: terrars::PrimField<std::string::String>,
}

impl EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification { }

impl terrars::ToListMappable for EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    type O = terrars::BlockAssignable<EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    #[doc = ""]
    pub allocation_strategy: terrars::PrimField<std::string::String>,
}

impl BuildEmrInstanceFleet_LaunchSpecifications_OnDemandSpecification {
    pub fn build(self) -> EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification {
        EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification { allocation_strategy: self.allocation_strategy }
    }
}

#[allow(dead_code)]
pub struct EmrInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
        EmrInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef {
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
pub struct EmrInstanceFleet_LaunchSpecifications_SpotSpecification {
    allocation_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    block_duration_minutes: std::option::Option<terrars::PrimField<f64>>,
    timeout_action: terrars::PrimField<std::string::String>,
    timeout_duration_minutes: terrars::PrimField<f64>,
}

impl EmrInstanceFleet_LaunchSpecifications_SpotSpecification {
    #[doc = "Set the field `block_duration_minutes`.\n"]
    pub fn set_block_duration_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.block_duration_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for EmrInstanceFleet_LaunchSpecifications_SpotSpecification {
    type O = terrars::BlockAssignable<EmrInstanceFleet_LaunchSpecifications_SpotSpecification>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleet_LaunchSpecifications_SpotSpecification {
    #[doc = ""]
    pub allocation_strategy: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub timeout_action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub timeout_duration_minutes: terrars::PrimField<f64>,
}

impl BuildEmrInstanceFleet_LaunchSpecifications_SpotSpecification {
    pub fn build(self) -> EmrInstanceFleet_LaunchSpecifications_SpotSpecification {
        EmrInstanceFleet_LaunchSpecifications_SpotSpecification {
            allocation_strategy: self.allocation_strategy,
            block_duration_minutes: core::default::Default::default(),
            timeout_action: self.timeout_action,
            timeout_duration_minutes: self.timeout_duration_minutes,
        }
    }
}

#[allow(dead_code)]
pub struct EmrInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> EmrInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
        EmrInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleet_LaunchSpecifications_SpotSpecificationRef {
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
struct EmrInstanceFleet_LaunchSpecificationsDynamic {
    on_demand_specification: std
    ::option
    ::Option<terrars::DynamicBlock<EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification>>,
    spot_specification: std
    ::option
    ::Option<terrars::DynamicBlock<EmrInstanceFleet_LaunchSpecifications_SpotSpecification>>,
}

#[derive(Serialize)]
pub struct EmrInstanceFleet_LaunchSpecifications {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    on_demand_specification: std
    ::option
    ::Option<std::vec::Vec<EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    spot_specification: std::option::Option<std::vec::Vec<EmrInstanceFleet_LaunchSpecifications_SpotSpecification>>,
    dynamic: EmrInstanceFleet_LaunchSpecificationsDynamic,
}

impl EmrInstanceFleet_LaunchSpecifications {
    #[doc = "Set the field `on_demand_specification`.\n"]
    pub fn set_on_demand_specification(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<EmrInstanceFleet_LaunchSpecifications_OnDemandSpecification>>,
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
                    ::Into<terrars::BlockAssignable<EmrInstanceFleet_LaunchSpecifications_SpotSpecification>>,
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

impl terrars::ToListMappable for EmrInstanceFleet_LaunchSpecifications {
    type O = terrars::BlockAssignable<EmrInstanceFleet_LaunchSpecifications>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEmrInstanceFleet_LaunchSpecifications {}

impl BuildEmrInstanceFleet_LaunchSpecifications {
    pub fn build(self) -> EmrInstanceFleet_LaunchSpecifications {
        EmrInstanceFleet_LaunchSpecifications {
            on_demand_specification: core::default::Default::default(),
            spot_specification: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct EmrInstanceFleet_LaunchSpecificationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for EmrInstanceFleet_LaunchSpecificationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> EmrInstanceFleet_LaunchSpecificationsRef {
        EmrInstanceFleet_LaunchSpecificationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl EmrInstanceFleet_LaunchSpecificationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `on_demand_specification` after provisioning.\n"]
    pub fn on_demand_specification(
        &self,
    ) -> terrars::ListRef<EmrInstanceFleet_LaunchSpecifications_OnDemandSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.on_demand_specification", self.base))
    }

    #[doc = "Get a reference to the value of field `spot_specification` after provisioning.\n"]
    pub fn spot_specification(&self) -> terrars::ListRef<EmrInstanceFleet_LaunchSpecifications_SpotSpecificationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spot_specification", self.base))
    }
}
