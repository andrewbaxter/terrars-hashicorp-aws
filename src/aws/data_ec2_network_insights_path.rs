#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2NetworkInsightsPathData_ {
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
    network_insights_path_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2NetworkInsightsPath_Filter>>,
    dynamic: DataEc2NetworkInsightsPathDynamic,
}

struct DataEc2NetworkInsightsPath_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2NetworkInsightsPathData_>,
}

#[derive(Clone)]
pub struct DataEc2NetworkInsightsPath(std::rc::Rc<DataEc2NetworkInsightsPath_>);

impl DataEc2NetworkInsightsPath {
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

    #[doc = "Set the field `network_insights_path_id`.\n"]
    pub fn set_network_insights_path_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().network_insights_path_id = std::option::Option::Some(v.into());
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
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2NetworkInsightsPath_Filter>>,
    ) -> Self {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(&self) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_insights_path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2NetworkInsightsPath {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2NetworkInsightsPath { }

impl terrars::ToListMappable for DataEc2NetworkInsightsPath {
    type O = terrars::ListRef<DataEc2NetworkInsightsPathRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2NetworkInsightsPath_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_network_insights_path".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2NetworkInsightsPath {
    pub tf_id: std::string::String,
}

impl BuildDataEc2NetworkInsightsPath {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2NetworkInsightsPath {
        let out = DataEc2NetworkInsightsPath(std::rc::Rc::new(DataEc2NetworkInsightsPath_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2NetworkInsightsPathData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                network_insights_path_id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2NetworkInsightsPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsPathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2NetworkInsightsPathRef {
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

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_arn` after provisioning.\n"]
    pub fn destination_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_ip` after provisioning.\n"]
    pub fn destination_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `destination_port` after provisioning.\n"]
    pub fn destination_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_port", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_destination` after provisioning.\n"]
    pub fn filter_at_destination(&self) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtDestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_destination", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_at_source` after provisioning.\n"]
    pub fn filter_at_source(&self) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_at_source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_insights_path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_arn` after provisioning.\n"]
    pub fn source_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_ip` after provisioning.\n"]
    pub fn source_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataEc2NetworkInsightsPathDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2NetworkInsightsPath_Filter>>,
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {}

impl BuildDataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
    pub fn build(self) -> DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
        DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
        DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    #[doc = "Set the field `from_port`.\n"]
    pub fn set_from_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to_port`.\n"]
    pub fn set_to_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {}

impl BuildDataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
    pub fn build(self) -> DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
        DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange {
            from_port: core::default::Default::default(),
            to_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
        DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from_port` after provisioning.\n"]
    pub fn from_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from_port", self.base))
    }

    #[doc = "Get a reference to the value of field `to_port` after provisioning.\n"]
    pub fn to_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to_port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPath_FilterAtDestination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
}

impl DataEc2NetworkInsightsPath_FilterAtDestination {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    ) -> Self {
        self.destination_port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
    ) -> Self {
        self.source_port_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsPath_FilterAtDestination {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsPath_FilterAtDestination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPath_FilterAtDestination {}

impl BuildDataEc2NetworkInsightsPath_FilterAtDestination {
    pub fn build(self) -> DataEc2NetworkInsightsPath_FilterAtDestination {
        DataEc2NetworkInsightsPath_FilterAtDestination {
            destination_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_address: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2NetworkInsightsPath_FilterAtDestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsPath_FilterAtDestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsPath_FilterAtDestinationRef {
        DataEc2NetworkInsightsPath_FilterAtDestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPath_FilterAtDestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPath_FilterAtSource {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
}

impl DataEc2NetworkInsightsPath_FilterAtSource {
    #[doc = "Set the field `destination_address`.\n"]
    pub fn set_destination_address(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_range`.\n"]
    pub fn set_destination_port_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRange>>,
    ) -> Self {
        self.destination_port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_address`.\n"]
    pub fn set_source_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_range`.\n"]
    pub fn set_source_port_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRange>>,
    ) -> Self {
        self.source_port_range = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsPath_FilterAtSource {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsPath_FilterAtSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPath_FilterAtSource {}

impl BuildDataEc2NetworkInsightsPath_FilterAtSource {
    pub fn build(self) -> DataEc2NetworkInsightsPath_FilterAtSource {
        DataEc2NetworkInsightsPath_FilterAtSource {
            destination_address: core::default::Default::default(),
            destination_port_range: core::default::Default::default(),
            source_address: core::default::Default::default(),
            source_port_range: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2NetworkInsightsPath_FilterAtSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsPath_FilterAtSourceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2NetworkInsightsPath_FilterAtSourceRef {
        DataEc2NetworkInsightsPath_FilterAtSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPath_FilterAtSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_address` after provisioning.\n"]
    pub fn destination_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_address", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_range` after provisioning.\n"]
    pub fn destination_port_range(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtSource_DestinationPortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `source_address` after provisioning.\n"]
    pub fn source_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_address", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_range` after provisioning.\n"]
    pub fn source_port_range(&self) -> terrars::ListRef<DataEc2NetworkInsightsPath_FilterAtSource_SourcePortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_range", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsPath_Filter {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsPath_Filter { }

impl terrars::ToListMappable for DataEc2NetworkInsightsPath_Filter {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsPath_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsPath_Filter {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2NetworkInsightsPath_Filter {
    pub fn build(self) -> DataEc2NetworkInsightsPath_Filter {
        DataEc2NetworkInsightsPath_Filter {
            name: self.name,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct DataEc2NetworkInsightsPath_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsPath_FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2NetworkInsightsPath_FilterRef {
        DataEc2NetworkInsightsPath_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsPath_FilterRef {
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
