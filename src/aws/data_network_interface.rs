use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkInterfaceData {
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
    filter: std::option::Option<std::vec::Vec<DataNetworkInterfaceFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<DataNetworkInterfaceTimeoutsEl>,
    dynamic: DataNetworkInterfaceDynamic,
}

struct DataNetworkInterface_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkInterfaceData>,
}

#[derive(Clone)]
pub struct DataNetworkInterface(std::rc::Rc<DataNetworkInterface_>);

impl DataNetworkInterface {
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
    pub fn set_filter(self, v: impl std::convert::Into<terrars::BlockAssignable<DataNetworkInterfaceFilterEl>>) -> Self {
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
    pub fn set_timeouts(self, v: impl std::convert::Into<DataNetworkInterfaceTimeoutsEl>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `association` after provisioning.\n"]
    pub fn association(&self) -> terrars::ListRef<DataNetworkInterfaceAssociationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.association", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::ListRef<DataNetworkInterfaceAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_id` after provisioning.\n"]
    pub fn requester_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataNetworkInterfaceTimeoutsElRef {
        DataNetworkInterfaceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkInterface {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkInterface { }

impl terrars::ToListMappable for DataNetworkInterface {
    type O = terrars::ListRef<DataNetworkInterfaceRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkInterface_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_network_interface".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkInterface {
    pub tf_id: std::string::String,
}

impl BuildDataNetworkInterface {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkInterface {
        let out = DataNetworkInterface(std::rc::Rc::new(DataNetworkInterface_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkInterfaceData {
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

pub struct DataNetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkInterfaceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkInterfaceRef {
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

    #[doc = "Get a reference to the value of field `association` after provisioning.\n"]
    pub fn association(&self) -> terrars::ListRef<DataNetworkInterfaceAssociationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.association", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::ListRef<DataNetworkInterfaceAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `interface_type` after provisioning.\n"]
    pub fn interface_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interface_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `ipv6_addresses` after provisioning.\n"]
    pub fn ipv6_addresses(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ipv6_addresses", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `mac_address` after provisioning.\n"]
    pub fn mac_address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mac_address", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `outpost_arn` after provisioning.\n"]
    pub fn outpost_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.outpost_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_id` after provisioning.\n"]
    pub fn owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_dns_name` after provisioning.\n"]
    pub fn private_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_dns_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ip` after provisioning.\n"]
    pub fn private_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.private_ip", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `private_ips` after provisioning.\n"]
    pub fn private_ips(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.private_ips", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `requester_id` after provisioning.\n"]
    pub fn requester_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requester_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.security_groups", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.extract_ref()))
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
    pub fn timeouts(&self) -> DataNetworkInterfaceTimeoutsElRef {
        DataNetworkInterfaceTimeoutsElRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceAssociationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allocation_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    association_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    carrier_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_owned_ip: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_dns_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    public_ip: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkInterfaceAssociationEl {
    #[doc = "Set the field `allocation_id`.\n"]
    pub fn set_allocation_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.allocation_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `association_id`.\n"]
    pub fn set_association_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.association_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `carrier_ip`.\n"]
    pub fn set_carrier_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.carrier_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_owned_ip`.\n"]
    pub fn set_customer_owned_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.customer_owned_ip = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_owner_id`.\n"]
    pub fn set_ip_owner_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_owner_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_dns_name`.\n"]
    pub fn set_public_dns_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_dns_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `public_ip`.\n"]
    pub fn set_public_ip(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.public_ip = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkInterfaceAssociationEl {
    type O = terrars::BlockAssignable<DataNetworkInterfaceAssociationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceAssociationEl {}

impl BuildDataNetworkInterfaceAssociationEl {
    pub fn build(self) -> DataNetworkInterfaceAssociationEl {
        DataNetworkInterfaceAssociationEl {
            allocation_id: core::default::Default::default(),
            association_id: core::default::Default::default(),
            carrier_ip: core::default::Default::default(),
            customer_owned_ip: core::default::Default::default(),
            ip_owner_id: core::default::Default::default(),
            public_dns_name: core::default::Default::default(),
            public_ip: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkInterfaceAssociationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkInterfaceAssociationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkInterfaceAssociationElRef {
        DataNetworkInterfaceAssociationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceAssociationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `allocation_id` after provisioning.\n"]
    pub fn allocation_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.allocation_id", self.base))
    }

    #[doc = "Get a reference to the value of field `association_id` after provisioning.\n"]
    pub fn association_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.association_id", self.base))
    }

    #[doc = "Get a reference to the value of field `carrier_ip` after provisioning.\n"]
    pub fn carrier_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.carrier_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_owned_ip` after provisioning.\n"]
    pub fn customer_owned_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.customer_owned_ip", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_owner_id` after provisioning.\n"]
    pub fn ip_owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_owner_id", self.base))
    }

    #[doc = "Get a reference to the value of field `public_dns_name` after provisioning.\n"]
    pub fn public_dns_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_dns_name", self.base))
    }

    #[doc = "Get a reference to the value of field `public_ip` after provisioning.\n"]
    pub fn public_ip(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.public_ip", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceAttachmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    device_index: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_owner_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkInterfaceAttachmentEl {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attachment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `device_index`.\n"]
    pub fn set_device_index(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.device_index = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_owner_id`.\n"]
    pub fn set_instance_owner_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_owner_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkInterfaceAttachmentEl {
    type O = terrars::BlockAssignable<DataNetworkInterfaceAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceAttachmentEl {}

impl BuildDataNetworkInterfaceAttachmentEl {
    pub fn build(self) -> DataNetworkInterfaceAttachmentEl {
        DataNetworkInterfaceAttachmentEl {
            attachment_id: core::default::Default::default(),
            device_index: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            instance_owner_id: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkInterfaceAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkInterfaceAttachmentElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkInterfaceAttachmentElRef {
        DataNetworkInterfaceAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceAttachmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `device_index` after provisioning.\n"]
    pub fn device_index(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.device_index", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_owner_id` after provisioning.\n"]
    pub fn instance_owner_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_owner_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl DataNetworkInterfaceFilterEl { }

impl terrars::ToListMappable for DataNetworkInterfaceFilterEl {
    type O = terrars::BlockAssignable<DataNetworkInterfaceFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildDataNetworkInterfaceFilterEl {
    pub fn build(self) -> DataNetworkInterfaceFilterEl {
        DataNetworkInterfaceFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataNetworkInterfaceFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkInterfaceFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkInterfaceFilterElRef {
        DataNetworkInterfaceFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkInterfaceTimeoutsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    read: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkInterfaceTimeoutsEl {
    #[doc = "Set the field `read`.\n"]
    pub fn set_read(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.read = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkInterfaceTimeoutsEl {
    type O = terrars::BlockAssignable<DataNetworkInterfaceTimeoutsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkInterfaceTimeoutsEl {}

impl BuildDataNetworkInterfaceTimeoutsEl {
    pub fn build(self) -> DataNetworkInterfaceTimeoutsEl {
        DataNetworkInterfaceTimeoutsEl { read: core::default::Default::default() }
    }
}

pub struct DataNetworkInterfaceTimeoutsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkInterfaceTimeoutsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkInterfaceTimeoutsElRef {
        DataNetworkInterfaceTimeoutsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkInterfaceTimeoutsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `read` after provisioning.\n"]
    pub fn read(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.read", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataNetworkInterfaceDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataNetworkInterfaceFilterEl>>,
}
