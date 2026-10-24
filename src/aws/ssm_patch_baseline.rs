use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SsmPatchBaselineData {
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
    approved_patches: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approved_patches_compliance_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approved_patches_enable_non_security: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    operating_system: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rejected_patches: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rejected_patches_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approval_rule: std::option::Option<std::vec::Vec<SsmPatchBaselineApprovalRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    global_filter: std::option::Option<std::vec::Vec<SsmPatchBaselineGlobalFilterEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source: std::option::Option<std::vec::Vec<SsmPatchBaselineSourceEl>>,
    dynamic: SsmPatchBaselineDynamic,
}

struct SsmPatchBaseline_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SsmPatchBaselineData>,
}

#[derive(Clone)]
pub struct SsmPatchBaseline(std::rc::Rc<SsmPatchBaseline_>);

impl SsmPatchBaseline {
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

    #[doc = "Set the field `approved_patches`.\n"]
    pub fn set_approved_patches(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().approved_patches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approved_patches_compliance_level`.\n"]
    pub fn set_approved_patches_compliance_level(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().approved_patches_compliance_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approved_patches_enable_non_security`.\n"]
    pub fn set_approved_patches_enable_non_security(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().approved_patches_enable_non_security = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `operating_system`.\n"]
    pub fn set_operating_system(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().operating_system = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rejected_patches`.\n"]
    pub fn set_rejected_patches(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().rejected_patches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rejected_patches_action`.\n"]
    pub fn set_rejected_patches_action(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().rejected_patches_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags_all`.\n"]
    pub fn set_tags_all(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().tags_all = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approval_rule`.\n"]
    pub fn set_approval_rule(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmPatchBaselineApprovalRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().approval_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.approval_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `global_filter`.\n"]
    pub fn set_global_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmPatchBaselineGlobalFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().global_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.global_filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source`.\n"]
    pub fn set_source(self, v: impl std::convert::Into<terrars::BlockAssignable<SsmPatchBaselineSourceEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approved_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_enable_non_security", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rejected_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rejected_patches_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> terrars::ListRef<SsmPatchBaselineApprovalRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> terrars::ListRef<SsmPatchBaselineGlobalFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<SsmPatchBaselineSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

impl terrars::Referable for SsmPatchBaseline {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SsmPatchBaseline { }

impl terrars::ToListMappable for SsmPatchBaseline {
    type O = terrars::ListRef<SsmPatchBaselineRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SsmPatchBaseline_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ssm_patch_baseline".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSsmPatchBaseline {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSsmPatchBaseline {
    pub fn build(self, stack: &mut terrars::Stack) -> SsmPatchBaseline {
        let out = SsmPatchBaseline(std::rc::Rc::new(SsmPatchBaseline_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SsmPatchBaselineData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                approved_patches: core::default::Default::default(),
                approved_patches_compliance_level: core::default::Default::default(),
                approved_patches_enable_non_security: core::default::Default::default(),
                description: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                operating_system: core::default::Default::default(),
                rejected_patches: core::default::Default::default(),
                rejected_patches_action: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                approval_rule: core::default::Default::default(),
                global_filter: core::default::Default::default(),
                source: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SsmPatchBaselineRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmPatchBaselineRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SsmPatchBaselineRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approved_patches` after provisioning.\n"]
    pub fn approved_patches(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.approved_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approved_patches_compliance_level` after provisioning.\n"]
    pub fn approved_patches_compliance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_compliance_level", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `approved_patches_enable_non_security` after provisioning.\n"]
    pub fn approved_patches_enable_non_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.approved_patches_enable_non_security", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `operating_system` after provisioning.\n"]
    pub fn operating_system(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operating_system", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches` after provisioning.\n"]
    pub fn rejected_patches(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.rejected_patches", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rejected_patches_action` after provisioning.\n"]
    pub fn rejected_patches_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rejected_patches_action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `approval_rule` after provisioning.\n"]
    pub fn approval_rule(&self) -> terrars::ListRef<SsmPatchBaselineApprovalRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.approval_rule", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `global_filter` after provisioning.\n"]
    pub fn global_filter(&self) -> terrars::ListRef<SsmPatchBaselineGlobalFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.global_filter", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source` after provisioning.\n"]
    pub fn source(&self) -> terrars::ListRef<SsmPatchBaselineSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SsmPatchBaselineApprovalRuleElPatchFilterEl {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SsmPatchBaselineApprovalRuleElPatchFilterEl { }

impl terrars::ToListMappable for SsmPatchBaselineApprovalRuleElPatchFilterEl {
    type O = terrars::BlockAssignable<SsmPatchBaselineApprovalRuleElPatchFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineApprovalRuleElPatchFilterEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmPatchBaselineApprovalRuleElPatchFilterEl {
    pub fn build(self) -> SsmPatchBaselineApprovalRuleElPatchFilterEl {
        SsmPatchBaselineApprovalRuleElPatchFilterEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmPatchBaselineApprovalRuleElPatchFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SsmPatchBaselineApprovalRuleElPatchFilterElRef {
        SsmPatchBaselineApprovalRuleElPatchFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineApprovalRuleElPatchFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmPatchBaselineApprovalRuleElDynamic {
    patch_filter: std::option::Option<terrars::DynamicBlock<SsmPatchBaselineApprovalRuleElPatchFilterEl>>,
}

#[derive(Serialize)]
pub struct SsmPatchBaselineApprovalRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approve_after_days: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    approve_until_date: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_level: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enable_non_security: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    patch_filter: std::option::Option<std::vec::Vec<SsmPatchBaselineApprovalRuleElPatchFilterEl>>,
    dynamic: SsmPatchBaselineApprovalRuleElDynamic,
}

impl SsmPatchBaselineApprovalRuleEl {
    #[doc = "Set the field `approve_after_days`.\n"]
    pub fn set_approve_after_days(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.approve_after_days = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `approve_until_date`.\n"]
    pub fn set_approve_until_date(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.approve_until_date = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `compliance_level`.\n"]
    pub fn set_compliance_level(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.compliance_level = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enable_non_security`.\n"]
    pub fn set_enable_non_security(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.enable_non_security = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `patch_filter`.\n"]
    pub fn set_patch_filter(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SsmPatchBaselineApprovalRuleElPatchFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.patch_filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.patch_filter = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SsmPatchBaselineApprovalRuleEl {
    type O = terrars::BlockAssignable<SsmPatchBaselineApprovalRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineApprovalRuleEl {}

impl BuildSsmPatchBaselineApprovalRuleEl {
    pub fn build(self) -> SsmPatchBaselineApprovalRuleEl {
        SsmPatchBaselineApprovalRuleEl {
            approve_after_days: core::default::Default::default(),
            approve_until_date: core::default::Default::default(),
            compliance_level: core::default::Default::default(),
            enable_non_security: core::default::Default::default(),
            patch_filter: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SsmPatchBaselineApprovalRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmPatchBaselineApprovalRuleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmPatchBaselineApprovalRuleElRef {
        SsmPatchBaselineApprovalRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineApprovalRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `approve_after_days` after provisioning.\n"]
    pub fn approve_after_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approve_after_days", self.base))
    }

    #[doc = "Get a reference to the value of field `approve_until_date` after provisioning.\n"]
    pub fn approve_until_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.approve_until_date", self.base))
    }

    #[doc = "Get a reference to the value of field `compliance_level` after provisioning.\n"]
    pub fn compliance_level(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.compliance_level", self.base))
    }

    #[doc = "Get a reference to the value of field `enable_non_security` after provisioning.\n"]
    pub fn enable_non_security(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enable_non_security", self.base))
    }

    #[doc = "Get a reference to the value of field `patch_filter` after provisioning.\n"]
    pub fn patch_filter(&self) -> terrars::ListRef<SsmPatchBaselineApprovalRuleElPatchFilterElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.patch_filter", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmPatchBaselineGlobalFilterEl {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SsmPatchBaselineGlobalFilterEl { }

impl terrars::ToListMappable for SsmPatchBaselineGlobalFilterEl {
    type O = terrars::BlockAssignable<SsmPatchBaselineGlobalFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineGlobalFilterEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmPatchBaselineGlobalFilterEl {
    pub fn build(self) -> SsmPatchBaselineGlobalFilterEl {
        SsmPatchBaselineGlobalFilterEl {
            key: self.key,
            values: self.values,
        }
    }
}

pub struct SsmPatchBaselineGlobalFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmPatchBaselineGlobalFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmPatchBaselineGlobalFilterElRef {
        SsmPatchBaselineGlobalFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineGlobalFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct SsmPatchBaselineSourceEl {
    configuration: terrars::PrimField<std::string::String>,
    name: terrars::PrimField<std::string::String>,
    products: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl SsmPatchBaselineSourceEl { }

impl terrars::ToListMappable for SsmPatchBaselineSourceEl {
    type O = terrars::BlockAssignable<SsmPatchBaselineSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSsmPatchBaselineSourceEl {
    #[doc = ""]
    pub configuration: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub products: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildSsmPatchBaselineSourceEl {
    pub fn build(self) -> SsmPatchBaselineSourceEl {
        SsmPatchBaselineSourceEl {
            configuration: self.configuration,
            name: self.name,
            products: self.products,
        }
    }
}

pub struct SsmPatchBaselineSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SsmPatchBaselineSourceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SsmPatchBaselineSourceElRef {
        SsmPatchBaselineSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SsmPatchBaselineSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `configuration` after provisioning.\n"]
    pub fn configuration(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `products` after provisioning.\n"]
    pub fn products(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.products", self.base))
    }
}

#[derive(Serialize, Default)]
struct SsmPatchBaselineDynamic {
    approval_rule: std::option::Option<terrars::DynamicBlock<SsmPatchBaselineApprovalRuleEl>>,
    global_filter: std::option::Option<terrars::DynamicBlock<SsmPatchBaselineGlobalFilterEl>>,
    source: std::option::Option<terrars::DynamicBlock<SsmPatchBaselineSourceEl>>,
}
