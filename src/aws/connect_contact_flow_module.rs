use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ConnectContactFlowModuleData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_hash: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    instance_id: PrimField<String>,
    name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tags_all: Option<RecField<PrimField<String>>>,
}

struct ConnectContactFlowModule_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ConnectContactFlowModuleData>,
}

#[derive(Clone)]
pub struct ConnectContactFlowModule(Rc<ConnectContactFlowModule_>);

impl ConnectContactFlowModule {
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

    #[doc= "Set the field `content`.\n"]
    pub fn set_content(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content = Some(v.into());
        self
    }

    #[doc= "Set the field `content_hash`.\n"]
    pub fn set_content_hash(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_hash = Some(v.into());
        self
    }

    #[doc= "Set the field `description`.\n"]
    pub fn set_description(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().description = Some(v.into());
        self
    }

    #[doc= "Set the field `filename`.\n"]
    pub fn set_filename(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().filename = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
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

    #[doc= "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `contact_flow_module_id` after provisioning.\n"]
    pub fn contact_flow_module_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_module_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_hash` after provisioning.\n"]
    pub fn content_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}

impl Resource for ConnectContactFlowModule {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ConnectContactFlowModule {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ConnectContactFlowModule {
    type O = ListRef<ConnectContactFlowModuleRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for ConnectContactFlowModule_ {
    fn extract_resource_type(&self) -> String {
        "aws_connect_contact_flow_module".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildConnectContactFlowModule {
    pub tf_id: String,
    #[doc= ""]
    pub instance_id: PrimField<String>,
    #[doc= ""]
    pub name: PrimField<String>,
}

impl BuildConnectContactFlowModule {
    pub fn build(self, stack: &mut Stack) -> ConnectContactFlowModule {
        let out = ConnectContactFlowModule(Rc::new(ConnectContactFlowModule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ConnectContactFlowModuleData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                content: core::default::Default::default(),
                content_hash: core::default::Default::default(),
                description: core::default::Default::default(),
                filename: core::default::Default::default(),
                id: core::default::Default::default(),
                instance_id: self.instance_id,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ConnectContactFlowModuleRef {
    shared: StackShared,
    base: String,
}

impl Ref for ConnectContactFlowModuleRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ConnectContactFlowModuleRef {
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

    #[doc= "Get a reference to the value of field `contact_flow_module_id` after provisioning.\n"]
    pub fn contact_flow_module_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.contact_flow_module_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content` after provisioning.\n"]
    pub fn content(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_hash` after provisioning.\n"]
    pub fn content_hash(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_hash", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `filename` after provisioning.\n"]
    pub fn filename(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.filename", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }
}
