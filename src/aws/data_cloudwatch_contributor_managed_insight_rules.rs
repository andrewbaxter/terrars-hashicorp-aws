#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataCloudwatchContributorManagedInsightRulesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_arn: terrars::PrimField<std::string::String>,
}

struct DataCloudwatchContributorManagedInsightRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataCloudwatchContributorManagedInsightRulesData_>,
}

#[derive(Clone)]
pub struct DataCloudwatchContributorManagedInsightRules(std::rc::Rc<DataCloudwatchContributorManagedInsightRules_>);

impl DataCloudwatchContributorManagedInsightRules {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `managed_rules` after provisioning.\n"]
    pub fn managed_rules(&self) -> terrars::ListRef<DataCloudwatchContributorManagedInsightRules_ManagedRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }
}

impl terrars::Referable for DataCloudwatchContributorManagedInsightRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataCloudwatchContributorManagedInsightRules { }

impl terrars::ToListMappable for DataCloudwatchContributorManagedInsightRules {
    type O = terrars::ListRef<DataCloudwatchContributorManagedInsightRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataCloudwatchContributorManagedInsightRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_cloudwatch_contributor_managed_insight_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataCloudwatchContributorManagedInsightRules {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildDataCloudwatchContributorManagedInsightRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataCloudwatchContributorManagedInsightRules {
        let out =
            DataCloudwatchContributorManagedInsightRules(
                std::rc::Rc::new(DataCloudwatchContributorManagedInsightRules_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(DataCloudwatchContributorManagedInsightRulesData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        region: core::default::Default::default(),
                        resource_arn: self.resource_arn,
                    }),
                }),
            );
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataCloudwatchContributorManagedInsightRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchContributorManagedInsightRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataCloudwatchContributorManagedInsightRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `managed_rules` after provisioning.\n"]
    pub fn managed_rules(&self) -> terrars::ListRef<DataCloudwatchContributorManagedInsightRules_ManagedRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.managed_rules", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {
    #[doc = "Set the field `rule_name`.\n"]
    pub fn set_rule_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {
    type O = terrars::BlockAssignable<DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {}

impl BuildDataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {
    pub fn build(self) -> DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {
        DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState {
            rule_name: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleStateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleStateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleStateRef {
        DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleStateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleStateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct DataCloudwatchContributorManagedInsightRules_ManagedRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_state: std
    ::option
    ::Option<terrars::ListField<DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    template_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataCloudwatchContributorManagedInsightRules_ManagedRules {
    #[doc = "Set the field `resource_arn`.\n"]
    pub fn set_resource_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_state`.\n"]
    pub fn set_rule_state(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleState>>,
    ) -> Self {
        self.rule_state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `template_name`.\n"]
    pub fn set_template_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.template_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataCloudwatchContributorManagedInsightRules_ManagedRules {
    type O = terrars::BlockAssignable<DataCloudwatchContributorManagedInsightRules_ManagedRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataCloudwatchContributorManagedInsightRules_ManagedRules {}

impl BuildDataCloudwatchContributorManagedInsightRules_ManagedRules {
    pub fn build(self) -> DataCloudwatchContributorManagedInsightRules_ManagedRules {
        DataCloudwatchContributorManagedInsightRules_ManagedRules {
            resource_arn: core::default::Default::default(),
            rule_state: core::default::Default::default(),
            template_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataCloudwatchContributorManagedInsightRules_ManagedRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataCloudwatchContributorManagedInsightRules_ManagedRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataCloudwatchContributorManagedInsightRules_ManagedRulesRef {
        DataCloudwatchContributorManagedInsightRules_ManagedRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataCloudwatchContributorManagedInsightRules_ManagedRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\n"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_state` after provisioning.\n"]
    pub fn rule_state(&self) -> terrars::ListRef<DataCloudwatchContributorManagedInsightRules_ManagedRules_RuleStateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_state", self.base))
    }

    #[doc = "Get a reference to the value of field `template_name` after provisioning.\n"]
    pub fn template_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.template_name", self.base))
    }
}
