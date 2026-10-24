use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataVpcData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dhcp_options_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataVpcFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataVpcTimeoutsEl>,
    dynamic: DataVpcDynamic,
}

struct DataVpc_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataVpcData>,
}

#[derive(Clone)]
pub struct DataVpc(std::rc::Rc<DataVpc_>);

impl DataVpc {
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

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default`.\n"]
    pub fn set_default(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dhcp_options_id`.\n"]
    pub fn set_dhcp_options_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dhcp_options_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
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
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataVpcFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataVpcTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block_associations` after provisioning.\n"]
    pub fn cidr_block_associations(&self) -> terrars::ListRef<DataVpcCidrBlockAssociationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_block_associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dhcp_options_id` after provisioning.\n"]
    pub fn dhcp_options_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_options_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns_hostnames` after provisioning.\n"]
    pub fn enable_dns_hostnames(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns_support` after provisioning.\n"]
    pub fn enable_dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_address_usage_metrics` after provisioning.\n"]
    pub fn enable_network_address_usage_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_address_usage_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tenancy` after provisioning.\n"]
    pub fn instance_tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_association_id` after provisioning.\n"]
    pub fn ipv6_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `main_route_table_id` after provisioning.\n"]
    pub fn main_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcTimeoutsElRef {
        DataVpcTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataVpc {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataVpc { }

impl terrars::ToListMappable for DataVpc {
    type O = terrars::ListRef<DataVpcRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataVpc_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_vpc".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataVpc {
    pub tf_id: std::string::String,
}

impl BuildDataVpc {
    pub fn build(self, stack: &mut terrars::Stack) -> DataVpc {
        let out = DataVpc(std::rc::Rc::new(DataVpc_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataVpcData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                cidr_block: core::default::Default::default(),
                default: core::default::Default::default(),
                dhcp_options_id: core::default::Default::default(),
                id: core::default::Default::default(),
                state: core::default::Default::default(),
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

pub struct DataVpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataVpcRef {
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

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block_associations` after provisioning.\n"]
    pub fn cidr_block_associations(&self) -> terrars::ListRef<DataVpcCidrBlockAssociationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidr_block_associations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default` after provisioning.\n"]
    pub fn default(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dhcp_options_id` after provisioning.\n"]
    pub fn dhcp_options_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dhcp_options_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns_hostnames` after provisioning.\n"]
    pub fn enable_dns_hostnames(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_hostnames", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns_support` after provisioning.\n"]
    pub fn enable_dns_support(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns_support", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_network_address_usage_metrics` after provisioning.\n"]
    pub fn enable_network_address_usage_metrics(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_network_address_usage_metrics", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `instance_tenancy` after provisioning.\n"]
    pub fn instance_tenancy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_tenancy", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_association_id` after provisioning.\n"]
    pub fn ipv6_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_association_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `main_route_table_id` after provisioning.\n"]
    pub fn main_route_table_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.main_route_table_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataVpcTimeoutsElRef {
        DataVpcTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataVpcCidrBlockAssociationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcCidrBlockAssociationsEl {
    #[doc = "Set the field `association_id`.\n"]
    pub fn set_association_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.association_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcCidrBlockAssociationsEl {
    type O = terrars::BlockAssignable<DataVpcCidrBlockAssociationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcCidrBlockAssociationsEl {}

impl BuildDataVpcCidrBlockAssociationsEl {
    pub fn build(self) -> DataVpcCidrBlockAssociationsEl {
        DataVpcCidrBlockAssociationsEl {
            association_id: core::default::Default::default(),
            cidr_block: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

pub struct DataVpcCidrBlockAssociationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcCidrBlockAssociationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcCidrBlockAssociationsElRef {
        DataVpcCidrBlockAssociationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcCidrBlockAssociationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.base))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataVpcFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataVpcFilterEl { }

impl terrars::ToListMappable for DataVpcFilterEl {
    type O = terrars::BlockAssignable<DataVpcFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataVpcFilterEl {
    pub fn build(self) -> DataVpcFilterEl {
        DataVpcFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataVpcFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcFilterElRef {
        DataVpcFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcFilterElRef {
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
pub struct DataVpcTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataVpcTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataVpcTimeoutsEl {
    type O = terrars::BlockAssignable<DataVpcTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataVpcTimeoutsEl {}

impl BuildDataVpcTimeoutsEl {
    pub fn build(self) -> DataVpcTimeoutsEl {
        DataVpcTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataVpcTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataVpcTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataVpcTimeoutsElRef {
        DataVpcTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataVpcTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataVpcDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataVpcFilterEl>>,
}
