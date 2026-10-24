use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafregionalWebAclData {
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
    default_action: std::option::Option<std::vec::Vec<WafregionalWebAclDefaultActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<WafregionalWebAclLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<WafregionalWebAclRuleEl>>,
    dynamic: WafregionalWebAclDynamic,
}

struct WafregionalWebAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WafregionalWebAclData>,
}

#[derive(Clone)]
pub struct WafregionalWebAcl(std::rc::Rc<WafregionalWebAcl_>);

impl WafregionalWebAcl {
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
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAclDefaultActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAclLoggingConfigurationEl>>,
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAclRuleEl>>) -> Self {
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
    pub fn default_action(&self) -> terrars::ListRef<WafregionalWebAclDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafregionalWebAclLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl terrars::Referable for WafregionalWebAcl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for WafregionalWebAcl { }

impl terrars::ToListMappable for WafregionalWebAcl {
    type O = terrars::ListRef<WafregionalWebAclRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for WafregionalWebAcl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_wafregional_web_acl".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafregionalWebAcl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub metric_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAcl {
    pub fn build(self, stack: &mut terrars::Stack) -> WafregionalWebAcl {
        let out = WafregionalWebAcl(std::rc::Rc::new(WafregionalWebAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(WafregionalWebAclData {
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
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafregionalWebAclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WafregionalWebAclRef {
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
    pub fn default_action(&self) -> terrars::ListRef<WafregionalWebAclDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafregionalWebAclLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclDefaultActionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAclDefaultActionEl { }

impl terrars::ToListMappable for WafregionalWebAclDefaultActionEl {
    type O = terrars::BlockAssignable<WafregionalWebAclDefaultActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclDefaultActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAclDefaultActionEl {
    pub fn build(self) -> WafregionalWebAclDefaultActionEl {
        WafregionalWebAclDefaultActionEl { type_: self.type_ }
    }
}

pub struct WafregionalWebAclDefaultActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclDefaultActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAclDefaultActionElRef {
        WafregionalWebAclDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclDefaultActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    type O = terrars::BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    pub fn build(self) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
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
struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std
    ::option
    ::Option<std::vec::Vec<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
    dynamic: WafregionalWebAclLoggingConfigurationElRedactedFieldsElDynamic,
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>,
                    >,
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

impl terrars::ToListMappable for WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    type O = terrars::BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsEl {}

impl BuildWafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
    pub fn build(self) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsEl {
            field_to_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
        WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclLoggingConfigurationElDynamic {
    redacted_fields: std
    ::option
    ::Option<terrars::DynamicBlock<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAclLoggingConfigurationEl {
    log_destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std::option::Option<std::vec::Vec<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>>,
    dynamic: WafregionalWebAclLoggingConfigurationElDynamic,
}

impl WafregionalWebAclLoggingConfigurationEl {
    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WafregionalWebAclLoggingConfigurationElRedactedFieldsEl>>,
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

impl terrars::ToListMappable for WafregionalWebAclLoggingConfigurationEl {
    type O = terrars::BlockAssignable<WafregionalWebAclLoggingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclLoggingConfigurationEl {
    #[doc = ""]
    pub log_destination: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAclLoggingConfigurationEl {
    pub fn build(self) -> WafregionalWebAclLoggingConfigurationEl {
        WafregionalWebAclLoggingConfigurationEl {
            log_destination: self.log_destination,
            redacted_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WafregionalWebAclLoggingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclLoggingConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAclLoggingConfigurationElRef {
        WafregionalWebAclLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclLoggingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<WafregionalWebAclLoggingConfigurationElRedactedFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclRuleElActionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAclRuleElActionEl { }

impl terrars::ToListMappable for WafregionalWebAclRuleElActionEl {
    type O = terrars::BlockAssignable<WafregionalWebAclRuleElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclRuleElActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAclRuleElActionEl {
    pub fn build(self) -> WafregionalWebAclRuleElActionEl {
        WafregionalWebAclRuleElActionEl { type_: self.type_ }
    }
}

pub struct WafregionalWebAclRuleElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclRuleElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAclRuleElActionElRef {
        WafregionalWebAclRuleElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclRuleElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafregionalWebAclRuleElOverrideActionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafregionalWebAclRuleElOverrideActionEl { }

impl terrars::ToListMappable for WafregionalWebAclRuleElOverrideActionEl {
    type O = terrars::BlockAssignable<WafregionalWebAclRuleElOverrideActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclRuleElOverrideActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAclRuleElOverrideActionEl {
    pub fn build(self) -> WafregionalWebAclRuleElOverrideActionEl {
        WafregionalWebAclRuleElOverrideActionEl { type_: self.type_ }
    }
}

pub struct WafregionalWebAclRuleElOverrideActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclRuleElOverrideActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAclRuleElOverrideActionElRef {
        WafregionalWebAclRuleElOverrideActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclRuleElOverrideActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclRuleElDynamic {
    action: std::option::Option<terrars::DynamicBlock<WafregionalWebAclRuleElActionEl>>,
    override_action: std::option::Option<terrars::DynamicBlock<WafregionalWebAclRuleElOverrideActionEl>>,
}

#[derive(Serialize)]
pub struct WafregionalWebAclRuleEl {
    priority: terrars::PrimField<f64>,
    rule_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<WafregionalWebAclRuleElActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_action: std::option::Option<std::vec::Vec<WafregionalWebAclRuleElOverrideActionEl>>,
    dynamic: WafregionalWebAclRuleElDynamic,
}

impl WafregionalWebAclRuleEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAclRuleElActionEl>>,
    ) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<WafregionalWebAclRuleElOverrideActionEl>>,
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

impl terrars::ToListMappable for WafregionalWebAclRuleEl {
    type O = terrars::BlockAssignable<WafregionalWebAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafregionalWebAclRuleEl {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildWafregionalWebAclRuleEl {
    pub fn build(self) -> WafregionalWebAclRuleEl {
        WafregionalWebAclRuleEl {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            override_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WafregionalWebAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafregionalWebAclRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafregionalWebAclRuleElRef {
        WafregionalWebAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafregionalWebAclRuleElRef {
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
    pub fn action(&self) -> terrars::ListRef<WafregionalWebAclRuleElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> terrars::ListRef<WafregionalWebAclRuleElOverrideActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafregionalWebAclDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<WafregionalWebAclDefaultActionEl>>,
    logging_configuration: std::option::Option<terrars::DynamicBlock<WafregionalWebAclLoggingConfigurationEl>>,
    rule: std::option::Option<terrars::DynamicBlock<WafregionalWebAclRuleEl>>,
}
