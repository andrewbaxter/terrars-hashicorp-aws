use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAuditmanagerControlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    name: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    control_mapping_sources: Option<Vec<DataAuditmanagerControlControlMappingSourcesEl>>,
    dynamic: DataAuditmanagerControlDynamic,
}

struct DataAuditmanagerControl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAuditmanagerControlData>,
}

#[derive(Clone)]
pub struct DataAuditmanagerControl(Rc<DataAuditmanagerControl_>);

impl DataAuditmanagerControl {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `control_mapping_sources`.\n"]
    pub fn set_control_mapping_sources(
        self,
        v: impl Into<BlockAssignable<DataAuditmanagerControlControlMappingSourcesEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().control_mapping_sources = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.control_mapping_sources = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `action_plan_instructions` after provisioning.\n"]
    pub fn action_plan_instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_plan_instructions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_plan_title` after provisioning.\n"]
    pub fn action_plan_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_plan_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.testing_information", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

impl Datasource for DataAuditmanagerControl {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataAuditmanagerControl {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataAuditmanagerControl {
    type O = ListRef<DataAuditmanagerControlRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataAuditmanagerControl_ {
    fn extract_datasource_type(&self) -> String {
        "aws_auditmanager_control".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAuditmanagerControl {
    pub tf_id: String,
    #[doc= ""]
    pub name: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildDataAuditmanagerControl {
    pub fn build(self, stack: &mut Stack) -> DataAuditmanagerControl {
        let out = DataAuditmanagerControl(Rc::new(DataAuditmanagerControl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAuditmanagerControlData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                name: self.name,
                type_: self.type_,
                control_mapping_sources: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAuditmanagerControlRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerControlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAuditmanagerControlRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `action_plan_instructions` after provisioning.\n"]
    pub fn action_plan_instructions(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_plan_instructions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `action_plan_title` after provisioning.\n"]
    pub fn action_plan_title(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action_plan_title", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `testing_information` after provisioning.\n"]
    pub fn testing_information(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.testing_information", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {}

impl DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl { }

impl ToListMappable for DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl {
    type O = BlockAssignable<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn new(shared: StackShared, base: String) -> DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
        DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `keyword_input_type` after provisioning.\n"]
    pub fn keyword_input_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keyword_input_type", self.base))
    }

    #[doc= "Get a reference to the value of field `keyword_value` after provisioning.\n"]
    pub fn keyword_value(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.keyword_value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAuditmanagerControlControlMappingSourcesElDynamic {
    source_keyword: Option<DynamicBlock<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
}

#[derive(Serialize)]
pub struct DataAuditmanagerControlControlMappingSourcesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    source_keyword: Option<Vec<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    dynamic: DataAuditmanagerControlControlMappingSourcesElDynamic,
}

impl DataAuditmanagerControlControlMappingSourcesEl {
    #[doc= "Set the field `source_keyword`.\n"]
    pub fn set_source_keyword(
        mut self,
        v: impl Into<BlockAssignable<DataAuditmanagerControlControlMappingSourcesElSourceKeywordEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.source_keyword = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.source_keyword = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for DataAuditmanagerControlControlMappingSourcesEl {
    type O = BlockAssignable<DataAuditmanagerControlControlMappingSourcesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct DataAuditmanagerControlControlMappingSourcesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAuditmanagerControlControlMappingSourcesElRef {
    fn new(shared: StackShared, base: String) -> DataAuditmanagerControlControlMappingSourcesElRef {
        DataAuditmanagerControlControlMappingSourcesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControlControlMappingSourcesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `source_description` after provisioning.\n"]
    pub fn source_description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_description", self.base))
    }

    #[doc= "Get a reference to the value of field `source_frequency` after provisioning.\n"]
    pub fn source_frequency(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_frequency", self.base))
    }

    #[doc= "Get a reference to the value of field `source_id` after provisioning.\n"]
    pub fn source_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_id", self.base))
    }

    #[doc= "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }

    #[doc= "Get a reference to the value of field `source_set_up_option` after provisioning.\n"]
    pub fn source_set_up_option(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_set_up_option", self.base))
    }

    #[doc= "Get a reference to the value of field `source_type` after provisioning.\n"]
    pub fn source_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source_type", self.base))
    }

    #[doc= "Get a reference to the value of field `troubleshooting_text` after provisioning.\n"]
    pub fn troubleshooting_text(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.troubleshooting_text", self.base))
    }

    #[doc= "Get a reference to the value of field `source_keyword` after provisioning.\n"]
    pub fn source_keyword(&self) -> ListRef<DataAuditmanagerControlControlMappingSourcesElSourceKeywordElRef> {
        ListRef::new(self.shared().clone(), format!("{}.source_keyword", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataAuditmanagerControlDynamic {
    control_mapping_sources: Option<DynamicBlock<DataAuditmanagerControlControlMappingSourcesEl>>,
}
