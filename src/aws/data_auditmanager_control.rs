#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataAuditmanagerControlData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

struct DataAuditmanagerControl_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAuditmanagerControlData_>,
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Get a reference to the value of field `control_mapping_sources` after provisioning.\n"]
    pub fn control_mapping_sources(&self) -> terrars::ListRef<DataAuditmanagerControl_ControlMappingSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_mapping_sources", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
            data: std::cell::RefCell::new(DataAuditmanagerControlData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                name: self.name,
                region: core::default::Default::default(),
                type_: self.type_,
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

    #[doc = "Get a reference to the value of field `control_mapping_sources` after provisioning.\n"]
    pub fn control_mapping_sources(&self) -> terrars::ListRef<DataAuditmanagerControl_ControlMappingSourcesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.control_mapping_sources", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
pub struct DataAuditmanagerControl_ControlMappingSources_SourceKeyword {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword_input_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAuditmanagerControl_ControlMappingSources_SourceKeyword {
    #[doc = "Set the field `keyword_input_type`.\n"]
    pub fn set_keyword_input_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.keyword_input_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `keyword_value`.\n"]
    pub fn set_keyword_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.keyword_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAuditmanagerControl_ControlMappingSources_SourceKeyword {
    type O = terrars::BlockAssignable<DataAuditmanagerControl_ControlMappingSources_SourceKeyword>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerControl_ControlMappingSources_SourceKeyword {}

impl BuildDataAuditmanagerControl_ControlMappingSources_SourceKeyword {
    pub fn build(self) -> DataAuditmanagerControl_ControlMappingSources_SourceKeyword {
        DataAuditmanagerControl_ControlMappingSources_SourceKeyword {
            keyword_input_type: core::default::Default::default(),
            keyword_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAuditmanagerControl_ControlMappingSources_SourceKeywordRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAuditmanagerControl_ControlMappingSources_SourceKeywordRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAuditmanagerControl_ControlMappingSources_SourceKeywordRef {
        DataAuditmanagerControl_ControlMappingSources_SourceKeywordRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControl_ControlMappingSources_SourceKeywordRef {
    #[allow(dead_code)]
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

#[derive(Serialize)]
pub struct DataAuditmanagerControl_ControlMappingSources {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_frequency: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_keyword: std::option::Option<terrars::ListField<DataAuditmanagerControl_ControlMappingSources_SourceKeyword>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_set_up_option: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    troubleshooting_text: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAuditmanagerControl_ControlMappingSources {
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

    #[doc = "Set the field `source_id`.\n"]
    pub fn set_source_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_keyword`.\n"]
    pub fn set_source_keyword(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataAuditmanagerControl_ControlMappingSources_SourceKeyword>>,
    ) -> Self {
        self.source_keyword = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_name`.\n"]
    pub fn set_source_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_set_up_option`.\n"]
    pub fn set_source_set_up_option(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.source_set_up_option = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_type`.\n"]
    pub fn set_source_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_type = std::option::Option::Some(v.into());
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
}

impl terrars::ToListMappable for DataAuditmanagerControl_ControlMappingSources {
    type O = terrars::BlockAssignable<DataAuditmanagerControl_ControlMappingSources>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAuditmanagerControl_ControlMappingSources {}

impl BuildDataAuditmanagerControl_ControlMappingSources {
    pub fn build(self) -> DataAuditmanagerControl_ControlMappingSources {
        DataAuditmanagerControl_ControlMappingSources {
            source_description: core::default::Default::default(),
            source_frequency: core::default::Default::default(),
            source_id: core::default::Default::default(),
            source_keyword: core::default::Default::default(),
            source_name: core::default::Default::default(),
            source_set_up_option: core::default::Default::default(),
            source_type: core::default::Default::default(),
            troubleshooting_text: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataAuditmanagerControl_ControlMappingSourcesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAuditmanagerControl_ControlMappingSourcesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataAuditmanagerControl_ControlMappingSourcesRef {
        DataAuditmanagerControl_ControlMappingSourcesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAuditmanagerControl_ControlMappingSourcesRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `source_keyword` after provisioning.\n"]
    pub fn source_keyword(&self) -> terrars::ListRef<DataAuditmanagerControl_ControlMappingSources_SourceKeywordRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_keyword", self.base))
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
}
