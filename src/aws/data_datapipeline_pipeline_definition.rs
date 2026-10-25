#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataDatapipelinePipelineDefinitionData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    pipeline_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std::option::Option<std::vec::Vec<DataDatapipelinePipelineDefinition_ParameterValue>>,
    dynamic: DataDatapipelinePipelineDefinitionDynamic,
}

struct DataDatapipelinePipelineDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataDatapipelinePipelineDefinitionData_>,
}

#[derive(Clone)]
pub struct DataDatapipelinePipelineDefinition(std::rc::Rc<DataDatapipelinePipelineDefinition_>);

impl DataDatapipelinePipelineDefinition {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataDatapipelinePipelineDefinition_ParameterValue>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_object` after provisioning.\n"]
    pub fn parameter_object(&self) -> terrars::SetRef<DataDatapipelinePipelineDefinition_ParameterObjectRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.parameter_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_object` after provisioning.\n"]
    pub fn pipeline_object(&self) -> terrars::SetRef<DataDatapipelinePipelineDefinition_PipelineObjectRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.pipeline_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DataDatapipelinePipelineDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataDatapipelinePipelineDefinition { }

impl terrars::ToListMappable for DataDatapipelinePipelineDefinition {
    type O = terrars::ListRef<DataDatapipelinePipelineDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataDatapipelinePipelineDefinition_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_datapipeline_pipeline_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataDatapipelinePipelineDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub pipeline_id: terrars::PrimField<std::string::String>,
}

impl BuildDataDatapipelinePipelineDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> DataDatapipelinePipelineDefinition {
        let out = DataDatapipelinePipelineDefinition(std::rc::Rc::new(DataDatapipelinePipelineDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataDatapipelinePipelineDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                pipeline_id: self.pipeline_id,
                region: core::default::Default::default(),
                parameter_value: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataDatapipelinePipelineDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatapipelinePipelineDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataDatapipelinePipelineDefinitionRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `parameter_object` after provisioning.\n"]
    pub fn parameter_object(&self) -> terrars::SetRef<DataDatapipelinePipelineDefinition_ParameterObjectRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.parameter_object", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_object` after provisioning.\n"]
    pub fn pipeline_object(&self) -> terrars::SetRef<DataDatapipelinePipelineDefinition_PipelineObjectRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.pipeline_object", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DataDatapipelinePipelineDefinitionDynamic {
    parameter_value: std::option::Option<terrars::DynamicBlock<DataDatapipelinePipelineDefinition_ParameterValue>>,
}

#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinition_ParameterObject_Attribute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDatapipelinePipelineDefinition_ParameterObject_Attribute {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDatapipelinePipelineDefinition_ParameterObject_Attribute {
    type O = terrars::BlockAssignable<DataDatapipelinePipelineDefinition_ParameterObject_Attribute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatapipelinePipelineDefinition_ParameterObject_Attribute {}

impl BuildDataDatapipelinePipelineDefinition_ParameterObject_Attribute {
    pub fn build(self) -> DataDatapipelinePipelineDefinition_ParameterObject_Attribute {
        DataDatapipelinePipelineDefinition_ParameterObject_Attribute {
            key: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDatapipelinePipelineDefinition_ParameterObject_AttributeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatapipelinePipelineDefinition_ParameterObject_AttributeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDatapipelinePipelineDefinition_ParameterObject_AttributeRef {
        DataDatapipelinePipelineDefinition_ParameterObject_AttributeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatapipelinePipelineDefinition_ParameterObject_AttributeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinition_ParameterObject {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::SetField<DataDatapipelinePipelineDefinition_ParameterObject_Attribute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDatapipelinePipelineDefinition_ParameterObject {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataDatapipelinePipelineDefinition_ParameterObject_Attribute>>,
    ) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDatapipelinePipelineDefinition_ParameterObject {
    type O = terrars::BlockAssignable<DataDatapipelinePipelineDefinition_ParameterObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatapipelinePipelineDefinition_ParameterObject {}

impl BuildDataDatapipelinePipelineDefinition_ParameterObject {
    pub fn build(self) -> DataDatapipelinePipelineDefinition_ParameterObject {
        DataDatapipelinePipelineDefinition_ParameterObject {
            attribute: core::default::Default::default(),
            id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDatapipelinePipelineDefinition_ParameterObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatapipelinePipelineDefinition_ParameterObjectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDatapipelinePipelineDefinition_ParameterObjectRef {
        DataDatapipelinePipelineDefinition_ParameterObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatapipelinePipelineDefinition_ParameterObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::SetRef<DataDatapipelinePipelineDefinition_ParameterObject_AttributeRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinition_PipelineObject_Field {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ref_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDatapipelinePipelineDefinition_PipelineObject_Field {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ref_value`.\n"]
    pub fn set_ref_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.ref_value = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `string_value`.\n"]
    pub fn set_string_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.string_value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDatapipelinePipelineDefinition_PipelineObject_Field {
    type O = terrars::BlockAssignable<DataDatapipelinePipelineDefinition_PipelineObject_Field>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatapipelinePipelineDefinition_PipelineObject_Field {}

impl BuildDataDatapipelinePipelineDefinition_PipelineObject_Field {
    pub fn build(self) -> DataDatapipelinePipelineDefinition_PipelineObject_Field {
        DataDatapipelinePipelineDefinition_PipelineObject_Field {
            key: core::default::Default::default(),
            ref_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDatapipelinePipelineDefinition_PipelineObject_FieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatapipelinePipelineDefinition_PipelineObject_FieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDatapipelinePipelineDefinition_PipelineObject_FieldRef {
        DataDatapipelinePipelineDefinition_PipelineObject_FieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatapipelinePipelineDefinition_PipelineObject_FieldRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `ref_value` after provisioning.\n"]
    pub fn ref_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.ref_value", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinition_PipelineObject {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<terrars::SetField<DataDatapipelinePipelineDefinition_PipelineObject_Field>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataDatapipelinePipelineDefinition_PipelineObject {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl std::convert::Into<terrars::SetField<DataDatapipelinePipelineDefinition_PipelineObject_Field>>,
    ) -> Self {
        self.field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataDatapipelinePipelineDefinition_PipelineObject {
    type O = terrars::BlockAssignable<DataDatapipelinePipelineDefinition_PipelineObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatapipelinePipelineDefinition_PipelineObject {}

impl BuildDataDatapipelinePipelineDefinition_PipelineObject {
    pub fn build(self) -> DataDatapipelinePipelineDefinition_PipelineObject {
        DataDatapipelinePipelineDefinition_PipelineObject {
            field: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataDatapipelinePipelineDefinition_PipelineObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatapipelinePipelineDefinition_PipelineObjectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDatapipelinePipelineDefinition_PipelineObjectRef {
        DataDatapipelinePipelineDefinition_PipelineObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatapipelinePipelineDefinition_PipelineObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::SetRef<DataDatapipelinePipelineDefinition_PipelineObject_FieldRef> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DataDatapipelinePipelineDefinition_ParameterValue {}

impl DataDatapipelinePipelineDefinition_ParameterValue { }

impl terrars::ToListMappable for DataDatapipelinePipelineDefinition_ParameterValue {
    type O = terrars::BlockAssignable<DataDatapipelinePipelineDefinition_ParameterValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataDatapipelinePipelineDefinition_ParameterValue {}

impl BuildDataDatapipelinePipelineDefinition_ParameterValue {
    pub fn build(self) -> DataDatapipelinePipelineDefinition_ParameterValue {
        DataDatapipelinePipelineDefinition_ParameterValue {}
    }
}

#[allow(dead_code)]
pub struct DataDatapipelinePipelineDefinition_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataDatapipelinePipelineDefinition_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataDatapipelinePipelineDefinition_ParameterValueRef {
        DataDatapipelinePipelineDefinition_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataDatapipelinePipelineDefinition_ParameterValueRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `string_value` after provisioning.\n"]
    pub fn string_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.string_value", self.base))
    }
}
