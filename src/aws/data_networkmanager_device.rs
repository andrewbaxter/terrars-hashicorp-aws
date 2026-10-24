use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkmanagerDeviceData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    device_id: terrars::PrimField<std::string::String>,
    global_network_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataNetworkmanagerDevice_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkmanagerDeviceData>,
}

#[derive(Clone)]
pub struct DataNetworkmanagerDevice(std::rc::Rc<DataNetworkmanagerDevice_>);

impl DataNetworkmanagerDevice {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_location` after provisioning.\n"]
    pub fn aws_location(&self) -> terrars::ListRef<DataNetworkmanagerDeviceAwsLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_id` after provisioning.\n"]
    pub fn device_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<DataNetworkmanagerDeviceLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vendor` after provisioning.\n"]
    pub fn vendor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkmanagerDevice {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkmanagerDevice { }

impl terrars::ToListMappable for DataNetworkmanagerDevice {
    type O = terrars::ListRef<DataNetworkmanagerDeviceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkmanagerDevice_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_networkmanager_device".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkmanagerDevice {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub device_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub global_network_id: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerDevice {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkmanagerDevice {
        let out = DataNetworkmanagerDevice(std::rc::Rc::new(DataNetworkmanagerDevice_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkmanagerDeviceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                device_id: self.device_id,
                global_network_id: self.global_network_id,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkmanagerDeviceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerDeviceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkmanagerDeviceRef {
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

    #[doc = "Get a reference to the value of field `aws_location` after provisioning.\n"]
    pub fn aws_location(&self) -> terrars::ListRef<DataNetworkmanagerDeviceAwsLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.aws_location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `device_id` after provisioning.\n"]
    pub fn device_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::ListRef<DataNetworkmanagerDeviceLocationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.location", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `model` after provisioning.\n"]
    pub fn model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.model", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `serial_number` after provisioning.\n"]
    pub fn serial_number(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.serial_number", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `site_id` after provisioning.\n"]
    pub fn site_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.site_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vendor` after provisioning.\n"]
    pub fn vendor(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vendor", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerDeviceAwsLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerDeviceAwsLocationEl {
    #[doc = "Set the field `subnet_arn`.\n"]
    pub fn set_subnet_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `zone`.\n"]
    pub fn set_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerDeviceAwsLocationEl {
    type O = terrars::BlockAssignable<DataNetworkmanagerDeviceAwsLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerDeviceAwsLocationEl {}

impl BuildDataNetworkmanagerDeviceAwsLocationEl {
    pub fn build(self) -> DataNetworkmanagerDeviceAwsLocationEl {
        DataNetworkmanagerDeviceAwsLocationEl {
            subnet_arn: core::default::Default::default(),
            zone: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerDeviceAwsLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerDeviceAwsLocationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkmanagerDeviceAwsLocationElRef {
        DataNetworkmanagerDeviceAwsLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerDeviceAwsLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_arn` after provisioning.\n"]
    pub fn subnet_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `zone` after provisioning.\n"]
    pub fn zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerDeviceLocationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    latitude: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    longitude: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkmanagerDeviceLocationEl {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `latitude`.\n"]
    pub fn set_latitude(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.latitude = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `longitude`.\n"]
    pub fn set_longitude(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.longitude = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerDeviceLocationEl {
    type O = terrars::BlockAssignable<DataNetworkmanagerDeviceLocationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerDeviceLocationEl {}

impl BuildDataNetworkmanagerDeviceLocationEl {
    pub fn build(self) -> DataNetworkmanagerDeviceLocationEl {
        DataNetworkmanagerDeviceLocationEl {
            address: core::default::Default::default(),
            latitude: core::default::Default::default(),
            longitude: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkmanagerDeviceLocationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerDeviceLocationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkmanagerDeviceLocationElRef {
        DataNetworkmanagerDeviceLocationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerDeviceLocationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `latitude` after provisioning.\n"]
    pub fn latitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.latitude", self.base))
    }

    #[doc = "Get a reference to the value of field `longitude` after provisioning.\n"]
    pub fn longitude(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.longitude", self.base))
    }
}
