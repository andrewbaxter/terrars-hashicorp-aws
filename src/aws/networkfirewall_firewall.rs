#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct NetworkfirewallFirewallData_ {
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
    availability_zone_change_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled_analysis_types: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    firewall_policy_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    firewall_policy_change_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_change_protection: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_mapping: std::option::Option<std::vec::Vec<NetworkfirewallFirewall_AvailabilityZoneMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std::option::Option<std::vec::Vec<NetworkfirewallFirewall_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_mapping: std::option::Option<std::vec::Vec<NetworkfirewallFirewall_SubnetMapping>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<NetworkfirewallFirewall_Timeouts>,
    dynamic: NetworkfirewallFirewallDynamic,
}

struct NetworkfirewallFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<NetworkfirewallFirewallData_>,
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

    #[doc = "Set the field `availability_zone_change_protection`.\n"]
    pub fn set_availability_zone_change_protection(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().availability_zone_change_protection = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `enabled_analysis_types`.\n"]
    pub fn set_enabled_analysis_types(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().enabled_analysis_types = std::option::Option::Some(v.into());
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().transit_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_id`.\n"]
    pub fn set_vpc_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().vpc_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zone_mapping`.\n"]
    pub fn set_availability_zone_mapping(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewall_AvailabilityZoneMapping>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().availability_zone_mapping = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.availability_zone_mapping = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewall_EncryptionConfiguration>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<NetworkfirewallFirewall_SubnetMapping>>,
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

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<NetworkfirewallFirewall_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `availability_zone_change_protection` after provisioning.\n"]
    pub fn availability_zone_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_analysis_types` after provisioning.\n"]
    pub fn enabled_analysis_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_analysis_types", self.extract_ref()))
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
    pub fn firewall_status(&self) -> terrars::ListRef<NetworkfirewallFirewall_FirewallStatusRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_owner_account_id` after provisioning.\n"]
    pub fn transit_gateway_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_owner_account_id", self.extract_ref()),
        )
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
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallFirewall_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallFirewall_TimeoutsRef {
        NetworkfirewallFirewall_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
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
}

impl BuildNetworkfirewallFirewall {
    pub fn build(self, stack: &mut terrars::Stack) -> NetworkfirewallFirewall {
        let out = NetworkfirewallFirewall(std::rc::Rc::new(NetworkfirewallFirewall_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(NetworkfirewallFirewallData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                availability_zone_change_protection: core::default::Default::default(),
                delete_protection: core::default::Default::default(),
                description: core::default::Default::default(),
                enabled_analysis_types: core::default::Default::default(),
                firewall_policy_arn: self.firewall_policy_arn,
                firewall_policy_change_protection: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                subnet_change_protection: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                transit_gateway_id: core::default::Default::default(),
                vpc_id: core::default::Default::default(),
                availability_zone_mapping: core::default::Default::default(),
                encryption_configuration: core::default::Default::default(),
                subnet_mapping: core::default::Default::default(),
                timeouts: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `availability_zone_change_protection` after provisioning.\n"]
    pub fn availability_zone_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `delete_protection` after provisioning.\n"]
    pub fn delete_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete_protection", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled_analysis_types` after provisioning.\n"]
    pub fn enabled_analysis_types(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.enabled_analysis_types", self.extract_ref()))
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
    pub fn firewall_status(&self) -> terrars::ListRef<NetworkfirewallFirewall_FirewallStatusRef> {
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_owner_account_id` after provisioning.\n"]
    pub fn transit_gateway_owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.transit_gateway_owner_account_id", self.extract_ref()),
        )
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
    pub fn encryption_configuration(&self) -> terrars::ListRef<NetworkfirewallFirewall_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> NetworkfirewallFirewall_TimeoutsRef {
        NetworkfirewallFirewall_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct NetworkfirewallFirewallDynamic {
    availability_zone_mapping: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewall_AvailabilityZoneMapping>>,
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<NetworkfirewallFirewall_EncryptionConfiguration>>,
    subnet_mapping: std::option::Option<terrars::DynamicBlock<NetworkfirewallFirewall_SubnetMapping>>,
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
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

impl terrars::ToListMappable for NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {}

impl BuildNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
    pub fn build(self) -> NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
        NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
            endpoint_id: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
        NetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
    #[allow(dead_code)]
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
pub struct NetworkfirewallFirewall_FirewallStatus_SyncStates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std::option::Option<terrars::ListField<NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewall_FirewallStatus_SyncStates {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v: impl std::convert::Into<terrars::ListField<NetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment>>,
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

impl terrars::ToListMappable for NetworkfirewallFirewall_FirewallStatus_SyncStates {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_FirewallStatus_SyncStates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_FirewallStatus_SyncStates {}

impl BuildNetworkfirewallFirewall_FirewallStatus_SyncStates {
    pub fn build(self) -> NetworkfirewallFirewall_FirewallStatus_SyncStates {
        NetworkfirewallFirewall_FirewallStatus_SyncStates {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
        NetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::ListRef<NetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attachment_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {}

impl BuildNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    pub fn build(self) -> NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
        NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
            attachment_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
        NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewall_FirewallStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_states: std::option::Option<terrars::SetField<NetworkfirewallFirewall_FirewallStatus_SyncStates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment_sync_states: std
    ::option
    ::Option<terrars::ListField<NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates>>,
}

impl NetworkfirewallFirewall_FirewallStatus {
    #[doc = "Set the field `sync_states`.\n"]
    pub fn set_sync_states(
        mut self,
        v: impl std::convert::Into<terrars::SetField<NetworkfirewallFirewall_FirewallStatus_SyncStates>>,
    ) -> Self {
        self.sync_states = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_sync_states`.\n"]
    pub fn set_transit_gateway_attachment_sync_states(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates>,
                    >,
    ) -> Self {
        self.transit_gateway_attachment_sync_states = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewall_FirewallStatus {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_FirewallStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_FirewallStatus {}

impl BuildNetworkfirewallFirewall_FirewallStatus {
    pub fn build(self) -> NetworkfirewallFirewall_FirewallStatus {
        NetworkfirewallFirewall_FirewallStatus {
            sync_states: core::default::Default::default(),
            transit_gateway_attachment_sync_states: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_FirewallStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_FirewallStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallFirewall_FirewallStatusRef {
        NetworkfirewallFirewall_FirewallStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_FirewallStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `sync_states` after provisioning.\n"]
    pub fn sync_states(&self) -> terrars::SetRef<NetworkfirewallFirewall_FirewallStatus_SyncStatesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sync_states", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_sync_states` after provisioning.\n"]
    pub fn transit_gateway_attachment_sync_states(
        &self,
    ) -> terrars::ListRef<NetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_attachment_sync_states", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewall_AvailabilityZoneMapping {
    availability_zone_id: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewall_AvailabilityZoneMapping { }

impl terrars::ToListMappable for NetworkfirewallFirewall_AvailabilityZoneMapping {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_AvailabilityZoneMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_AvailabilityZoneMapping {
    #[doc = ""]
    pub availability_zone_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewall_AvailabilityZoneMapping {
    pub fn build(self) -> NetworkfirewallFirewall_AvailabilityZoneMapping {
        NetworkfirewallFirewall_AvailabilityZoneMapping { availability_zone_id: self.availability_zone_id }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_AvailabilityZoneMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_AvailabilityZoneMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewall_AvailabilityZoneMappingRef {
        NetworkfirewallFirewall_AvailabilityZoneMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_AvailabilityZoneMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zone_id` after provisioning.\n"]
    pub fn availability_zone_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewall_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewall_EncryptionConfiguration {
    #[doc = "Set the field `key_id`.\n"]
    pub fn set_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewall_EncryptionConfiguration {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_EncryptionConfiguration {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewall_EncryptionConfiguration {
    pub fn build(self) -> NetworkfirewallFirewall_EncryptionConfiguration {
        NetworkfirewallFirewall_EncryptionConfiguration {
            key_id: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> NetworkfirewallFirewall_EncryptionConfigurationRef {
        NetworkfirewallFirewall_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_EncryptionConfigurationRef {
    #[allow(dead_code)]
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
pub struct NetworkfirewallFirewall_SubnetMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_address_type: std::option::Option<terrars::PrimField<std::string::String>>,
    subnet_id: terrars::PrimField<std::string::String>,
}

impl NetworkfirewallFirewall_SubnetMapping {
    #[doc = "Set the field `ip_address_type`.\n"]
    pub fn set_ip_address_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ip_address_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewall_SubnetMapping {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_SubnetMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_SubnetMapping {
    #[doc = ""]
    pub subnet_id: terrars::PrimField<std::string::String>,
}

impl BuildNetworkfirewallFirewall_SubnetMapping {
    pub fn build(self) -> NetworkfirewallFirewall_SubnetMapping {
        NetworkfirewallFirewall_SubnetMapping {
            ip_address_type: core::default::Default::default(),
            subnet_id: self.subnet_id,
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_SubnetMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_SubnetMappingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallFirewall_SubnetMappingRef {
        NetworkfirewallFirewall_SubnetMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_SubnetMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_address_type` after provisioning.\n"]
    pub fn ip_address_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ip_address_type", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct NetworkfirewallFirewall_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl NetworkfirewallFirewall_Timeouts {
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

    #[doc = "Set the field `update`.\n"]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for NetworkfirewallFirewall_Timeouts {
    type O = terrars::BlockAssignable<NetworkfirewallFirewall_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildNetworkfirewallFirewall_Timeouts {}

impl BuildNetworkfirewallFirewall_Timeouts {
    pub fn build(self) -> NetworkfirewallFirewall_Timeouts {
        NetworkfirewallFirewall_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct NetworkfirewallFirewall_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for NetworkfirewallFirewall_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> NetworkfirewallFirewall_TimeoutsRef {
        NetworkfirewallFirewall_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl NetworkfirewallFirewall_TimeoutsRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `update` after provisioning.\n"]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
