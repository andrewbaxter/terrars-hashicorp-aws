use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SfnStateMachineData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    definition: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
    role_arn: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    type_: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    logging_configuration: Option<Vec<SfnStateMachineLoggingConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tracing_configuration: Option<Vec<SfnStateMachineTracingConfigurationEl>>,
    dynamic: SfnStateMachineDynamic,
}

struct SfnStateMachine_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SfnStateMachineData>,
}

#[derive(Clone)]
pub struct SfnStateMachine(Rc<SfnStateMachine_>);

impl SfnStateMachine {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
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

    #[doc= "Set the field `type_`.\n"]
    pub fn set_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().type_ = Some(v.into());
        self
    }

    #[doc= "Set the field `logging_configuration`.\n"]
    pub fn set_logging_configuration(
        self,
        v: impl Into<BlockAssignable<SfnStateMachineLoggingConfigurationEl>>,
    ) -> Self {
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

    #[doc= "Set the field `tracing_configuration`.\n"]
    pub fn set_tracing_configuration(
        self,
        v: impl Into<BlockAssignable<SfnStateMachineTracingConfigurationEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().tracing_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.tracing_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<SfnStateMachineLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tracing_configuration` after provisioning.\n"]
    pub fn tracing_configuration(&self) -> ListRef<SfnStateMachineTracingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tracing_configuration", self.extract_ref()))
    }
}

impl Resource for SfnStateMachine {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for SfnStateMachine {
    type O = ListRef<SfnStateMachineRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SfnStateMachine_ {
    fn extract_resource_type(&self) -> String {
        "aws_sfn_state_machine".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSfnStateMachine {
    pub tf_id: String,
    #[doc= ""]
    pub definition: PrimField<String>,
    #[doc= ""]
    pub role_arn: PrimField<String>,
}

impl BuildSfnStateMachine {
    pub fn build(self, stack: &mut Stack) -> SfnStateMachine {
        let out = SfnStateMachine(Rc::new(SfnStateMachine_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(SfnStateMachineData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                definition: self.definition,
                id: core::default::Default::default(),
                name: core::default::Default::default(),
                name_prefix: core::default::Default::default(),
                role_arn: self.role_arn,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                type_: core::default::Default::default(),
                logging_configuration: core::default::Default::default(),
                tracing_configuration: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SfnStateMachineRef {
    shared: StackShared,
    base: String,
}

impl Ref for SfnStateMachineRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SfnStateMachineRef {
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

    #[doc= "Get a reference to the value of field `creation_date` after provisioning.\n"]
    pub fn creation_date(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.creation_date", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `definition` after provisioning.\n"]
    pub fn definition(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.definition", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `logging_configuration` after provisioning.\n"]
    pub fn logging_configuration(&self) -> ListRef<SfnStateMachineLoggingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.logging_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tracing_configuration` after provisioning.\n"]
    pub fn tracing_configuration(&self) -> ListRef<SfnStateMachineTracingConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.tracing_configuration", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SfnStateMachineLoggingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    include_execution_data: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    log_destination: Option<PrimField<String>>,
}

impl SfnStateMachineLoggingConfigurationEl {
    #[doc= "Set the field `include_execution_data`.\n"]
    pub fn set_include_execution_data(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.include_execution_data = Some(v.into());
        self
    }

    #[doc= "Set the field `level`.\n"]
    pub fn set_level(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.level = Some(v.into());
        self
    }

    #[doc= "Set the field `log_destination`.\n"]
    pub fn set_log_destination(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.log_destination = Some(v.into());
        self
    }
}

impl ToListMappable for SfnStateMachineLoggingConfigurationEl {
    type O = BlockAssignable<SfnStateMachineLoggingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSfnStateMachineLoggingConfigurationEl {}

impl BuildSfnStateMachineLoggingConfigurationEl {
    pub fn build(self) -> SfnStateMachineLoggingConfigurationEl {
        SfnStateMachineLoggingConfigurationEl {
            include_execution_data: core::default::Default::default(),
            level: core::default::Default::default(),
            log_destination: core::default::Default::default(),
        }
    }
}

pub struct SfnStateMachineLoggingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SfnStateMachineLoggingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SfnStateMachineLoggingConfigurationElRef {
        SfnStateMachineLoggingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SfnStateMachineLoggingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `include_execution_data` after provisioning.\n"]
    pub fn include_execution_data(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.include_execution_data", self.base))
    }

    #[doc= "Get a reference to the value of field `level` after provisioning.\n"]
    pub fn level(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.level", self.base))
    }

    #[doc= "Get a reference to the value of field `log_destination` after provisioning.\n"]
    pub fn log_destination(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.log_destination", self.base))
    }
}

#[derive(Serialize)]
pub struct SfnStateMachineTracingConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<PrimField<bool>>,
}

impl SfnStateMachineTracingConfigurationEl {
    #[doc= "Set the field `enabled`.\n"]
    pub fn set_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.enabled = Some(v.into());
        self
    }
}

impl ToListMappable for SfnStateMachineTracingConfigurationEl {
    type O = BlockAssignable<SfnStateMachineTracingConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSfnStateMachineTracingConfigurationEl {}

impl BuildSfnStateMachineTracingConfigurationEl {
    pub fn build(self) -> SfnStateMachineTracingConfigurationEl {
        SfnStateMachineTracingConfigurationEl { enabled: core::default::Default::default() }
    }
}

pub struct SfnStateMachineTracingConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for SfnStateMachineTracingConfigurationElRef {
    fn new(shared: StackShared, base: String) -> SfnStateMachineTracingConfigurationElRef {
        SfnStateMachineTracingConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SfnStateMachineTracingConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }
}

#[derive(Serialize, Default)]
struct SfnStateMachineDynamic {
    logging_configuration: Option<DynamicBlock<SfnStateMachineLoggingConfigurationEl>>,
    tracing_configuration: Option<DynamicBlock<SfnStateMachineTracingConfigurationEl>>,
}
