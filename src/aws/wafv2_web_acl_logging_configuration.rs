use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Wafv2WebAclLoggingConfigurationData {
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
    resource_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_filter: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationLoggingFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationDynamic,
}

struct Wafv2WebAclLoggingConfiguration_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Wafv2WebAclLoggingConfigurationData>,
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

    #[doc = "Set the field `logging_filter`.\n"]
    pub fn set_logging_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>>,
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

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\nAWS WebACL ARN"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationLoggingFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElRef> {
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
            data: std::cell::RefCell::new(Wafv2WebAclLoggingConfigurationData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                log_destination_configs: self.log_destination_configs,
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

    #[doc = "Get a reference to the value of field `resource_arn` after provisioning.\nAWS WebACL ARN"]
    pub fn resource_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_filter` after provisioning.\n"]
    pub fn logging_filter(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationLoggingFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    action: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    type O =
        terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl { action: self.action }
    }
}

pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
    label_name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
    type O =
        terrars
        ::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
    #[doc = ""]
    pub label_name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl {
            label_name: self.label_name,
        }
    }
}

pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `label_name` after provisioning.\n"]
    pub fn label_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.label_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElDynamic {
    action_condition: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl>,
    >,
    label_name_condition: std
    ::option
    ::Option<
        terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl>,
    >,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_condition: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    label_name_condition: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElDynamic,
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
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
                            Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionEl,
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
                            Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionEl,
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

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {}

impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl {
            action_condition: core::default::Default::default(),
            label_name_condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_condition` after provisioning.\n"]
    pub fn action_condition(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElActionConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action_condition", self.base))
    }

    #[doc = "Get a reference to the value of field `label_name_condition` after provisioning.\n"]
    pub fn label_name_condition(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionElLabelNameConditionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.label_name_condition", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElDynamic {
    condition: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    behavior: terrars::PrimField<std::string::String>,
    requirement: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElDynamic,
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElConditionEl>,
                    >,
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

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    #[doc = ""]
    pub behavior: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub requirement: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl {
            behavior: self.behavior,
            requirement: self.requirement,
            condition: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElFilterElRef {
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
struct Wafv2WebAclLoggingConfigurationLoggingFilterElDynamic {
    filter: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationLoggingFilterEl {
    default_behavior: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationLoggingFilterElDynamic,
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterElFilterEl>>,
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

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationLoggingFilterEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationLoggingFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationLoggingFilterEl {
    #[doc = ""]
    pub default_behavior: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfigurationLoggingFilterEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationLoggingFilterEl {
        Wafv2WebAclLoggingConfigurationLoggingFilterEl {
            default_behavior: self.default_behavior,
            filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
        Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationLoggingFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `default_behavior` after provisioning.\n"]
    pub fn default_behavior(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_behavior", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl {}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl {}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl {}
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl {}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl {}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl {}
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl {}
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl {}
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl { name: self.name }
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl {
    name: terrars::PrimField<std::string::String>,
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl { name: self.name }
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl { }

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl {}
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationRedactedFieldsElDynamic {
    all_query_arguments: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl>>,
    body: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl>>,
    method: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>>,
    query_string: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>>,
    single_header: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>>,
    single_query_argument: std
    ::option
    ::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl>>,
    uri_path: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>>,
}

#[derive(Serialize)]
pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    all_query_arguments: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    body: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    method: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    query_string: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_header: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    single_query_argument: std
    ::option
    ::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    uri_path: std::option::Option<std::vec::Vec<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>>,
    dynamic: Wafv2WebAclLoggingConfigurationRedactedFieldsElDynamic,
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
    #[doc = "Set the field `all_query_arguments`.\n"]
    pub fn set_all_query_arguments(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.all_query_arguments = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.all_query_arguments = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `body`.\n"]
    pub fn set_body(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.body = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.body = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `method`.\n"]
    pub fn set_method(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodEl>>,
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
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringEl>>,
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
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderEl>>,
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

    #[doc = "Set the field `single_query_argument`.\n"]
    pub fn set_single_query_argument(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.single_query_argument = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.single_query_argument = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `uri_path`.\n"]
    pub fn set_uri_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathEl>>,
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

impl terrars::ToListMappable for Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
    type O = terrars::BlockAssignable<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafv2WebAclLoggingConfigurationRedactedFieldsEl {}

impl BuildWafv2WebAclLoggingConfigurationRedactedFieldsEl {
    pub fn build(self) -> Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
        Wafv2WebAclLoggingConfigurationRedactedFieldsEl {
            all_query_arguments: core::default::Default::default(),
            body: core::default::Default::default(),
            method: core::default::Default::default(),
            query_string: core::default::Default::default(),
            single_header: core::default::Default::default(),
            single_query_argument: core::default::Default::default(),
            uri_path: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
        Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Wafv2WebAclLoggingConfigurationRedactedFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `all_query_arguments` after provisioning.\n"]
    pub fn all_query_arguments(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElAllQueryArgumentsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.all_query_arguments", self.base))
    }

    #[doc = "Get a reference to the value of field `body` after provisioning.\n"]
    pub fn body(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElBodyElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.body", self.base))
    }

    #[doc = "Get a reference to the value of field `method` after provisioning.\n"]
    pub fn method(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElMethodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.method", self.base))
    }

    #[doc = "Get a reference to the value of field `query_string` after provisioning.\n"]
    pub fn query_string(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElQueryStringElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.query_string", self.base))
    }

    #[doc = "Get a reference to the value of field `single_header` after provisioning.\n"]
    pub fn single_header(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_header", self.base))
    }

    #[doc = "Get a reference to the value of field `single_query_argument` after provisioning.\n"]
    pub fn single_query_argument(
        &self,
    ) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElSingleQueryArgumentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.single_query_argument", self.base))
    }

    #[doc = "Get a reference to the value of field `uri_path` after provisioning.\n"]
    pub fn uri_path(&self) -> terrars::ListRef<Wafv2WebAclLoggingConfigurationRedactedFieldsElUriPathElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.uri_path", self.base))
    }
}

#[derive(Serialize, Default)]
struct Wafv2WebAclLoggingConfigurationDynamic {
    logging_filter: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationLoggingFilterEl>>,
    redacted_fields: std::option::Option<terrars::DynamicBlock<Wafv2WebAclLoggingConfigurationRedactedFieldsEl>>,
}
