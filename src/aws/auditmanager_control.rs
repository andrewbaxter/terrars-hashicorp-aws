use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct AuditmanagerControlData {
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
    action_plan_instructions: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_plan_title: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    testing_information: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_mapping_sources: std::option::Option<std::vec::Vec<AuditmanagerControlControlMappingSourcesEl>>,
    dynamic: AuditmanagerControlDynamic,
}

struct AuditmanagerControl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<AuditmanagerControlData>,
}

#[derive(Clone)]
pub struct AuditmanagerControl(std::rc::Rc<AuditmanagerControl_>);

impl AuditmanagerControl {
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

    #[doc = "Set the field `action_plan_instructions`.\n"]
    pub fn set_action_plan_instructions(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().action_plan_instructions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action_plan_title`.\n"]
    pub fn set_action_plan_title(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().action_plan_title = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `testing_information`.\n"]
    pub fn set_testing_information(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().testing_information = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `control_mapping_sources`.\n"]
    pub fn set_control_mapping_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<AuditmanagerControlControlMappingSourcesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_mapping_sources = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_mapping_sources = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `action_plan_instructions` after provisioning.\n"]
    pub fn action_plan_instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_plan_instructions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_plan_title` after provisioning.\n"]
    pub fn action_plan_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_plan_title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.testing_information", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for AuditmanagerControl {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for AuditmanagerControl { }

impl terrars::ToListMappable for AuditmanagerControl {
    type O = terrars::ListRef<AuditmanagerControlRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for AuditmanagerControl_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_auditmanager_control".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildAuditmanagerControl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerControl {
    pub fn build(self, stack: &mut terrars::Stack) -> AuditmanagerControl {
        let out = AuditmanagerControl(std::rc::Rc::new(AuditmanagerControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(AuditmanagerControlData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action_plan_instructions: core::default::Default::default(),
                action_plan_title: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                tags: core::default::Default::default(),
                testing_information: core::default::Default::default(),
                control_mapping_sources: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct AuditmanagerControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerControlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl AuditmanagerControlRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_plan_instructions` after provisioning.\n"]
    pub fn action_plan_instructions(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_plan_instructions", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `action_plan_title` after provisioning.\n"]
    pub fn action_plan_title(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_plan_title", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.testing_information", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    keyword_input_type: terrars::PrimField<std::string::String>,
    keyword_value: terrars::PrimField<std::string::String>,
}

impl AuditmanagerControlControlMappingSourcesElSourceKeywordEl { }

impl terrars::ToListMappable for AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    type O = terrars::BlockAssignable<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    #[doc = ""]
    pub keyword_input_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub keyword_value: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    pub fn build(self) -> AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
        AuditmanagerControlControlMappingSourcesElSourceKeywordEl {
            keyword_input_type: self.keyword_input_type,
            keyword_value: self.keyword_value,
        }
    }
}

pub struct AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
        AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `keyword_input_type` after provisioning.\n"]
    pub fn keyword_input_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_input_type", self.base))
    }

    #[doc = "Get a reference to the value of field `keyword_value` after provisioning.\n"]
    pub fn keyword_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.keyword_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct AuditmanagerControlControlMappingSourcesElDynamic {
    source_keyword: std
    ::option
    ::Option<terrars::DynamicBlock<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
}

#[derive(Serialize)]
pub struct AuditmanagerControlControlMappingSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    source_name: terrars::PrimField<std::string::String>,
    source_set_up_option: terrars::PrimField<std::string::String>,
    source_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    troubleshooting_text: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_keyword: std::option::Option<std::vec::Vec<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    dynamic: AuditmanagerControlControlMappingSourcesElDynamic,
}

impl AuditmanagerControlControlMappingSourcesEl {
    #[doc = "Set the field `source_description`.\n"]
    pub fn set_source_description(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_frequency`.\n"]
    pub fn set_source_frequency(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_frequency = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `troubleshooting_text`.\n"]
    pub fn set_troubleshooting_text(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.troubleshooting_text = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_keyword`.\n"]
    pub fn set_source_keyword(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<AuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_keyword = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_keyword = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for AuditmanagerControlControlMappingSourcesEl {
    type O = terrars::BlockAssignable<AuditmanagerControlControlMappingSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildAuditmanagerControlControlMappingSourcesEl {
    #[doc = ""]
    pub source_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_set_up_option: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub source_type: terrars::PrimField<std::string::String>,
}

impl BuildAuditmanagerControlControlMappingSourcesEl {
    pub fn build(self) -> AuditmanagerControlControlMappingSourcesEl {
        AuditmanagerControlControlMappingSourcesEl {
            source_description: core::default::Default::default(),
            source_frequency: core::default::Default::default(),
            source_name: self.source_name,
            source_set_up_option: self.source_set_up_option,
            source_type: self.source_type,
            troubleshooting_text: core::default::Default::default(),
            source_keyword: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct AuditmanagerControlControlMappingSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for AuditmanagerControlControlMappingSourcesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> AuditmanagerControlControlMappingSourcesElRef {
        AuditmanagerControlControlMappingSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl AuditmanagerControlControlMappingSourcesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source_description` after provisioning.\n"]
    pub fn source_description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_description", self.base))
    }

    #[doc = "Get a reference to the value of field `source_frequency` after provisioning.\n"]
    pub fn source_frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_frequency", self.base))
    }

    #[doc = "Get a reference to the value of field `source_id` after provisioning.\n"]
    pub fn source_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }

    #[doc = "Get a reference to the value of field `source_set_up_option` after provisioning.\n"]
    pub fn source_set_up_option(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_set_up_option", self.base))
    }

    #[doc = "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc = "Get a reference to the value of field `troubleshooting_text` after provisioning.\n"]
    pub fn troubleshooting_text(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.troubleshooting_text", self.base))
    }

    #[doc = "Get a reference to the value of field `source_keyword` after provisioning.\n"]
    pub fn source_keyword(&self) -> terrars::ListRef<AuditmanagerControlControlMappingSourcesElSourceKeywordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_keyword", self.base))
    }
}

#[derive(Serialize, Default)]
struct AuditmanagerControlDynamic {
    control_mapping_sources: std::option::Option<terrars::DynamicBlock<AuditmanagerControlControlMappingSourcesEl>>,
}
