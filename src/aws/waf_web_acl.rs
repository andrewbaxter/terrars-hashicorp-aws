use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafWebAclData {
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
    default_action: std::option::Option<std::vec::Vec<WafWebAclDefaultActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    logging_configuration: std::option::Option<std::vec::Vec<WafWebAclLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rules: std::option::Option<std::vec::Vec<WafWebAclRulesEl>>,
    dynamic: WafWebAclDynamic,
}

struct WafWebAcl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<WafWebAclData>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAclDefaultActionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAclLoggingConfigurationEl>>,
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
    pub fn set_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<WafWebAclRulesEl>>) -> Self {
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
    pub fn default_action(&self) -> terrars::ListRef<WafWebAclDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafWebAclLoggingConfigurationElRef> {
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
            data: std::cell::RefCell::new(WafWebAclData {
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
    pub fn default_action(&self) -> terrars::ListRef<WafWebAclDefaultActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> terrars::ListRef<WafWebAclLoggingConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafWebAclDefaultActionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAclDefaultActionEl { }

impl terrars::ToListMappable for WafWebAclDefaultActionEl {
    type O = terrars::BlockAssignable<WafWebAclDefaultActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclDefaultActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAclDefaultActionEl {
    pub fn build(self) -> WafWebAclDefaultActionEl {
        WafWebAclDefaultActionEl { type_: self.type_ }
    }
}

pub struct WafWebAclDefaultActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclDefaultActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAclDefaultActionElRef {
        WafWebAclDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclDefaultActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc = "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.data = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    type O = terrars::BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    pub fn build(self) -> WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
        WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
            data: core::default::Default::default(),
            type_: self.type_,
        }
    }
}

pub struct WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
        WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
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
struct WafWebAclLoggingConfigurationElRedactedFieldsElDynamic {
    field_to_match: std
    ::option
    ::Option<terrars::DynamicBlock<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
}

#[derive(Serialize)]
pub struct WafWebAclLoggingConfigurationElRedactedFieldsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field_to_match: std::option::Option<std::vec::Vec<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
    dynamic: WafWebAclLoggingConfigurationElRedactedFieldsElDynamic,
}

impl WafWebAclLoggingConfigurationElRedactedFieldsEl {
    #[doc = "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
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

impl terrars::ToListMappable for WafWebAclLoggingConfigurationElRedactedFieldsEl {
    type O = terrars::BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclLoggingConfigurationElRedactedFieldsEl {}

impl BuildWafWebAclLoggingConfigurationElRedactedFieldsEl {
    pub fn build(self) -> WafWebAclLoggingConfigurationElRedactedFieldsEl {
        WafWebAclLoggingConfigurationElRedactedFieldsEl {
            field_to_match: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WafWebAclLoggingConfigurationElRedactedFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> WafWebAclLoggingConfigurationElRedactedFieldsElRef {
        WafWebAclLoggingConfigurationElRedactedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WafWebAclLoggingConfigurationElDynamic {
    redacted_fields: std::option::Option<terrars::DynamicBlock<WafWebAclLoggingConfigurationElRedactedFieldsEl>>,
}

#[derive(Serialize)]
pub struct WafWebAclLoggingConfigurationEl {
    log_destination: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    redacted_fields: std::option::Option<std::vec::Vec<WafWebAclLoggingConfigurationElRedactedFieldsEl>>,
    dynamic: WafWebAclLoggingConfigurationElDynamic,
}

impl WafWebAclLoggingConfigurationEl {
    #[doc = "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsEl>>,
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

impl terrars::ToListMappable for WafWebAclLoggingConfigurationEl {
    type O = terrars::BlockAssignable<WafWebAclLoggingConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclLoggingConfigurationEl {
    #[doc = ""]
    pub log_destination: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAclLoggingConfigurationEl {
    pub fn build(self) -> WafWebAclLoggingConfigurationEl {
        WafWebAclLoggingConfigurationEl {
            log_destination: self.log_destination,
            redacted_fields: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WafWebAclLoggingConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclLoggingConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAclLoggingConfigurationElRef {
        WafWebAclLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclLoggingConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc = "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> terrars::ListRef<WafWebAclLoggingConfigurationElRedactedFieldsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAclRulesElActionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAclRulesElActionEl { }

impl terrars::ToListMappable for WafWebAclRulesElActionEl {
    type O = terrars::BlockAssignable<WafWebAclRulesElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclRulesElActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAclRulesElActionEl {
    pub fn build(self) -> WafWebAclRulesElActionEl {
        WafWebAclRulesElActionEl { type_: self.type_ }
    }
}

pub struct WafWebAclRulesElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclRulesElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAclRulesElActionElRef {
        WafWebAclRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclRulesElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAclRulesElOverrideActionEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl WafWebAclRulesElOverrideActionEl { }

impl terrars::ToListMappable for WafWebAclRulesElOverrideActionEl {
    type O = terrars::BlockAssignable<WafWebAclRulesElOverrideActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclRulesElOverrideActionEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAclRulesElOverrideActionEl {
    pub fn build(self) -> WafWebAclRulesElOverrideActionEl {
        WafWebAclRulesElOverrideActionEl { type_: self.type_ }
    }
}

pub struct WafWebAclRulesElOverrideActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclRulesElOverrideActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAclRulesElOverrideActionElRef {
        WafWebAclRulesElOverrideActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclRulesElOverrideActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAclRulesElDynamic {
    action: std::option::Option<terrars::DynamicBlock<WafWebAclRulesElActionEl>>,
    override_action: std::option::Option<terrars::DynamicBlock<WafWebAclRulesElOverrideActionEl>>,
}

#[derive(Serialize)]
pub struct WafWebAclRulesEl {
    priority: terrars::PrimField<f64>,
    rule_id: terrars::PrimField<std::string::String>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<WafWebAclRulesElActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    override_action: std::option::Option<std::vec::Vec<WafWebAclRulesElOverrideActionEl>>,
    dynamic: WafWebAclRulesElDynamic,
}

impl WafWebAclRulesEl {
    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.type_ = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::BlockAssignable<WafWebAclRulesElActionEl>>) -> Self {
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
        v: impl std::convert::Into<terrars::BlockAssignable<WafWebAclRulesElOverrideActionEl>>,
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

impl terrars::ToListMappable for WafWebAclRulesEl {
    type O = terrars::BlockAssignable<WafWebAclRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclRulesEl {
    #[doc = ""]
    pub priority: terrars::PrimField<f64>,
    #[doc = ""]
    pub rule_id: terrars::PrimField<std::string::String>,
}

impl BuildWafWebAclRulesEl {
    pub fn build(self) -> WafWebAclRulesEl {
        WafWebAclRulesEl {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            override_action: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct WafWebAclRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for WafWebAclRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> WafWebAclRulesElRef {
        WafWebAclRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclRulesElRef {
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
    pub fn action(&self) -> terrars::ListRef<WafWebAclRulesElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> terrars::ListRef<WafWebAclRulesElOverrideActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAclDynamic {
    default_action: std::option::Option<terrars::DynamicBlock<WafWebAclDefaultActionEl>>,
    logging_configuration: std::option::Option<terrars::DynamicBlock<WafWebAclLoggingConfigurationEl>>,
    rules: std::option::Option<terrars::DynamicBlock<WafWebAclRulesEl>>,
}
