#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataMedialiveInputData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataMedialiveInput_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataMedialiveInputData_>,
}

#[derive(Clone)]
pub struct DataMedialiveInput(std::rc::Rc<DataMedialiveInput_>);

impl DataMedialiveInput {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_channels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<DataMedialiveInput_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_devices` after provisioning.\n"]
    pub fn input_devices(&self) -> terrars::ListRef<DataMedialiveInput_InputDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_devices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_partner_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_connect_flows` after provisioning.\n"]
    pub fn media_connect_flows(&self) -> terrars::ListRef<DataMedialiveInput_MediaConnectFlowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_connect_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<DataMedialiveInput_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataMedialiveInput {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataMedialiveInput { }

impl terrars::ToListMappable for DataMedialiveInput {
    type O = terrars::ListRef<DataMedialiveInputRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataMedialiveInput_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_medialive_input".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataMedialiveInput {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDataMedialiveInput {
    pub fn build(self, stack: &mut terrars::Stack) -> DataMedialiveInput {
        let out = DataMedialiveInput(std::rc::Rc::new(DataMedialiveInput_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataMedialiveInputData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: self.id,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataMedialiveInputRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMedialiveInputRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataMedialiveInputRef {
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

    #[doc = "Get a reference to the value of field `attached_channels` after provisioning.\n"]
    pub fn attached_channels(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_channels", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destinations` after provisioning.\n"]
    pub fn destinations(&self) -> terrars::ListRef<DataMedialiveInput_DestinationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destinations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_class` after provisioning.\n"]
    pub fn input_class(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_class", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_devices` after provisioning.\n"]
    pub fn input_devices(&self) -> terrars::ListRef<DataMedialiveInput_InputDevicesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_devices", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_partner_ids` after provisioning.\n"]
    pub fn input_partner_ids(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.input_partner_ids", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `input_source_type` after provisioning.\n"]
    pub fn input_source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.input_source_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `media_connect_flows` after provisioning.\n"]
    pub fn media_connect_flows(&self) -> terrars::ListRef<DataMedialiveInput_MediaConnectFlowsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.media_connect_flows", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `sources` after provisioning.\n"]
    pub fn sources(&self) -> terrars::ListRef<DataMedialiveInput_SourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.sources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataMedialiveInput_Destinations_Vpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMedialiveInput_Destinations_Vpc {
    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataMedialiveInput_Destinations_Vpc {
    type O = terrars::BlockAssignable<DataMedialiveInput_Destinations_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMedialiveInput_Destinations_Vpc {}

impl BuildDataMedialiveInput_Destinations_Vpc {
    pub fn build(self) -> DataMedialiveInput_Destinations_Vpc {
        DataMedialiveInput_Destinations_Vpc {
            availability_zone: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMedialiveInput_Destinations_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMedialiveInput_Destinations_VpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMedialiveInput_Destinations_VpcRef {
        DataMedialiveInput_Destinations_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMedialiveInput_Destinations_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMedialiveInput_Destinations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<DataMedialiveInput_Destinations_Vpc>>,
}

impl DataMedialiveInput_Destinations {
    #[doc = "Set the field `ip`.\n"]
    pub fn set_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataMedialiveInput_Destinations_Vpc>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMedialiveInput_Destinations {
    type O = terrars::BlockAssignable<DataMedialiveInput_Destinations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMedialiveInput_Destinations {}

impl BuildDataMedialiveInput_Destinations {
    pub fn build(self) -> DataMedialiveInput_Destinations {
        DataMedialiveInput_Destinations {
            ip: core::default::Default::default(),
            port: core::default::Default::default(),
            url: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMedialiveInput_DestinationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMedialiveInput_DestinationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMedialiveInput_DestinationsRef {
        DataMedialiveInput_DestinationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMedialiveInput_DestinationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip` after provisioning.\n"]
    pub fn ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<DataMedialiveInput_Destinations_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMedialiveInput_InputDevices {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMedialiveInput_InputDevices {
    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMedialiveInput_InputDevices {
    type O = terrars::BlockAssignable<DataMedialiveInput_InputDevices>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMedialiveInput_InputDevices {}

impl BuildDataMedialiveInput_InputDevices {
    pub fn build(self) -> DataMedialiveInput_InputDevices {
        DataMedialiveInput_InputDevices { id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMedialiveInput_InputDevicesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMedialiveInput_InputDevicesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMedialiveInput_InputDevicesRef {
        DataMedialiveInput_InputDevicesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMedialiveInput_InputDevicesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMedialiveInput_MediaConnectFlows {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMedialiveInput_MediaConnectFlows {
    #[doc = "Set the field `flow_arn`.\n"]
    pub fn set_flow_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.flow_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMedialiveInput_MediaConnectFlows {
    type O = terrars::BlockAssignable<DataMedialiveInput_MediaConnectFlows>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMedialiveInput_MediaConnectFlows {}

impl BuildDataMedialiveInput_MediaConnectFlows {
    pub fn build(self) -> DataMedialiveInput_MediaConnectFlows {
        DataMedialiveInput_MediaConnectFlows { flow_arn: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataMedialiveInput_MediaConnectFlowsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMedialiveInput_MediaConnectFlowsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMedialiveInput_MediaConnectFlowsRef {
        DataMedialiveInput_MediaConnectFlowsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMedialiveInput_MediaConnectFlowsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_arn` after provisioning.\n"]
    pub fn flow_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.flow_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataMedialiveInput_Sources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    password_param: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    username: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataMedialiveInput_Sources {
    #[doc = "Set the field `password_param`.\n"]
    pub fn set_password_param(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.password_param = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `username`.\n"]
    pub fn set_username(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.username = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataMedialiveInput_Sources {
    type O = terrars::BlockAssignable<DataMedialiveInput_Sources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataMedialiveInput_Sources {}

impl BuildDataMedialiveInput_Sources {
    pub fn build(self) -> DataMedialiveInput_Sources {
        DataMedialiveInput_Sources {
            password_param: core::default::Default::default(),
            url: core::default::Default::default(),
            username: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataMedialiveInput_SourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataMedialiveInput_SourcesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataMedialiveInput_SourcesRef {
        DataMedialiveInput_SourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataMedialiveInput_SourcesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `password_param` after provisioning.\n"]
    pub fn password_param(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.password_param", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `username` after provisioning.\n"]
    pub fn username(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.username", self.base))
    }
}
