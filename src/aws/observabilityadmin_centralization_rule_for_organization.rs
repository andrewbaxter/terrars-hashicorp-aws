#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ObservabilityadminCentralizationRuleForOrganizationData_ {
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
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<ObservabilityadminCentralizationRuleForOrganization_Timeouts>,
    dynamic: ObservabilityadminCentralizationRuleForOrganizationDynamic,
}

struct ObservabilityadminCentralizationRuleForOrganization_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<ObservabilityadminCentralizationRuleForOrganizationData_>,
}

#[derive(Clone)]
pub struct ObservabilityadminCentralizationRuleForOrganization(
    std::rc::Rc<ObservabilityadminCentralizationRuleForOrganization_>,
);

impl ObservabilityadminCentralizationRuleForOrganization {
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

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(
        self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(
        self,
        v: impl std::convert::Into<ObservabilityadminCentralizationRuleForOrganization_Timeouts>,
    ) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_arn` after provisioning.\n"]
    pub fn rule_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_propagation_failure_reason` after provisioning.\n"]
    pub fn tag_propagation_failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tag_propagation_failure_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tag_propagation_status` after provisioning.\n"]
    pub fn tag_propagation_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_propagation_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<ObservabilityadminCentralizationRuleForOrganization_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
        ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

impl terrars::Referable for ObservabilityadminCentralizationRuleForOrganization {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for ObservabilityadminCentralizationRuleForOrganization { }

impl terrars::ToListMappable for ObservabilityadminCentralizationRuleForOrganization {
    type O = terrars::ListRef<ObservabilityadminCentralizationRuleForOrganizationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for ObservabilityadminCentralizationRuleForOrganization_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_observabilityadmin_centralization_rule_for_organization".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub rule_name: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization {
    pub fn build(self, stack: &mut terrars::Stack) -> ObservabilityadminCentralizationRuleForOrganization {
        let out =
            ObservabilityadminCentralizationRuleForOrganization(
                std::rc::Rc::new(ObservabilityadminCentralizationRuleForOrganization_ {
                    shared: stack.shared.clone(),
                    tf_id: self.tf_id,
                    data: std::cell::RefCell::new(ObservabilityadminCentralizationRuleForOrganizationData_ {
                        depends_on: core::default::Default::default(),
                        provider: std::option::Option::None,
                        lifecycle: core::default::Default::default(),
                        for_each: std::option::Option::None,
                        provisioner: core::default::Default::default(),
                        region: core::default::Default::default(),
                        rule_name: self.rule_name,
                        tags: core::default::Default::default(),
                        rule: core::default::Default::default(),
                        timeouts: core::default::Default::default(),
                        dynamic: core::default::Default::default(),
                    }),
                }),
            );
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ObservabilityadminCentralizationRuleForOrganizationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganizationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganizationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_arn` after provisioning.\n"]
    pub fn rule_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_name` after provisioning.\n"]
    pub fn rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tag_propagation_failure_reason` after provisioning.\n"]
    pub fn tag_propagation_failure_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.tag_propagation_failure_reason", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `tag_propagation_status` after provisioning.\n"]
    pub fn tag_propagation_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_propagation_status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<ObservabilityadminCentralizationRuleForOrganization_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
        ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef::new(
            self.shared().clone(),
            format!("{}.timeouts", self.extract_ref()),
        )
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganizationDynamic {
    rule: std::option::Option<terrars::DynamicBlock<ObservabilityadminCentralizationRuleForOrganization_Rule>>,
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `region`.\n"]
    pub fn set_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.region = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration {
            kms_key_arn: core::default::Default::default(),
            region: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {
    log_group_name_pattern: terrars::PrimField<std::string::String>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {

}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {
    #[doc = ""]
    pub log_group_name_pattern: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration {
            log_group_name_pattern: self.log_group_name_pattern,
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_group_name_pattern` after provisioning.\n"]
    pub fn log_group_name_pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_name_pattern", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_conflict_resolution_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_scope: std::option::Option<terrars::PrimField<std::string::String>>,
    encryption_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
    #[doc = "Set the field `encryption_conflict_resolution_strategy`.\n"]
    pub fn set_encryption_conflict_resolution_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.encryption_conflict_resolution_strategy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encryption_scope`.\n"]
    pub fn set_encryption_scope(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encryption_scope = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
    #[doc = ""]
    pub encryption_strategy: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration {
            encryption_conflict_resolution_strategy: core::default::Default::default(),
            encryption_scope: core::default::Default::default(),
            encryption_strategy: self.encryption_strategy,
            kms_key_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encryption_conflict_resolution_strategy` after provisioning.\n"]
    pub fn encryption_conflict_resolution_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.encryption_conflict_resolution_strategy", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `encryption_scope` after provisioning.\n"]
    pub fn encryption_scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_scope", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_strategy` after provisioning.\n"]
    pub fn encryption_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encryption_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
    destination_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_conflict_resolution_strategy: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
    #[doc = "Set the field `tag_conflict_resolution_strategy`.\n"]
    pub fn set_tag_conflict_resolution_strategy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.tag_conflict_resolution_strategy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
    #[doc = ""]
    pub destination_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration {
            destination_role_arn: self.destination_role_arn,
            tag_conflict_resolution_strategy: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_role_arn` after provisioning.\n"]
    pub fn destination_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_conflict_resolution_strategy` after provisioning.\n"]
    pub fn tag_conflict_resolution_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tag_conflict_resolution_strategy", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationDynamic {
    backup_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration,
        >,
    >,
    log_group_name_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration,
        >,
    >,
    logs_encryption_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration,
        >,
    >,
    tag_propagation_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_name_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logs_encryption_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration,
        >,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tag_propagation_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration,
        >,
    >,
    dynamic: ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationDynamic,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {
    #[doc = "Set the field `backup_configuration`.\n"]
    pub fn set_backup_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `log_group_name_configuration`.\n"]
    pub fn set_log_group_name_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.log_group_name_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.log_group_name_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logs_encryption_configuration`.\n"]
    pub fn set_logs_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.logs_encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.logs_encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tag_propagation_configuration`.\n"]
    pub fn set_tag_propagation_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tag_propagation_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tag_propagation_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration {
            backup_configuration: core::default::Default::default(),
            log_group_name_configuration: core::default::Default::default(),
            logs_encryption_configuration: core::default::Default::default(),
            tag_propagation_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_BackupConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_name_configuration` after provisioning.\n"]
    pub fn log_group_name_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogGroupNameConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.log_group_name_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `logs_encryption_configuration` after provisioning.\n"]
    pub fn logs_encryption_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_LogsEncryptionConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logs_encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `tag_propagation_configuration` after provisioning.\n"]
    pub fn tag_propagation_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration_TagPropagationConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tag_propagation_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {
    region: terrars::PrimField<std::string::String>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {

}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration {
            region: self.region,
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationDynamic {
    backup_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    backup_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration,
        >,
    >,
    dynamic: ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationDynamic,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {
    #[doc = "Set the field `backup_configuration`.\n"]
    pub fn set_backup_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.backup_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.backup_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {
    type O =
        terrars
        ::BlockAssignable<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {
    pub fn build(
        self,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration {
            backup_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars
::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `backup_configuration` after provisioning.\n"]
    pub fn backup_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration_BackupConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.backup_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationDynamic {
    destination_logs_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration,
        >,
    >,
    destination_metrics_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<
            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration,
        >,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
    account: terrars::PrimField<std::string::String>,
    region: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_logs_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_metrics_configuration: std
    ::option
    ::Option<
        std
        ::vec
        ::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration>,
    >,
    dynamic: ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationDynamic,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
    #[doc = "Set the field `destination_logs_configuration`.\n"]
    pub fn set_destination_logs_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_logs_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_logs_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `destination_metrics_configuration`.\n"]
    pub fn set_destination_metrics_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination_metrics_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination_metrics_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
    type O = terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
    #[doc = ""]
    pub account: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub region: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination {
            account: self.account,
            region: self.region,
            destination_logs_configuration: core::default::Default::default(),
            destination_metrics_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account` after provisioning.\n"]
    pub fn account(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account", self.base))
    }

    #[doc = "Get a reference to the value of field `region` after provisioning.\n"]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_logs_configuration` after provisioning.\n"]
    pub fn destination_logs_configuration(
        &self,
    ) -> terrars
    ::ListRef<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationLogsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_logs_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_metrics_configuration` after provisioning.\n"]
    pub fn destination_metrics_configuration(
        &self,
    ) -> terrars
    ::ListRef<
        ObservabilityadminCentralizationRuleForOrganization_Rule_Destination_DestinationMetricsConfigurationRef,
    > {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_metrics_configuration", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_source_selection_criteria: std::option::Option<terrars::PrimField<std::string::String>>,
    encrypted_log_group_strategy: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    log_group_selection_criteria: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
    #[doc = "Set the field `data_source_selection_criteria`.\n"]
    pub fn set_data_source_selection_criteria(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.data_source_selection_criteria = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `log_group_selection_criteria`.\n"]
    pub fn set_log_group_selection_criteria(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.log_group_selection_criteria = std::option::Option::Some(v.into());
        self
    }
}

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
    type O =
        terrars
        ::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
    #[doc = ""]
    pub encrypted_log_group_strategy: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration {
            data_source_selection_criteria: core::default::Default::default(),
            encrypted_log_group_strategy: self.encrypted_log_group_strategy,
            log_group_selection_criteria: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data_source_selection_criteria` after provisioning.\n"]
    pub fn data_source_selection_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data_source_selection_criteria", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted_log_group_strategy` after provisioning.\n"]
    pub fn encrypted_log_group_strategy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted_log_group_strategy", self.base))
    }

    #[doc = "Get a reference to the value of field `log_group_selection_criteria` after provisioning.\n"]
    pub fn log_group_selection_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_group_selection_criteria", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration {
    metrics_selection_criteria: terrars::PrimField<std::string::String>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration { }

impl terrars
::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration {
    type O =
        terrars
        ::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration {
    #[doc = ""]
    pub metrics_selection_criteria: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration {
            metrics_selection_criteria: self.metrics_selection_criteria,
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfigurationRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `metrics_selection_criteria` after provisioning.\n"]
    pub fn metrics_selection_criteria(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metrics_selection_criteria", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganization_Rule_SourceDynamic {
    source_logs_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration>,
    >,
    source_metrics_configuration: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration>,
    >,
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_Source {
    regions: terrars::SetField<terrars::PrimField<std::string::String>>,
    scope: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_logs_configuration: std
    ::option
    ::Option<std::vec::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_metrics_configuration: std
    ::option
    ::Option<
        std::vec::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration>,
    >,
    dynamic: ObservabilityadminCentralizationRuleForOrganization_Rule_SourceDynamic,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_Source {
    #[doc = "Set the field `source_logs_configuration`.\n"]
    pub fn set_source_logs_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_logs_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_logs_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_metrics_configuration`.\n"]
    pub fn set_source_metrics_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfiguration,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_metrics_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_metrics_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule_Source {
    type O = terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule_Source>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule_Source {
    #[doc = ""]
    pub regions: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule_Source {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganization_Rule_Source {
        ObservabilityadminCentralizationRuleForOrganization_Rule_Source {
            regions: self.regions,
            scope: self.scope,
            source_logs_configuration: core::default::Default::default(),
            source_metrics_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule_SourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganization_Rule_SourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_Rule_SourceRef {
        ObservabilityadminCentralizationRuleForOrganization_Rule_SourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule_SourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `regions` after provisioning.\n"]
    pub fn regions(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.regions", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `source_logs_configuration` after provisioning.\n"]
    pub fn source_logs_configuration(
        &self,
    ) -> terrars::ListRef<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceLogsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_logs_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `source_metrics_configuration` after provisioning.\n"]
    pub fn source_metrics_configuration(
        &self,
    ) -> terrars
    ::ListRef<ObservabilityadminCentralizationRuleForOrganization_Rule_Source_SourceMetricsConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_metrics_configuration", self.base))
    }
}

#[derive(Serialize, Default)]
struct ObservabilityadminCentralizationRuleForOrganization_RuleDynamic {
    destination: std
    ::option
    ::Option<terrars::DynamicBlock<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination>>,
    source: std
    ::option
    ::Option<terrars::DynamicBlock<ObservabilityadminCentralizationRuleForOrganization_Rule_Source>>,
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<std::vec::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<ObservabilityadminCentralizationRuleForOrganization_Rule_Source>>,
    dynamic: ObservabilityadminCentralizationRuleForOrganization_RuleDynamic,
}

impl ObservabilityadminCentralizationRuleForOrganization_Rule {
    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule_Destination>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.destination = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.destination = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule_Source>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Rule {
    type O = terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Rule {}

impl BuildObservabilityadminCentralizationRuleForOrganization_Rule {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganization_Rule {
        ObservabilityadminCentralizationRuleForOrganization_Rule {
            destination: core::default::Default::default(),
            source: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganization_RuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_RuleRef {
        ObservabilityadminCentralizationRuleForOrganization_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(
        &self,
    ) -> terrars::ListRef<ObservabilityadminCentralizationRuleForOrganization_Rule_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<ObservabilityadminCentralizationRuleForOrganization_Rule_SourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.base))
    }
}

#[derive(Serialize)]
pub struct ObservabilityadminCentralizationRuleForOrganization_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl ObservabilityadminCentralizationRuleForOrganization_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for ObservabilityadminCentralizationRuleForOrganization_Timeouts {
    type O = terrars::BlockAssignable<ObservabilityadminCentralizationRuleForOrganization_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildObservabilityadminCentralizationRuleForOrganization_Timeouts {}

impl BuildObservabilityadminCentralizationRuleForOrganization_Timeouts {
    pub fn build(self) -> ObservabilityadminCentralizationRuleForOrganization_Timeouts {
        ObservabilityadminCentralizationRuleForOrganization_Timeouts {
            create: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
        ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl ObservabilityadminCentralizationRuleForOrganization_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
