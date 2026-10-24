use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DxHostedPublicVirtualInterfaceData {
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
    address_family: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    amazon_address: std::option::Option<terrars::PrimField<std::string::String>>,
    bgp_asn: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bgp_auth_key: std::option::Option<terrars::PrimField<std::string::String>>,
    connection_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    owner_account_id: terrars::PrimField<std::string::String>,
    route_filter_prefixes: terrars::SetField<terrars::PrimField<std::string::String>>,
    vlan: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DxHostedPublicVirtualInterfaceTimeoutsEl>,
}

struct DxHostedPublicVirtualInterface_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DxHostedPublicVirtualInterfaceData>,
}

#[derive(Clone)]
pub struct DxHostedPublicVirtualInterface(std::rc::Rc<DxHostedPublicVirtualInterface_>);

impl DxHostedPublicVirtualInterface {
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

    #[doc = "Set the field `amazon_address`.\n"]
    pub fn set_amazon_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().amazon_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bgp_auth_key`.\n"]
    pub fn set_bgp_auth_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().bgp_auth_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_address`.\n"]
    pub fn set_customer_address(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().customer_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<DxHostedPublicVirtualInterfaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_address` after provisioning.\n"]
    pub fn amazon_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_side_asn` after provisioning.\n"]
    pub fn amazon_side_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_side_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_asn` after provisioning.\n"]
    pub fn bgp_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_auth_key` after provisioning.\n"]
    pub fn bgp_auth_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_auth_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_address` after provisioning.\n"]
    pub fn customer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_filter_prefixes` after provisioning.\n"]
    pub fn route_filter_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.route_filter_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DxHostedPublicVirtualInterfaceTimeoutsElRef {
        DxHostedPublicVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for DxHostedPublicVirtualInterface {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DxHostedPublicVirtualInterface { }

impl terrars::ToListMappable for DxHostedPublicVirtualInterface {
    type O = terrars::ListRef<DxHostedPublicVirtualInterfaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DxHostedPublicVirtualInterface_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dx_hosted_public_virtual_interface".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDxHostedPublicVirtualInterface {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub address_family: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub bgp_asn: terrars::PrimField<f64>,
    #[doc = ""]
    pub connection_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub owner_account_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub route_filter_prefixes: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub vlan: terrars::PrimField<f64>,
}

impl BuildDxHostedPublicVirtualInterface {
    pub fn build(self, stack: &mut terrars::Stack) -> DxHostedPublicVirtualInterface {
        let out = DxHostedPublicVirtualInterface(std::rc::Rc::new(DxHostedPublicVirtualInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DxHostedPublicVirtualInterfaceData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                address_family: self.address_family,
                amazon_address: core::default::Default::default(),
                bgp_asn: self.bgp_asn,
                bgp_auth_key: core::default::Default::default(),
                connection_id: self.connection_id,
                customer_address: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                owner_account_id: self.owner_account_id,
                route_filter_prefixes: self.route_filter_prefixes,
                vlan: self.vlan,
                timeouts: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DxHostedPublicVirtualInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DxHostedPublicVirtualInterfaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DxHostedPublicVirtualInterfaceRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_family` after provisioning.\n"]
    pub fn address_family(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address_family", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_address` after provisioning.\n"]
    pub fn amazon_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `amazon_side_asn` after provisioning.\n"]
    pub fn amazon_side_asn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.amazon_side_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `aws_device` after provisioning.\n"]
    pub fn aws_device(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.aws_device", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_asn` after provisioning.\n"]
    pub fn bgp_asn(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_asn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bgp_auth_key` after provisioning.\n"]
    pub fn bgp_auth_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bgp_auth_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `connection_id` after provisioning.\n"]
    pub fn connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connection_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `customer_address` after provisioning.\n"]
    pub fn customer_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `route_filter_prefixes` after provisioning.\n"]
    pub fn route_filter_prefixes(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.route_filter_prefixes", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vlan` after provisioning.\n"]
    pub fn vlan(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vlan", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> DxHostedPublicVirtualInterfaceTimeoutsElRef {
        DxHostedPublicVirtualInterfaceTimeoutsElRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize)]
pub struct DxHostedPublicVirtualInterfaceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DxHostedPublicVirtualInterfaceTimeoutsEl {
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

impl terrars::ToListMappable for DxHostedPublicVirtualInterfaceTimeoutsEl {
    type O = terrars::BlockAssignable<DxHostedPublicVirtualInterfaceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDxHostedPublicVirtualInterfaceTimeoutsEl {}

impl BuildDxHostedPublicVirtualInterfaceTimeoutsEl {
    pub fn build(self) -> DxHostedPublicVirtualInterfaceTimeoutsEl {
        DxHostedPublicVirtualInterfaceTimeoutsEl {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
        }
    }
}

pub struct DxHostedPublicVirtualInterfaceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DxHostedPublicVirtualInterfaceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DxHostedPublicVirtualInterfaceTimeoutsElRef {
        DxHostedPublicVirtualInterfaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DxHostedPublicVirtualInterfaceTimeoutsElRef {
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
