use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DlmLifecyclePolicyData {
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
    description: terrars::PrimField<std::string::String>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_details: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsEl>>,
    dynamic: DlmLifecyclePolicyDynamic,
}

struct DlmLifecyclePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DlmLifecyclePolicyData>,
}

#[derive(Clone)]
pub struct DlmLifecyclePolicy(std::rc::Rc<DlmLifecyclePolicy_>);

impl DlmLifecyclePolicy {
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

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().state = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `policy_details`.\n"]
    pub fn set_policy_details(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().policy_details = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.policy_details = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_details` after provisioning.\n"]
    pub fn policy_details(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_details", self.extract_ref()))
    }
}

impl terrars::Referable for DlmLifecyclePolicy {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for DlmLifecyclePolicy { }

impl terrars::ToListMappable for DlmLifecyclePolicy {
    type O = terrars::ListRef<DlmLifecyclePolicyRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for DlmLifecyclePolicy_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_dlm_lifecycle_policy".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDlmLifecyclePolicy {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub description: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub execution_role_arn: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy {
    pub fn build(self, stack: &mut terrars::Stack) -> DlmLifecyclePolicy {
        let out = DlmLifecyclePolicy(std::rc::Rc::new(DlmLifecyclePolicy_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DlmLifecyclePolicyData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: self.description,
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                state: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                policy_details: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct DlmLifecyclePolicyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DlmLifecyclePolicyRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `execution_role_arn` after provisioning.\n"]
    pub fn execution_role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_role_arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `policy_details` after provisioning.\n"]
    pub fn policy_details(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_details", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmk_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    #[doc = "Set the field `cmk_arn`.\n"]
    pub fn set_cmk_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cmk_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encrypted`.\n"]
    pub fn set_encrypted(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.encrypted = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    type O =
        terrars
        ::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl {
            cmk_arn: core::default::Default::default(),
            encrypted: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cmk_arn` after provisioning.\n"]
    pub fn cmk_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cmk_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    interval: terrars::PrimField<f64>,
    interval_unit: terrars::PrimField<std::string::String>,
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl { }

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval_unit: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElDynamic {
    encryption_configuration: std
    ::option
    ::Option<
        terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl>,
    >,
    retain_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    #[doc = "Set the field `encryption_configuration`.\n"]
    pub fn set_encryption_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.encryption_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.encryption_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retain_rule`.\n"]
    pub fn set_retain_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retain_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retain_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl {
            target: self.target,
            encryption_configuration: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `encryption_configuration` after provisioning.\n"]
    pub fn encryption_configuration(
        &self,
    ) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElEncryptionConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(
        &self,
    ) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyElRetainRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElActionElDynamic {
    cross_region_copy: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElActionEl {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_copy: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElActionElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElActionEl {
    #[doc = "Set the field `cross_region_copy`.\n"]
    pub fn set_cross_region_copy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionElCrossRegionCopyEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cross_region_copy = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_region_copy = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElActionEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElActionEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElActionEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElActionEl {
        DlmLifecyclePolicyPolicyDetailsElActionEl {
            name: self.name,
            cross_region_copy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlmLifecyclePolicyPolicyDetailsElActionElRef {
        DlmLifecyclePolicyPolicyDetailsElActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    description_regex: terrars::PrimField<std::string::String>,
    event_type: terrars::PrimField<std::string::String>,
    snapshot_owner: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl { }

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    #[doc = ""]
    pub description_regex: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub snapshot_owner: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
        DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl {
            description_regex: self.description_regex,
            event_type: self.event_type,
            snapshot_owner: self.snapshot_owner,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
        DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `description_regex` after provisioning.\n"]
    pub fn description_regex(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description_regex", self.base))
    }

    #[doc = "Get a reference to the value of field `event_type` after provisioning.\n"]
    pub fn event_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.event_type", self.base))
    }

    #[doc = "Get a reference to the value of field `snapshot_owner` after provisioning.\n"]
    pub fn snapshot_owner(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.snapshot_owner", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElEventSourceElDynamic {
    parameters: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElEventSourceElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElEventSourceEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
        DlmLifecyclePolicyPolicyDetailsElEventSourceEl {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
        DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElEventSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElEventSourceElParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElParametersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_boot_volume: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_reboot: std::option::Option<terrars::PrimField<bool>>,
}

impl DlmLifecyclePolicyPolicyDetailsElParametersEl {
    #[doc = "Set the field `exclude_boot_volume`.\n"]
    pub fn set_exclude_boot_volume(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_boot_volume = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_reboot`.\n"]
    pub fn set_no_reboot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_reboot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElParametersEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElParametersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElParametersEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElParametersEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElParametersEl {
        DlmLifecyclePolicyPolicyDetailsElParametersEl {
            exclude_boot_volume: core::default::Default::default(),
            no_reboot: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElParametersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElParametersElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElParametersElRef {
        DlmLifecyclePolicyPolicyDetailsElParametersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElParametersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_boot_volume` after provisioning.\n"]
    pub fn exclude_boot_volume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_boot_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `no_reboot` after provisioning.\n"]
    pub fn no_reboot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_reboot", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cron_expression: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    location: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    times: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    #[doc = "Set the field `cron_expression`.\n"]
    pub fn set_cron_expression(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cron_expression = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval_unit = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `location`.\n"]
    pub fn set_location(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.location = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `times`.\n"]
    pub fn set_times(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.times = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl {
            cron_expression: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
            location: core::default::Default::default(),
            times: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cron_expression` after provisioning.\n"]
    pub fn cron_expression(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cron_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }

    #[doc = "Get a reference to the value of field `location` after provisioning.\n"]
    pub fn location(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.location", self.base))
    }

    #[doc = "Get a reference to the value of field `times` after provisioning.\n"]
    pub fn times(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.times", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    interval: terrars::PrimField<f64>,
    interval_unit: terrars::PrimField<std::string::String>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl { }

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    type O =
        terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval_unit: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    interval: terrars::PrimField<f64>,
    interval_unit: terrars::PrimField<std::string::String>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl { }

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval_unit: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDynamic {
    deprecate_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl>>,
    retain_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmk_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags: std::option::Option<terrars::PrimField<bool>>,
    encrypted: terrars::PrimField<bool>,
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecate_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    #[doc = "Set the field `cmk_arn`.\n"]
    pub fn set_cmk_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cmk_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `copy_tags`.\n"]
    pub fn set_copy_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.copy_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `deprecate_rule`.\n"]
    pub fn set_deprecate_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deprecate_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deprecate_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retain_rule`.\n"]
    pub fn set_retain_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleEl,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retain_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retain_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    #[doc = ""]
    pub encrypted: terrars::PrimField<bool>,
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl {
            cmk_arn: core::default::Default::default(),
            copy_tags: core::default::Default::default(),
            encrypted: self.encrypted,
            target: self.target,
            deprecate_rule: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cmk_arn` after provisioning.\n"]
    pub fn cmk_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cmk_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `copy_tags` after provisioning.\n"]
    pub fn copy_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `encrypted` after provisioning.\n"]
    pub fn encrypted(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encrypted", self.base))
    }

    #[doc = "Get a reference to the value of field `target` after provisioning.\n"]
    pub fn target(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecate_rule` after provisioning.\n"]
    pub fn deprecate_rule(
        &self,
    ) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElDeprecateRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecate_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(
        &self,
    ) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleElRetainRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    availability_zones: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    #[doc = ""]
    pub availability_zones: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl {
            availability_zones: self.availability_zones,
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    #[doc = "Set the field `count`.\n"]
    pub fn set_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval`.\n"]
    pub fn set_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `interval_unit`.\n"]
    pub fn set_interval_unit(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.interval_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `count` after provisioning.\n"]
    pub fn count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.count", self.base))
    }

    #[doc = "Get a reference to the value of field `interval` after provisioning.\n"]
    pub fn interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval", self.base))
    }

    #[doc = "Get a reference to the value of field `interval_unit` after provisioning.\n"]
    pub fn interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.interval_unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    target_accounts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unshare_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unshare_interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    #[doc = "Set the field `unshare_interval`.\n"]
    pub fn set_unshare_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.unshare_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `unshare_interval_unit`.\n"]
    pub fn set_unshare_interval_unit(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.unshare_interval_unit = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    #[doc = ""]
    pub target_accounts: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl {
            target_accounts: self.target_accounts,
            unshare_interval: core::default::Default::default(),
            unshare_interval_unit: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `target_accounts` after provisioning.\n"]
    pub fn target_accounts(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.target_accounts", self.base))
    }

    #[doc = "Get a reference to the value of field `unshare_interval` after provisioning.\n"]
    pub fn unshare_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unshare_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `unshare_interval_unit` after provisioning.\n"]
    pub fn unshare_interval_unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unshare_interval_unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElScheduleElDynamic {
    create_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>>,
    cross_region_copy_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>>,
    deprecate_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>>,
    fast_restore_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>>,
    retain_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>>,
    share_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsElScheduleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_to_add: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variable_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_copy_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecate_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_restore_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElScheduleElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleEl {
    #[doc = "Set the field `copy_tags`.\n"]
    pub fn set_copy_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.copy_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tags_to_add`.\n"]
    pub fn set_tags_to_add(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.tags_to_add = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `variable_tags`.\n"]
    pub fn set_variable_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.variable_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_rule`.\n"]
    pub fn set_create_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.create_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.create_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cross_region_copy_rule`.\n"]
    pub fn set_cross_region_copy_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElCrossRegionCopyRuleEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cross_region_copy_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cross_region_copy_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deprecate_rule`.\n"]
    pub fn set_deprecate_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deprecate_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deprecate_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `fast_restore_rule`.\n"]
    pub fn set_fast_restore_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fast_restore_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fast_restore_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `retain_rule`.\n"]
    pub fn set_retain_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retain_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retain_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `share_rule`.\n"]
    pub fn set_share_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.share_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.share_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsElScheduleEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsElScheduleEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicyPolicyDetailsElScheduleEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsElScheduleEl {
        DlmLifecyclePolicyPolicyDetailsElScheduleEl {
            copy_tags: core::default::Default::default(),
            name: self.name,
            tags_to_add: core::default::Default::default(),
            variable_tags: core::default::Default::default(),
            create_rule: core::default::Default::default(),
            cross_region_copy_rule: core::default::Default::default(),
            deprecate_rule: core::default::Default::default(),
            fast_restore_rule: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            share_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
        DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElScheduleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_tags` after provisioning.\n"]
    pub fn copy_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `tags_to_add` after provisioning.\n"]
    pub fn tags_to_add(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_to_add", self.base))
    }

    #[doc = "Get a reference to the value of field `variable_tags` after provisioning.\n"]
    pub fn variable_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.variable_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `create_rule` after provisioning.\n"]
    pub fn create_rule(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElCreateRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecate_rule` after provisioning.\n"]
    pub fn deprecate_rule(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElDeprecateRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecate_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_restore_rule` after provisioning.\n"]
    pub fn fast_restore_rule(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElFastRestoreRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_restore_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElRetainRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `share_rule` after provisioning.\n"]
    pub fn share_rule(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElShareRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyPolicyDetailsElDynamic {
    action: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElActionEl>>,
    event_source: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>>,
    parameters: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElParametersEl>>,
    schedule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsElScheduleEl>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicyPolicyDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_source: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElParametersEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<DlmLifecyclePolicyPolicyDetailsElScheduleEl>>,
    dynamic: DlmLifecyclePolicyPolicyDetailsElDynamic,
}

impl DlmLifecyclePolicyPolicyDetailsEl {
    #[doc = "Set the field `policy_type`.\n"]
    pub fn set_policy_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_locations`.\n"]
    pub fn set_resource_locations(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_locations = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_types`.\n"]
    pub fn set_resource_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.resource_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_tags`.\n"]
    pub fn set_target_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.target_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `event_source`.\n"]
    pub fn set_event_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElEventSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.event_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.event_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElParametersEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.parameters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.parameters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `schedule`.\n"]
    pub fn set_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsElScheduleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicyPolicyDetailsEl {
    type O = terrars::BlockAssignable<DlmLifecyclePolicyPolicyDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicyPolicyDetailsEl {}

impl BuildDlmLifecyclePolicyPolicyDetailsEl {
    pub fn build(self) -> DlmLifecyclePolicyPolicyDetailsEl {
        DlmLifecyclePolicyPolicyDetailsEl {
            policy_type: core::default::Default::default(),
            resource_locations: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            target_tags: core::default::Default::default(),
            action: core::default::Default::default(),
            event_source: core::default::Default::default(),
            parameters: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct DlmLifecyclePolicyPolicyDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicyPolicyDetailsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlmLifecyclePolicyPolicyDetailsElRef {
        DlmLifecyclePolicyPolicyDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicyPolicyDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_locations` after provisioning.\n"]
    pub fn resource_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tags` after provisioning.\n"]
    pub fn target_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.target_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElActionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElEventSourceElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElParametersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DlmLifecyclePolicyPolicyDetailsElScheduleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyDynamic {
    policy_details: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicyPolicyDetailsEl>>,
}
