#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatapipelinePipelineDefinitionData_ {
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
    pipeline_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_object: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinition_ParameterObject>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinition_ParameterValue>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_object: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinition_PipelineObject>>,
    dynamic: DatapipelinePipelineDefinitionDynamic,
}

struct DatapipelinePipelineDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatapipelinePipelineDefinitionData_>,
}

#[derive(Clone)]
pub struct DatapipelinePipelineDefinition(std::rc::Rc<DatapipelinePipelineDefinition_>);

impl DatapipelinePipelineDefinition {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `parameter_object`.\n"]
    pub fn set_parameter_object(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinition_ParameterObject>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().parameter_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.parameter_object = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameter_value`.\n"]
    pub fn set_parameter_value(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinition_ParameterValue>>,
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

    #[doc = "Set the field `pipeline_object`.\n"]
    pub fn set_pipeline_object(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinition_PipelineObject>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().pipeline_object = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.pipeline_object = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

impl terrars::Referable for DatapipelinePipelineDefinition {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DatapipelinePipelineDefinition { }

impl terrars::ToListMappable for DatapipelinePipelineDefinition {
    type O = terrars::ListRef<DatapipelinePipelineDefinitionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DatapipelinePipelineDefinition_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_datapipeline_pipeline_definition".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDatapipelinePipelineDefinition {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub pipeline_id: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinition {
    pub fn build(self, stack: &mut terrars::Stack) -> DatapipelinePipelineDefinition {
        let out = DatapipelinePipelineDefinition(std::rc::Rc::new(DatapipelinePipelineDefinition_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DatapipelinePipelineDefinitionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                pipeline_id: self.pipeline_id,
                region: core::default::Default::default(),
                parameter_object: core::default::Default::default(),
                parameter_value: core::default::Default::default(),
                pipeline_object: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DatapipelinePipelineDefinitionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinitionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DatapipelinePipelineDefinitionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `pipeline_id` after provisioning.\n"]
    pub fn pipeline_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pipeline_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinitionDynamic {
    parameter_object: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinition_ParameterObject>>,
    parameter_value: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinition_ParameterValue>>,
    pipeline_object: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinition_PipelineObject>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinition_ParameterObject_Attribute {
    key: terrars::PrimField<std::string::String>,
    string_value: terrars::PrimField<std::string::String>,
}

impl DatapipelinePipelineDefinition_ParameterObject_Attribute { }

impl terrars::ToListMappable for DatapipelinePipelineDefinition_ParameterObject_Attribute {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinition_ParameterObject_Attribute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinition_ParameterObject_Attribute {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub string_value: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinition_ParameterObject_Attribute {
    pub fn build(self) -> DatapipelinePipelineDefinition_ParameterObject_Attribute {
        DatapipelinePipelineDefinition_ParameterObject_Attribute {
            key: self.key,
            string_value: self.string_value,
        }
    }
}

#[allow(dead_code)]
pub struct DatapipelinePipelineDefinition_ParameterObject_AttributeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinition_ParameterObject_AttributeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinition_ParameterObject_AttributeRef {
        DatapipelinePipelineDefinition_ParameterObject_AttributeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinition_ParameterObject_AttributeRef {
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

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinition_ParameterObjectDynamic {
    attribute: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinition_ParameterObject_Attribute>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinition_ParameterObject {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinition_ParameterObject_Attribute>>,
    dynamic: DatapipelinePipelineDefinition_ParameterObjectDynamic,
}

impl DatapipelinePipelineDefinition_ParameterObject {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatapipelinePipelineDefinition_ParameterObject_Attribute>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attribute = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatapipelinePipelineDefinition_ParameterObject {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinition_ParameterObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinition_ParameterObject {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinition_ParameterObject {
    pub fn build(self) -> DatapipelinePipelineDefinition_ParameterObject {
        DatapipelinePipelineDefinition_ParameterObject {
            id: self.id,
            attribute: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatapipelinePipelineDefinition_ParameterObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinition_ParameterObjectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinition_ParameterObjectRef {
        DatapipelinePipelineDefinition_ParameterObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinition_ParameterObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinition_ParameterValue {
    id: terrars::PrimField<std::string::String>,
    string_value: terrars::PrimField<std::string::String>,
}

impl DatapipelinePipelineDefinition_ParameterValue { }

impl terrars::ToListMappable for DatapipelinePipelineDefinition_ParameterValue {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinition_ParameterValue>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinition_ParameterValue {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub string_value: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinition_ParameterValue {
    pub fn build(self) -> DatapipelinePipelineDefinition_ParameterValue {
        DatapipelinePipelineDefinition_ParameterValue {
            id: self.id,
            string_value: self.string_value,
        }
    }
}

#[allow(dead_code)]
pub struct DatapipelinePipelineDefinition_ParameterValueRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinition_ParameterValueRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinition_ParameterValueRef {
        DatapipelinePipelineDefinition_ParameterValueRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinition_ParameterValueRef {
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

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinition_PipelineObject_Field {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ref_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatapipelinePipelineDefinition_PipelineObject_Field {
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

impl terrars::ToListMappable for DatapipelinePipelineDefinition_PipelineObject_Field {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinition_PipelineObject_Field>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinition_PipelineObject_Field {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinition_PipelineObject_Field {
    pub fn build(self) -> DatapipelinePipelineDefinition_PipelineObject_Field {
        DatapipelinePipelineDefinition_PipelineObject_Field {
            key: self.key,
            ref_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatapipelinePipelineDefinition_PipelineObject_FieldRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinition_PipelineObject_FieldRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinition_PipelineObject_FieldRef {
        DatapipelinePipelineDefinition_PipelineObject_FieldRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinition_PipelineObject_FieldRef {
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

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinition_PipelineObjectDynamic {
    field: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinition_PipelineObject_Field>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinition_PipelineObject {
    id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinition_PipelineObject_Field>>,
    dynamic: DatapipelinePipelineDefinition_PipelineObjectDynamic,
}

impl DatapipelinePipelineDefinition_PipelineObject {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinition_PipelineObject_Field>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.field = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.field = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DatapipelinePipelineDefinition_PipelineObject {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinition_PipelineObject>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinition_PipelineObject {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinition_PipelineObject {
    pub fn build(self) -> DatapipelinePipelineDefinition_PipelineObject {
        DatapipelinePipelineDefinition_PipelineObject {
            id: self.id,
            name: self.name,
            field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DatapipelinePipelineDefinition_PipelineObjectRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinition_PipelineObjectRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinition_PipelineObjectRef {
        DatapipelinePipelineDefinition_PipelineObjectRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinition_PipelineObjectRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
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
