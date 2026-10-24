use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayVpnAttachmentData {
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
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2TransitGatewayVpnAttachmentFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2TransitGatewayVpnAttachmentTimeoutsEl>,
    dynamic: DataEc2TransitGatewayVpnAttachmentDynamic,
}

struct DataEc2TransitGatewayVpnAttachment_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2TransitGatewayVpnAttachmentData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGatewayVpnAttachment(std::rc::Rc<DataEc2TransitGatewayVpnAttachment_>);

impl DataEc2TransitGatewayVpnAttachment {
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

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_connection_id`.\n"]
    pub fn set_vpn_connection_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpn_connection_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2TransitGatewayVpnAttachmentFilterEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2TransitGatewayVpnAttachmentTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_connection_id` after provisioning.\n"]
    pub fn vpn_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
        DataEc2TransitGatewayVpnAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DataEc2TransitGatewayVpnAttachment {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2TransitGatewayVpnAttachment { }

impl terrars::ToListMappable for DataEc2TransitGatewayVpnAttachment {
    type O = terrars::ListRef<DataEc2TransitGatewayVpnAttachmentRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2TransitGatewayVpnAttachment_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_transit_gateway_vpn_attachment".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGatewayVpnAttachment {
    pub tf_id: std::string::String,
}

impl BuildDataEc2TransitGatewayVpnAttachment {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2TransitGatewayVpnAttachment {
        let out = DataEc2TransitGatewayVpnAttachment(std::rc::Rc::new(DataEc2TransitGatewayVpnAttachment_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2TransitGatewayVpnAttachmentData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                tags: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                vpn_connection_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2TransitGatewayVpnAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayVpnAttachmentRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2TransitGatewayVpnAttachmentRef {
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_connection_id` after provisioning.\n"]
    pub fn vpn_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
        DataEc2TransitGatewayVpnAttachmentTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayVpnAttachmentFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayVpnAttachmentFilterEl { }

impl terrars::ToListMappable for DataEc2TransitGatewayVpnAttachmentFilterEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayVpnAttachmentFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayVpnAttachmentFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2TransitGatewayVpnAttachmentFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayVpnAttachmentFilterEl {
        DataEc2TransitGatewayVpnAttachmentFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayVpnAttachmentFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayVpnAttachmentFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2TransitGatewayVpnAttachmentFilterElRef {
        DataEc2TransitGatewayVpnAttachmentFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayVpnAttachmentFilterElRef {
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
pub struct DataEc2TransitGatewayVpnAttachmentTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayVpnAttachmentTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2TransitGatewayVpnAttachmentTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayVpnAttachmentTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayVpnAttachmentTimeoutsEl {}

impl BuildDataEc2TransitGatewayVpnAttachmentTimeoutsEl {
    pub fn build(self) -> DataEc2TransitGatewayVpnAttachmentTimeoutsEl {
        DataEc2TransitGatewayVpnAttachmentTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
        DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayVpnAttachmentTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayVpnAttachmentDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2TransitGatewayVpnAttachmentFilterEl>>,
}
