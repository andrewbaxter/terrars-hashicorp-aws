#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DlmLifecyclePolicyData_ {
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
    default_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    description: terrars::PrimField<std::string::String>,
    execution_role_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_details: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails>>,
    dynamic: DlmLifecyclePolicyDynamic,
}

struct DlmLifecyclePolicy_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DlmLifecyclePolicyData_>,
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

    #[doc = "Set the field `default_policy`.\n"]
    pub fn set_default_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().default_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails>>,
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

    #[doc = "Get a reference to the value of field `default_policy` after provisioning.\n"]
    pub fn default_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn policy_details(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetailsRef> {
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
            data: std::cell::RefCell::new(DlmLifecyclePolicyData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                default_policy: core::default::Default::default(),
                description: self.description,
                execution_role_arn: self.execution_role_arn,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `default_policy` after provisioning.\n"]
    pub fn default_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_policy", self.extract_ref()))
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

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
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
    pub fn policy_details(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.policy_details", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicyDynamic {
    policy_details: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmk_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encrypted: std::option::Option<terrars::PrimField<bool>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {
    type O =
        terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {
        DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration {
            cmk_arn: core::default::Default::default(),
            encrypted: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfigurationRef {
        DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfigurationRef {
    #[allow(dead_code)]
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
pub struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule {
    interval: terrars::PrimField<f64>,
    interval_unit: terrars::PrimField<std::string::String>,
}

impl DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule { }

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule {
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval_unit: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule {
        DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef {
    #[allow(dead_code)]
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
struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyDynamic {
    encryption_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration>>,
    retain_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
    target: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encryption_configuration: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
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
                            DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfiguration,
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
                        terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>,
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
    #[doc = ""]
    pub target: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
        DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy {
            target: self.target,
            encryption_configuration: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyRef {
        DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopyRef {
    #[allow(dead_code)]
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
    ) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_EncryptionConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.encryption_configuration", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicy_PolicyDetails_ActionDynamic {
    cross_region_copy: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Action {
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_copy: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_ActionDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Action {
    #[doc = "Set the field `cross_region_copy`.\n"]
    pub fn set_cross_region_copy(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy>>,
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Action {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Action {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Action {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Action {
        DlmLifecyclePolicy_PolicyDetails_Action {
            name: self.name,
            cross_region_copy: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlmLifecyclePolicy_PolicyDetails_ActionRef {
        DlmLifecyclePolicy_PolicyDetails_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters {
    description_regex: terrars::PrimField<std::string::String>,
    event_type: terrars::PrimField<std::string::String>,
    snapshot_owner: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters { }

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_EventSource_Parameters {
    #[doc = ""]
    pub description_regex: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub event_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub snapshot_owner: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_EventSource_Parameters {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters {
        DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters {
            description_regex: self.description_regex,
            event_type: self.event_type,
            snapshot_owner: self.snapshot_owner,
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_EventSource_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_EventSource_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_EventSource_ParametersRef {
        DlmLifecyclePolicy_PolicyDetails_EventSource_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_EventSource_ParametersRef {
    #[allow(dead_code)]
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
struct DlmLifecyclePolicy_PolicyDetails_EventSourceDynamic {
    parameters: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_EventSource {
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_EventSourceDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_EventSource {
    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_EventSource_Parameters>>,
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_EventSource {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_EventSource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_EventSource {
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_EventSource {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_EventSource {
        DlmLifecyclePolicy_PolicyDetails_EventSource {
            type_: self.type_,
            parameters: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_EventSourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_EventSourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_EventSourceRef {
        DlmLifecyclePolicy_PolicyDetails_EventSourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_EventSourceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_EventSource_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Exclusions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_boot_volumes: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_volume_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Exclusions {
    #[doc = "Set the field `exclude_boot_volumes`.\n"]
    pub fn set_exclude_boot_volumes(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_boot_volumes = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_tags`.\n"]
    pub fn set_exclude_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_volume_types`.\n"]
    pub fn set_exclude_volume_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_volume_types = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Exclusions {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Exclusions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Exclusions {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Exclusions {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Exclusions {
        DlmLifecyclePolicy_PolicyDetails_Exclusions {
            exclude_boot_volumes: core::default::Default::default(),
            exclude_tags: core::default::Default::default(),
            exclude_volume_types: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_ExclusionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_ExclusionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_ExclusionsRef {
        DlmLifecyclePolicy_PolicyDetails_ExclusionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_ExclusionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_boot_volumes` after provisioning.\n"]
    pub fn exclude_boot_volumes(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_boot_volumes", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_tags` after provisioning.\n"]
    pub fn exclude_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exclude_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_volume_types` after provisioning.\n"]
    pub fn exclude_volume_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclude_volume_types", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Parameters {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_boot_volume: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclude_data_volume_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    no_reboot: std::option::Option<terrars::PrimField<bool>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Parameters {
    #[doc = "Set the field `exclude_boot_volume`.\n"]
    pub fn set_exclude_boot_volume(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.exclude_boot_volume = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `exclude_data_volume_tags`.\n"]
    pub fn set_exclude_data_volume_tags(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.exclude_data_volume_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `no_reboot`.\n"]
    pub fn set_no_reboot(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.no_reboot = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Parameters {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Parameters>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Parameters {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Parameters {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Parameters {
        DlmLifecyclePolicy_PolicyDetails_Parameters {
            exclude_boot_volume: core::default::Default::default(),
            exclude_data_volume_tags: core::default::Default::default(),
            no_reboot: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_ParametersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_ParametersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_ParametersRef {
        DlmLifecyclePolicy_PolicyDetails_ParametersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_ParametersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `exclude_boot_volume` after provisioning.\n"]
    pub fn exclude_boot_volume(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.exclude_boot_volume", self.base))
    }

    #[doc = "Get a reference to the value of field `exclude_data_volume_tags` after provisioning.\n"]
    pub fn exclude_data_volume_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.exclude_data_volume_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `no_reboot` after provisioning.\n"]
    pub fn no_reboot(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.no_reboot", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {
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

impl terrars
::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {
    type O =
        terrars
        ::BlockAssignable<
            DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {
    pub fn build(
        self,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTierRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTierRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTierRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTierRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTierRef {
    #[allow(dead_code)]
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

#[derive(Serialize, Default)]
struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleDynamic {
    retention_archive_tier: std
    ::option
    ::Option<
        terrars
        ::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier>,
    >,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retention_archive_tier: std
    ::option
    ::Option<
        std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier>,
    >,
    dynamic: DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {
    #[doc = "Set the field `retention_archive_tier`.\n"]
    pub fn set_retention_archive_tier(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<
                            DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTier,
                        >,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.retention_archive_tier = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.retention_archive_tier = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule {
            retention_archive_tier: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `retention_archive_tier` after provisioning.\n"]
    pub fn retention_archive_tier(
        &self,
    ) -> terrars
    ::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule_RetentionArchiveTierRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retention_archive_tier", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleDynamic {
    archive_retain_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_retain_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {
    #[doc = "Set the field `archive_retain_rule`.\n"]
    pub fn set_archive_retain_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRule>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive_retain_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_retain_rule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule {
            archive_retain_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `archive_retain_rule` after provisioning.\n"]
    pub fn archive_retain_rule(
        &self,
    ) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule_ArchiveRetainRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_retain_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execute_operation_on_script_failure: std::option::Option<terrars::PrimField<bool>>,
    execution_handler: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_handler_service: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    execution_timeout: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    maximum_retry_count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stages: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
    #[doc = "Set the field `execute_operation_on_script_failure`.\n"]
    pub fn set_execute_operation_on_script_failure(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<bool>>,
    ) -> Self {
        self.execute_operation_on_script_failure = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_handler_service`.\n"]
    pub fn set_execution_handler_service(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.execution_handler_service = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `execution_timeout`.\n"]
    pub fn set_execution_timeout(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.execution_timeout = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `maximum_retry_count`.\n"]
    pub fn set_maximum_retry_count(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.maximum_retry_count = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `stages`.\n"]
    pub fn set_stages(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.stages = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
    #[doc = ""]
    pub execution_handler: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts {
            execute_operation_on_script_failure: core::default::Default::default(),
            execution_handler: self.execution_handler,
            execution_handler_service: core::default::Default::default(),
            execution_timeout: core::default::Default::default(),
            maximum_retry_count: core::default::Default::default(),
            stages: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_ScriptsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_ScriptsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_ScriptsRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_ScriptsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_ScriptsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `execute_operation_on_script_failure` after provisioning.\n"]
    pub fn execute_operation_on_script_failure(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execute_operation_on_script_failure", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_handler` after provisioning.\n"]
    pub fn execution_handler(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_handler", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_handler_service` after provisioning.\n"]
    pub fn execution_handler_service(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_handler_service", self.base))
    }

    #[doc = "Get a reference to the value of field `execution_timeout` after provisioning.\n"]
    pub fn execution_timeout(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.execution_timeout", self.base))
    }

    #[doc = "Get a reference to the value of field `maximum_retry_count` after provisioning.\n"]
    pub fn maximum_retry_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.maximum_retry_count", self.base))
    }

    #[doc = "Get a reference to the value of field `stages` after provisioning.\n"]
    pub fn stages(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.stages", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleDynamic {
    scripts: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {
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
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scripts: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {
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

    #[doc = "Set the field `scripts`.\n"]
    pub fn set_scripts(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_Scripts>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.scripts = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.scripts = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule {
            cron_expression: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
            location: core::default::Default::default(),
            times: core::default::Default::default(),
            scripts: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `scripts` after provisioning.\n"]
    pub fn scripts(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule_ScriptsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.scripts", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule {
    interval: terrars::PrimField<f64>,
    interval_unit: terrars::PrimField<std::string::String>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule { }

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule {
    #[doc = ""]
    pub interval: terrars::PrimField<f64>,
    #[doc = ""]
    pub interval_unit: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule {
            interval: self.interval,
            interval_unit: self.interval_unit,
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRuleRef {
    #[allow(dead_code)]
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
struct DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleDynamic {
    deprecate_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule>>,
    retain_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cmk_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags: std::option::Option<terrars::PrimField<bool>>,
    encrypted: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecate_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
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

    #[doc = "Set the field `target`.\n"]
    pub fn set_target(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_region`.\n"]
    pub fn set_target_region(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_region = std::option::Option::Some(v.into());
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
                        ::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRule>,
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
                        terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRule>,
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
    #[doc = ""]
    pub encrypted: terrars::PrimField<bool>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule {
            cmk_arn: core::default::Default::default(),
            copy_tags: core::default::Default::default(),
            encrypted: self.encrypted,
            target: core::default::Default::default(),
            target_region: core::default::Default::default(),
            deprecate_rule: core::default::Default::default(),
            retain_rule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRuleRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `target_region` after provisioning.\n"]
    pub fn target_region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_region", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecate_rule` after provisioning.\n"]
    pub fn deprecate_rule(
        &self,
    ) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule_DeprecateRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecate_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Action_CrossRegionCopy_RetainRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRuleRef {
    #[allow(dead_code)]
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
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
    availability_zones: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
    #[doc = ""]
    pub availability_zones: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule {
            availability_zones: self.availability_zones,
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRuleRef {
    #[allow(dead_code)]
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
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    count: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule {
            count: core::default::Default::default(),
            interval: core::default::Default::default(),
            interval_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRuleRef {
    #[allow(dead_code)]
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
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
    target_accounts: terrars::SetField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unshare_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unshare_interval_unit: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
    #[doc = ""]
    pub target_accounts: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule {
            target_accounts: self.target_accounts,
            unshare_interval: core::default::Default::default(),
            unshare_interval_unit: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRuleRef {
        DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRuleRef {
    #[allow(dead_code)]
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
struct DlmLifecyclePolicy_PolicyDetails_ScheduleDynamic {
    archive_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule>>,
    create_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule>>,
    cross_region_copy_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule>>,
    deprecate_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule>>,
    fast_restore_rule: std
    ::option
    ::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule>>,
    retain_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule>>,
    share_rule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails_Schedule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_to_add: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    variable_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cross_region_copy_rule: std
    ::option
    ::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deprecate_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fast_restore_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    share_rule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule>>,
    dynamic: DlmLifecyclePolicy_PolicyDetails_ScheduleDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails_Schedule {
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

    #[doc = "Set the field `archive_rule`.\n"]
    pub fn set_archive_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive_rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive_rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `create_rule`.\n"]
    pub fn set_create_rule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRule>>,
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
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_CrossRegionCopyRule>>,
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
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRule>>,
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
                    ::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRule>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRule>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRule>>,
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails_Schedule {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails_Schedule {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildDlmLifecyclePolicy_PolicyDetails_Schedule {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails_Schedule {
        DlmLifecyclePolicy_PolicyDetails_Schedule {
            copy_tags: core::default::Default::default(),
            name: self.name,
            tags_to_add: core::default::Default::default(),
            variable_tags: core::default::Default::default(),
            archive_rule: core::default::Default::default(),
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

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetails_ScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetails_ScheduleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlmLifecyclePolicy_PolicyDetails_ScheduleRef {
        DlmLifecyclePolicy_PolicyDetails_ScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetails_ScheduleRef {
    #[allow(dead_code)]
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

    #[doc = "Get a reference to the value of field `archive_rule` after provisioning.\n"]
    pub fn archive_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_ArchiveRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `create_rule` after provisioning.\n"]
    pub fn create_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_CreateRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.create_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `deprecate_rule` after provisioning.\n"]
    pub fn deprecate_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_DeprecateRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deprecate_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `fast_restore_rule` after provisioning.\n"]
    pub fn fast_restore_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_FastRestoreRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.fast_restore_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_rule` after provisioning.\n"]
    pub fn retain_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_RetainRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.retain_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `share_rule` after provisioning.\n"]
    pub fn share_rule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_Schedule_ShareRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.share_rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct DlmLifecyclePolicy_PolicyDetailsDynamic {
    action: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Action>>,
    event_source: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_EventSource>>,
    exclusions: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Exclusions>>,
    parameters: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Parameters>>,
    schedule: std::option::Option<terrars::DynamicBlock<DlmLifecyclePolicy_PolicyDetails_Schedule>>,
}

#[derive(Serialize)]
pub struct DlmLifecyclePolicy_PolicyDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    copy_tags: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    extend_deletion: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_language: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    policy_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_locations: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retain_interval: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_source: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_EventSource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exclusions: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Exclusions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    parameters: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Parameters>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    schedule: std::option::Option<std::vec::Vec<DlmLifecyclePolicy_PolicyDetails_Schedule>>,
    dynamic: DlmLifecyclePolicy_PolicyDetailsDynamic,
}

impl DlmLifecyclePolicy_PolicyDetails {
    #[doc = "Set the field `copy_tags`.\n"]
    pub fn set_copy_tags(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.copy_tags = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `create_interval`.\n"]
    pub fn set_create_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.create_interval = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `extend_deletion`.\n"]
    pub fn set_extend_deletion(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.extend_deletion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `policy_language`.\n"]
    pub fn set_policy_language(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.policy_language = std::option::Option::Some(v.into());
        self
    }

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

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `retain_interval`.\n"]
    pub fn set_retain_interval(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retain_interval = std::option::Option::Some(v.into());
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
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Action>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_EventSource>>,
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

    #[doc = "Set the field `exclusions`.\n"]
    pub fn set_exclusions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Exclusions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exclusions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exclusions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `parameters`.\n"]
    pub fn set_parameters(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Parameters>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails_Schedule>>,
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

impl terrars::ToListMappable for DlmLifecyclePolicy_PolicyDetails {
    type O = terrars::BlockAssignable<DlmLifecyclePolicy_PolicyDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDlmLifecyclePolicy_PolicyDetails {}

impl BuildDlmLifecyclePolicy_PolicyDetails {
    pub fn build(self) -> DlmLifecyclePolicy_PolicyDetails {
        DlmLifecyclePolicy_PolicyDetails {
            copy_tags: core::default::Default::default(),
            create_interval: core::default::Default::default(),
            extend_deletion: core::default::Default::default(),
            policy_language: core::default::Default::default(),
            policy_type: core::default::Default::default(),
            resource_locations: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            resource_types: core::default::Default::default(),
            retain_interval: core::default::Default::default(),
            target_tags: core::default::Default::default(),
            action: core::default::Default::default(),
            event_source: core::default::Default::default(),
            exclusions: core::default::Default::default(),
            parameters: core::default::Default::default(),
            schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DlmLifecyclePolicy_PolicyDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DlmLifecyclePolicy_PolicyDetailsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DlmLifecyclePolicy_PolicyDetailsRef {
        DlmLifecyclePolicy_PolicyDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DlmLifecyclePolicy_PolicyDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `copy_tags` after provisioning.\n"]
    pub fn copy_tags(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.copy_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `create_interval` after provisioning.\n"]
    pub fn create_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `extend_deletion` after provisioning.\n"]
    pub fn extend_deletion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.extend_deletion", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_language` after provisioning.\n"]
    pub fn policy_language(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_language", self.base))
    }

    #[doc = "Get a reference to the value of field `policy_type` after provisioning.\n"]
    pub fn policy_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.policy_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_locations` after provisioning.\n"]
    pub fn resource_locations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_locations", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_types` after provisioning.\n"]
    pub fn resource_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.resource_types", self.base))
    }

    #[doc = "Get a reference to the value of field `retain_interval` after provisioning.\n"]
    pub fn retain_interval(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retain_interval", self.base))
    }

    #[doc = "Get a reference to the value of field `target_tags` after provisioning.\n"]
    pub fn target_tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.target_tags", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `event_source` after provisioning.\n"]
    pub fn event_source(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_EventSourceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_source", self.base))
    }

    #[doc = "Get a reference to the value of field `exclusions` after provisioning.\n"]
    pub fn exclusions(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_ExclusionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exclusions", self.base))
    }

    #[doc = "Get a reference to the value of field `parameters` after provisioning.\n"]
    pub fn parameters(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_ParametersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.parameters", self.base))
    }

    #[doc = "Get a reference to the value of field `schedule` after provisioning.\n"]
    pub fn schedule(&self) -> terrars::ListRef<DlmLifecyclePolicy_PolicyDetails_ScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.schedule", self.base))
    }
}
