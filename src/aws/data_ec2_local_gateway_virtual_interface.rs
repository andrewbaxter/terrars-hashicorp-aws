use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2LocalGatewayVirtualInterfaceData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2LocalGatewayVirtualInterfaceFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2LocalGatewayVirtualInterfaceTimeoutsEl>,
    dynamic: DataEc2LocalGatewayVirtualInterfaceDynamic,
}

struct DataEc2LocalGatewayVirtualInterface_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2LocalGatewayVirtualInterfaceData>,
}

#[derive(Clone)]
pub struct DataEc2LocalGatewayVirtualInterface(std::rc::Rc<DataEc2LocalGatewayVirtualInterface_>);

impl DataEc2LocalGatewayVirtualInterface {
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2LocalGatewayVirtualInterfaceFilterEl>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2LocalGatewayVirtualInterfaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_address` after provisioning.\n"]
    pub fn local_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_bgp_asn` after provisioning.\n"]
    pub fn local_bgp_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_bgp_asn` after provisioning.\n"]
    pub fn peer_bgp_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataEc2LocalGatewayVirtualInterface {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2LocalGatewayVirtualInterface { }

impl terrars::ToListMappable for DataEc2LocalGatewayVirtualInterface {
    type O = terrars::ListRef<DataEc2LocalGatewayVirtualInterfaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2LocalGatewayVirtualInterface_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_local_gateway_virtual_interface".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterface {
    pub tf_id: std::string::String,
}

impl BuildDataEc2LocalGatewayVirtualInterface {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2LocalGatewayVirtualInterface {
        let out = DataEc2LocalGatewayVirtualInterface(std::rc::Rc::new(DataEc2LocalGatewayVirtualInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2LocalGatewayVirtualInterfaceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
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

pub struct DataEc2LocalGatewayVirtualInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2LocalGatewayVirtualInterfaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_address` after provisioning.\n"]
    pub fn local_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_bgp_asn` after provisioning.\n"]
    pub fn local_bgp_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_id` after provisioning.\n"]
    pub fn local_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.local_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `local_gateway_virtual_interface_ids` after provisioning.\n"]
    pub fn local_gateway_virtual_interface_ids(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(
            self.shared().clone(),
            format!("{}.local_gateway_virtual_interface_ids", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `peer_address` after provisioning.\n"]
    pub fn peer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `peer_bgp_asn` after provisioning.\n"]
    pub fn peer_bgp_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.peer_bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2LocalGatewayVirtualInterfaceFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceFilterEl { }

impl terrars::ToListMappable for DataEc2LocalGatewayVirtualInterfaceFilterEl {
    type O = terrars::BlockAssignable<DataEc2LocalGatewayVirtualInterfaceFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2LocalGatewayVirtualInterfaceFilterEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceFilterEl {
        DataEc2LocalGatewayVirtualInterfaceFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2LocalGatewayVirtualInterfaceFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2LocalGatewayVirtualInterfaceFilterElRef {
        DataEc2LocalGatewayVirtualInterfaceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceFilterElRef {
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
pub struct DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2LocalGatewayVirtualInterfaceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2LocalGatewayVirtualInterfaceTimeoutsEl {}

impl BuildDataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
    pub fn build(self) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsEl {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
        DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2LocalGatewayVirtualInterfaceTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2LocalGatewayVirtualInterfaceDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2LocalGatewayVirtualInterfaceFilterEl>>,
}
