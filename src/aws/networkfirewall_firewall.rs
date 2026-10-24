use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallFirewallData {
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
    delete_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    firewall_policy_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_policy_change_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_change_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    vpc_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<NetworkfirewallFirewallEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_mapping: std::option::Option<std::vec::Vec<NetworkfirewallFirewallSubnetMappingEl>>,
    dynamic: NetworkfirewallFirewallDynamic,
}

struct NetworkfirewallFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallFirewallData>,
}

#[derive(Clone)]
pub struct NetworkfirewallFirewall(std::rc::Rc<NetworkfirewallFirewall_>);

impl NetworkfirewallFirewall {
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

    #[doc = "Set the field `delete_protection`.\n"]
    pub fn set_delete_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `firewall_policy_change_protection`.\n"]
    pub fn set_firewall_policy_change_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().firewall_policy_change_protection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_change_protection`.\n"]
    pub fn set_subnet_change_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().subnet_change_protection = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewallEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `subnet_mapping`.\n"]
    pub fn set_subnet_mapping(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewallSubnetMappingEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy_arn` after provisioning.\n"]
    pub fn firewall_policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy_change_protection` after provisioning.\n"]
    pub fn firewall_policy_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_policy_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_status` after provisioning.\n"]
    pub fn firewall_status(&self) -> terrars::ListRef<NetworkfirewallFirewallFirewallStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_change_protection` after provisioning.\n"]
    pub fn subnet_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_change_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallFirewallEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for NetworkfirewallFirewall {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for NetworkfirewallFirewall { }

impl terrars::ToListMappable for NetworkfirewallFirewall {
    type O = terrars::ListRef<NetworkfirewallFirewallRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for NetworkfirewallFirewall_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_networkfirewall_firewall".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallFirewall {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub firewall_policy_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub vpc_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewall {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallFirewall {
        let out = NetworkfirewallFirewall(std::rc::Rc::new(NetworkfirewallFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkfirewallFirewallData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                delete_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                firewall_policy_arn: self.firewall_policy_arn,
                firewall_policy_change_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                subnet_change_protection: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                vpc_id: self.vpc_id,
                encryption_configuration: core::default::Default::default(),
                subnet_mapping: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallFirewallRef {
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

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy_arn` after provisioning.\n"]
    pub fn firewall_policy_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy_change_protection` after provisioning.\n"]
    pub fn firewall_policy_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.firewall_policy_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `firewall_status` after provisioning.\n"]
    pub fn firewall_status(&self) -> terrars::ListRef<NetworkfirewallFirewallFirewallStatusElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `subnet_change_protection` after provisioning.\n"]
    pub fn subnet_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_change_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallFirewallEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {}

impl BuildNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    pub fn build(self) -> NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
        NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
            endpoint_id: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
        NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std::option::Option<terrars::ListField<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    ) -> Self {
        self.attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.availability_zone = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallFirewallStatusElSyncStatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallFirewallStatusElSyncStatesEl {}

impl BuildNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    pub fn build(self) -> NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
        NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

pub struct NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
        NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::ListRef<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallFirewallStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_states: std::option::Option<terrars::SetField<NetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
}

impl NetworkfirewallFirewallFirewallStatusEl {
    #[doc = "Set the field `sync_states`.\n"]
    pub fn set_sync_states(
        mut self,
        v: impl std::convert::Into<terrars::SetField<NetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
    ) -> Self {
        self.sync_states = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallFirewallStatusEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallFirewallStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallFirewallStatusEl {}

impl BuildNetworkfirewallFirewallFirewallStatusEl {
    pub fn build(self) -> NetworkfirewallFirewallFirewallStatusEl {
        NetworkfirewallFirewallFirewallStatusEl { sync_states: core::default::Default::default() }
    }
}

pub struct NetworkfirewallFirewallFirewallStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallFirewallStatusElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallFirewallFirewallStatusElRef {
        NetworkfirewallFirewallFirewallStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallFirewallStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sync_states` after provisioning.\n"]
    pub fn sync_states(&self) -> terrars::SetRef<NetworkfirewallFirewallFirewallStatusElSyncStatesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sync_states", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewallEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallEncryptionConfigurationEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallEncryptionConfigurationEl {
    pub fn build(self) -> NetworkfirewallFirewallEncryptionConfigurationEl {
        NetworkfirewallFirewallEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct NetworkfirewallFirewallEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewallEncryptionConfigurationElRef {
        NetworkfirewallFirewallEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallSubnetMappingEl {
    subnet_id: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewallSubnetMappingEl { }

impl terrars::ToListMappable for NetworkfirewallFirewallSubnetMappingEl {
    type O = terrars::BlockAssignable<NetworkfirewallFirewallSubnetMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallSubnetMappingEl {
    #[doc = ""]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewallSubnetMappingEl {
    pub fn build(self) -> NetworkfirewallFirewallSubnetMappingEl {
        NetworkfirewallFirewallSubnetMappingEl { subnet_id: self.subnet_id }
    }
}

pub struct NetworkfirewallFirewallSubnetMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewallSubnetMappingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallFirewallSubnetMappingElRef {
        NetworkfirewallFirewallSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallSubnetMappingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewallEncryptionConfigurationEl>>,
    subnet_mapping: std::option::Option<terrars::DynamicBlock<NetworkfirewallFirewallSubnetMappingEl>>,
}
