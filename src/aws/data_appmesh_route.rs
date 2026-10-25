#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAppmeshRouteData_ {
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
    mesh_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mesh_owner: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    virtual_router_name: terrars::PrimField<std::string::String>,
}

struct DataAppmeshRoute_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAppmeshRouteData_>,
}

#[derive(Clone)]
pub struct DataAppmeshRoute(std::rc::Rc<DataAppmeshRoute_>);

impl DataAppmeshRoute {
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

    #[doc = "Set the field `mesh_owner`.\n"]
    pub fn set_mesh_owner(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().mesh_owner = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.extract_ref()))
    }
}

impl terrars::Referable for DataAppmeshRoute {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAppmeshRoute { }

impl terrars::ToListMappable for DataAppmeshRoute {
    type O = terrars::ListRef<DataAppmeshRouteRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAppmeshRoute_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_appmesh_route".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAppmeshRoute {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub mesh_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub virtual_router_name: terrars::PrimField<std::string::String>,
}

impl BuildDataAppmeshRoute {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAppmeshRoute {
        let out = DataAppmeshRoute(std::rc::Rc::new(DataAppmeshRoute_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAppmeshRouteData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                mesh_name: self.mesh_name,
                mesh_owner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                virtual_router_name: self.virtual_router_name,
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAppmeshRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAppmeshRouteRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_updated_date` after provisioning.\n"]
    pub fn last_updated_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_updated_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_name` after provisioning.\n"]
    pub fn mesh_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mesh_owner` after provisioning.\n"]
    pub fn mesh_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mesh_owner", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `resource_owner` after provisioning.\n"]
    pub fn resource_owner(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_owner", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `spec` after provisioning.\n"]
    pub fn spec(&self) -> terrars::ListRef<DataAppmeshRoute_SpecRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.spec", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `virtual_router_name` after provisioning.\n"]
    pub fn virtual_router_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_router_name", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    virtual_node: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weight: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `virtual_node`.\n"]
    pub fn set_virtual_node(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.virtual_node = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `weight`.\n"]
    pub fn set_weight(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.weight = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {}

impl BuildDataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
    pub fn build(self) -> DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
        DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget {
            port: core::default::Default::default(),
            virtual_node: core::default::Default::default(),
            weight: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
        DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `virtual_node` after provisioning.\n"]
    pub fn virtual_node(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.virtual_node", self.base))
    }

    #[doc = "Get a reference to the value of field `weight` after provisioning.\n"]
    pub fn weight(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.weight", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    weighted_target: std::option::Option<terrars::SetField<DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>>,
}

impl DataAppmeshRoute_Spec_TcpRoute_Action {
    #[doc = "Set the field `weighted_target`.\n"]
    pub fn set_weighted_target(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTarget>>,
    ) -> Self {
        self.weighted_target = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_TcpRoute_Action {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_TcpRoute_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_TcpRoute_Action {}

impl BuildDataAppmeshRoute_Spec_TcpRoute_Action {
    pub fn build(self) -> DataAppmeshRoute_Spec_TcpRoute_Action {
        DataAppmeshRoute_Spec_TcpRoute_Action { weighted_target: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_TcpRoute_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_TcpRoute_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_TcpRoute_ActionRef {
        DataAppmeshRoute_Spec_TcpRoute_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_TcpRoute_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `weighted_target` after provisioning.\n"]
    pub fn weighted_target(&self) -> terrars::SetRef<DataAppmeshRoute_Spec_TcpRoute_Action_WeightedTargetRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.weighted_target", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
        DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
        DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    range: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suffix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `range`.\n"]
    pub fn set_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_Range>>,
    ) -> Self {
        self.range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suffix`.\n"]
    pub fn set_suffix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suffix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
        DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match {
            exact: core::default::Default::default(),
            prefix: core::default::Default::default(),
            range: core::default::Default::default(),
            regex: core::default::Default::default(),
            suffix: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
        DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `range` after provisioning.\n"]
    pub fn range(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match_RangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.range", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }

    #[doc = "Get a reference to the value of field `suffix` after provisioning.\n"]
    pub fn suffix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suffix", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
        DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
        DataAppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRoute_Match_MetadataRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    metadata: std::option::Option<terrars::SetField<DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    service_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute_Match {
    #[doc = "Set the field `metadata`.\n"]
    pub fn set_metadata(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAppmeshRoute_Spec_GrpcRoute_Match_Metadata>>,
    ) -> Self {
        self.metadata = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method_name`.\n"]
    pub fn set_method_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `service_name`.\n"]
    pub fn set_service_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.service_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute_Match {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute_Match {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute_Match {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute_Match {
        DataAppmeshRoute_Spec_GrpcRoute_Match {
            metadata: core::default::Default::default(),
            method_name: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            service_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_GrpcRoute_MatchRef {
        DataAppmeshRoute_Spec_GrpcRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metadata` after provisioning.\n"]
    pub fn metadata(&self) -> terrars::SetRef<DataAppmeshRoute_Spec_GrpcRoute_Match_MetadataRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.metadata", self.base))
    }

    #[doc = "Get a reference to the value of field `method_name` after provisioning.\n"]
    pub fn method_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method_name", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `service_name` after provisioning.\n"]
    pub fn service_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.service_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
        DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
        DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std
    ::option
    ::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    #[doc = "Set the field `grpc_retry_events`.\n"]
    pub fn set_grpc_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.grpc_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.http_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    ) -> Self {
        self.per_retry_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tcp_retry_events = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
        DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy {
            grpc_retry_events: core::default::Default::default(),
            http_retry_events: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            tcp_retry_events: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
        DataAppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRoute_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_retry_events` after provisioning.\n"]
    pub fn grpc_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.grpc_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {}

impl BuildDataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
    pub fn build(self) -> DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
        DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
        DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    #[doc = "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
        DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest {
            unit: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
        DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_request: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute_Timeout {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    ) -> Self {
        self.idle = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_request`.\n"]
    pub fn set_per_request(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequest>>,
    ) -> Self {
        self.per_request = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute_Timeout {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute_Timeout {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute_Timeout {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute_Timeout {
        DataAppmeshRoute_Spec_GrpcRoute_Timeout {
            idle: core::default::Default::default(),
            per_request: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef {
        DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }

    #[doc = "Get a reference to the value of field `per_request` after provisioning.\n"]
    pub fn per_request(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_Timeout_PerRequestRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_request", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_GrpcRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout>>,
}

impl DataAppmeshRoute_Spec_GrpcRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy>>,
    ) -> Self {
        self.retry_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout>>,
    ) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_GrpcRoute {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_GrpcRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_GrpcRoute {}

impl BuildDataAppmeshRoute_Spec_GrpcRoute {
    pub fn build(self) -> DataAppmeshRoute_Spec_GrpcRoute {
        DataAppmeshRoute_Spec_GrpcRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_GrpcRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_GrpcRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_GrpcRouteRef {
        DataAppmeshRoute_Spec_GrpcRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_GrpcRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_Header {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invert: std::option::Option<terrars::PrimField<bool>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_Header {
    #[doc = "Set the field `invert`.\n"]
    pub fn set_invert(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.invert = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Header_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match_Header {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match_Header>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match_Header {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match_Header {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match_Header {
        DataAppmeshRoute_Spec_HttpRoute_Match_Header {
            invert: core::default::Default::default(),
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
        DataAppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_HeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `invert` after provisioning.\n"]
    pub fn invert(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invert", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_Match_Header_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_Path {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    regex: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_Path {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `regex`.\n"]
    pub fn set_regex(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.regex = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match_Path {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match_Path>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match_Path {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match_Path {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match_Path {
        DataAppmeshRoute_Spec_HttpRoute_Match_Path {
            exact: core::default::Default::default(),
            regex: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_PathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_Match_PathRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_HttpRoute_Match_PathRef {
        DataAppmeshRoute_Spec_HttpRoute_Match_PathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_PathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }

    #[doc = "Get a reference to the value of field `regex` after provisioning.\n"]
    pub fn regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.regex", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exact: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    #[doc = "Set the field `exact`.\n"]
    pub fn set_exact(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.exact = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match {
        DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match { exact: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
        DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exact` after provisioning.\n"]
    pub fn exact(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exact", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
        DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter {
            match_: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
        DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    header: std::option::Option<terrars::SetField<DataAppmeshRoute_Spec_HttpRoute_Match_Header>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    path: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Path>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_parameter: std::option::Option<terrars::SetField<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scheme: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_Match {
    #[doc = "Set the field `header`.\n"]
    pub fn set_header(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAppmeshRoute_Spec_HttpRoute_Match_Header>>,
    ) -> Self {
        self.header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.method = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `path`.\n"]
    pub fn set_path(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match_Path>>,
    ) -> Self {
        self.path = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix`.\n"]
    pub fn set_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `query_parameter`.\n"]
    pub fn set_query_parameter(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameter>>,
    ) -> Self {
        self.query_parameter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scheme`.\n"]
    pub fn set_scheme(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.scheme = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_Match {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_Match {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_Match {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_Match {
        DataAppmeshRoute_Spec_HttpRoute_Match {
            header: core::default::Default::default(),
            method: core::default::Default::default(),
            path: core::default::Default::default(),
            port: core::default::Default::default(),
            prefix: core::default::Default::default(),
            query_parameter: core::default::Default::default(),
            scheme: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_HttpRoute_MatchRef {
        DataAppmeshRoute_Spec_HttpRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header` after provisioning.\n"]
    pub fn header(&self) -> terrars::SetRef<DataAppmeshRoute_Spec_HttpRoute_Match_HeaderRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.header", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `path` after provisioning.\n"]
    pub fn path(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_Match_PathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.path", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix` after provisioning.\n"]
    pub fn prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `query_parameter` after provisioning.\n"]
    pub fn query_parameter(&self) -> terrars::SetRef<DataAppmeshRoute_Spec_HttpRoute_Match_QueryParameterRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.query_parameter", self.base))
    }

    #[doc = "Get a reference to the value of field `scheme` after provisioning.\n"]
    pub fn scheme(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scheme", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute_RetryPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_retries: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    per_retry_timeout: std
    ::option
    ::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_retry_events: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataAppmeshRoute_Spec_HttpRoute_RetryPolicy {
    #[doc = "Set the field `http_retry_events`.\n"]
    pub fn set_http_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.http_retry_events = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_retries`.\n"]
    pub fn set_max_retries(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.max_retries = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `per_retry_timeout`.\n"]
    pub fn set_per_retry_timeout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeout>>,
    ) -> Self {
        self.per_retry_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_retry_events`.\n"]
    pub fn set_tcp_retry_events(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tcp_retry_events = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute_RetryPolicy {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute_RetryPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute_RetryPolicy {}

impl BuildDataAppmeshRoute_Spec_HttpRoute_RetryPolicy {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute_RetryPolicy {
        DataAppmeshRoute_Spec_HttpRoute_RetryPolicy {
            http_retry_events: core::default::Default::default(),
            max_retries: core::default::Default::default(),
            per_retry_timeout: core::default::Default::default(),
            tcp_retry_events: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
        DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `http_retry_events` after provisioning.\n"]
    pub fn http_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.http_retry_events", self.base))
    }

    #[doc = "Get a reference to the value of field `max_retries` after provisioning.\n"]
    pub fn max_retries(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_retries", self.base))
    }

    #[doc = "Get a reference to the value of field `per_retry_timeout` after provisioning.\n"]
    pub fn per_retry_timeout(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_RetryPolicy_PerRetryTimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.per_retry_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_retry_events` after provisioning.\n"]
    pub fn tcp_retry_events(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.tcp_retry_events", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_Http2Route {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout>>,
}

impl DataAppmeshRoute_Spec_Http2Route {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    ) -> Self {
        self.retry_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout>>,
    ) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_Http2Route {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_Http2Route>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_Http2Route {}

impl BuildDataAppmeshRoute_Spec_Http2Route {
    pub fn build(self) -> DataAppmeshRoute_Spec_Http2Route {
        DataAppmeshRoute_Spec_Http2Route {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_Http2RouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_Http2RouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_Http2RouteRef {
        DataAppmeshRoute_Spec_Http2RouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_Http2RouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_HttpRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_policy: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout>>,
}

impl DataAppmeshRoute_Spec_HttpRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_policy`.\n"]
    pub fn set_retry_policy(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute_RetryPolicy>>,
    ) -> Self {
        self.retry_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute_Timeout>>,
    ) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_HttpRoute {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_HttpRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_HttpRoute {}

impl BuildDataAppmeshRoute_Spec_HttpRoute {
    pub fn build(self) -> DataAppmeshRoute_Spec_HttpRoute {
        DataAppmeshRoute_Spec_HttpRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            retry_policy: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_HttpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_HttpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_HttpRouteRef {
        DataAppmeshRoute_Spec_HttpRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_HttpRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_policy` after provisioning.\n"]
    pub fn retry_policy(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRoute_RetryPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retry_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Match {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataAppmeshRoute_Spec_TcpRoute_Match {
    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_TcpRoute_Match {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_TcpRoute_Match>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_TcpRoute_Match {}

impl BuildDataAppmeshRoute_Spec_TcpRoute_Match {
    pub fn build(self) -> DataAppmeshRoute_Spec_TcpRoute_Match {
        DataAppmeshRoute_Spec_TcpRoute_Match { port: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_TcpRoute_MatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_TcpRoute_MatchRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_TcpRoute_MatchRef {
        DataAppmeshRoute_Spec_TcpRoute_MatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_TcpRoute_MatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_TcpRoute_Timeout {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    idle: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
}

impl DataAppmeshRoute_Spec_TcpRoute_Timeout {
    #[doc = "Set the field `idle`.\n"]
    pub fn set_idle(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Timeout_Idle>>,
    ) -> Self {
        self.idle = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_TcpRoute_Timeout {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_TcpRoute_Timeout>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_TcpRoute_Timeout {}

impl BuildDataAppmeshRoute_Spec_TcpRoute_Timeout {
    pub fn build(self) -> DataAppmeshRoute_Spec_TcpRoute_Timeout {
        DataAppmeshRoute_Spec_TcpRoute_Timeout { idle: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_TcpRoute_TimeoutRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_TcpRoute_TimeoutRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_TcpRoute_TimeoutRef {
        DataAppmeshRoute_Spec_TcpRoute_TimeoutRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_TcpRoute_TimeoutRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `idle` after provisioning.\n"]
    pub fn idle(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_Timeout_IdleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.idle", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec_TcpRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    #[serde(rename = "match", skip_serializing_if = "std::option::Option::is_none")]
    match_: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Match>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeout: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Timeout>>,
}

impl DataAppmeshRoute_Spec_TcpRoute {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Action>>,
    ) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_`.\n"]
    pub fn set_match(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Match>>,
    ) -> Self {
        self.match_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeout`.\n"]
    pub fn set_timeout(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute_Timeout>>,
    ) -> Self {
        self.timeout = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec_TcpRoute {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec_TcpRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec_TcpRoute {}

impl BuildDataAppmeshRoute_Spec_TcpRoute {
    pub fn build(self) -> DataAppmeshRoute_Spec_TcpRoute {
        DataAppmeshRoute_Spec_TcpRoute {
            action: core::default::Default::default(),
            match_: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_Spec_TcpRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_Spec_TcpRouteRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_Spec_TcpRouteRef {
        DataAppmeshRoute_Spec_TcpRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_Spec_TcpRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `match_` after provisioning.\n"]
    pub fn match_(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_MatchRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.match", self.base))
    }

    #[doc = "Get a reference to the value of field `timeout` after provisioning.\n"]
    pub fn timeout(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRoute_TimeoutRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.timeout", self.base))
    }
}

#[derive(Serialize)]
pub struct DataAppmeshRoute_Spec {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    grpc_route: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http2_route: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_Http2Route>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    http_route: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_route: std::option::Option<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute>>,
}

impl DataAppmeshRoute_Spec {
    #[doc = "Set the field `grpc_route`.\n"]
    pub fn set_grpc_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_GrpcRoute>>,
    ) -> Self {
        self.grpc_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http2_route`.\n"]
    pub fn set_http2_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_Http2Route>>,
    ) -> Self {
        self.http2_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `http_route`.\n"]
    pub fn set_http_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_HttpRoute>>,
    ) -> Self {
        self.http_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tcp_route`.\n"]
    pub fn set_tcp_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAppmeshRoute_Spec_TcpRoute>>,
    ) -> Self {
        self.tcp_route = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAppmeshRoute_Spec {
    type O = terrars::BlockAssignable<DataAppmeshRoute_Spec>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAppmeshRoute_Spec {}

impl BuildDataAppmeshRoute_Spec {
    pub fn build(self) -> DataAppmeshRoute_Spec {
        DataAppmeshRoute_Spec {
            grpc_route: core::default::Default::default(),
            http2_route: core::default::Default::default(),
            http_route: core::default::Default::default(),
            priority: core::default::Default::default(),
            tcp_route: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAppmeshRoute_SpecRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAppmeshRoute_SpecRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAppmeshRoute_SpecRef {
        DataAppmeshRoute_SpecRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAppmeshRoute_SpecRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `grpc_route` after provisioning.\n"]
    pub fn grpc_route(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_GrpcRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.grpc_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http2_route` after provisioning.\n"]
    pub fn http2_route(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_Http2RouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http2_route", self.base))
    }

    #[doc = "Get a reference to the value of field `http_route` after provisioning.\n"]
    pub fn http_route(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_HttpRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.http_route", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `tcp_route` after provisioning.\n"]
    pub fn tcp_route(&self) -> terrars::ListRef<DataAppmeshRoute_Spec_TcpRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tcp_route", self.base))
    }
}
