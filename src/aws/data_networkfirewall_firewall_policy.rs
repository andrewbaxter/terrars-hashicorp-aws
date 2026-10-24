use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataNetworkfirewallFirewallPolicyData {
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

struct DataNetworkfirewallFirewallPolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataNetworkfirewallFirewallPolicyData>,
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
    pub fn firewall_policy(&self) -> terrars::ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElRef> {
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
            data: std::cell::RefCell::new(DataNetworkfirewallFirewallPolicyData {
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
    pub fn firewall_policy(&self) -> terrars::ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElRef> {
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
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_order: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    #[doc = "Set the field `rule_order`.\n"]
    pub fn set_rule_order(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_order = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl {
            rule_order: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_order` after provisioning.\n"]
    pub fn rule_order(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
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

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl {
            priority: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef {
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
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl {
            value: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std
    ::option
    ::Option<
        terrars
        ::SetField<
            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
        >,
    >,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
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
                            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionEl,
                        >,
                    >,
    ) -> Self {
        self.dimension = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    type O =
        terrars
        ::BlockAssignable<
            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
    pub fn build(
        self,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl {
            dimension: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(
        &self,
    ) -> terrars
    ::SetRef<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElDimensionElRef,
    > {
        terrars::SetRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_metric_action: std
    ::option
    ::Option<
        terrars
        ::ListField<
            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
        >,
    >,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
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
                            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionEl,
                        >,
                    >,
    ) -> Self {
        self.publish_metric_action = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    type O =
        terrars
        ::BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl {
            publish_metric_action: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `publish_metric_action` after provisioning.\n"]
    pub fn publish_metric_action(
        &self,
    ) -> terrars
    ::ListRef<
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElPublishMetricActionElRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_metric_action", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_definition: std
    ::option
    ::Option<
        terrars::ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
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
                            DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionEl,
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

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl {
            action_definition: core::default::Default::default(),
            action_name: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_definition` after provisioning.\n"]
    pub fn action_definition(
        &self,
    ) -> terrars
    ::ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElActionDefinitionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_definition", self.base))
    }

    #[doc = "Get a reference to the value of field `action_name` after provisioning.\n"]
    pub fn action_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    priority: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
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

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    type O =
        terrars::BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl {
            priority: core::default::Default::default(),
            resource_arn: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef {
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
pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_engine_options: std
    ::option
    ::Option<terrars::ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateful_rule_group_reference: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_custom_action: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_fragment_default_actions: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stateless_rule_group_reference: std
    ::option
    ::Option<terrars::SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>>,
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
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
                        terrars::ListField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsEl>,
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
                        terrars
                        ::SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceEl>,
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
                    ::Into<
                        terrars::SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionEl>,
                    >,
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
                        ::SetField<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceEl>,
                    >,
    ) -> Self {
        self.stateless_rule_group_reference = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    type O = terrars::BlockAssignable<DataNetworkfirewallFirewallPolicyFirewallPolicyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataNetworkfirewallFirewallPolicyFirewallPolicyEl {}

impl BuildDataNetworkfirewallFirewallPolicyFirewallPolicyEl {
    pub fn build(self) -> DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
        DataNetworkfirewallFirewallPolicyFirewallPolicyEl {
            stateful_default_actions: core::default::Default::default(),
            stateful_engine_options: core::default::Default::default(),
            stateful_rule_group_reference: core::default::Default::default(),
            stateless_custom_action: core::default::Default::default(),
            stateless_default_actions: core::default::Default::default(),
            stateless_fragment_default_actions: core::default::Default::default(),
            stateless_rule_group_reference: core::default::Default::default(),
        }
    }
}

pub struct DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
        DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataNetworkfirewallFirewallPolicyFirewallPolicyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `stateful_default_actions` after provisioning.\n"]
    pub fn stateful_default_actions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_default_actions", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_engine_options` after provisioning.\n"]
    pub fn stateful_engine_options(
        &self,
    ) -> terrars::ListRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulEngineOptionsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stateful_engine_options", self.base))
    }

    #[doc = "Get a reference to the value of field `stateful_rule_group_reference` after provisioning.\n"]
    pub fn stateful_rule_group_reference(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatefulRuleGroupReferenceElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateful_rule_group_reference", self.base))
    }

    #[doc = "Get a reference to the value of field `stateless_custom_action` after provisioning.\n"]
    pub fn stateless_custom_action(
        &self,
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessCustomActionElRef> {
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
    ) -> terrars::SetRef<DataNetworkfirewallFirewallPolicyFirewallPolicyElStatelessRuleGroupReferenceElRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.stateless_rule_group_reference", self.base))
    }
}
