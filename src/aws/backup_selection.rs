use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BackupSelectionData {
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
    iam_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    plan_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resources: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<BackupSelectionConditionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    selection_tag: std::option::Option<std::vec::Vec<BackupSelectionSelectionTagEl>>,
    dynamic: BackupSelectionDynamic,
}

struct BackupSelection_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BackupSelectionData>,
}

#[derive(Clone)]
pub struct BackupSelection(std::rc::Rc<BackupSelection_>);

impl BackupSelection {
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

    #[doc = "Set the field `not_resources`.\n"]
    pub fn set_not_resources(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().not_resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resources`.\n"]
    pub fn set_resources(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().resources = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupSelectionConditionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `selection_tag`.\n"]
    pub fn set_selection_tag(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupSelectionSelectionTagEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().selection_tag = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.selection_tag = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_resources` after provisioning.\n"]
    pub fn not_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.not_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }
}

impl terrars::Referable for BackupSelection {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BackupSelection { }

impl terrars::ToListMappable for BackupSelection {
    type O = terrars::ListRef<BackupSelectionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BackupSelection_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_backup_selection".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBackupSelection {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub iam_role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub plan_id: terrars::PrimField<std::string::String>,
}

impl BuildBackupSelection {
    pub fn build(self, stack: &mut terrars::Stack) -> BackupSelection {
        let out = BackupSelection(std::rc::Rc::new(BackupSelection_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BackupSelectionData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                iam_role_arn: self.iam_role_arn,
                id: core::default::Default::default(),
                name: self.name,
                not_resources: core::default::Default::default(),
                plan_id: self.plan_id,
                resources: core::default::Default::default(),
                condition: core::default::Default::default(),
                selection_tag: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BackupSelectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BackupSelectionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `iam_role_arn` after provisioning.\n"]
    pub fn iam_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.iam_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `not_resources` after provisioning.\n"]
    pub fn not_resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.not_resources", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `plan_id` after provisioning.\n"]
    pub fn plan_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.plan_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `resources` after provisioning.\n"]
    pub fn resources(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.resources", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BackupSelectionConditionElStringEqualsEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupSelectionConditionElStringEqualsEl { }

impl terrars::ToListMappable for BackupSelectionConditionElStringEqualsEl {
    type O = terrars::BlockAssignable<BackupSelectionConditionElStringEqualsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringEqualsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupSelectionConditionElStringEqualsEl {
    pub fn build(self) -> BackupSelectionConditionElStringEqualsEl {
        BackupSelectionConditionElStringEqualsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringEqualsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionConditionElStringEqualsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupSelectionConditionElStringEqualsElRef {
        BackupSelectionConditionElStringEqualsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringEqualsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupSelectionConditionElStringLikeEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupSelectionConditionElStringLikeEl { }

impl terrars::ToListMappable for BackupSelectionConditionElStringLikeEl {
    type O = terrars::BlockAssignable<BackupSelectionConditionElStringLikeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringLikeEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupSelectionConditionElStringLikeEl {
    pub fn build(self) -> BackupSelectionConditionElStringLikeEl {
        BackupSelectionConditionElStringLikeEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringLikeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionConditionElStringLikeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupSelectionConditionElStringLikeElRef {
        BackupSelectionConditionElStringLikeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringLikeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupSelectionConditionElStringNotEqualsEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupSelectionConditionElStringNotEqualsEl { }

impl terrars::ToListMappable for BackupSelectionConditionElStringNotEqualsEl {
    type O = terrars::BlockAssignable<BackupSelectionConditionElStringNotEqualsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringNotEqualsEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupSelectionConditionElStringNotEqualsEl {
    pub fn build(self) -> BackupSelectionConditionElStringNotEqualsEl {
        BackupSelectionConditionElStringNotEqualsEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringNotEqualsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionConditionElStringNotEqualsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BackupSelectionConditionElStringNotEqualsElRef {
        BackupSelectionConditionElStringNotEqualsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringNotEqualsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct BackupSelectionConditionElStringNotLikeEl {
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupSelectionConditionElStringNotLikeEl { }

impl terrars::ToListMappable for BackupSelectionConditionElStringNotLikeEl {
    type O = terrars::BlockAssignable<BackupSelectionConditionElStringNotLikeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionElStringNotLikeEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupSelectionConditionElStringNotLikeEl {
    pub fn build(self) -> BackupSelectionConditionElStringNotLikeEl {
        BackupSelectionConditionElStringNotLikeEl {
            key: self.key,
            value: self.value,
        }
    }
}

pub struct BackupSelectionConditionElStringNotLikeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionConditionElStringNotLikeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupSelectionConditionElStringNotLikeElRef {
        BackupSelectionConditionElStringNotLikeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElStringNotLikeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupSelectionConditionElDynamic {
    string_equals: std::option::Option<terrars::DynamicBlock<BackupSelectionConditionElStringEqualsEl>>,
    string_like: std::option::Option<terrars::DynamicBlock<BackupSelectionConditionElStringLikeEl>>,
    string_not_equals: std::option::Option<terrars::DynamicBlock<BackupSelectionConditionElStringNotEqualsEl>>,
    string_not_like: std::option::Option<terrars::DynamicBlock<BackupSelectionConditionElStringNotLikeEl>>,
}

#[derive(Serialize)]
pub struct BackupSelectionConditionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_equals: std::option::Option<std::vec::Vec<BackupSelectionConditionElStringEqualsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_like: std::option::Option<std::vec::Vec<BackupSelectionConditionElStringLikeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_not_equals: std::option::Option<std::vec::Vec<BackupSelectionConditionElStringNotEqualsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_not_like: std::option::Option<std::vec::Vec<BackupSelectionConditionElStringNotLikeEl>>,
    dynamic: BackupSelectionConditionElDynamic,
}

impl BackupSelectionConditionEl {
    #[doc = "Set the field `string_equals`.\n"]
    pub fn set_string_equals(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupSelectionConditionElStringEqualsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_equals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_equals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_like`.\n"]
    pub fn set_string_like(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupSelectionConditionElStringLikeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_like = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_like = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_not_equals`.\n"]
    pub fn set_string_not_equals(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupSelectionConditionElStringNotEqualsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_not_equals = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_not_equals = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_not_like`.\n"]
    pub fn set_string_not_like(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BackupSelectionConditionElStringNotLikeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_not_like = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_not_like = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BackupSelectionConditionEl {
    type O = terrars::BlockAssignable<BackupSelectionConditionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionConditionEl {}

impl BuildBackupSelectionConditionEl {
    pub fn build(self) -> BackupSelectionConditionEl {
        BackupSelectionConditionEl {
            string_equals: core::default::Default::default(),
            string_like: core::default::Default::default(),
            string_not_equals: core::default::Default::default(),
            string_not_like: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BackupSelectionConditionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionConditionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupSelectionConditionElRef {
        BackupSelectionConditionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionConditionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct BackupSelectionSelectionTagEl {
    key: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BackupSelectionSelectionTagEl { }

impl terrars::ToListMappable for BackupSelectionSelectionTagEl {
    type O = terrars::BlockAssignable<BackupSelectionSelectionTagEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBackupSelectionSelectionTagEl {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBackupSelectionSelectionTagEl {
    pub fn build(self) -> BackupSelectionSelectionTagEl {
        BackupSelectionSelectionTagEl {
            key: self.key,
            type_: self.type_,
            value: self.value,
        }
    }
}

pub struct BackupSelectionSelectionTagElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BackupSelectionSelectionTagElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BackupSelectionSelectionTagElRef {
        BackupSelectionSelectionTagElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BackupSelectionSelectionTagElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BackupSelectionDynamic {
    condition: std::option::Option<terrars::DynamicBlock<BackupSelectionConditionEl>>,
    selection_tag: std::option::Option<terrars::DynamicBlock<BackupSelectionSelectionTagEl>>,
}
