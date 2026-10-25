#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMqBrokerInstanceTypeOfferingsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMqBrokerInstanceTypeOfferings_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMqBrokerInstanceTypeOfferingsData_>,
}

#[derive(Clone)]
pub struct DataMqBrokerInstanceTypeOfferings(std::rc::Rc<DataMqBrokerInstanceTypeOfferings_>);

impl DataMqBrokerInstanceTypeOfferings {
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

    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().engine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_instance_type`.\n"]
    pub fn set_host_instance_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().host_instance_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `broker_instance_options` after provisioning.\n"]
    pub fn broker_instance_options(
        &self,
    ) -> terrars::ListRef<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_instance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }
}

impl terrars::Referable for DataMqBrokerInstanceTypeOfferings {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMqBrokerInstanceTypeOfferings { }

impl terrars::ToListMappable for DataMqBrokerInstanceTypeOfferings {
    type O = terrars::ListRef<DataMqBrokerInstanceTypeOfferingsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMqBrokerInstanceTypeOfferings_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_mq_broker_instance_type_offerings".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMqBrokerInstanceTypeOfferings {
    pub tf_id: std::string::String,
}

impl BuildDataMqBrokerInstanceTypeOfferings {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMqBrokerInstanceTypeOfferings {
        let out = DataMqBrokerInstanceTypeOfferings(std::rc::Rc::new(DataMqBrokerInstanceTypeOfferings_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMqBrokerInstanceTypeOfferingsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                engine_type: core::default::Default::default(),
                host_instance_type: core::default::Default::default(),
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                storage_type: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMqBrokerInstanceTypeOfferingsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerInstanceTypeOfferingsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMqBrokerInstanceTypeOfferingsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `broker_instance_options` after provisioning.\n"]
    pub fn broker_instance_options(
        &self,
    ) -> terrars::ListRef<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.broker_instance_options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {
    type O = terrars::BlockAssignable<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {}

impl BuildDataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {
    pub fn build(self) -> DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {
        DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones {
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZonesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZonesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZonesRef {
        DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZonesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZonesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std
    ::option
    ::Option<terrars::SetField<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    engine_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    host_instance_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    storage_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_deployment_modes: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    supported_engine_versions: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {
    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZones>,
                    >,
    ) -> Self {
        self.availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `engine_type`.\n"]
    pub fn set_engine_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.engine_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `host_instance_type`.\n"]
    pub fn set_host_instance_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.host_instance_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `storage_type`.\n"]
    pub fn set_storage_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.storage_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_deployment_modes`.\n"]
    pub fn set_supported_deployment_modes(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_deployment_modes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `supported_engine_versions`.\n"]
    pub fn set_supported_engine_versions(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.supported_engine_versions = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {
    type O = terrars::BlockAssignable<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {}

impl BuildDataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {
    pub fn build(self) -> DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {
        DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions {
            availability_zones: core::default::Default::default(),
            engine_type: core::default::Default::default(),
            host_instance_type: core::default::Default::default(),
            storage_type: core::default::Default::default(),
            supported_deployment_modes: core::default::Default::default(),
            supported_engine_versions: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef {
        DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(
        &self,
    ) -> terrars::SetRef<DataMqBrokerInstanceTypeOfferings_BrokerInstanceOptions_AvailabilityZonesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `engine_type` after provisioning.\n"]
    pub fn engine_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.engine_type", self.base))
    }

    #[doc = "Get a reference to the value of field `host_instance_type` after provisioning.\n"]
    pub fn host_instance_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.host_instance_type", self.base))
    }

    #[doc = "Get a reference to the value of field `storage_type` after provisioning.\n"]
    pub fn storage_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.storage_type", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_deployment_modes` after provisioning.\n"]
    pub fn supported_deployment_modes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.supported_deployment_modes", self.base))
    }

    #[doc = "Get a reference to the value of field `supported_engine_versions` after provisioning.\n"]
    pub fn supported_engine_versions(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.supported_engine_versions", self.base))
    }
}
