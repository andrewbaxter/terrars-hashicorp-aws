use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GrafanaWorkspaceApiKeyData {
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
    key_name: PrimField<String>,
    key_role: PrimField<String>,
    seconds_to_live: PrimField<f64>,
    workspace_id: PrimField<String>,
}

struct GrafanaWorkspaceApiKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<GrafanaWorkspaceApiKeyData>,
}

#[derive(Clone)]
pub struct GrafanaWorkspaceApiKey(Rc<GrafanaWorkspaceApiKey_>);

impl GrafanaWorkspaceApiKey {
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

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_role` after provisioning.\n"]
    pub fn key_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `seconds_to_live` after provisioning.\n"]
    pub fn seconds_to_live(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds_to_live", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }
}

impl Resource for GrafanaWorkspaceApiKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for GrafanaWorkspaceApiKey {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for GrafanaWorkspaceApiKey {
    type O = ListRef<GrafanaWorkspaceApiKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for GrafanaWorkspaceApiKey_ {
    fn extract_resource_type(&self) -> String {
        "aws_grafana_workspace_api_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGrafanaWorkspaceApiKey {
    pub tf_id: String,
    #[doc= ""]
    pub key_name: PrimField<String>,
    #[doc= ""]
    pub key_role: PrimField<String>,
    #[doc= ""]
    pub seconds_to_live: PrimField<f64>,
    #[doc= ""]
    pub workspace_id: PrimField<String>,
}

impl BuildGrafanaWorkspaceApiKey {
    pub fn build(self, stack: &mut Stack) -> GrafanaWorkspaceApiKey {
        let out = GrafanaWorkspaceApiKey(Rc::new(GrafanaWorkspaceApiKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(GrafanaWorkspaceApiKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                id: core::default::Default::default(),
                key_name: self.key_name,
                key_role: self.key_role,
                seconds_to_live: self.seconds_to_live,
                workspace_id: self.workspace_id,
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GrafanaWorkspaceApiKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for GrafanaWorkspaceApiKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GrafanaWorkspaceApiKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_name` after provisioning.\n"]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_role` after provisioning.\n"]
    pub fn key_role(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_role", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `seconds_to_live` after provisioning.\n"]
    pub fn seconds_to_live(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.seconds_to_live", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `workspace_id` after provisioning.\n"]
    pub fn workspace_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.workspace_id", self.extract_ref()))
    }
}
