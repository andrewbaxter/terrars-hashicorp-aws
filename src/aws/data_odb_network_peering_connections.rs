#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataOdbNetworkPeeringConnectionsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataOdbNetworkPeeringConnections_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataOdbNetworkPeeringConnectionsData_>,
}

#[derive(Clone)]
pub struct DataOdbNetworkPeeringConnections(std::rc::Rc<DataOdbNetworkPeeringConnections_>);

impl DataOdbNetworkPeeringConnections {
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

    #[doc =
        "Get a reference to the value of field `odb_peering_connections` after provisioning.\nThe list of ODB peering connections. A summary of an ODB peering connection."]
    pub fn odb_peering_connections(&self) -> terrars::ListRef<DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.odb_peering_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataOdbNetworkPeeringConnections {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataOdbNetworkPeeringConnections { }

impl terrars::ToListMappable for DataOdbNetworkPeeringConnections {
    type O = terrars::ListRef<DataOdbNetworkPeeringConnectionsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataOdbNetworkPeeringConnections_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_odb_network_peering_connections".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataOdbNetworkPeeringConnections {
    pub tf_id: std::string::String,
}

impl BuildDataOdbNetworkPeeringConnections {
    pub fn build(self, stack: &mut terrars::Stack) -> DataOdbNetworkPeeringConnections {
        let out = DataOdbNetworkPeeringConnections(std::rc::Rc::new(DataOdbNetworkPeeringConnections_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataOdbNetworkPeeringConnectionsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataOdbNetworkPeeringConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetworkPeeringConnectionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataOdbNetworkPeeringConnectionsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `odb_peering_connections` after provisioning.\nThe list of ODB peering connections. A summary of an ODB peering connection."]
    pub fn odb_peering_connections(&self) -> terrars::ListRef<DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.odb_peering_connections", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataOdbNetworkPeeringConnections_OdbPeeringConnections {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    display_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    odb_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    peer_network_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataOdbNetworkPeeringConnections_OdbPeeringConnections {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `display_name`.\n"]
    pub fn set_display_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.display_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `odb_network_arn`.\n"]
    pub fn set_odb_network_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.odb_network_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `peer_network_arn`.\n"]
    pub fn set_peer_network_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.peer_network_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataOdbNetworkPeeringConnections_OdbPeeringConnections {
    type O = terrars::BlockAssignable<DataOdbNetworkPeeringConnections_OdbPeeringConnections>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataOdbNetworkPeeringConnections_OdbPeeringConnections {}

impl BuildDataOdbNetworkPeeringConnections_OdbPeeringConnections {
    pub fn build(self) -> DataOdbNetworkPeeringConnections_OdbPeeringConnections {
        DataOdbNetworkPeeringConnections_OdbPeeringConnections {
            arn: core::default::Default::default(),
            display_name: core::default::Default::default(),
            id: core::default::Default::default(),
            odb_network_arn: core::default::Default::default(),
            peer_network_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef {
        DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataOdbNetworkPeeringConnections_OdbPeeringConnectionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `display_name` after provisioning.\n"]
    pub fn display_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.display_name", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `odb_network_arn` after provisioning.\n"]
    pub fn odb_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.odb_network_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `peer_network_arn` after provisioning.\n"]
    pub fn peer_network_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_network_arn", self.base))
    }
}
