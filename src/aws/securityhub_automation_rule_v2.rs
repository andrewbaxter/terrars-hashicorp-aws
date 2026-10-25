#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubAutomationRuleV2Data_ {
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
    description: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_name: terrars::PrimField<std::string::String>,
    rule_order: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<SecurityhubAutomationRuleV2_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criteria: std::option::Option<std::vec::Vec<SecurityhubAutomationRuleV2_Criteria>>,
    dynamic: SecurityhubAutomationRuleV2Dynamic,
}

struct SecurityhubAutomationRuleV2_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityhubAutomationRuleV2Data_>,
}

#[derive(Clone)]
pub struct SecurityhubAutomationRuleV2(std::rc::Rc<SecurityhubAutomationRuleV2_>);

impl SecurityhubAutomationRuleV2 {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_status`.\nThe status of the rule: ENABLED or DISABLED."]
    pub fn set_rule_status(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().rule_status = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRuleV2_Action>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `criteria`.\n"]
    pub fn set_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRuleV2_Criteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the automation rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nThe name of the automation rule."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_order` after provisioning.\nThe priority of the rule (lower values = higher priority)."]
    pub fn rule_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_status` after provisioning.\nThe status of the rule: ENABLED or DISABLED."]
    pub fn rule_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<SecurityhubAutomationRuleV2_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> terrars::ListRef<SecurityhubAutomationRuleV2_CriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criteria", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityhubAutomationRuleV2 {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityhubAutomationRuleV2 { }

impl terrars::ToListMappable for SecurityhubAutomationRuleV2 {
    type O = terrars::ListRef<SecurityhubAutomationRuleV2Ref>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityhubAutomationRuleV2_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securityhub_automation_rule_v2".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubAutomationRuleV2 {
    pub tf_id: std::string::String,
    #[doc = "A description of the automation rule."]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = "The name of the automation rule."]
    pub rule_name: terrars::PrimField<std::string::String>,
    #[doc = "The priority of the rule (lower values = higher priority)."]
    pub rule_order: terrars::PrimField<f64>,
}

impl BuildSecurityhubAutomationRuleV2 {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityhubAutomationRuleV2 {
        let out = SecurityhubAutomationRuleV2(std::rc::Rc::new(SecurityhubAutomationRuleV2_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityhubAutomationRuleV2Data_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                region: core::default::Default::default(),
                rule_name: self.rule_name,
                rule_order: self.rule_order,
                rule_status: core::default::Default::default(),
                tags: core::default::Default::default(),
                action: core::default::Default::default(),
                criteria: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubAutomationRuleV2Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRuleV2Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubAutomationRuleV2Ref {
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

    #[doc =
        "Get a reference to the value of field `description` after provisioning.\nA description of the automation rule."]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\nThe name of the automation rule."]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_order` after provisioning.\nThe priority of the rule (lower values = higher priority)."]
    pub fn rule_order(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_order", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `rule_status` after provisioning.\nThe status of the rule: ENABLED or DISABLED."]
    pub fn rule_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<SecurityhubAutomationRuleV2_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `criteria` after provisioning.\n"]
    pub fn criteria(&self) -> terrars::ListRef<SecurityhubAutomationRuleV2_CriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.criteria", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleV2Dynamic {
    action: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRuleV2_Action>>,
    criteria: std::option::Option<terrars::DynamicBlock<SecurityhubAutomationRuleV2_Criteria>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration {
    connector_arn: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration { }

impl terrars::ToListMappable for SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration {
    type O = terrars::BlockAssignable<SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration {
    #[doc = "The ARN of the connector."]
    pub connector_arn: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration {
    pub fn build(self) -> SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration {
        SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration { connector_arn: self.connector_arn }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfigurationRef {
        SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `connector_arn` after provisioning.\nThe ARN of the connector."]
    pub fn connector_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.connector_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    comment: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_id: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_id: std::option::Option<terrars::PrimField<f64>>,
}

impl SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {
    #[doc = "Set the field `comment`.\nA comment for the finding."]
    pub fn set_comment(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.comment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `severity_id`.\nThe severity ID to assign."]
    pub fn set_severity_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.severity_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status_id`.\nThe status ID to assign."]
    pub fn set_status_id(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.status_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {
    type O = terrars::BlockAssignable<SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {}

impl BuildSecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {
    pub fn build(self) -> SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {
        SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate {
            comment: core::default::Default::default(),
            severity_id: core::default::Default::default(),
            status_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRuleV2_Action_FindingFieldsUpdateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRuleV2_Action_FindingFieldsUpdateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubAutomationRuleV2_Action_FindingFieldsUpdateRef {
        SecurityhubAutomationRuleV2_Action_FindingFieldsUpdateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleV2_Action_FindingFieldsUpdateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comment` after provisioning.\nA comment for the finding."]
    pub fn comment(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comment", self.base))
    }

    #[doc = "Get a reference to the value of field `severity_id` after provisioning.\nThe severity ID to assign."]
    pub fn severity_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.severity_id", self.base))
    }

    #[doc = "Get a reference to the value of field `status_id` after provisioning.\nThe status ID to assign."]
    pub fn status_id(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubAutomationRuleV2_ActionDynamic {
    external_integration_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration>>,
    finding_fields_update: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate>>,
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleV2_Action {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    external_integration_configuration: std
    ::option
    ::Option<std::vec::Vec<SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_fields_update: std::option::Option<std::vec::Vec<SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate>>,
    dynamic: SecurityhubAutomationRuleV2_ActionDynamic,
}

impl SecurityhubAutomationRuleV2_Action {
    #[doc = "Set the field `external_integration_configuration`.\n"]
    pub fn set_external_integration_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfiguration>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.external_integration_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.external_integration_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_fields_update`.\n"]
    pub fn set_finding_fields_update(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubAutomationRuleV2_Action_FindingFieldsUpdate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_fields_update = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_fields_update = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubAutomationRuleV2_Action {
    type O = terrars::BlockAssignable<SecurityhubAutomationRuleV2_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleV2_Action {
    #[doc = "The action type: FINDING_FIELDS_UPDATE or EXTERNAL_INTEGRATION."]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRuleV2_Action {
    pub fn build(self) -> SecurityhubAutomationRuleV2_Action {
        SecurityhubAutomationRuleV2_Action {
            type_: self.type_,
            external_integration_configuration: core::default::Default::default(),
            finding_fields_update: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRuleV2_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRuleV2_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRuleV2_ActionRef {
        SecurityhubAutomationRuleV2_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleV2_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `type_` after provisioning.\nThe action type: FINDING_FIELDS_UPDATE or EXTERNAL_INTEGRATION."]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `external_integration_configuration` after provisioning.\n"]
    pub fn external_integration_configuration(
        &self,
    ) -> terrars::ListRef<SecurityhubAutomationRuleV2_Action_ExternalIntegrationConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.external_integration_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `finding_fields_update` after provisioning.\n"]
    pub fn finding_fields_update(&self) -> terrars::ListRef<SecurityhubAutomationRuleV2_Action_FindingFieldsUpdateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.finding_fields_update", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubAutomationRuleV2_Criteria {
    ocsf_finding_criteria_json: terrars::PrimField<std::string::String>,
}

impl SecurityhubAutomationRuleV2_Criteria { }

impl terrars::ToListMappable for SecurityhubAutomationRuleV2_Criteria {
    type O = terrars::BlockAssignable<SecurityhubAutomationRuleV2_Criteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubAutomationRuleV2_Criteria {
    #[doc = "JSON-encoded OCSF finding criteria for the rule."]
    pub ocsf_finding_criteria_json: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubAutomationRuleV2_Criteria {
    pub fn build(self) -> SecurityhubAutomationRuleV2_Criteria {
        SecurityhubAutomationRuleV2_Criteria { ocsf_finding_criteria_json: self.ocsf_finding_criteria_json }
    }
}

#[allow(dead_code)]
pub struct SecurityhubAutomationRuleV2_CriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubAutomationRuleV2_CriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubAutomationRuleV2_CriteriaRef {
        SecurityhubAutomationRuleV2_CriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubAutomationRuleV2_CriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `ocsf_finding_criteria_json` after provisioning.\nJSON-encoded OCSF finding criteria for the rule."]
    pub fn ocsf_finding_criteria_json(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ocsf_finding_criteria_json", self.base))
    }
}
