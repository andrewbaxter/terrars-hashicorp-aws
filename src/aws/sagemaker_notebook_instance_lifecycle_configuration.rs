use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SagemakerNotebookInstanceLifecycleConfigurationData {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_create: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_start: Option<PrimField<String>>,
}

struct SagemakerNotebookInstanceLifecycleConfiguration_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<SagemakerNotebookInstanceLifecycleConfigurationData>,
}

#[derive(Clone)]
pub struct SagemakerNotebookInstanceLifecycleConfiguration(Rc<SagemakerNotebookInstanceLifecycleConfiguration_>);

impl SagemakerNotebookInstanceLifecycleConfiguration {
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

    #[doc= "Set the field `name`.\n"]
    pub fn set_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name = Some(v.into());
        self
    }

    #[doc= "Set the field `on_create`.\n"]
    pub fn set_on_create(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_create = Some(v.into());
        self
    }

    #[doc= "Set the field `on_start`.\n"]
    pub fn set_on_start(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().on_start = Some(v.into());
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `on_create` after provisioning.\n"]
    pub fn on_create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `on_start` after provisioning.\n"]
    pub fn on_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_start", self.extract_ref()))
    }
}

impl Resource for SagemakerNotebookInstanceLifecycleConfiguration {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for SagemakerNotebookInstanceLifecycleConfiguration {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for SagemakerNotebookInstanceLifecycleConfiguration {
    type O = ListRef<SagemakerNotebookInstanceLifecycleConfigurationRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for SagemakerNotebookInstanceLifecycleConfiguration_ {
    fn extract_resource_type(&self) -> String {
        "aws_sagemaker_notebook_instance_lifecycle_configuration".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSagemakerNotebookInstanceLifecycleConfiguration {
    pub tf_id: String,
}

impl BuildSagemakerNotebookInstanceLifecycleConfiguration {
    pub fn build(self, stack: &mut Stack) -> SagemakerNotebookInstanceLifecycleConfiguration {
        let out =
            SagemakerNotebookInstanceLifecycleConfiguration(Rc::new(SagemakerNotebookInstanceLifecycleConfiguration_ {
                shared: stack.shared.clone(),
                tf_id: self.tf_id,
                data: RefCell::new(SagemakerNotebookInstanceLifecycleConfigurationData {
                    depends_on: core::default::Default::default(),
                    provider: None,
                    lifecycle: core::default::Default::default(),
                    for_each: None,
                    id: core::default::Default::default(),
                    name: core::default::Default::default(),
                    on_create: core::default::Default::default(),
                    on_start: core::default::Default::default(),
                }),
            }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SagemakerNotebookInstanceLifecycleConfigurationRef {
    shared: StackShared,
    base: String,
}

impl Ref for SagemakerNotebookInstanceLifecycleConfigurationRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SagemakerNotebookInstanceLifecycleConfigurationRef {
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

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `on_create` after provisioning.\n"]
    pub fn on_create(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_create", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `on_start` after provisioning.\n"]
    pub fn on_start(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.on_start", self.extract_ref()))
    }
}
