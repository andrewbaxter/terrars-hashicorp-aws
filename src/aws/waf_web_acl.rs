#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafWebAclData_ {
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
    metric_name: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_action: std::option::Option<std::vec::Vec<WafWebAcl_DefaultAction>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<WafWebAcl_LoggingConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<WafWebAcl_Rules>>,
    dynamic: WafWebAclDynamic,
}

struct WafWebAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WafWebAclData_>,
}

#[derive(Clone)]
pub struct WafWebAcl(std::rc::Rc<WafWebAcl_>);

impl WafWebAcl {
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

    #[doc = "Set the field `default_action`.\n"]
    pub fn set_default_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAcl_DefaultAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAcl_LoggingConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<WafWebAcl_Rules>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<WafWebAcl_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafWebAcl_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for WafWebAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WafWebAcl { }

impl terrars::ToListMappable for WafWebAcl {
    type O = terrars::ListRef<WafWebAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WafWebAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_waf_web_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafWebAcl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> WafWebAcl {
        let out = WafWebAcl(std::rc::Rc::new(WafWebAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WafWebAclData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                metric_name: self.metric_name,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                default_action: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                rules: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafWebAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WafWebAclRef {
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

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> terrars::ListRef<WafWebAcl_DefaultActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafWebAcl_LoggingConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct WafWebAclDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<WafWebAcl_DefaultAction>>,
    logging_configuration: std::option::Option<terrars::DynamicBlock<WafWebAcl_LoggingConfiguration>>,
    rules: std::option::Option<terrars::DynamicBlock<WafWebAcl_Rules>>,
}

#[derive(Serialize)]
pub struct WafWebAcl_DefaultAction {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAcl_DefaultAction { }

impl terrars::ToListMappable for WafWebAcl_DefaultAction {
    type O = terrars::BlockAssignable<WafWebAcl_DefaultAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_DefaultAction {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl_DefaultAction {
    pub fn build(self) -> WafWebAcl_DefaultAction {
        WafWebAcl_DefaultAction { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_DefaultActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_DefaultActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAcl_DefaultActionRef {
        WafWebAcl_DefaultActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_DefaultActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    type O = terrars::BlockAssignable<WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
    pub fn build(self) -> WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
        WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
        WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatchRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAcl_LoggingConfiguration_RedactedFieldsDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>>,
}

#[derive(Serialize)]
pub struct WafWebAcl_LoggingConfiguration_RedactedFields {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>>,
    dynamic: WafWebAcl_LoggingConfiguration_RedactedFieldsDynamic,
}

impl WafWebAcl_LoggingConfiguration_RedactedFields {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WafWebAcl_LoggingConfiguration_RedactedFields_FieldToMatch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field_to_match = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafWebAcl_LoggingConfiguration_RedactedFields {
    type O = terrars::BlockAssignable<WafWebAcl_LoggingConfiguration_RedactedFields>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_LoggingConfiguration_RedactedFields {}

impl BuildWafWebAcl_LoggingConfiguration_RedactedFields {
    pub fn build(self) -> WafWebAcl_LoggingConfiguration_RedactedFields {
        WafWebAcl_LoggingConfiguration_RedactedFields {
            field_to_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_LoggingConfiguration_RedactedFieldsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_LoggingConfiguration_RedactedFieldsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafWebAcl_LoggingConfiguration_RedactedFieldsRef {
        WafWebAcl_LoggingConfiguration_RedactedFieldsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_LoggingConfiguration_RedactedFieldsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WafWebAcl_LoggingConfigurationDynamic {
    redacted_fields: std::option::Option<terrars::DynamicBlock<WafWebAcl_LoggingConfiguration_RedactedFields>>,
}

#[derive(Serialize)]
pub struct WafWebAcl_LoggingConfiguration {
    log_destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std::option::Option<std::vec::Vec<WafWebAcl_LoggingConfiguration_RedactedFields>>,
    dynamic: WafWebAcl_LoggingConfigurationDynamic,
}

impl WafWebAcl_LoggingConfiguration {
    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAcl_LoggingConfiguration_RedactedFields>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.redacted_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.redacted_fields = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafWebAcl_LoggingConfiguration {
    type O = terrars::BlockAssignable<WafWebAcl_LoggingConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_LoggingConfiguration {
    #[doc = ""]
    pub log_destination: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl_LoggingConfiguration {
    pub fn build(self) -> WafWebAcl_LoggingConfiguration {
        WafWebAcl_LoggingConfiguration {
            log_destination: self.log_destination,
            redacted_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_LoggingConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_LoggingConfigurationRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAcl_LoggingConfigurationRef {
        WafWebAcl_LoggingConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_LoggingConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<WafWebAcl_LoggingConfiguration_RedactedFieldsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAcl_Rules_Action {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAcl_Rules_Action { }

impl terrars::ToListMappable for WafWebAcl_Rules_Action {
    type O = terrars::BlockAssignable<WafWebAcl_Rules_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_Rules_Action {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl_Rules_Action {
    pub fn build(self) -> WafWebAcl_Rules_Action {
        WafWebAcl_Rules_Action { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_Rules_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_Rules_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAcl_Rules_ActionRef {
        WafWebAcl_Rules_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_Rules_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAcl_Rules_OverrideAction {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAcl_Rules_OverrideAction { }

impl terrars::ToListMappable for WafWebAcl_Rules_OverrideAction {
    type O = terrars::BlockAssignable<WafWebAcl_Rules_OverrideAction>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_Rules_OverrideAction {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl_Rules_OverrideAction {
    pub fn build(self) -> WafWebAcl_Rules_OverrideAction {
        WafWebAcl_Rules_OverrideAction { type_: self.type_ }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_Rules_OverrideActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_Rules_OverrideActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAcl_Rules_OverrideActionRef {
        WafWebAcl_Rules_OverrideActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_Rules_OverrideActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAcl_RulesDynamic {
    action: std::option::Option<terrars::DynamicBlock<WafWebAcl_Rules_Action>>,
    override_action: std::option::Option<terrars::DynamicBlock<WafWebAcl_Rules_OverrideAction>>,
}

#[derive(Serialize)]
pub struct WafWebAcl_Rules {
    priority: terrars::PrimField<f64>,
    rule_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<WafWebAcl_Rules_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_action: std::option::Option<std::vec::Vec<WafWebAcl_Rules_OverrideAction>>,
    dynamic: WafWebAcl_RulesDynamic,
}

impl WafWebAcl_Rules {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::BlockAssignable<WafWebAcl_Rules_Action>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `override_action`.\n"]
    pub fn set_override_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAcl_Rules_OverrideAction>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.override_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.override_action = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for WafWebAcl_Rules {
    type O = terrars::BlockAssignable<WafWebAcl_Rules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAcl_Rules {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAcl_Rules {
    pub fn build(self) -> WafWebAcl_Rules {
        WafWebAcl_Rules {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            override_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct WafWebAcl_RulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAcl_RulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAcl_RulesRef {
        WafWebAcl_RulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAcl_RulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<WafWebAcl_Rules_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> terrars::ListRef<WafWebAcl_Rules_OverrideActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }
}
