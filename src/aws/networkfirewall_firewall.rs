use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallFirewallData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    firewall_policy_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    firewall_policy_change_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_change_protection: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    vpc_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryption_configuration: Option<Vec<NetworkfirewallFirewallEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_mapping: Option<Vec<NetworkfirewallFirewallSubnetMappingEl>>,
    dynamic: NetworkfirewallFirewallDynamic,
}

struct NetworkfirewallFirewall_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<NetworkfirewallFirewallData>,
}

#[derive(Clone)]
pub struct NetworkfirewallFirewall(Rc<NetworkfirewallFirewall_>);

impl NetworkfirewallFirewall {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `delete_protection`.\n"]
    pub fn set_delete_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().delete_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `firewall_policy_change_protection`.\n"]
    pub fn set_firewall_policy_change_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().firewall_policy_change_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_change_protection`.\n"]
    pub fn set_subnet_change_protection(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().subnet_change_protection = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl Into<BlockAssignable<NetworkfirewallFirewallEncryptionConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().encryption_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.encryption_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `subnet_mapping`.\n"]
    pub fn set_subnet_mapping(self, v: impl Into<BlockAssignable<NetworkfirewallFirewallSubnetMappingEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subnet_mapping = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subnet_mapping = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy_arn` after provisioning.\n"]
    pub fn firewall_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy_change_protection` after provisioning.\n"]
    pub fn firewall_policy_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy_change_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_status` after provisioning.\n"]
    pub fn firewall_status(&self) -> ListRef<NetworkfirewallFirewallFirewallStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firewall_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_change_protection` after provisioning.\n"]
    pub fn subnet_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_change_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<NetworkfirewallFirewallEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

impl Resource for NetworkfirewallFirewall {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for NetworkfirewallFirewall {
    type O = ListRef<NetworkfirewallFirewallRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for NetworkfirewallFirewall_ {
    fn extract_resource_type(&self) -> String {
        "aws_networkfirewall_firewall".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildNetworkfirewallFirewall {
    pub tf_id: String,
    #[doc= ""]
    pub firewall_policy_arn: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub vpc_id: PrimField<String>,
}

impl BuildNetworkfirewallFirewall {
    pub fn build(self, stack: &mut Stack) -> NetworkfirewallFirewall {
        let out = NetworkfirewallFirewall(Rc::new(NetworkfirewallFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(NetworkfirewallFirewallData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
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
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct NetworkfirewallFirewallRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl NetworkfirewallFirewallRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy_arn` after provisioning.\n"]
    pub fn firewall_policy_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_policy_change_protection` after provisioning.\n"]
    pub fn firewall_policy_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.firewall_policy_change_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `firewall_status` after provisioning.\n"]
    pub fn firewall_status(&self) -> ListRef<NetworkfirewallFirewallFirewallStatusElRef> {
        ListRef::new(self.shared().clone(), format!("{}.firewall_status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `subnet_change_protection` after provisioning.\n"]
    pub fn subnet_change_protection(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_change_protection", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> ListRef<NetworkfirewallFirewallEncryptionConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<PrimField<String>>,
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[doc= "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.endpoint_id = Some(v.into());
        self
    }

    #[doc= "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.subnet_id = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    type O = BlockAssignable<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
        NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    attachment: Option<ListField<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    availability_zone: Option<PrimField<String>>,
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[doc= "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v: impl Into<ListField<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    ) -> Self {
        self.attachment = Some(v.into());
        self
    }

    #[doc= "Set the field `availability_zone`.\n"]
    pub fn set_availability_zone(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.availability_zone = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    type O = BlockAssignable<NetworkfirewallFirewallFirewallStatusElSyncStatesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
        NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> ListRef<NetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef> {
        ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc= "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallFirewallStatusEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    sync_states: Option<SetField<NetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
}

impl NetworkfirewallFirewallFirewallStatusEl {
    #[doc= "Set the field `sync_states`.\n"]
    pub fn set_sync_states(
        mut self,
        v: impl Into<SetField<NetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
    ) -> Self {
        self.sync_states = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallFirewallStatusEl {
    type O = BlockAssignable<NetworkfirewallFirewallFirewallStatusEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallFirewallStatusElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallFirewallStatusElRef {
        NetworkfirewallFirewallFirewallStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallFirewallStatusElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `sync_states` after provisioning.\n"]
    pub fn sync_states(&self) -> SetRef<NetworkfirewallFirewallFirewallStatusElSyncStatesElRef> {
        SetRef::new(self.shared().clone(), format!("{}.sync_states", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl NetworkfirewallFirewallEncryptionConfigurationEl {
    #[doc= "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }
}

impl ToListMappable for NetworkfirewallFirewallEncryptionConfigurationEl {
    type O = BlockAssignable<NetworkfirewallFirewallEncryptionConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallEncryptionConfigurationEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallEncryptionConfigurationElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallEncryptionConfigurationElRef {
        NetworkfirewallFirewallEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallEncryptionConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key_id` after provisioning.\n"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewallSubnetMappingEl {
    subnet_id: PrimField<String>,
}

impl NetworkfirewallFirewallSubnetMappingEl { }

impl ToListMappable for NetworkfirewallFirewallSubnetMappingEl {
    type O = BlockAssignable<NetworkfirewallFirewallSubnetMappingEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewallSubnetMappingEl {
    #[doc= ""]
    pub subnet_id: PrimField<String>,
}

impl BuildNetworkfirewallFirewallSubnetMappingEl {
    pub fn build(self) -> NetworkfirewallFirewallSubnetMappingEl {
        NetworkfirewallFirewallSubnetMappingEl { subnet_id: self.subnet_id }
    }
}

pub struct NetworkfirewallFirewallSubnetMappingElRef {
    shared: StackShared,
    base: String,
}

impl Ref for NetworkfirewallFirewallSubnetMappingElRef {
    fn new(shared: StackShared, base: String) -> NetworkfirewallFirewallSubnetMappingElRef {
        NetworkfirewallFirewallSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewallSubnetMappingElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallDynamic {
    encryption_configuration: Option<DynamicBlock<NetworkfirewallFirewallEncryptionConfigurationEl>>,
    subnet_mapping: Option<DynamicBlock<NetworkfirewallFirewallSubnetMappingEl>>,
}
