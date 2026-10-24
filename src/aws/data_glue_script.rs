use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataGlueScriptData {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_edge: std::option::Option<std::vec::Vec<DataGlueScriptDagEdgeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dag_node: std::option::Option<std::vec::Vec<DataGlueScriptDagNodeEl>>,
    dynamic: DataGlueScriptDynamic,
}

struct DataGlueScript_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataGlueScriptData>,
}

#[derive(Clone)]
pub struct DataGlueScript(std::rc::Rc<DataGlueScript_>);

impl DataGlueScript {
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

    #[doc = "Set the field `language`.\n"]
    pub fn set_language(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().language = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `dag_edge`.\n"]
    pub fn set_dag_edge(self, v: impl std::convert::Into<terrars::BlockAssignable<DataGlueScriptDagEdgeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dag_edge = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dag_edge = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dag_node`.\n"]
    pub fn set_dag_node(self, v: impl std::convert::Into<terrars::BlockAssignable<DataGlueScriptDagNodeEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().dag_node = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.dag_node = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `python_script` after provisioning.\n"]
    pub fn python_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_script", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scala_code` after provisioning.\n"]
    pub fn scala_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scala_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_edge` after provisioning.\n"]
    pub fn dag_edge(&self) -> terrars::ListRef<DataGlueScriptDagEdgeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_edge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_node` after provisioning.\n"]
    pub fn dag_node(&self) -> terrars::ListRef<DataGlueScriptDagNodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_node", self.extract_ref()))
    }
}

impl terrars::Referable for DataGlueScript {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataGlueScript { }

impl terrars::ToListMappable for DataGlueScript {
    type O = terrars::ListRef<DataGlueScriptRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataGlueScript_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_glue_script".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataGlueScript {
    pub tf_id: std::string::String,
}

impl BuildDataGlueScript {
    pub fn build(self, stack: &mut terrars::Stack) -> DataGlueScript {
        let out = DataGlueScript(std::rc::Rc::new(DataGlueScript_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataGlueScriptData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                language: core::default::Default::default(),
                dag_edge: core::default::Default::default(),
                dag_node: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataGlueScriptRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueScriptRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataGlueScriptRef {
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

    #[doc = "Get a reference to the value of field `language` after provisioning.\n"]
    pub fn language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.language", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `python_script` after provisioning.\n"]
    pub fn python_script(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.python_script", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scala_code` after provisioning.\n"]
    pub fn scala_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scala_code", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_edge` after provisioning.\n"]
    pub fn dag_edge(&self) -> terrars::ListRef<DataGlueScriptDagEdgeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_edge", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dag_node` after provisioning.\n"]
    pub fn dag_node(&self) -> terrars::ListRef<DataGlueScriptDagNodeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dag_node", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataGlueScriptDagEdgeEl {
    source: terrars::PrimField<std::string::String>,
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_parameter: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataGlueScriptDagEdgeEl {
    #[doc = "Set the field `target_parameter`.\n"]
    pub fn set_target_parameter(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_parameter = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueScriptDagEdgeEl {
    type O = terrars::BlockAssignable<DataGlueScriptDagEdgeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueScriptDagEdgeEl {
    #[doc = ""]
    pub source: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildDataGlueScriptDagEdgeEl {
    pub fn build(self) -> DataGlueScriptDagEdgeEl {
        DataGlueScriptDagEdgeEl {
            source: self.source,
            target: self.target,
            target_parameter: core::default::Default::default(),
        }
    }
}

pub struct DataGlueScriptDagEdgeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueScriptDagEdgeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueScriptDagEdgeElRef {
        DataGlueScriptDagEdgeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueScriptDagEdgeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `target_parameter` after provisioning.\n"]
    pub fn target_parameter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_parameter", self.base))
    }
}

#[derive(Serialize)]
pub struct DataGlueScriptDagNodeElArgsEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    param: std::option::Option<terrars::PrimField<bool>>,
    value: terrars::PrimField<std::string::String>,
}

impl DataGlueScriptDagNodeElArgsEl {
    #[doc = "Set the field `param`.\n"]
    pub fn set_param(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.param = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataGlueScriptDagNodeElArgsEl {
    type O = terrars::BlockAssignable<DataGlueScriptDagNodeElArgsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueScriptDagNodeElArgsEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildDataGlueScriptDagNodeElArgsEl {
    pub fn build(self) -> DataGlueScriptDagNodeElArgsEl {
        DataGlueScriptDagNodeElArgsEl {
            name: self.name,
            param: core::default::Default::default(),
            value: self.value,
        }
    }
}

pub struct DataGlueScriptDagNodeElArgsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueScriptDagNodeElArgsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueScriptDagNodeElArgsElRef {
        DataGlueScriptDagNodeElArgsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueScriptDagNodeElArgsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `param` after provisioning.\n"]
    pub fn param(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.param", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataGlueScriptDagNodeElDynamic {
    args: std::option::Option<terrars::DynamicBlock<DataGlueScriptDagNodeElArgsEl>>,
}

#[derive(Serialize)]
pub struct DataGlueScriptDagNodeEl {
    id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    line_number: std::option::Option<terrars::PrimField<f64>>,
    node_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    args: std::option::Option<std::vec::Vec<DataGlueScriptDagNodeElArgsEl>>,
    dynamic: DataGlueScriptDagNodeElDynamic,
}

impl DataGlueScriptDagNodeEl {
    #[doc = "Set the field `line_number`.\n"]
    pub fn set_line_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.line_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `args`.\n"]
    pub fn set_args(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataGlueScriptDagNodeElArgsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.args = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.args = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DataGlueScriptDagNodeEl {
    type O = terrars::BlockAssignable<DataGlueScriptDagNodeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataGlueScriptDagNodeEl {
    #[doc = ""]
    pub id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub node_type: terrars::PrimField<std::string::String>,
}

impl BuildDataGlueScriptDagNodeEl {
    pub fn build(self) -> DataGlueScriptDagNodeEl {
        DataGlueScriptDagNodeEl {
            id: self.id,
            line_number: core::default::Default::default(),
            node_type: self.node_type,
            args: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DataGlueScriptDagNodeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataGlueScriptDagNodeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataGlueScriptDagNodeElRef {
        DataGlueScriptDagNodeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataGlueScriptDagNodeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `line_number` after provisioning.\n"]
    pub fn line_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.line_number", self.base))
    }

    #[doc = "Get a reference to the value of field `node_type` after provisioning.\n"]
    pub fn node_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.node_type", self.base))
    }

    #[doc = "Get a reference to the value of field `args` after provisioning.\n"]
    pub fn args(&self) -> terrars::ListRef<DataGlueScriptDagNodeElArgsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.args", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataGlueScriptDynamic {
    dag_edge: std::option::Option<terrars::DynamicBlock<DataGlueScriptDagEdgeEl>>,
    dag_node: std::option::Option<terrars::DynamicBlock<DataGlueScriptDagNodeEl>>,
}
