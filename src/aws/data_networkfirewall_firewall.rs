#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkfirewallFirewallData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
}

struct DataNetworkfirewallFirewall_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkfirewallFirewallData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `availability_zone_change_protection` after provisioning.\n"]
    pub fn availability_zone_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_mapping` after provisioning.\n"]
    pub fn availability_zone_mapping(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_AvailabilityZoneMappingRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zone_mapping", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_EncryptionConfigurationRef> {
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
    pub fn firewall_status(&self) -> terrars::ListRef<DataNetworkfirewallFirewall_FirewallStatusRef> {
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

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_SubnetMappingRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataNetworkfirewallFirewallData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                arn: core::default::Default::default(),
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `availability_zone_change_protection` after provisioning.\n"]
    pub fn availability_zone_change_protection(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.availability_zone_change_protection", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `availability_zone_mapping` after provisioning.\n"]
    pub fn availability_zone_mapping(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_AvailabilityZoneMappingRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zone_mapping", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_EncryptionConfigurationRef> {
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
    pub fn firewall_status(&self) -> terrars::ListRef<DataNetworkfirewallFirewall_FirewallStatusRef> {
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

    #[doc = "Get a reference to the value of field `subnet_mapping` after provisioning.\n"]
    pub fn subnet_mapping(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_SubnetMappingRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.subnet_mapping", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
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
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_AvailabilityZoneMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewall_AvailabilityZoneMapping {
    #[doc = "Set the field `availability_zone_id`.\n"]
    pub fn set_availability_zone_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.availability_zone_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewall_AvailabilityZoneMapping {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_AvailabilityZoneMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_AvailabilityZoneMapping {}

impl BuildDataNetworkfirewallFirewall_AvailabilityZoneMapping {
    pub fn build(self) -> DataNetworkfirewallFirewall_AvailabilityZoneMapping {
        DataNetworkfirewallFirewall_AvailabilityZoneMapping {
            availability_zone_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_AvailabilityZoneMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_AvailabilityZoneMappingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_AvailabilityZoneMappingRef {
        DataNetworkfirewallFirewall_AvailabilityZoneMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_AvailabilityZoneMappingRef {
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
pub struct DataNetworkfirewallFirewall_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewall_EncryptionConfiguration {
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

impl terrars::ToListMappable for DataNetworkfirewallFirewall_EncryptionConfiguration {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_EncryptionConfiguration {}

impl BuildDataNetworkfirewallFirewall_EncryptionConfiguration {
    pub fn build(self) -> DataNetworkfirewallFirewall_EncryptionConfiguration {
        DataNetworkfirewallFirewall_EncryptionConfiguration {
            key_id: core::default::Default::default(),
            type_: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_EncryptionConfigurationRef {
        DataNetworkfirewallFirewall_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_EncryptionConfigurationRef {
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
pub struct DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resolved_cidr_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {
    #[doc = "Set the field `resolved_cidr_count`.\n"]
    pub fn set_resolved_cidr_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.resolved_cidr_count = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {
        DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences {
            resolved_cidr_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferencesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferencesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferencesRef {
        DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferencesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferencesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resolved_cidr_count` after provisioning.\n"]
    pub fn resolved_cidr_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resolved_cidr_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    available_cidr_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set_references: std
    ::option
    ::Option<
        terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    utilized_cidr_count: std::option::Option<terrars::PrimField<f64>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {
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
                            DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferences,
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

impl terrars::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {
        DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs {
            available_cidr_count: core::default::Default::default(),
            ip_set_references: core::default::Default::default(),
            utilized_cidr_count: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_CidrsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_CidrsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_CidrsRef {
        DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_CidrsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_CidrsRef {
    #[allow(dead_code)]
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
    ) -> terrars::SetRef<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs_IpSetReferencesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.ip_set_references", self.base))
    }

    #[doc = "Get a reference to the value of field `utilized_cidr_count` after provisioning.\n"]
    pub fn utilized_cidr_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.utilized_cidr_count", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary {
    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_Cidrs>>,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary {
        DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary { cidrs: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummaryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummaryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummaryRef {
        DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummaryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummaryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary_CidrsRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
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

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
        DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment {
            endpoint_id: core::default::Default::default(),
            status: core::default::Default::default(),
            status_message: core::default::Default::default(),
            subnet_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
        DataNetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_SyncStates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment: std::option::Option<terrars::ListField<DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zone: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus_SyncStates {
    #[doc = "Set the field `attachment`.\n"]
    pub fn set_attachment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataNetworkfirewallFirewall_FirewallStatus_SyncStates_Attachment>>,
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

impl terrars::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus_SyncStates {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus_SyncStates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus_SyncStates {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus_SyncStates {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus_SyncStates {
        DataNetworkfirewallFirewall_FirewallStatus_SyncStates {
            attachment: core::default::Default::default(),
            availability_zone: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
        DataNetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatus_SyncStatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment` after provisioning.\n"]
    pub fn attachment(&self) -> terrars::ListRef<DataNetworkfirewallFirewall_FirewallStatus_SyncStates_AttachmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zone` after provisioning.\n"]
    pub fn availability_zone(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.availability_zone", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_message: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment_status: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attachment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_message`.\n"]
    pub fn set_status_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_message = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment_status`.\n"]
    pub fn set_transit_gateway_attachment_status(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_gateway_attachment_status = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
        DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates {
            attachment_id: core::default::Default::default(),
            status_message: core::default::Default::default(),
            transit_gateway_attachment_status: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
        DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_status` after provisioning.\n"]
    pub fn transit_gateway_attachment_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_attachment_status", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_FirewallStatus {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capacity_usage_summary: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    configuration_sync_state_summary: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sync_states: std::option::Option<terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_SyncStates>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment_sync_states: std
    ::option
    ::Option<terrars::ListField<DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates>>,
}

impl DataNetworkfirewallFirewall_FirewallStatus {
    #[doc = "Set the field `capacity_usage_summary`.\n"]
    pub fn set_capacity_usage_summary(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummary>>,
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
        v: impl std::convert::Into<terrars::SetField<DataNetworkfirewallFirewall_FirewallStatus_SyncStates>>,
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
                        terrars
                        ::ListField<DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStates>,
                    >,
    ) -> Self {
        self.transit_gateway_attachment_sync_states = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewall_FirewallStatus {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_FirewallStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_FirewallStatus {}

impl BuildDataNetworkfirewallFirewall_FirewallStatus {
    pub fn build(self) -> DataNetworkfirewallFirewall_FirewallStatus {
        DataNetworkfirewallFirewall_FirewallStatus {
            capacity_usage_summary: core::default::Default::default(),
            configuration_sync_state_summary: core::default::Default::default(),
            status: core::default::Default::default(),
            sync_states: core::default::Default::default(),
            transit_gateway_attachment_sync_states: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_FirewallStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_FirewallStatusRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkfirewallFirewall_FirewallStatusRef {
        DataNetworkfirewallFirewall_FirewallStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_FirewallStatusRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `capacity_usage_summary` after provisioning.\n"]
    pub fn capacity_usage_summary(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewall_FirewallStatus_CapacityUsageSummaryRef> {
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
    pub fn sync_states(&self) -> terrars::SetRef<DataNetworkfirewallFirewall_FirewallStatus_SyncStatesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.sync_states", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment_sync_states` after provisioning.\n"]
    pub fn transit_gateway_attachment_sync_states(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewall_FirewallStatus_TransitGatewayAttachmentSyncStatesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_attachment_sync_states", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewall_SubnetMapping {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewall_SubnetMapping {
    #[doc = "Set the field `subnet_id`.\n"]
    pub fn set_subnet_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.subnet_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewall_SubnetMapping {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewall_SubnetMapping>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewall_SubnetMapping {}

impl BuildDataNetworkfirewallFirewall_SubnetMapping {
    pub fn build(self) -> DataNetworkfirewallFirewall_SubnetMapping {
        DataNetworkfirewallFirewall_SubnetMapping { subnet_id: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewall_SubnetMappingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewall_SubnetMappingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataNetworkfirewallFirewall_SubnetMappingRef {
        DataNetworkfirewallFirewall_SubnetMappingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewall_SubnetMappingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `subnet_id` after provisioning.\n"]
    pub fn subnet_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.subnet_id", self.base))
    }
}
