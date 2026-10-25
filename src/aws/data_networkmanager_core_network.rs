#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkmanagerCoreNetworkData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    core_network_id: terrars::PrimField<std::string::String>,
}

struct DataNetworkmanagerCoreNetwork_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkmanagerCoreNetworkData_>,
}

#[derive(Clone)]
pub struct DataNetworkmanagerCoreNetwork(std::rc::Rc<DataNetworkmanagerCoreNetwork_>);

impl DataNetworkmanagerCoreNetwork {
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edges` after provisioning.\n"]
    pub fn edges(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_EdgesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_function_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_SegmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkmanagerCoreNetwork {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkmanagerCoreNetwork { }

impl terrars::ToListMappable for DataNetworkmanagerCoreNetwork {
    type O = terrars::ListRef<DataNetworkmanagerCoreNetworkRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkmanagerCoreNetwork_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_networkmanager_core_network".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkmanagerCoreNetwork {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub core_network_id: terrars::PrimField<std::string::String>,
}

impl BuildDataNetworkmanagerCoreNetwork {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkmanagerCoreNetwork {
        let out = DataNetworkmanagerCoreNetwork(std::rc::Rc::new(DataNetworkmanagerCoreNetwork_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkmanagerCoreNetworkData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                core_network_id: self.core_network_id,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkmanagerCoreNetworkRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetworkRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkmanagerCoreNetworkRef {
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

    #[doc = "Get a reference to the value of field `core_network_id` after provisioning.\n"]
    pub fn core_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.core_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `edges` after provisioning.\n"]
    pub fn edges(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_EdgesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edges", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_network_id` after provisioning.\n"]
    pub fn global_network_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.global_network_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_function_groups` after provisioning.\n"]
    pub fn network_function_groups(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_function_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_SegmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segments", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetwork_Edges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    asn: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inside_cidr_blocks: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataNetworkmanagerCoreNetwork_Edges {
    #[doc = "Set the field `asn`.\n"]
    pub fn set_asn(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.asn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `edge_location`.\n"]
    pub fn set_edge_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.edge_location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inside_cidr_blocks`.\n"]
    pub fn set_inside_cidr_blocks(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.inside_cidr_blocks = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetwork_Edges {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetwork_Edges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetwork_Edges {}

impl BuildDataNetworkmanagerCoreNetwork_Edges {
    pub fn build(self) -> DataNetworkmanagerCoreNetwork_Edges {
        DataNetworkmanagerCoreNetwork_Edges {
            asn: core::default::Default::default(),
            edge_location: core::default::Default::default(),
            inside_cidr_blocks: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetwork_EdgesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetwork_EdgesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkmanagerCoreNetwork_EdgesRef {
        DataNetworkmanagerCoreNetwork_EdgesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetwork_EdgesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `asn` after provisioning.\n"]
    pub fn asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.asn", self.base))
    }

    #[doc = "Get a reference to the value of field `edge_location` after provisioning.\n"]
    pub fn edge_location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.edge_location", self.base))
    }

    #[doc = "Get a reference to the value of field `inside_cidr_blocks` after provisioning.\n"]
    pub fn inside_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inside_cidr_blocks", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_to: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send_via: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {
    #[doc = "Set the field `send_to`.\n"]
    pub fn set_send_to(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.send_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `send_via`.\n"]
    pub fn set_send_via(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.send_via = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {}

impl BuildDataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {
    pub fn build(self) -> DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {
        DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments {
            send_to: core::default::Default::default(),
            send_via: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_SegmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_SegmentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_SegmentsRef {
        DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_SegmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_SegmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `send_to` after provisioning.\n"]
    pub fn send_to(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.send_to", self.base))
    }

    #[doc = "Get a reference to the value of field `send_via` after provisioning.\n"]
    pub fn send_via(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.send_via", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetwork_NetworkFunctionGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    segments: std::option::Option<terrars::ListField<DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments>>,
}

impl DataNetworkmanagerCoreNetwork_NetworkFunctionGroups {
    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.edge_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `segments`.\n"]
    pub fn set_segments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_Segments>>,
    ) -> Self {
        self.segments = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetwork_NetworkFunctionGroups {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetwork_NetworkFunctionGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetwork_NetworkFunctionGroups {}

impl BuildDataNetworkmanagerCoreNetwork_NetworkFunctionGroups {
    pub fn build(self) -> DataNetworkmanagerCoreNetwork_NetworkFunctionGroups {
        DataNetworkmanagerCoreNetwork_NetworkFunctionGroups {
            edge_locations: core::default::Default::default(),
            name: core::default::Default::default(),
            segments: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef {
        DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetwork_NetworkFunctionGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `segments` after provisioning.\n"]
    pub fn segments(&self) -> terrars::ListRef<DataNetworkmanagerCoreNetwork_NetworkFunctionGroups_SegmentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.segments", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkmanagerCoreNetwork_Segments {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    edge_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    shared_segments: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DataNetworkmanagerCoreNetwork_Segments {
    #[doc = "Set the field `edge_locations`.\n"]
    pub fn set_edge_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.edge_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `shared_segments`.\n"]
    pub fn set_shared_segments(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.shared_segments = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkmanagerCoreNetwork_Segments {
    type O = terrars::BlockAssignable<DataNetworkmanagerCoreNetwork_Segments>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkmanagerCoreNetwork_Segments {}

impl BuildDataNetworkmanagerCoreNetwork_Segments {
    pub fn build(self) -> DataNetworkmanagerCoreNetwork_Segments {
        DataNetworkmanagerCoreNetwork_Segments {
            edge_locations: core::default::Default::default(),
            name: core::default::Default::default(),
            shared_segments: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkmanagerCoreNetwork_SegmentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkmanagerCoreNetwork_SegmentsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkmanagerCoreNetwork_SegmentsRef {
        DataNetworkmanagerCoreNetwork_SegmentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkmanagerCoreNetwork_SegmentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `edge_locations` after provisioning.\n"]
    pub fn edge_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.edge_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `shared_segments` after provisioning.\n"]
    pub fn shared_segments(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.shared_segments", self.base))
    }
}
