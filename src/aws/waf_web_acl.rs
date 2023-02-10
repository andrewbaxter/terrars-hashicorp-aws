use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct WafWebAclData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    metric_name: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_action: Option<Vec<WafWebAclDefaultActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_configuration: Option<Vec<WafWebAclLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rules: Option<Vec<WafWebAclRulesEl>>,
    dynamic: WafWebAclDynamic,
}

struct WafWebAcl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<WafWebAclData>,
}

#[derive(Clone)]
pub struct WafWebAcl(Rc<WafWebAcl_>);

impl WafWebAcl {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderAws) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `tags`.\n"]
    pub fn set_tags(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags = Some(v.into());
        self
    }

    #[doc= "Set the field `tags_all`.\n"]
    pub fn set_tags_all(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().tags_all = Some(v.into());
        self
    }

    #[doc= "Set the field `default_action`.\n"]
    pub fn set_default_action(self, v: impl Into<BlockAssignable<WafWebAclDefaultActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(self, v: impl Into<BlockAssignable<WafWebAclLoggingConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().logging_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.logging_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `rules`.\n"]
    pub fn set_rules(self, v: impl Into<BlockAssignable<WafWebAclRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rules = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<WafWebAclDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<WafWebAclLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

impl Resource for WafWebAcl {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for WafWebAcl {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for WafWebAcl {
    type O = ListRef<WafWebAclRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(&self))
    }
}

impl Resource_ for WafWebAcl_ {
    fn extract_resource_type(&self) -> String {
        "aws_waf_web_acl".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildWafWebAcl {
    pub tf_id: String,
    #[doc= ""]
    pub metric_name: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildWafWebAcl {
    pub fn build(self, stack: &mut Stack) -> WafWebAcl {
        let out = WafWebAcl(Rc::new(WafWebAcl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(WafWebAclData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                metric_name: self.metric_name,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                default_action: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct WafWebAclRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl WafWebAclRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `metric_name` after provisioning.\n"]
    pub fn metric_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.metric_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_action` after provisioning.\n"]
    pub fn default_action(&self) -> ListRef<WafWebAclDefaultActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<WafWebAclLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct WafWebAclDefaultActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafWebAclDefaultActionEl { }

impl ToListMappable for WafWebAclDefaultActionEl {
    type O = BlockAssignable<WafWebAclDefaultActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclDefaultActionEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildWafWebAclDefaultActionEl {
    pub fn build(self) -> WafWebAclDefaultActionEl {
        WafWebAclDefaultActionEl { type_: self.type_ }
    }
}

pub struct WafWebAclDefaultActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclDefaultActionElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclDefaultActionElRef {
        WafWebAclDefaultActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclDefaultActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<PrimField<String>>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc= "Set the field `data`.\n"]
    pub fn set_data(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.data = Some(v.into());
        self
    }
}

impl ToListMappable for WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    type O = BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl {
    #[doc= ""]
    pub type_: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
        WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `data` after provisioning.\n"]
    pub fn data(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.data", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAclLoggingConfigurationElRedactedFieldsElDynamic {
    field_to_match: Option<DynamicBlock<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
}

#[derive(Serialize)]
pub struct WafWebAclLoggingConfigurationElRedactedFieldsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_to_match: Option<Vec<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
    dynamic: WafWebAclLoggingConfigurationElRedactedFieldsElDynamic,
}

impl WafWebAclLoggingConfigurationElRedactedFieldsEl {
    #[doc= "Set the field `field_to_match`.\n"]
    pub fn set_field_to_match(
        mut self,
        v: impl Into<BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsElFieldToMatchEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.field_to_match = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.field_to_match = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafWebAclLoggingConfigurationElRedactedFieldsEl {
    type O = BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct WafWebAclLoggingConfigurationElRedactedFieldsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclLoggingConfigurationElRedactedFieldsElRef {
        WafWebAclLoggingConfigurationElRedactedFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclLoggingConfigurationElRedactedFieldsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct WafWebAclLoggingConfigurationElDynamic {
    redacted_fields: Option<DynamicBlock<WafWebAclLoggingConfigurationElRedactedFieldsEl>>,
}

#[derive(Serialize)]
pub struct WafWebAclLoggingConfigurationEl {
    log_destination: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    redacted_fields: Option<Vec<WafWebAclLoggingConfigurationElRedactedFieldsEl>>,
    dynamic: WafWebAclLoggingConfigurationElDynamic,
}

impl WafWebAclLoggingConfigurationEl {
    #[doc= "Set the field `redacted_fields`.\n"]
    pub fn set_redacted_fields(
        mut self,
        v: impl Into<BlockAssignable<WafWebAclLoggingConfigurationElRedactedFieldsEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.redacted_fields = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.redacted_fields = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafWebAclLoggingConfigurationEl {
    type O = BlockAssignable<WafWebAclLoggingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclLoggingConfigurationEl {
    #[doc= ""]
    pub log_destination: PrimField<String>,
}

impl BuildWafWebAclLoggingConfigurationEl {
    pub fn build(self) -> WafWebAclLoggingConfigurationEl {
        WafWebAclLoggingConfigurationEl {
            log_destination: self.log_destination,
            redacted_fields: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafWebAclLoggingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclLoggingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclLoggingConfigurationElRef {
        WafWebAclLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclLoggingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }

    #[doc= "Get a reference to the value of field `redacted_fields` after provisioning.\n"]
    pub fn redacted_fields(&self) -> ListRef<WafWebAclLoggingConfigurationElRedactedFieldsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.redacted_fields", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAclRulesElActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafWebAclRulesElActionEl { }

impl ToListMappable for WafWebAclRulesElActionEl {
    type O = BlockAssignable<WafWebAclRulesElActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclRulesElActionEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildWafWebAclRulesElActionEl {
    pub fn build(self) -> WafWebAclRulesElActionEl {
        WafWebAclRulesElActionEl { type_: self.type_ }
    }
}

pub struct WafWebAclRulesElActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclRulesElActionElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclRulesElActionElRef {
        WafWebAclRulesElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclRulesElActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct WafWebAclRulesElOverrideActionEl {
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl WafWebAclRulesElOverrideActionEl { }

impl ToListMappable for WafWebAclRulesElOverrideActionEl {
    type O = BlockAssignable<WafWebAclRulesElOverrideActionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclRulesElOverrideActionEl {
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildWafWebAclRulesElOverrideActionEl {
    pub fn build(self) -> WafWebAclRulesElOverrideActionEl {
        WafWebAclRulesElOverrideActionEl { type_: self.type_ }
    }
}

pub struct WafWebAclRulesElOverrideActionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclRulesElOverrideActionElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclRulesElOverrideActionElRef {
        WafWebAclRulesElOverrideActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclRulesElOverrideActionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAclRulesElDynamic {
    action: Option<DynamicBlock<WafWebAclRulesElActionEl>>,
    override_action: Option<DynamicBlock<WafWebAclRulesElOverrideActionEl>>,
}

#[derive(Serialize)]
pub struct WafWebAclRulesEl {
    priority: PrimField<f64>,
    rule_id: PrimField<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<Vec<WafWebAclRulesElActionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    override_action: Option<Vec<WafWebAclRulesElOverrideActionEl>>,
    dynamic: WafWebAclRulesElDynamic,
}

impl WafWebAclRulesEl {
    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<BlockAssignable<WafWebAclRulesElActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.action = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `override_action`.\n"]
    pub fn set_override_action(mut self, v: impl Into<BlockAssignable<WafWebAclRulesElOverrideActionEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.override_action = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.override_action = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for WafWebAclRulesEl {
    type O = BlockAssignable<WafWebAclRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildWafWebAclRulesEl {
    #[doc= ""]
    pub priority: PrimField<f64>,
    #[doc= ""]
    pub rule_id: PrimField<String>,
}

impl BuildWafWebAclRulesEl {
    pub fn build(self) -> WafWebAclRulesEl {
        WafWebAclRulesEl {
            priority: self.priority,
            rule_id: self.rule_id,
            type_: core::default::Default::default(),
            action: core::default::Default::default(),
            override_action: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct WafWebAclRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for WafWebAclRulesElRef {
    fn new(shared: StackShared, base: String) -> WafWebAclRulesElRef {
        WafWebAclRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl WafWebAclRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `priority` after provisioning.\n"]
    pub fn priority(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.priority", self.base))
    }

    #[doc= "Get a reference to the value of field `rule_id` after provisioning.\n"]
    pub fn rule_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.rule_id", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> ListRef<WafWebAclRulesElActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `override_action` after provisioning.\n"]
    pub fn override_action(&self) -> ListRef<WafWebAclRulesElOverrideActionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.override_action", self.base))
    }
}

#[derive(Serialize, Default)]
struct WafWebAclDynamic {
    default_action: Option<DynamicBlock<WafWebAclDefaultActionEl>>,
    logging_configuration: Option<DynamicBlock<WafWebAclLoggingConfigurationEl>>,
    rules: Option<DynamicBlock<WafWebAclRulesEl>>,
}
