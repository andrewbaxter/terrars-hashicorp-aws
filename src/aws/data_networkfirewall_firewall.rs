use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkfirewallFirewallData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataNetworkfirewallFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkfirewallFirewallData>,
}

#[derive(Clone)]
pub struct DataNetworkfirewallFirewall(std::rc::Rc<DataNetworkfirewallFirewall_>);

impl DataNetworkfirewallFirewall {
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

    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().name = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::SetRef<DataNetworkfirewallFirewallEncryptionConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
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
    pub fn firewall_status(&self) -> terrars::ListRef<DataNetworkfirewallFirewallFirewallStatusElRef> {
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

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> terrars::SetRef<DataNetworkfirewallFirewallSubnetMappingElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkfirewallFirewall {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkfirewallFirewall { }

impl terrars::ToListMappable for DataNetworkfirewallFirewall {
    type O = terrars::ListRef<DataNetworkfirewallFirewallRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkfirewallFirewall_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_networkfirewall_firewall".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkfirewallFirewall {
    pub tf_id: std::string::String,
}

impl BuildDataNetworkfirewallFirewall {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkfirewallFirewall {
        let out = DataNetworkfirewallFirewall(std::rc::Rc::new(DataNetworkfirewallFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkfirewallFirewallData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                tags: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataNetworkfirewallFirewallRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkfirewallFirewallRef {
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

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::SetRef<DataNetworkfirewallFirewallEncryptionConfigurationElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
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
    pub fn firewall_status(&self) -> terrars::ListRef<DataNetworkfirewallFirewallFirewallStatusElRef> {
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

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> terrars::SetRef<DataNetworkfirewallFirewallSubnetMappingElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `vpc_id` after provisioning.\n"]
    pub fn vpc_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_id", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallEncryptionConfigurationEl {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallEncryptionConfigurationEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallEncryptionConfigurationEl {}

impl BuildDataNetworkfirewallFirewallEncryptionConfigurationEl {
    pub fn build(self) -> DataNetworkfirewallFirewallEncryptionConfigurationEl {
        DataNetworkfirewallFirewallEncryptionConfigurationEl {
            key_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallEncryptionConfigurationElRef {
        DataNetworkfirewallFirewallEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallEncryptionConfigurationElRef {
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
pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved_cidr_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
    #[doc = "Set the field `resolved_cidr_count`.\n"]
    pub fn set_resolved_cidr_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.resolved_cidr_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl {
            resolved_cidr_count: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolved_cidr_count` after provisioning.\n"]
    pub fn resolved_cidr_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved_cidr_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_cidr_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_references: std
    ::option
    ::Option<
        terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    utilized_cidr_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    #[doc = "Set the field `available_cidr_count`.\n"]
    pub fn set_available_cidr_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.available_cidr_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ip_set_references`.\n"]
    pub fn set_ip_set_references(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesEl,
                        >,
                    >,
    ) -> Self {
        self.ip_set_references = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `utilized_cidr_count`.\n"]
    pub fn set_utilized_cidr_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.utilized_cidr_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl {
            available_cidr_count: core::default::Default::default(),
            ip_set_references: core::default::Default::default(),
            utilized_cidr_count: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `available_cidr_count` after provisioning.\n"]
    pub fn available_cidr_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.available_cidr_count", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_set_references` after provisioning.\n"]
    pub fn ip_set_references(
        &self,
    ) -> terrars
    ::SetRef<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElIpSetReferencesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_set_references", self.base))
    }

    #[doc = "Get a reference to the value of field `utilized_cidr_count` after provisioning.\n"]
    pub fn utilized_cidr_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.utilized_cidr_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsEl>,
                    >,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl {
            cidrs: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
        DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::SetRef<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElCidrsElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    #[doc = "Set the field `endpoint_id`.\n"]
    pub fn set_endpoint_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.endpoint_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl {
            endpoint_id: core::default::Default::default(),
            status: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `endpoint_id` after provisioning.\n"]
    pub fn endpoint_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.endpoint_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std
    ::option
    ::Option<terrars::ListField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentEl>>,
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

impl terrars::ToListMappable for DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
        DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::ListRef<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallFirewallStatusEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_usage_summary: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_sync_state_summary: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_states: std::option::Option<terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
}

impl DataNetworkfirewallFirewallFirewallStatusEl {
    #[doc = "Set the field `capacity_usage_summary`.\n"]
    pub fn set_capacity_usage_summary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryEl>>,
    ) -> Self {
        self.capacity_usage_summary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `configuration_sync_state_summary`.\n"]
    pub fn set_configuration_sync_state_summary(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.configuration_sync_state_summary = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sync_states`.\n"]
    pub fn set_sync_states(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataNetworkfirewallFirewallFirewallStatusElSyncStatesEl>>,
    ) -> Self {
        self.sync_states = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallFirewallStatusEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallFirewallStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallFirewallStatusEl {}

impl BuildDataNetworkfirewallFirewallFirewallStatusEl {
    pub fn build(self) -> DataNetworkfirewallFirewallFirewallStatusEl {
        DataNetworkfirewallFirewallFirewallStatusEl {
            capacity_usage_summary: core::default::Default::default(),
            configuration_sync_state_summary: core::default::Default::default(),
            status: core::default::Default::default(),
            sync_states: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallFirewallStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallFirewallStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallFirewallStatusElRef {
        DataNetworkfirewallFirewallFirewallStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallFirewallStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_usage_summary` after provisioning.\n"]
    pub fn capacity_usage_summary(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallFirewallStatusElCapacityUsageSummaryElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capacity_usage_summary", self.base))
    }

    #[doc = "Get a reference to the value of field `configuration_sync_state_summary` after provisioning.\n"]
    pub fn configuration_sync_state_summary(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration_sync_state_summary", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `sync_states` after provisioning.\n"]
    pub fn sync_states(&self) -> terrars::SetRef<DataNetworkfirewallFirewallFirewallStatusElSyncStatesElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sync_states", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallSubnetMappingEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallSubnetMappingEl {
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallSubnetMappingEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallSubnetMappingEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallSubnetMappingEl {}

impl BuildDataNetworkfirewallFirewallSubnetMappingEl {
    pub fn build(self) -> DataNetworkfirewallFirewallSubnetMappingEl {
        DataNetworkfirewallFirewallSubnetMappingEl { subnet_id: core::default::Default::default() }
    }
}

pub struct DataNetworkfirewallFirewallSubnetMappingElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallSubnetMappingElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkfirewallFirewallSubnetMappingElRef {
        DataNetworkfirewallFirewallSubnetMappingElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallSubnetMappingElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
