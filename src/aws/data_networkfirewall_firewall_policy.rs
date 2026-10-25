#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkfirewallFirewallPolicyData_ {
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

struct DataNetworkfirewallFirewallPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkfirewallFirewallPolicyData_>,
}

#[derive(Clone)]
pub struct DataNetworkfirewallFirewallPolicy(std::rc::Rc<DataNetworkfirewallFirewallPolicy_>);

impl DataNetworkfirewallFirewallPolicy {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }
}

impl terrars::Referable for DataNetworkfirewallFirewallPolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataNetworkfirewallFirewallPolicy { }

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy {
    type O = terrars::ListRef<DataNetworkfirewallFirewallPolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataNetworkfirewallFirewallPolicy_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_networkfirewall_firewall_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy {
    pub tf_id: std::string::String,
}

impl BuildDataNetworkfirewallFirewallPolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DataNetworkfirewallFirewallPolicy {
        let out = DataNetworkfirewallFirewallPolicy(std::rc::Rc::new(DataNetworkfirewallFirewallPolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataNetworkfirewallFirewallPolicyData_ {
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

pub struct DataNetworkfirewallFirewallPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataNetworkfirewallFirewallPolicyRef {
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

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `firewall_policy` after provisioning.\n"]
    pub fn firewall_policy(&self) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicyRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.firewall_policy", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `update_token` after provisioning.\n"]
    pub fn update_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update_token", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    definition: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    #[doc = "Set the field `definition`.\n"]
    pub fn set_definition(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.definition = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet {
            definition: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.definition", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_set: std
    ::option
    ::Option<
        terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    #[doc = "Set the field `ip_set`.\n"]
    pub fn set_ip_set(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSet,
                        >,
                    >,
    ) -> Self {
        self.ip_set = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    type O =
        terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables {
            ip_set: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `ip_set` after provisioning.\n"]
    pub fn ip_set(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables_IpSetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_set", self.base))
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_variables: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    #[doc = "Set the field `rule_variables`.\n"]
    pub fn set_rule_variables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariables>,
                    >,
    ) -> Self {
        self.rule_variables = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables {
            rule_variables: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_variables` after provisioning.\n"]
    pub fn rule_variables(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables_RuleVariablesRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rule_variables", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tcp_idle_timeout_seconds: std::option::Option<terrars::PrimField<f64>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    #[doc = "Set the field `tcp_idle_timeout_seconds`.\n"]
    pub fn set_tcp_idle_timeout_seconds(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.tcp_idle_timeout_seconds = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts {
            tcp_idle_timeout_seconds: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `tcp_idle_timeout_seconds` after provisioning.\n"]
    pub fn tcp_idle_timeout_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tcp_idle_timeout_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    flow_timeouts: std
    ::option
    ::Option<terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_order: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stream_exception_policy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    #[doc = "Set the field `flow_timeouts`.\n"]
    pub fn set_flow_timeouts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeouts,
                        >,
                    >,
    ) -> Self {
        self.flow_timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_order`.\n"]
    pub fn set_rule_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_order = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stream_exception_policy`.\n"]
    pub fn set_stream_exception_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.stream_exception_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions {
            flow_timeouts: core::default::Default::default(),
            rule_order: core::default::Default::default(),
            stream_exception_policy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `flow_timeouts` after provisioning.\n"]
    pub fn flow_timeouts(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions_FlowTimeoutsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.flow_timeouts", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }

    #[doc = "Get a reference to the value of field `stream_exception_policy` after provisioning.\n"]
    pub fn stream_exception_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.stream_exception_policy", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override {
            action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deep_threat_inspection: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "override", skip_serializing_if = "std::option::Option::is_none")]
    override_: std
    ::option
    ::Option<
        terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    #[doc = "Set the field `deep_threat_inspection`.\n"]
    pub fn set_deep_threat_inspection(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.deep_threat_inspection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `override_`.\n"]
    pub fn set_override(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_Override,
                        >,
                    >,
    ) -> Self {
        self.override_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference {
            deep_threat_inspection: core::default::Default::default(),
            override_: core::default::Default::default(),
            priority: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `deep_threat_inspection` after provisioning.\n"]
    pub fn deep_threat_inspection(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.deep_threat_inspection", self.base))
    }

    #[doc = "Get a reference to the value of field `override_` after provisioning.\n"]
    pub fn override_(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference_OverrideRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override", self.base))
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension {
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
        >,
    >,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<
                            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_Dimension,
                        >,
                    >,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction {
            dimension: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(
        &self,
    ) -> terrars
    ::SetRef<
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction_DimensionRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_metric_action: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction,
        >,
    >,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    #[doc = "Set the field `publish_metric_action`.\n"]
    pub fn set_publish_metric_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricAction,
                        >,
                    >,
    ) -> Self {
        self.publish_metric_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition {
            publish_metric_action: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> terrars
    ::ListRef<
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition_PublishMetricActionRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_definition: std
    ::option
    ::Option<
        terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    #[doc = "Set the field `action_definition`.\n"]
    pub fn set_action_definition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinition,
                        >,
                    >,
    ) -> Self {
        self.action_definition = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action_name`.\n"]
    pub fn set_action_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction {
            action_definition: core::default::Default::default(),
            action_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_definition` after provisioning.\n"]
    pub fn action_definition(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction_ActionDefinitionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    #[doc = "Set the field `priority`.\n"]
    pub fn set_priority(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.priority = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference {
            priority: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicy {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_tls_session_holding: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_variables: std
    ::option
    ::Option<terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_engine_options: std
    ::option
    ::Option<terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule_group_reference: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_custom_action: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_fragment_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rule_group_reference: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_inspection_configuration_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicy {
    #[doc = "Set the field `enable_tls_session_holding`.\n"]
    pub fn set_enable_tls_session_holding(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_tls_session_holding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_variables`.\n"]
    pub fn set_policy_variables(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariables>>,
    ) -> Self {
        self.policy_variables = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_default_actions`.\n"]
    pub fn set_stateful_default_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stateful_default_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_engine_options`.\n"]
    pub fn set_stateful_engine_options(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptions>,
                    >,
    ) -> Self {
        self.stateful_engine_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateful_rule_group_reference`.\n"]
    pub fn set_stateful_rule_group_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReference>,
                    >,
    ) -> Self {
        self.stateful_rule_group_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateless_custom_action`.\n"]
    pub fn set_stateless_custom_action(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomAction>>,
    ) -> Self {
        self.stateless_custom_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateless_default_actions`.\n"]
    pub fn set_stateless_default_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stateless_default_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateless_fragment_default_actions`.\n"]
    pub fn set_stateless_fragment_default_actions(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stateless_fragment_default_actions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stateless_rule_group_reference`.\n"]
    pub fn set_stateless_rule_group_reference(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::SetField<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReference>,
                    >,
    ) -> Self {
        self.stateless_rule_group_reference = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_inspection_configuration_arn`.\n"]
    pub fn set_tls_inspection_configuration_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tls_inspection_configuration_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicy_FirewallPolicy {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicy_FirewallPolicy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy {}

impl BuildDataNetworkfirewallFirewallPolicy_FirewallPolicy {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicy_FirewallPolicy {
        DataNetworkfirewallFirewallPolicy_FirewallPolicy {
            enable_tls_session_holding: core::default::Default::default(),
            policy_variables: core::default::Default::default(),
            stateful_default_actions: core::default::Default::default(),
            stateful_engine_options: core::default::Default::default(),
            stateful_rule_group_reference: core::default::Default::default(),
            stateless_custom_action: core::default::Default::default(),
            stateless_default_actions: core::default::Default::default(),
            stateless_fragment_default_actions: core::default::Default::default(),
            stateless_rule_group_reference: core::default::Default::default(),
            tls_inspection_configuration_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataNetworkfirewallFirewallPolicy_FirewallPolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicy_FirewallPolicyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicy_FirewallPolicyRef {
        DataNetworkfirewallFirewallPolicy_FirewallPolicyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicy_FirewallPolicyRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enable_tls_session_holding` after provisioning.\n"]
    pub fn enable_tls_session_holding(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_tls_session_holding", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_variables` after provisioning.\n"]
    pub fn policy_variables(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_PolicyVariablesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_variables", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_default_actions` after provisioning.\n"]
    pub fn stateful_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_engine_options` after provisioning.\n"]
    pub fn stateful_engine_options(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulEngineOptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_engine_options", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_rule_group_reference` after provisioning.\n"]
    pub fn stateful_rule_group_reference(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatefulRuleGroupReferenceRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_rule_group_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_custom_action` after provisioning.\n"]
    pub fn stateless_custom_action(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessCustomActionRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_custom_action", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_default_actions` after provisioning.\n"]
    pub fn stateless_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_fragment_default_actions` after provisioning.\n"]
    pub fn stateless_fragment_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_fragment_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_rule_group_reference` after provisioning.\n"]
    pub fn stateless_rule_group_reference(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicy_FirewallPolicy_StatelessRuleGroupReferenceRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_rule_group_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `tls_inspection_configuration_arn` after provisioning.\n"]
    pub fn tls_inspection_configuration_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_inspection_configuration_arn", self.base))
    }
}
