use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct ApiGatewayBasePathMappingData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    api_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base_path: Option<PrimField<String>>,
    domain_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stage_name: Option<PrimField<String>>,
}

struct ApiGatewayBasePathMapping_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApiGatewayBasePathMappingData>,
}

#[derive(Clone)]
pub struct ApiGatewayBasePathMapping(Rc<ApiGatewayBasePathMapping_>);

impl ApiGatewayBasePathMapping {
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

    #[doc= "Set the field `base_path`.\n"]
    pub fn set_base_path(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().base_path = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `stage_name`.\n"]
    pub fn set_stage_name(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().stage_name = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_path` after provisioning.\n"]
    pub fn base_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }
}

impl Resource for ApiGatewayBasePathMapping {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ApiGatewayBasePathMapping {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ApiGatewayBasePathMapping {
    type O = ListRef<ApiGatewayBasePathMappingRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for ApiGatewayBasePathMapping_ {
    fn extract_resource_type(&self) -> String {
        "aws_api_gateway_base_path_mapping".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApiGatewayBasePathMapping {
    pub tf_id: String,
    #[doc= ""]
    pub api_id: PrimField<String>,
    #[doc= ""]
    pub domain_name: PrimField<String>,
}

impl BuildApiGatewayBasePathMapping {
    pub fn build(self, stack: &mut Stack) -> ApiGatewayBasePathMapping {
        let out = ApiGatewayBasePathMapping(Rc::new(ApiGatewayBasePathMapping_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApiGatewayBasePathMappingData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                api_id: self.api_id,
                base_path: core::default::Default::default(),
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                stage_name: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApiGatewayBasePathMappingRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApiGatewayBasePathMappingRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApiGatewayBasePathMappingRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `api_id` after provisioning.\n"]
    pub fn api_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `base_path` after provisioning.\n"]
    pub fn base_path(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.base_path", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `stage_name` after provisioning.\n"]
    pub fn stage_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.stage_name", self.extract_ref()))
    }
}
