use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAuditmanagerControlData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    control_mapping_sources: std::option::Option<std::vec::Vec<DataAuditmanagerControlControlMappingSourcesEl>>,
    dynamic: DataAuditmanagerControlDynamic,
}

struct DataAuditmanagerControl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAuditmanagerControlData>,
}

#[derive(Clone)]
pub struct DataAuditmanagerControl(std::rc::Rc<DataAuditmanagerControl_>);

impl DataAuditmanagerControl {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `control_mapping_sources`.\n"]
    pub fn set_control_mapping_sources(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataAuditmanagerControlControlMappingSourcesEl>>,
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

    #[doc = "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.testing_information", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl terrars::Referable for DataAuditmanagerControl {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAuditmanagerControl { }

impl terrars::ToListMappable for DataAuditmanagerControl {
    type O = terrars::ListRef<DataAuditmanagerControlRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAuditmanagerControl_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_auditmanager_control".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAuditmanagerControl {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDataAuditmanagerControl {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAuditmanagerControl {
        let out = DataAuditmanagerControl(std::rc::Rc::new(DataAuditmanagerControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAuditmanagerControlData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                type_: self.type_,
                control_mapping_sources: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAuditmanagerControlRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAuditmanagerControlRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAuditmanagerControlRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
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
pub struct DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {}

impl DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl { }

impl terrars::ToListMappable for DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    type O = terrars::BlockAssignable<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {}

impl BuildDataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    pub fn build(self) -> DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
        DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {}
    }
}

pub struct DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
        DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
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
struct DataAuditmanagerControlControlMappingSourcesElDynamic {
    source_keyword: std
    ::option
    ::Option<terrars::DynamicBlock<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
}

#[derive(Serialize)]
pub struct DataAuditmanagerControlControlMappingSourcesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_keyword: std::option::Option<std::vec::Vec<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    dynamic: DataAuditmanagerControlControlMappingSourcesElDynamic,
}

impl DataAuditmanagerControlControlMappingSourcesEl {
    #[doc = "Set the field `source_keyword`.\n"]
    pub fn set_source_keyword(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
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

impl terrars::ToListMappable for DataAuditmanagerControlControlMappingSourcesEl {
    type O = terrars::BlockAssignable<DataAuditmanagerControlControlMappingSourcesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerControlControlMappingSourcesEl {}

impl BuildDataAuditmanagerControlControlMappingSourcesEl {
    pub fn build(self) -> DataAuditmanagerControlControlMappingSourcesEl {
        DataAuditmanagerControlControlMappingSourcesEl {
            source_keyword: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataAuditmanagerControlControlMappingSourcesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAuditmanagerControlControlMappingSourcesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAuditmanagerControlControlMappingSourcesElRef {
        DataAuditmanagerControlControlMappingSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControlControlMappingSourcesElRef {
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
    pub fn source_keyword(&self) -> terrars::ListRef<DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_keyword", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAuditmanagerControlDynamic {
    control_mapping_sources: std
    ::option
    ::Option<terrars::DynamicBlock<DataAuditmanagerControlControlMappingSourcesEl>>,
}
