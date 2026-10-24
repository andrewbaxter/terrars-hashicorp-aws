use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DatapipelinePipelineDefinitionData {
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
    parameter_object: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinitionParameterObjectEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameter_value: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinitionParameterValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    pipeline_object: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinitionPipelineObjectEl>>,
    dynamic: DatapipelinePipelineDefinitionDynamic,
}

struct DatapipelinePipelineDefinition_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DatapipelinePipelineDefinitionData>,
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

    #[doc = "Set the field `parameter_object`.\n"]
    pub fn set_parameter_object(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinitionParameterObjectEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinitionParameterValueEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectEl>>,
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
            data: std::cell::RefCell::new(DatapipelinePipelineDefinitionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                id: core::default::Default::default(),
                pipeline_id: self.pipeline_id,
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
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    key: terrars::PrimField<std::string::String>,
    string_value: terrars::PrimField<std::string::String>,
}

impl DatapipelinePipelineDefinitionParameterObjectElAttributeEl { }

impl terrars::ToListMappable for DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub string_value: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinitionParameterObjectElAttributeEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
        DatapipelinePipelineDefinitionParameterObjectElAttributeEl {
            key: self.key,
            string_value: self.string_value,
        }
    }
}

pub struct DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
        DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionParameterObjectElAttributeElRef {
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
struct DatapipelinePipelineDefinitionParameterObjectElDynamic {
    attribute: std
    ::option
    ::Option<terrars::DynamicBlock<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionParameterObjectEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
    dynamic: DatapipelinePipelineDefinitionParameterObjectElDynamic,
}

impl DatapipelinePipelineDefinitionParameterObjectEl {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DatapipelinePipelineDefinitionParameterObjectElAttributeEl>>,
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

impl terrars::ToListMappable for DatapipelinePipelineDefinitionParameterObjectEl {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinitionParameterObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionParameterObjectEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinitionParameterObjectEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionParameterObjectEl {
        DatapipelinePipelineDefinitionParameterObjectEl {
            id: self.id,
            attribute: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatapipelinePipelineDefinitionParameterObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinitionParameterObjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinitionParameterObjectElRef {
        DatapipelinePipelineDefinitionParameterObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionParameterObjectElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionParameterValueEl {
    id: terrars::PrimField<std::string::String>,
    string_value: terrars::PrimField<std::string::String>,
}

impl DatapipelinePipelineDefinitionParameterValueEl { }

impl terrars::ToListMappable for DatapipelinePipelineDefinitionParameterValueEl {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinitionParameterValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionParameterValueEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub string_value: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinitionParameterValueEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionParameterValueEl {
        DatapipelinePipelineDefinitionParameterValueEl {
            id: self.id,
            string_value: self.string_value,
        }
    }
}

pub struct DatapipelinePipelineDefinitionParameterValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinitionParameterValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinitionParameterValueElRef {
        DatapipelinePipelineDefinitionParameterValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionParameterValueElRef {
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
pub struct DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ref_value: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
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

impl terrars::ToListMappable for DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinitionPipelineObjectElFieldEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
        DatapipelinePipelineDefinitionPipelineObjectElFieldEl {
            key: self.key,
            ref_value: core::default::Default::default(),
            string_value: core::default::Default::default(),
        }
    }
}

pub struct DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
        DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionPipelineObjectElFieldElRef {
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
struct DatapipelinePipelineDefinitionPipelineObjectElDynamic {
    field: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
}

#[derive(Serialize)]
pub struct DatapipelinePipelineDefinitionPipelineObjectEl {
    id: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    field: std::option::Option<std::vec::Vec<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
    dynamic: DatapipelinePipelineDefinitionPipelineObjectElDynamic,
}

impl DatapipelinePipelineDefinitionPipelineObjectEl {
    #[doc = "Set the field `field`.\n"]
    pub fn set_field(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectElFieldEl>>,
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

impl terrars::ToListMappable for DatapipelinePipelineDefinitionPipelineObjectEl {
    type O = terrars::BlockAssignable<DatapipelinePipelineDefinitionPipelineObjectEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDatapipelinePipelineDefinitionPipelineObjectEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDatapipelinePipelineDefinitionPipelineObjectEl {
    pub fn build(self) -> DatapipelinePipelineDefinitionPipelineObjectEl {
        DatapipelinePipelineDefinitionPipelineObjectEl {
            id: self.id,
            name: self.name,
            field: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DatapipelinePipelineDefinitionPipelineObjectElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DatapipelinePipelineDefinitionPipelineObjectElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DatapipelinePipelineDefinitionPipelineObjectElRef {
        DatapipelinePipelineDefinitionPipelineObjectElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DatapipelinePipelineDefinitionPipelineObjectElRef {
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

#[derive(Serialize, Default)]
struct DatapipelinePipelineDefinitionDynamic {
    parameter_object: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinitionParameterObjectEl>>,
    parameter_value: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinitionParameterValueEl>>,
    pipeline_object: std::option::Option<terrars::DynamicBlock<DatapipelinePipelineDefinitionPipelineObjectEl>>,
}
