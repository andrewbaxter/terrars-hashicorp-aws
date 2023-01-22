use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildWebhookData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    branch_filter: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    project_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter_group: Option<Vec<CodebuildWebhookFilterGroupEl>>,
    dynamic: CodebuildWebhookDynamic,
}

struct CodebuildWebhook_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<CodebuildWebhookData>,
}

#[derive(Clone)]
pub struct CodebuildWebhook(Rc<CodebuildWebhook_>);

impl CodebuildWebhook {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Resource) -> Self {
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

    #[doc= "Set the field `branch_filter`.\n"]
    pub fn set_branch_filter(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().branch_filter = Some(v.into());
        self
    }

    #[doc= "Set the field `build_type`.\n"]
    pub fn set_build_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().build_type = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `filter_group`.\n"]
    pub fn set_filter_group(self, v: impl Into<BlockAssignable<CodebuildWebhookFilterGroupEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_group = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_group = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `branch_filter` after provisioning.\n"]
    pub fn branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `payload_url` after provisioning.\n"]
    pub fn payload_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl Resource for CodebuildWebhook {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for CodebuildWebhook {
    type O = ListRef<CodebuildWebhookRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for CodebuildWebhook_ {
    fn extract_resource_type(&self) -> String {
        "aws_codebuild_webhook".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildWebhook {
    pub tf_id: String,
    #[doc= ""]
    pub project_name: PrimField<String>,
}

impl BuildCodebuildWebhook {
    pub fn build(self, stack: &mut Stack) -> CodebuildWebhook {
        let out = CodebuildWebhook(Rc::new(CodebuildWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(CodebuildWebhookData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                branch_filter: core::default::Default::default(),
                build_type: core::default::Default::default(),
                id: core::default::Default::default(),
                project_name: self.project_name,
                filter_group: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildWebhookRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildWebhookRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodebuildWebhookRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `branch_filter` after provisioning.\n"]
    pub fn branch_filter(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.branch_filter", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `payload_url` after provisioning.\n"]
    pub fn payload_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.payload_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodebuildWebhookFilterGroupElFilterEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_matched_pattern: Option<PrimField<bool>>,
    pattern: PrimField<String>,
    #[serde(rename = "type")]
    type_: PrimField<String>,
}

impl CodebuildWebhookFilterGroupElFilterEl {
    #[doc= "Set the field `exclude_matched_pattern`.\n"]
    pub fn set_exclude_matched_pattern(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.exclude_matched_pattern = Some(v.into());
        self
    }
}

impl ToListMappable for CodebuildWebhookFilterGroupElFilterEl {
    type O = BlockAssignable<CodebuildWebhookFilterGroupElFilterEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhookFilterGroupElFilterEl {
    #[doc= ""]
    pub pattern: PrimField<String>,
    #[doc= ""]
    pub type_: PrimField<String>,
}

impl BuildCodebuildWebhookFilterGroupElFilterEl {
    pub fn build(self) -> CodebuildWebhookFilterGroupElFilterEl {
        CodebuildWebhookFilterGroupElFilterEl {
            exclude_matched_pattern: core::default::Default::default(),
            pattern: self.pattern,
            type_: self.type_,
        }
    }
}

pub struct CodebuildWebhookFilterGroupElFilterElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildWebhookFilterGroupElFilterElRef {
    fn new(shared: StackShared, base: String) -> CodebuildWebhookFilterGroupElFilterElRef {
        CodebuildWebhookFilterGroupElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhookFilterGroupElFilterElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `exclude_matched_pattern` after provisioning.\n"]
    pub fn exclude_matched_pattern(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.exclude_matched_pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc= "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildWebhookFilterGroupElDynamic {
    filter: Option<DynamicBlock<CodebuildWebhookFilterGroupElFilterEl>>,
}

#[derive(Serialize)]
pub struct CodebuildWebhookFilterGroupEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Vec<CodebuildWebhookFilterGroupElFilterEl>>,
    dynamic: CodebuildWebhookFilterGroupElDynamic,
}

impl CodebuildWebhookFilterGroupEl {
    #[doc= "Set the field `filter`.\n"]
    pub fn set_filter(mut self, v: impl Into<BlockAssignable<CodebuildWebhookFilterGroupElFilterEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.filter = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for CodebuildWebhookFilterGroupEl {
    type O = BlockAssignable<CodebuildWebhookFilterGroupEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhookFilterGroupEl {}

impl BuildCodebuildWebhookFilterGroupEl {
    pub fn build(self) -> CodebuildWebhookFilterGroupEl {
        CodebuildWebhookFilterGroupEl {
            filter: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct CodebuildWebhookFilterGroupElRef {
    shared: StackShared,
    base: String,
}

impl Ref for CodebuildWebhookFilterGroupElRef {
    fn new(shared: StackShared, base: String) -> CodebuildWebhookFilterGroupElRef {
        CodebuildWebhookFilterGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhookFilterGroupElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> ListRef<CodebuildWebhookFilterGroupElFilterElRef> {
        ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildWebhookDynamic {
    filter_group: Option<DynamicBlock<CodebuildWebhookFilterGroupEl>>,
}
