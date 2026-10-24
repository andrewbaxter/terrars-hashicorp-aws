use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SubnetData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    assign_ipv6_address_on_creation: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_owned_ipv4_pool: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_dns64: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_a_record_on_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_resource_name_dns_aaaa_record_on_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_cidr_block: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ipv6_native: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_customer_owned_ip_on_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    map_public_ip_on_launch: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outpost_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    private_dns_hostname_type_on_launch: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<SubnetTimeoutsEl>,
}

struct Subnet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SubnetData>,
}

#[derive(Clone)]
pub struct Subnet(std::rc::Rc<Subnet_>);

impl Subnet {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `assign_ipv6_address_on_creation`.\n"]
    pub fn set_assign_ipv6_address_on_creation(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().assign_ipv6_address_on_creation = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `customer_owned_ipv4_pool`.\n"]
    pub fn set_customer_owned_ipv4_pool(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().customer_owned_ipv4_pool = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_dns64`.\n"]
    pub fn set_enable_dns64(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enable_dns64 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_a_record_on_launch`.\n"]
    pub fn set_enable_resource_name_dns_a_record_on_launch(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_resource_name_dns_a_record_on_launch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_resource_name_dns_aaaa_record_on_launch`.\n"]
    pub fn set_enable_resource_name_dns_aaaa_record_on_launch(
        self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.0.data.borrow_mut().enable_resource_name_dns_aaaa_record_on_launch =
            std::option::Option::Some(v.into());
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

    #[doc = "Set the field `ipv6_native`.\n"]
    pub fn set_ipv6_native(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().ipv6_native = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `map_customer_owned_ip_on_launch`.\n"]
    pub fn set_map_customer_owned_ip_on_launch(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().map_customer_owned_ip_on_launch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `map_public_ip_on_launch`.\n"]
    pub fn set_map_public_ip_on_launch(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().map_public_ip_on_launch = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outpost_arn`.\n"]
    pub fn set_outpost_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().outpost_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `private_dns_hostname_type_on_launch`.\n"]
    pub fn set_private_dns_hostname_type_on_launch(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().private_dns_hostname_type_on_launch = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<SubnetTimeoutsEl>) -> Self {
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

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SubnetTimeoutsElRef {
        SubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for Subnet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Subnet { }

impl terrars::ToListMappable for Subnet {
    type O = terrars::ListRef<SubnetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Subnet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_subnet".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSubnet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildSubnet {
    pub fn build(self, stack: &mut terrars::Stack) -> Subnet {
        let out = Subnet(std::rc::Rc::new(Subnet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SubnetData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                assign_ipv6_address_on_creation: core::default::Default::default(),
                availability_zone: core::default::Default::default(),
                availability_zone_id: core::default::Default::default(),
                cidr_block: core::default::Default::default(),
                customer_owned_ipv4_pool: core::default::Default::default(),
                enable_dns64: core::default::Default::default(),
                enable_resource_name_dns_a_record_on_launch: core::default::Default::default(),
                enable_resource_name_dns_aaaa_record_on_launch: core::default::Default::default(),
                id: core::default::Default::default(),
                ipv6_cidr_block: core::default::Default::default(),
                ipv6_native: core::default::Default::default(),
                map_customer_owned_ip_on_launch: core::default::Default::default(),
                map_public_ip_on_launch: core::default::Default::default(),
                outpost_arn: core::default::Default::default(),
                private_dns_hostname_type_on_launch: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SubnetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SubnetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SubnetRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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

    #[doc = "Get a reference to the value of field `cidr_block` after provisioning.\n"]
    pub fn cidr_block(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr_block", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ipv4_pool` after provisioning.\n"]
    pub fn customer_owned_ipv4_pool(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ipv4_pool", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> SubnetTimeoutsElRef {
        SubnetTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SubnetTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SubnetTimeoutsEl {
    #[doc = "Set the field `create`.\n"]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `delete`.\n"]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SubnetTimeoutsEl {
    type O = terrars::BlockAssignable<SubnetTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSubnetTimeoutsEl {}

impl BuildSubnetTimeoutsEl {
    pub fn build(self) -> SubnetTimeoutsEl {
        SubnetTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct SubnetTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SubnetTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SubnetTimeoutsElRef {
        SubnetTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SubnetTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `create` after provisioning.\n"]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc = "Get a reference to the value of field `delete` after provisioning.\n"]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }
}
