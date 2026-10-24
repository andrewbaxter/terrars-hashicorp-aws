use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CodebuildWebhookData {
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
    branch_filter: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    build_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    project_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_group: std::option::Option<std::vec::Vec<CodebuildWebhookFilterGroupEl>>,
    dynamic: CodebuildWebhookDynamic,
}

struct CodebuildWebhook_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CodebuildWebhookData>,
}

#[derive(Clone)]
pub struct CodebuildWebhook(std::rc::Rc<CodebuildWebhook_>);

impl CodebuildWebhook {
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

    #[doc = "Set the field `branch_filter`.\n"]
    pub fn set_branch_filter(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().branch_filter = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `build_type`.\n"]
    pub fn set_build_type(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().build_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `filter_group`.\n"]
    pub fn set_filter_group(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildWebhookFilterGroupEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_group = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_group = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `branch_filter` after provisioning.\n"]
    pub fn branch_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payload_url` after provisioning.\n"]
    pub fn payload_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

impl terrars::Referable for CodebuildWebhook {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CodebuildWebhook { }

impl terrars::ToListMappable for CodebuildWebhook {
    type O = terrars::ListRef<CodebuildWebhookRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CodebuildWebhook_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_codebuild_webhook".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCodebuildWebhook {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub project_name: terrars::PrimField<std::string::String>,
}

impl BuildCodebuildWebhook {
    pub fn build(self, stack: &mut terrars::Stack) -> CodebuildWebhook {
        let out = CodebuildWebhook(std::rc::Rc::new(CodebuildWebhook_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CodebuildWebhookData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                branch_filter: core::default::Default::default(),
                build_type: core::default::Default::default(),
                id: core::default::Default::default(),
                project_name: self.project_name,
                filter_group: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CodebuildWebhookRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhookRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CodebuildWebhookRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `branch_filter` after provisioning.\n"]
    pub fn branch_filter(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.branch_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `build_type` after provisioning.\n"]
    pub fn build_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.build_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `payload_url` after provisioning.\n"]
    pub fn payload_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `project_name` after provisioning.\n"]
    pub fn project_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.project_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `secret` after provisioning.\n"]
    pub fn secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.secret", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CodebuildWebhookFilterGroupElFilterEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_matched_pattern: std::option::Option<terrars::PrimField<bool>>,
    pattern: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CodebuildWebhookFilterGroupElFilterEl {
    #[doc = "Set the field `exclude_matched_pattern`.\n"]
    pub fn set_exclude_matched_pattern(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_matched_pattern = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CodebuildWebhookFilterGroupElFilterEl {
    type O = terrars::BlockAssignable<CodebuildWebhookFilterGroupElFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCodebuildWebhookFilterGroupElFilterEl {
    #[doc = ""]
    pub pattern: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
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
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhookFilterGroupElFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildWebhookFilterGroupElFilterElRef {
        CodebuildWebhookFilterGroupElFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhookFilterGroupElFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_matched_pattern` after provisioning.\n"]
    pub fn exclude_matched_pattern(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_matched_pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `pattern` after provisioning.\n"]
    pub fn pattern(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.pattern", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildWebhookFilterGroupElDynamic {
    filter: std::option::Option<terrars::DynamicBlock<CodebuildWebhookFilterGroupElFilterEl>>,
}

#[derive(Serialize)]
pub struct CodebuildWebhookFilterGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<CodebuildWebhookFilterGroupElFilterEl>>,
    dynamic: CodebuildWebhookFilterGroupElDynamic,
}

impl CodebuildWebhookFilterGroupEl {
    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CodebuildWebhookFilterGroupElFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CodebuildWebhookFilterGroupEl {
    type O = terrars::BlockAssignable<CodebuildWebhookFilterGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
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
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CodebuildWebhookFilterGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CodebuildWebhookFilterGroupElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CodebuildWebhookFilterGroupElRef {
        CodebuildWebhookFilterGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CodebuildWebhookFilterGroupElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `filter` after provisioning.\n"]
    pub fn filter(&self) -> terrars::ListRef<CodebuildWebhookFilterGroupElFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter", self.base))
    }
}

#[derive(Serialize, Default)]
struct CodebuildWebhookDynamic {
    filter_group: std::option::Option<terrars::DynamicBlock<CodebuildWebhookFilterGroupEl>>,
}
