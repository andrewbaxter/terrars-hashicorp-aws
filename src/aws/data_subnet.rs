use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSubnetData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_for_az: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataSubnetFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataSubnetTimeoutsEl>,
    dynamic: DataSubnetDynamic,
}

struct DataSubnet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSubnetData>,
}

#[derive(Clone)]
pub struct DataSubnet(std::rc::Rc<DataSubnet_>);

impl DataSubnet {
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

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().availability_zone_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidr_block`.\n"]
    pub fn set_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().cidr_block = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_for_az`.\n"]
    pub fn set_default_for_az(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().default_for_az = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ipv6_cidr_block`.\n"]
    pub fn set_ipv6_cidr_block(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().ipv6_cidr_block = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataSubnetFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataSubnetTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `assign_ipv6_address_on_creation` after provisioning.\n"]
    pub fn assign_ipv6_address_on_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_ipv6_address_on_creation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_ip_address_count` after provisioning.\n"]
    pub fn available_ip_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_ip_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_for_az` after provisioning.\n"]
    pub fn default_for_az(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_for_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns64` after provisioning.\n"]
    pub fn enable_dns64(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_a_record_on_launch", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_resource_name_dns_aaaa_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_aaaa_record_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_association_id` after provisioning.\n"]
    pub fn ipv6_cidr_block_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_native` after provisioning.\n"]
    pub fn ipv6_native(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_native", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `map_customer_owned_ip_on_launch` after provisioning.\n"]
    pub fn map_customer_owned_ip_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_customer_owned_ip_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `map_public_ip_on_launch` after provisioning.\n"]
    pub fn map_public_ip_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_public_ip_on_launch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_hostname_type_on_launch` after provisioning.\n"]
    pub fn private_dns_hostname_type_on_launch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_hostname_type_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataSubnetTimeoutsElRef {
        DataSubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataSubnet {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSubnet { }

impl terrars::ToListMappable for DataSubnet {
    type O = terrars::ListRef<DataSubnetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSubnet_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_subnet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSubnet {
    pub tf_id: std::string::String,
}

impl BuildDataSubnet {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSubnet {
        let out = DataSubnet(std::rc::Rc::new(DataSubnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSubnetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                availability_zone: core::default::Default::default(),
                availability_zone_id: core::default::Default::default(),
                cidr_block: core::default::Default::default(),
                default_for_az: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                filter: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSubnetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSubnetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSubnetRef {
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

    #[doc = "Get a reference to the value of field `assign_ipv6_address_on_creation` after provisioning.\n"]
    pub fn assign_ipv6_address_on_creation(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.assign_ipv6_address_on_creation", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `available_ip_address_count` after provisioning.\n"]
    pub fn available_ip_address_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_ip_address_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_for_az` after provisioning.\n"]
    pub fn default_for_az(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_for_az", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_dns64` after provisioning.\n"]
    pub fn enable_dns64(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_dns64", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enable_resource_name_dns_a_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_a_record_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_a_record_on_launch", self.extract_ref()),
        )
    }

    #[doc =
        "Get a reference to the value of field `enable_resource_name_dns_aaaa_record_on_launch` after provisioning.\n"]
    pub fn enable_resource_name_dns_aaaa_record_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.enable_resource_name_dns_aaaa_record_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block` after provisioning.\n"]
    pub fn ipv6_cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_cidr_block_association_id` after provisioning.\n"]
    pub fn ipv6_cidr_block_association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.ipv6_cidr_block_association_id", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `ipv6_native` after provisioning.\n"]
    pub fn ipv6_native(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ipv6_native", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `map_customer_owned_ip_on_launch` after provisioning.\n"]
    pub fn map_customer_owned_ip_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.map_customer_owned_ip_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `map_public_ip_on_launch` after provisioning.\n"]
    pub fn map_public_ip_on_launch(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.map_public_ip_on_launch", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_hostname_type_on_launch` after provisioning.\n"]
    pub fn private_dns_hostname_type_on_launch(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.private_dns_hostname_type_on_launch", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DataSubnetTimeoutsElRef {
        DataSubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSubnetFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataSubnetFilterEl { }

impl terrars::ToListMappable for DataSubnetFilterEl {
    type O = terrars::BlockAssignable<DataSubnetFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSubnetFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataSubnetFilterEl {
    pub fn build(self) -> DataSubnetFilterEl {
        DataSubnetFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataSubnetFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSubnetFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSubnetFilterElRef {
        DataSubnetFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSubnetFilterElRef {
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
pub struct DataSubnetTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSubnetTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSubnetTimeoutsEl {
    type O = terrars::BlockAssignable<DataSubnetTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSubnetTimeoutsEl {}

impl BuildDataSubnetTimeoutsEl {
    pub fn build(self) -> DataSubnetTimeoutsEl {
        DataSubnetTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataSubnetTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSubnetTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataSubnetTimeoutsElRef {
        DataSubnetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSubnetTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataSubnetDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataSubnetFilterEl>>,
}
