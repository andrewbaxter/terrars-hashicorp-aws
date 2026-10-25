#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2HostData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2Host_Filter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2Host_Timeouts>,
    dynamic: DataEc2HostDynamic,
}

struct DataEc2Host_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2HostData_>,
}

#[derive(Clone)]
pub struct DataEc2Host(std::rc::Rc<DataEc2Host_>);

impl DataEc2Host {
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

    #[doc = "Set the field `host_id`.\n"]
    pub fn set_host_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataEc2Host_Filter>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2Host_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `allocation_time` after provisioning.\n"]
    pub fn allocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allows_multiple_instance_types` after provisioning.\n"]
    pub fn allows_multiple_instance_types(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allows_multiple_instance_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `asset_id` after provisioning.\n"]
    pub fn asset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_placement` after provisioning.\n"]
    pub fn auto_placement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_capacity` after provisioning.\n"]
    pub fn available_capacity(&self) -> terrars::ListRef<DataEc2Host_AvailableCapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cores` after provisioning.\n"]
    pub fn cores(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cores", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_maintenance` after provisioning.\n"]
    pub fn host_maintenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_maintenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_recovery` after provisioning.\n"]
    pub fn host_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_reservation_id` after provisioning.\n"]
    pub fn host_reservation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_reservation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_family` after provisioning.\n"]
    pub fn instance_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<DataEc2Host_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_of_service_linked_resource_group` after provisioning.\n"]
    pub fn member_of_service_linked_resource_group(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.member_of_service_linked_resource_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_time` after provisioning.\n"]
    pub fn release_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sockets` after provisioning.\n"]
    pub fn sockets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sockets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_vcpus` after provisioning.\n"]
    pub fn total_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_vcpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2Host_TimeoutsRef {
        DataEc2Host_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2Host {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2Host { }

impl terrars::ToListMappable for DataEc2Host {
    type O = terrars::ListRef<DataEc2HostRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2Host_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_host".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2Host {
    pub tf_id: std::string::String,
}

impl BuildDataEc2Host {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2Host {
        let out = DataEc2Host(std::rc::Rc::new(DataEc2Host_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2HostData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                host_id: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
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

pub struct DataEc2HostRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2HostRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2HostRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `allocation_time` after provisioning.\n"]
    pub fn allocation_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `allows_multiple_instance_types` after provisioning.\n"]
    pub fn allows_multiple_instance_types(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.allows_multiple_instance_types", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `asset_id` after provisioning.\n"]
    pub fn asset_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asset_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `auto_placement` after provisioning.\n"]
    pub fn auto_placement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.auto_placement", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_capacity` after provisioning.\n"]
    pub fn available_capacity(&self) -> terrars::ListRef<DataEc2Host_AvailableCapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_capacity", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cores` after provisioning.\n"]
    pub fn cores(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cores", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_id` after provisioning.\n"]
    pub fn host_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_maintenance` after provisioning.\n"]
    pub fn host_maintenance(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_maintenance", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_recovery` after provisioning.\n"]
    pub fn host_recovery(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_recovery", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_reservation_id` after provisioning.\n"]
    pub fn host_reservation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_reservation_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_family` after provisioning.\n"]
    pub fn instance_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instances` after provisioning.\n"]
    pub fn instances(&self) -> terrars::ListRef<DataEc2Host_InstancesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.instances", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `member_of_service_linked_resource_group` after provisioning.\n"]
    pub fn member_of_service_linked_resource_group(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.member_of_service_linked_resource_group", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `release_time` after provisioning.\n"]
    pub fn release_time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.release_time", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sockets` after provisioning.\n"]
    pub fn sockets(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sockets", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `total_vcpus` after provisioning.\n"]
    pub fn total_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_vcpus", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2Host_TimeoutsRef {
        DataEc2Host_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEc2HostDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2Host_Filter>>,
}

#[derive(Serialize)]
pub struct DataEc2Host_AvailableCapacity_AvailableInstanceCapacity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_capacity: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    total_capacity: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2Host_AvailableCapacity_AvailableInstanceCapacity {
    #[doc = "Set the field `available_capacity`.\n"]
    pub fn set_available_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `total_capacity`.\n"]
    pub fn set_total_capacity(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.total_capacity = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2Host_AvailableCapacity_AvailableInstanceCapacity {
    type O = terrars::BlockAssignable<DataEc2Host_AvailableCapacity_AvailableInstanceCapacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2Host_AvailableCapacity_AvailableInstanceCapacity {}

impl BuildDataEc2Host_AvailableCapacity_AvailableInstanceCapacity {
    pub fn build(self) -> DataEc2Host_AvailableCapacity_AvailableInstanceCapacity {
        DataEc2Host_AvailableCapacity_AvailableInstanceCapacity {
            available_capacity: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            total_capacity: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2Host_AvailableCapacity_AvailableInstanceCapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2Host_AvailableCapacity_AvailableInstanceCapacityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2Host_AvailableCapacity_AvailableInstanceCapacityRef {
        DataEc2Host_AvailableCapacity_AvailableInstanceCapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2Host_AvailableCapacity_AvailableInstanceCapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_capacity` after provisioning.\n"]
    pub fn available_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `total_capacity` after provisioning.\n"]
    pub fn total_capacity(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.total_capacity", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2Host_AvailableCapacity {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_instance_capacity: std
    ::option
    ::Option<terrars::ListField<DataEc2Host_AvailableCapacity_AvailableInstanceCapacity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_vcpus: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2Host_AvailableCapacity {
    #[doc = "Set the field `available_instance_capacity`.\n"]
    pub fn set_available_instance_capacity(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2Host_AvailableCapacity_AvailableInstanceCapacity>>,
    ) -> Self {
        self.available_instance_capacity = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `available_vcpus`.\n"]
    pub fn set_available_vcpus(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_vcpus = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2Host_AvailableCapacity {
    type O = terrars::BlockAssignable<DataEc2Host_AvailableCapacity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2Host_AvailableCapacity {}

impl BuildDataEc2Host_AvailableCapacity {
    pub fn build(self) -> DataEc2Host_AvailableCapacity {
        DataEc2Host_AvailableCapacity {
            available_instance_capacity: core::default::Default::default(),
            available_vcpus: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2Host_AvailableCapacityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2Host_AvailableCapacityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2Host_AvailableCapacityRef {
        DataEc2Host_AvailableCapacityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2Host_AvailableCapacityRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_instance_capacity` after provisioning.\n"]
    pub fn available_instance_capacity(
        &self,
    ) -> terrars::ListRef<DataEc2Host_AvailableCapacity_AvailableInstanceCapacityRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.available_instance_capacity", self.base))
    }

    #[doc = "Get a reference to the value of field `available_vcpus` after provisioning.\n"]
    pub fn available_vcpus(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_vcpus", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2Host_Instances {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2Host_Instances {
    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_type`.\n"]
    pub fn set_instance_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `owner_id`.\n"]
    pub fn set_owner_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.owner_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2Host_Instances {
    type O = terrars::BlockAssignable<DataEc2Host_Instances>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2Host_Instances {}

impl BuildDataEc2Host_Instances {
    pub fn build(self) -> DataEc2Host_Instances {
        DataEc2Host_Instances {
            instance_id: core::default::Default::default(),
            instance_type: core::default::Default::default(),
            owner_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2Host_InstancesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2Host_InstancesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2Host_InstancesRef {
        DataEc2Host_InstancesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2Host_InstancesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_type` after provisioning.\n"]
    pub fn instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2Host_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2Host_Filter { }

impl terrars::ToListMappable for DataEc2Host_Filter {
    type O = terrars::BlockAssignable<DataEc2Host_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2Host_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2Host_Filter {
    pub fn build(self) -> DataEc2Host_Filter {
        DataEc2Host_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2Host_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2Host_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2Host_FilterRef {
        DataEc2Host_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2Host_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2Host_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2Host_Timeouts {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2Host_Timeouts {
    type O = terrars::BlockAssignable<DataEc2Host_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2Host_Timeouts {}

impl BuildDataEc2Host_Timeouts {
    pub fn build(self) -> DataEc2Host_Timeouts {
        DataEc2Host_Timeouts { read: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataEc2Host_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2Host_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2Host_TimeoutsRef {
        DataEc2Host_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2Host_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}
