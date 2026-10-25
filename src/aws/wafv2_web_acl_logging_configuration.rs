#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Wafv2WebAclLoggingConfigurationData_ {
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
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    log_destination_configs: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    resource_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_filter: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_LoggingFilter>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_RedactedFields>>,
    dynamic: Wafv2WebAclLoggingConfigurationDynamic,
}

struct Wafv2WebAclLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Wafv2WebAclLoggingConfigurationData_>,
}

#[derive(Clone)]
pub struct Wafv2WebAclLoggingConfiguration(std::rc::Rc<Wafv2WebAclLoggingConfiguration_>);

impl Wafv2WebAclLoggingConfiguration {
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

    #[doc = "Set the field `logging_filter`.\n"]
    pub fn set_logging_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().redacted_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.redacted_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_destination_configs` after provisioning.\nAWS Kinesis Firehose Delivery Stream ARNs"]
    pub fn log_destination_configs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_destination_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\nAWS WebACL ARN"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_LoggingFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_RedactedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.extract_ref()))
    }
}

impl terrars::Referable for Wafv2WebAclLoggingConfiguration {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Wafv2WebAclLoggingConfiguration { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration {
    type O = terrars::ListRef<Wafv2WebAclLoggingConfigurationRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Wafv2WebAclLoggingConfiguration_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_wafv2_web_acl_logging_configuration".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration {
    pub tf_id: std::string::String,
    #[doc = "AWS Kinesis Firehose Delivery Stream ARNs"]
    pub log_destination_configs: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[doc = "AWS WebACL ARN"]
    pub resource_arn: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfiguration {
    pub fn build(self, stack: &mut terrars::Stack) -> Wafv2WebAclLoggingConfiguration {
        let out = Wafv2WebAclLoggingConfiguration(std::rc::Rc::new(Wafv2WebAclLoggingConfiguration_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Wafv2WebAclLoggingConfigurationData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                log_destination_configs: self.log_destination_configs,
                region: core::default::Default::default(),
                resource_arn: self.resource_arn,
                logging_filter: core::default::Default::default(),
                redacted_fields: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Wafv2WebAclLoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `log_destination_configs` after provisioning.\nAWS Kinesis Firehose Delivery Stream ARNs"]
    pub fn log_destination_configs(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.log_destination_configs", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\nAWS WebACL ARN"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_LoggingFilterRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_RedactedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationDynamic {
    logging_filter: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_LoggingFilter>>,
    redacted_fields: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_RedactedFields>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition {
    action: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition {
    type O =
        terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition { action: self.action }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionConditionRef {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionConditionRef {
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
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition {
    label_name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition {
    type O =
        terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition {
    #[doc = ""]
    pub label_name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition {
            label_name: self.label_name,
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameConditionRef {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_name` after provisioning.\n"]
    pub fn label_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionDynamic {
    action_condition: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition>>,
    label_name_condition: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_condition: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_name_condition: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition>>,
    dynamic: Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionDynamic,
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {
    #[doc = "Set the field `action_condition`.\n"]
    pub fn set_action_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action_condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `label_name_condition`.\n"]
    pub fn set_label_name_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameCondition,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.label_name_condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.label_name_condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {}

impl BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition {
            action_condition: core::default::Default::default(),
            label_name_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionRef {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_condition` after provisioning.\n"]
    pub fn action_condition(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_ActionConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `label_name_condition` after provisioning.\n"]
    pub fn label_name_condition(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition_LabelNameConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_name_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterDynamic {
    condition: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
    behavior: terrars::PrimField<std::string::String>,
    requirement: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition>>,
    dynamic: Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterDynamic,
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub requirement: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter {
            behavior: self.behavior,
            requirement: self.requirement,
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterRef {
        Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter_FilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `behavior` after provisioning.\n"]
    pub fn behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.behavior", self.base))
    }

    #[doc = "Get a reference to the value of field `requirement` after provisioning.\n"]
    pub fn requirement(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.requirement", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfiguration_LoggingFilterDynamic {
    filter: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilter {
    default_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter>>,
    dynamic: Wafv2WebAclLoggingConfiguration_LoggingFilterDynamic,
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilter {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter_Filter>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_LoggingFilter {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_LoggingFilter>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_LoggingFilter {
    #[doc = ""]
    pub default_behavior: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfiguration_LoggingFilter {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_LoggingFilter {
        Wafv2WebAclLoggingConfiguration_LoggingFilter {
            default_behavior: self.default_behavior,
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_LoggingFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_LoggingFilterRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_LoggingFilterRef {
        Wafv2WebAclLoggingConfiguration_LoggingFilterRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_LoggingFilterRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_behavior` after provisioning.\n"]
    pub fn default_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_Method {}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_Method { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_RedactedFields_Method {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_Method>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_RedactedFields_Method {}

impl BuildWafv2WebAclLoggingConfiguration_RedactedFields_Method {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_RedactedFields_Method {
        Wafv2WebAclLoggingConfiguration_RedactedFields_Method {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_MethodRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_RedactedFields_MethodRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_RedactedFields_MethodRef {
        Wafv2WebAclLoggingConfiguration_RedactedFields_MethodRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_MethodRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString {}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_RedactedFields_QueryString {}

impl BuildWafv2WebAclLoggingConfiguration_RedactedFields_QueryString {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString {
        Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_QueryStringRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_RedactedFields_QueryStringRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_RedactedFields_QueryStringRef {
        Wafv2WebAclLoggingConfiguration_RedactedFields_QueryStringRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_QueryStringRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader {
    name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader {
        Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader { name: self.name }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeaderRef {
        Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath {}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_RedactedFields_UriPath {}

impl BuildWafv2WebAclLoggingConfiguration_RedactedFields_UriPath {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath {
        Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath {}
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields_UriPathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_RedactedFields_UriPathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_RedactedFields_UriPathRef {
        Wafv2WebAclLoggingConfiguration_RedactedFields_UriPathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_RedactedFields_UriPathRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfiguration_RedactedFieldsDynamic {
    method: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_RedactedFields_Method>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString>>,
    single_header: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader>>,
    uri_path: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_RedactedFields_Method>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_header: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_path: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath>>,
    dynamic: Wafv2WebAclLoggingConfiguration_RedactedFieldsDynamic,
}

impl Wafv2WebAclLoggingConfiguration_RedactedFields {
    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_Method>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.method = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.method = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `query_string`.\n"]
    pub fn set_query_string(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_QueryString>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.query_string = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.query_string = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `single_header`.\n"]
    pub fn set_single_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uri_path`.\n"]
    pub fn set_uri_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields_UriPath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.uri_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.uri_path = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Wafv2WebAclLoggingConfiguration_RedactedFields {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfiguration_RedactedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfiguration_RedactedFields {}

impl BuildWafv2WebAclLoggingConfiguration_RedactedFields {
    pub fn build(self) -> Wafv2WebAclLoggingConfiguration_RedactedFields {
        Wafv2WebAclLoggingConfiguration_RedactedFields {
            method: core::default::Default::default(),
            query_string: core::default::Default::default(),
            single_header: core::default::Default::default(),
            uri_path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Wafv2WebAclLoggingConfiguration_RedactedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfiguration_RedactedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfiguration_RedactedFieldsRef {
        Wafv2WebAclLoggingConfiguration_RedactedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfiguration_RedactedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_RedactedFields_MethodRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_RedactedFields_QueryStringRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `single_header` after provisioning.\n"]
    pub fn single_header(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_RedactedFields_SingleHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_header", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_path` after provisioning.\n"]
    pub fn uri_path(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfiguration_RedactedFields_UriPathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri_path", self.base))
    }
}
