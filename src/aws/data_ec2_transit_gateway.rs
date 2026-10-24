use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2TransitGatewayData {
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
    filter: std::option::Option<std::vec::Vec<DataEc2TransitGatewayFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataEc2TransitGatewayTimeoutsEl>,
    dynamic: DataEc2TransitGatewayDynamic,
}

struct DataEc2TransitGateway_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2TransitGatewayData>,
}

#[derive(Clone)]
pub struct DataEc2TransitGateway(std::rc::Rc<DataEc2TransitGateway_>);

impl DataEc2TransitGateway {
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
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2TransitGatewayFilterEl>>,
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataEc2TransitGatewayTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `amazon_side_asn` after provisioning.\n"]
    pub fn amazon_side_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_side_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_default_route_table_id` after provisioning.\n"]
    pub fn association_default_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_default_route_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_accept_shared_attachments` after provisioning.\n"]
    pub fn auto_accept_shared_attachments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept_shared_attachments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_route_table_association` after provisioning.\n"]
    pub fn default_route_table_association(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_route_table_association", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_route_table_propagation` after provisioning.\n"]
    pub fn default_route_table_propagation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_route_table_propagation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_support` after provisioning.\n"]
    pub fn dns_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multicast_support` after provisioning.\n"]
    pub fn multicast_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multicast_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagation_default_route_table_id` after provisioning.\n"]
    pub fn propagation_default_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagation_default_route_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_cidr_blocks` after provisioning.\n"]
    pub fn transit_gateway_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_ecmp_support` after provisioning.\n"]
    pub fn vpn_ecmp_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_ecmp_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayTimeoutsElRef {
        DataEc2TransitGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2TransitGateway {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2TransitGateway { }

impl terrars::ToListMappable for DataEc2TransitGateway {
    type O = terrars::ListRef<DataEc2TransitGatewayRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2TransitGateway_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_transit_gateway".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2TransitGateway {
    pub tf_id: std::string::String,
}

impl BuildDataEc2TransitGateway {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2TransitGateway {
        let out = DataEc2TransitGateway(std::rc::Rc::new(DataEc2TransitGateway_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2TransitGatewayData {
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

pub struct DataEc2TransitGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2TransitGatewayRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `amazon_side_asn` after provisioning.\n"]
    pub fn amazon_side_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_side_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association_default_route_table_id` after provisioning.\n"]
    pub fn association_default_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.association_default_route_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `auto_accept_shared_attachments` after provisioning.\n"]
    pub fn auto_accept_shared_attachments(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.auto_accept_shared_attachments", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_route_table_association` after provisioning.\n"]
    pub fn default_route_table_association(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_route_table_association", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `default_route_table_propagation` after provisioning.\n"]
    pub fn default_route_table_propagation(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.default_route_table_propagation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dns_support` after provisioning.\n"]
    pub fn dns_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dns_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `multicast_support` after provisioning.\n"]
    pub fn multicast_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.multicast_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `propagation_default_route_table_id` after provisioning.\n"]
    pub fn propagation_default_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.propagation_default_route_table_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_cidr_blocks` after provisioning.\n"]
    pub fn transit_gateway_cidr_blocks(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_cidr_blocks", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpn_ecmp_support` after provisioning.\n"]
    pub fn vpn_ecmp_support(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpn_ecmp_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataEc2TransitGatewayTimeoutsElRef {
        DataEc2TransitGatewayTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2TransitGatewayFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayFilterEl { }

impl terrars::ToListMappable for DataEc2TransitGatewayFilterEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2TransitGatewayFilterEl {
    pub fn build(self) -> DataEc2TransitGatewayFilterEl {
        DataEc2TransitGatewayFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2TransitGatewayFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2TransitGatewayFilterElRef {
        DataEc2TransitGatewayFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayFilterElRef {
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
pub struct DataEc2TransitGatewayTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2TransitGatewayTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2TransitGatewayTimeoutsEl {
    type O = terrars::BlockAssignable<DataEc2TransitGatewayTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2TransitGatewayTimeoutsEl {}

impl BuildDataEc2TransitGatewayTimeoutsEl {
    pub fn build(self) -> DataEc2TransitGatewayTimeoutsEl {
        DataEc2TransitGatewayTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataEc2TransitGatewayTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2TransitGatewayTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2TransitGatewayTimeoutsElRef {
        DataEc2TransitGatewayTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2TransitGatewayTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2TransitGatewayDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2TransitGatewayFilterEl>>,
}
