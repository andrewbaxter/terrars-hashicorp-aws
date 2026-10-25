#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct MailmanagerRuleSetData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule>>,
    dynamic: MailmanagerRuleSetDynamic,
}

struct MailmanagerRuleSet_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<MailmanagerRuleSetData_>,
}

#[derive(Clone)]
pub struct MailmanagerRuleSet(std::rc::Rc<MailmanagerRuleSet_>);

impl MailmanagerRuleSet {
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

    #[doc =
        "Set the field `region`.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn set_region(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().region = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `rule`.\n"]
    pub fn set_rule(self, v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modification_date` after provisioning.\n"]
    pub fn last_modification_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modification_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<MailmanagerRuleSet_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

impl terrars::Referable for MailmanagerRuleSet {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for MailmanagerRuleSet { }

impl terrars::ToListMappable for MailmanagerRuleSet {
    type O = terrars::ListRef<MailmanagerRuleSetRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for MailmanagerRuleSet_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_mailmanager_rule_set".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildMailmanagerRuleSet {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet {
    pub fn build(self, stack: &mut terrars::Stack) -> MailmanagerRuleSet {
        let out = MailmanagerRuleSet(std::rc::Rc::new(MailmanagerRuleSet_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(MailmanagerRuleSetData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                name: self.name,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                rule: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct MailmanagerRuleSetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSetRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl MailmanagerRuleSetRef {
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

    #[doc = "Get a reference to the value of field `created_date` after provisioning.\n"]
    pub fn created_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `last_modification_date` after provisioning.\n"]
    pub fn last_modification_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.last_modification_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<MailmanagerRuleSet_RuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSetDynamic {
    rule: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_AddHeader {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_AddHeader { }

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_AddHeader {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_AddHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_AddHeader {
    #[doc = ""]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub header_value: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_AddHeader {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_AddHeader {
        MailmanagerRuleSet_Rule_Action_AddHeader {
            header_name: self.header_name,
            header_value: self.header_value,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_AddHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_AddHeaderRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_AddHeaderRef {
        MailmanagerRuleSet_Rule_Action_AddHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_AddHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `header_name` after provisioning.\n"]
    pub fn header_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_name", self.base))
    }

    #[doc = "Get a reference to the value of field `header_value` after provisioning.\n"]
    pub fn header_value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.header_value", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_Archive {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    target_archive: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_Archive {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_Archive {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Archive>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_Archive {
    #[doc = ""]
    pub target_archive: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_Archive {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_Archive {
        MailmanagerRuleSet_Rule_Action_Archive {
            action_failure_policy: core::default::Default::default(),
            target_archive: self.target_archive,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_ArchiveRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_ArchiveRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_ArchiveRef {
        MailmanagerRuleSet_Rule_Action_ArchiveRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_ArchiveRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `target_archive` after provisioning.\n"]
    pub fn target_archive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_archive", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_Bounce {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    diagnostic_message: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    message: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    sender: terrars::PrimField<std::string::String>,
    smtp_reply_code: terrars::PrimField<std::string::String>,
    status_code: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_Bounce {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `message`.\n"]
    pub fn set_message(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.message = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_Bounce {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Bounce>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_Bounce {
    #[doc = ""]
    pub diagnostic_message: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub sender: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub smtp_reply_code: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub status_code: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_Bounce {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_Bounce {
        MailmanagerRuleSet_Rule_Action_Bounce {
            action_failure_policy: core::default::Default::default(),
            diagnostic_message: self.diagnostic_message,
            message: core::default::Default::default(),
            role_arn: self.role_arn,
            sender: self.sender,
            smtp_reply_code: self.smtp_reply_code,
            status_code: self.status_code,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_BounceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_BounceRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_BounceRef {
        MailmanagerRuleSet_Rule_Action_BounceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_BounceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `diagnostic_message` after provisioning.\n"]
    pub fn diagnostic_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.diagnostic_message", self.base))
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `sender` after provisioning.\n"]
    pub fn sender(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sender", self.base))
    }

    #[doc = "Get a reference to the value of field `smtp_reply_code` after provisioning.\n"]
    pub fn smtp_reply_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.smtp_reply_code", self.base))
    }

    #[doc = "Get a reference to the value of field `status_code` after provisioning.\n"]
    pub fn status_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_code", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_DeliverToMailbox {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    mailbox_arn: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_DeliverToMailbox {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_DeliverToMailbox {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_DeliverToMailbox>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_DeliverToMailbox {
    #[doc = ""]
    pub mailbox_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_DeliverToMailbox {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_DeliverToMailbox {
        MailmanagerRuleSet_Rule_Action_DeliverToMailbox {
            action_failure_policy: core::default::Default::default(),
            mailbox_arn: self.mailbox_arn,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_DeliverToMailboxRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_DeliverToMailboxRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Action_DeliverToMailboxRef {
        MailmanagerRuleSet_Rule_Action_DeliverToMailboxRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_DeliverToMailboxRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `mailbox_arn` after provisioning.\n"]
    pub fn mailbox_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mailbox_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    application_id: terrars::PrimField<std::string::String>,
    index_id: terrars::PrimField<std::string::String>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_DeliverToQBusiness>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
    #[doc = ""]
    pub application_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub index_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
        MailmanagerRuleSet_Rule_Action_DeliverToQBusiness {
            action_failure_policy: core::default::Default::default(),
            application_id: self.application_id,
            index_id: self.index_id,
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_DeliverToQBusinessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_DeliverToQBusinessRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Action_DeliverToQBusinessRef {
        MailmanagerRuleSet_Rule_Action_DeliverToQBusinessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_DeliverToQBusinessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `application_id` after provisioning.\n"]
    pub fn application_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.application_id", self.base))
    }

    #[doc = "Get a reference to the value of field `index_id` after provisioning.\n"]
    pub fn index_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.index_id", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_Drop {}

impl MailmanagerRuleSet_Rule_Action_Drop { }

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_Drop {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Drop>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_Drop {}

impl BuildMailmanagerRuleSet_Rule_Action_Drop {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_Drop {
        MailmanagerRuleSet_Rule_Action_Drop {}
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_DropRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_DropRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_DropRef {
        MailmanagerRuleSet_Rule_Action_DropRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_DropRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_InvokeLambda {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    function_arn: terrars::PrimField<std::string::String>,
    invocation_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    retry_time_minutes: std::option::Option<terrars::PrimField<f64>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_InvokeLambda {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `retry_time_minutes`.\n"]
    pub fn set_retry_time_minutes(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.retry_time_minutes = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_InvokeLambda {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_InvokeLambda>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_InvokeLambda {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub invocation_type: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_InvokeLambda {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_InvokeLambda {
        MailmanagerRuleSet_Rule_Action_InvokeLambda {
            action_failure_policy: core::default::Default::default(),
            function_arn: self.function_arn,
            invocation_type: self.invocation_type,
            retry_time_minutes: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_InvokeLambdaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_InvokeLambdaRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Action_InvokeLambdaRef {
        MailmanagerRuleSet_Rule_Action_InvokeLambdaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_InvokeLambdaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `retry_time_minutes` after provisioning.\n"]
    pub fn retry_time_minutes(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.retry_time_minutes", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_PublishToSns {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    payload_type: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    topic_arn: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_PublishToSns {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `payload_type`.\n"]
    pub fn set_payload_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.payload_type = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_PublishToSns {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_PublishToSns>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_PublishToSns {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_PublishToSns {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_PublishToSns {
        MailmanagerRuleSet_Rule_Action_PublishToSns {
            action_failure_policy: core::default::Default::default(),
            encoding: core::default::Default::default(),
            payload_type: core::default::Default::default(),
            role_arn: self.role_arn,
            topic_arn: self.topic_arn,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_PublishToSnsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_PublishToSnsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Action_PublishToSnsRef {
        MailmanagerRuleSet_Rule_Action_PublishToSnsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_PublishToSnsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `payload_type` after provisioning.\n"]
    pub fn payload_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.payload_type", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_Relay {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mail_from: std::option::Option<terrars::PrimField<std::string::String>>,
    relay: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_Relay {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mail_from`.\n"]
    pub fn set_mail_from(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mail_from = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_Relay {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Relay>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_Relay {
    #[doc = ""]
    pub relay: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_Relay {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_Relay {
        MailmanagerRuleSet_Rule_Action_Relay {
            action_failure_policy: core::default::Default::default(),
            mail_from: core::default::Default::default(),
            relay: self.relay,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_RelayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_RelayRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_RelayRef {
        MailmanagerRuleSet_Rule_Action_RelayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_RelayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `mail_from` after provisioning.\n"]
    pub fn mail_from(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mail_from", self.base))
    }

    #[doc = "Get a reference to the value of field `relay` after provisioning.\n"]
    pub fn relay(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.relay", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_ReplaceRecipient {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_with: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl MailmanagerRuleSet_Rule_Action_ReplaceRecipient {
    #[doc = "Set the field `replace_with`.\n"]
    pub fn set_replace_with(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.replace_with = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_ReplaceRecipient {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_ReplaceRecipient>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_ReplaceRecipient {}

impl BuildMailmanagerRuleSet_Rule_Action_ReplaceRecipient {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_ReplaceRecipient {
        MailmanagerRuleSet_Rule_Action_ReplaceRecipient { replace_with: core::default::Default::default() }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_ReplaceRecipientRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_ReplaceRecipientRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Action_ReplaceRecipientRef {
        MailmanagerRuleSet_Rule_Action_ReplaceRecipientRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_ReplaceRecipientRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `replace_with` after provisioning.\n"]
    pub fn replace_with(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replace_with", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_Send {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Action_Send {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_Send {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Send>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_Send {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_Send {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_Send {
        MailmanagerRuleSet_Rule_Action_Send {
            action_failure_policy: core::default::Default::default(),
            role_arn: self.role_arn,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_SendRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_SendRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_SendRef {
        MailmanagerRuleSet_Rule_Action_SendRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_SendRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action_WriteToS3 {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action_failure_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    role_arn: terrars::PrimField<std::string::String>,
    s3_bucket: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_sse_kms_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl MailmanagerRuleSet_Rule_Action_WriteToS3 {
    #[doc = "Set the field `action_failure_policy`.\n"]
    pub fn set_action_failure_policy(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.action_failure_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_prefix`.\n"]
    pub fn set_s3_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `s3_sse_kms_key_id`.\n"]
    pub fn set_s3_sse_kms_key_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_sse_kms_key_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action_WriteToS3 {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_WriteToS3>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action_WriteToS3 {
    #[doc = ""]
    pub role_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub s3_bucket: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Action_WriteToS3 {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action_WriteToS3 {
        MailmanagerRuleSet_Rule_Action_WriteToS3 {
            action_failure_policy: core::default::Default::default(),
            role_arn: self.role_arn,
            s3_bucket: self.s3_bucket,
            s3_prefix: core::default::Default::default(),
            s3_sse_kms_key_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Action_WriteToS3Ref {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Action_WriteToS3Ref {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_Action_WriteToS3Ref {
        MailmanagerRuleSet_Rule_Action_WriteToS3Ref {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Action_WriteToS3Ref {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action_failure_policy` after provisioning.\n"]
    pub fn action_failure_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action_failure_policy", self.base))
    }

    #[doc = "Get a reference to the value of field `role_arn` after provisioning.\n"]
    pub fn role_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.role_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_bucket` after provisioning.\n"]
    pub fn s3_bucket(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_prefix` after provisioning.\n"]
    pub fn s3_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_sse_kms_key_id` after provisioning.\n"]
    pub fn s3_sse_kms_key_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_sse_kms_key_id", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_ActionDynamic {
    add_header: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_AddHeader>>,
    archive: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_Archive>>,
    bounce: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_Bounce>>,
    deliver_to_mailbox: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_DeliverToMailbox>>,
    deliver_to_q_business: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_DeliverToQBusiness>>,
    drop: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_Drop>>,
    invoke_lambda: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_InvokeLambda>>,
    publish_to_sns: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_PublishToSns>>,
    relay: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_Relay>>,
    replace_recipient: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_ReplaceRecipient>>,
    send: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_Send>>,
    write_to_s3: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action_WriteToS3>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Action {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_header: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_AddHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    archive: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_Archive>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bounce: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_Bounce>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deliver_to_mailbox: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_DeliverToMailbox>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    deliver_to_q_business: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_DeliverToQBusiness>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    drop: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_Drop>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invoke_lambda: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_InvokeLambda>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    publish_to_sns: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_PublishToSns>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    relay: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_Relay>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    replace_recipient: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_ReplaceRecipient>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    send: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_Send>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    write_to_s3: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action_WriteToS3>>,
    dynamic: MailmanagerRuleSet_Rule_ActionDynamic,
}

impl MailmanagerRuleSet_Rule_Action {
    #[doc = "Set the field `add_header`.\n"]
    pub fn set_add_header(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_AddHeader>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.add_header = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.add_header = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `archive`.\n"]
    pub fn set_archive(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Archive>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.archive = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.archive = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bounce`.\n"]
    pub fn set_bounce(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Bounce>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.bounce = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.bounce = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deliver_to_mailbox`.\n"]
    pub fn set_deliver_to_mailbox(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_DeliverToMailbox>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deliver_to_mailbox = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deliver_to_mailbox = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `deliver_to_q_business`.\n"]
    pub fn set_deliver_to_q_business(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_DeliverToQBusiness>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.deliver_to_q_business = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.deliver_to_q_business = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `drop`.\n"]
    pub fn set_drop(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Drop>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.drop = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.drop = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `invoke_lambda`.\n"]
    pub fn set_invoke_lambda(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_InvokeLambda>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.invoke_lambda = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.invoke_lambda = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `publish_to_sns`.\n"]
    pub fn set_publish_to_sns(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_PublishToSns>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.publish_to_sns = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.publish_to_sns = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `relay`.\n"]
    pub fn set_relay(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Relay>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.relay = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.relay = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `replace_recipient`.\n"]
    pub fn set_replace_recipient(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_ReplaceRecipient>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.replace_recipient = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.replace_recipient = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `send`.\n"]
    pub fn set_send(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_Send>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.send = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.send = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `write_to_s3`.\n"]
    pub fn set_write_to_s3(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action_WriteToS3>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.write_to_s3 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.write_to_s3 = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Action {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Action {}

impl BuildMailmanagerRuleSet_Rule_Action {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Action {
        MailmanagerRuleSet_Rule_Action {
            add_header: core::default::Default::default(),
            archive: core::default::Default::default(),
            bounce: core::default::Default::default(),
            deliver_to_mailbox: core::default::Default::default(),
            deliver_to_q_business: core::default::Default::default(),
            drop: core::default::Default::default(),
            invoke_lambda: core::default::Default::default(),
            publish_to_sns: core::default::Default::default(),
            relay: core::default::Default::default(),
            replace_recipient: core::default::Default::default(),
            send: core::default::Default::default(),
            write_to_s3: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_ActionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_ActionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_ActionRef {
        MailmanagerRuleSet_Rule_ActionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_ActionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `add_header` after provisioning.\n"]
    pub fn add_header(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_AddHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.add_header", self.base))
    }

    #[doc = "Get a reference to the value of field `archive` after provisioning.\n"]
    pub fn archive(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_ArchiveRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.archive", self.base))
    }

    #[doc = "Get a reference to the value of field `bounce` after provisioning.\n"]
    pub fn bounce(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_BounceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.bounce", self.base))
    }

    #[doc = "Get a reference to the value of field `deliver_to_mailbox` after provisioning.\n"]
    pub fn deliver_to_mailbox(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_DeliverToMailboxRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deliver_to_mailbox", self.base))
    }

    #[doc = "Get a reference to the value of field `deliver_to_q_business` after provisioning.\n"]
    pub fn deliver_to_q_business(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_DeliverToQBusinessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.deliver_to_q_business", self.base))
    }

    #[doc = "Get a reference to the value of field `drop` after provisioning.\n"]
    pub fn drop(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_DropRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.drop", self.base))
    }

    #[doc = "Get a reference to the value of field `invoke_lambda` after provisioning.\n"]
    pub fn invoke_lambda(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_InvokeLambdaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.invoke_lambda", self.base))
    }

    #[doc = "Get a reference to the value of field `publish_to_sns` after provisioning.\n"]
    pub fn publish_to_sns(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_PublishToSnsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.publish_to_sns", self.base))
    }

    #[doc = "Get a reference to the value of field `relay` after provisioning.\n"]
    pub fn relay(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_RelayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.relay", self.base))
    }

    #[doc = "Get a reference to the value of field `replace_recipient` after provisioning.\n"]
    pub fn replace_recipient(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_ReplaceRecipientRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.replace_recipient", self.base))
    }

    #[doc = "Get a reference to the value of field `send` after provisioning.\n"]
    pub fn send(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_SendRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.send", self.base))
    }

    #[doc = "Get a reference to the value of field `write_to_s3` after provisioning.\n"]
    pub fn write_to_s3(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Action_WriteToS3Ref> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.write_to_s3", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis {
    analyzer: terrars::PrimField<std::string::String>,
    result_field: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis { }

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis {
    #[doc = ""]
    pub analyzer: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub result_field: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis {
        MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis {
            analyzer: self.analyzer,
            result_field: self.result_field,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef {
        MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `analyzer` after provisioning.\n"]
    pub fn analyzer(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.analyzer", self.base))
    }

    #[doc = "Get a reference to the value of field `result_field` after provisioning.\n"]
    pub fn result_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.result_field", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList {
    address_lists: terrars::ListField<terrars::PrimField<std::string::String>>,
    attribute: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList { }

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList {
    #[doc = ""]
    pub address_lists: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub attribute: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList {
        MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList {
            address_lists: self.address_lists,
            attribute: self.attribute,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressListRef {
        MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address_lists` after provisioning.\n"]
    pub fn address_lists(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address_lists", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>>,
    is_in_address_list: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_in_address_list: std
    ::option
    ::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `is_in_address_list`.\n"]
    pub fn set_is_in_address_list(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressList>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.is_in_address_list = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.is_in_address_list = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {}

impl BuildMailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {
        MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate {
            attribute: core::default::Default::default(),
            analysis: core::default::Default::default(),
            is_in_address_list: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateRef {
        MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }

    #[doc = "Get a reference to the value of field `is_in_address_list` after provisioning.\n"]
    pub fn is_in_address_list(
        &self,
    ) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate_IsInAddressListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.is_in_address_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_BooleanExpressionDynamic {
    evaluate: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_BooleanExpression {
    operator: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_BooleanExpressionDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_BooleanExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression_Evaluate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_BooleanExpression {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_BooleanExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_BooleanExpression {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_BooleanExpression {
        MailmanagerRuleSet_Rule_Unless_BooleanExpression {
            operator: self.operator,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef {
        MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_BooleanExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_DmarcExpression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl MailmanagerRuleSet_Rule_Unless_DmarcExpression { }

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_DmarcExpression {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_DmarcExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_DmarcExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_DmarcExpression {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_DmarcExpression {
        MailmanagerRuleSet_Rule_Unless_DmarcExpression {
            operator: self.operator,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef {
        MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate {
    attribute: terrars::PrimField<std::string::String>,
}

impl MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate { }

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate {
    #[doc = ""]
    pub attribute: terrars::PrimField<std::string::String>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate {
        MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate { attribute: self.attribute }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef {
        MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_IpExpressionDynamic {
    evaluate: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_IpExpression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_IpExpressionDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_IpExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_IpExpression {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_IpExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_IpExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_IpExpression {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_IpExpression {
        MailmanagerRuleSet_Rule_Unless_IpExpression {
            operator: self.operator,
            values: self.values,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_IpExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_IpExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_IpExpressionRef {
        MailmanagerRuleSet_Rule_Unless_IpExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_IpExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_NumberExpressionDynamic {
    evaluate: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_NumberExpression {
    operator: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_NumberExpressionDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_NumberExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_IpExpression_Evaluate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_NumberExpression {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_NumberExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_NumberExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_NumberExpression {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_NumberExpression {
        MailmanagerRuleSet_Rule_Unless_NumberExpression {
            operator: self.operator,
            value: self.value,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_NumberExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_NumberExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_NumberExpressionRef {
        MailmanagerRuleSet_Rule_Unless_NumberExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_NumberExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_IpExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    client_certificate_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mime_header_attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `client_certificate_attribute`.\n"]
    pub fn set_client_certificate_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.client_certificate_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mime_header_attribute`.\n"]
    pub fn set_mime_header_attribute(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.mime_header_attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {}

impl BuildMailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {
        MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate {
            attribute: core::default::Default::default(),
            client_certificate_attribute: core::default::Default::default(),
            mime_header_attribute: core::default::Default::default(),
            analysis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateRef {
        MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `client_certificate_attribute` after provisioning.\n"]
    pub fn client_certificate_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.client_certificate_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `mime_header_attribute` after provisioning.\n"]
    pub fn mime_header_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mime_header_attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_StringExpressionDynamic {
    evaluate: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_StringExpression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_StringExpressionDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_StringExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_StringExpression {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_StringExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_StringExpression {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_StringExpression {
        MailmanagerRuleSet_Rule_Unless_StringExpression {
            operator: self.operator,
            values: self.values,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_StringExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_StringExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_StringExpressionRef {
        MailmanagerRuleSet_Rule_Unless_StringExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_StringExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_StringExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateDynamic {
    analysis: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    analysis: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {
    #[doc = "Set the field `attribute`.\n"]
    pub fn set_attribute(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attribute = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `analysis`.\n"]
    pub fn set_analysis(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_Analysis>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.analysis = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.analysis = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {}

impl BuildMailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {
        MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate {
            attribute: core::default::Default::default(),
            analysis: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateRef {
        MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attribute` after provisioning.\n"]
    pub fn attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute", self.base))
    }

    #[doc = "Get a reference to the value of field `analysis` after provisioning.\n"]
    pub fn analysis(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_StringExpression_Evaluate_AnalysisRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.analysis", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_Unless_VerdictExpressionDynamic {
    evaluate: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless_VerdictExpression {
    operator: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    evaluate: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate>>,
    dynamic: MailmanagerRuleSet_Rule_Unless_VerdictExpressionDynamic,
}

impl MailmanagerRuleSet_Rule_Unless_VerdictExpression {
    #[doc = "Set the field `evaluate`.\n"]
    pub fn set_evaluate(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_VerdictExpression_Evaluate>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.evaluate = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.evaluate = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless_VerdictExpression {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_VerdictExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless_VerdictExpression {
    #[doc = ""]
    pub operator: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildMailmanagerRuleSet_Rule_Unless_VerdictExpression {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless_VerdictExpression {
        MailmanagerRuleSet_Rule_Unless_VerdictExpression {
            operator: self.operator,
            values: self.values,
            evaluate: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef {
        MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `operator` after provisioning.\n"]
    pub fn operator(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.operator", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.values", self.base))
    }

    #[doc = "Get a reference to the value of field `evaluate` after provisioning.\n"]
    pub fn evaluate(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_VerdictExpression_EvaluateRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.evaluate", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_ConditionDynamic {
    boolean_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_BooleanExpression>>,
    dmarc_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_DmarcExpression>>,
    ip_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_IpExpression>>,
    number_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_NumberExpression>>,
    string_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_StringExpression>>,
    verdict_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_VerdictExpression>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Condition {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_BooleanExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dmarc_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_DmarcExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_IpExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_NumberExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_StringExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verdict_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_VerdictExpression>>,
    dynamic: MailmanagerRuleSet_Rule_ConditionDynamic,
}

impl MailmanagerRuleSet_Rule_Condition {
    #[doc = "Set the field `boolean_expression`.\n"]
    pub fn set_boolean_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boolean_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boolean_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dmarc_expression`.\n"]
    pub fn set_dmarc_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_DmarcExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dmarc_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dmarc_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_expression`.\n"]
    pub fn set_ip_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_IpExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `number_expression`.\n"]
    pub fn set_number_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_NumberExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.number_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.number_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_expression`.\n"]
    pub fn set_string_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verdict_expression`.\n"]
    pub fn set_verdict_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_VerdictExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verdict_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verdict_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Condition {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Condition>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Condition {}

impl BuildMailmanagerRuleSet_Rule_Condition {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Condition {
        MailmanagerRuleSet_Rule_Condition {
            boolean_expression: core::default::Default::default(),
            dmarc_expression: core::default::Default::default(),
            ip_expression: core::default::Default::default(),
            number_expression: core::default::Default::default(),
            string_expression: core::default::Default::default(),
            verdict_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_ConditionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_ConditionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_ConditionRef {
        MailmanagerRuleSet_Rule_ConditionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_ConditionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boolean_expression` after provisioning.\n"]
    pub fn boolean_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `dmarc_expression` after provisioning.\n"]
    pub fn dmarc_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dmarc_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_expression` after provisioning.\n"]
    pub fn ip_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_IpExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `number_expression` after provisioning.\n"]
    pub fn number_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_NumberExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.number_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `string_expression` after provisioning.\n"]
    pub fn string_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_StringExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `verdict_expression` after provisioning.\n"]
    pub fn verdict_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verdict_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_Rule_UnlessDynamic {
    boolean_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_BooleanExpression>>,
    dmarc_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_DmarcExpression>>,
    ip_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_IpExpression>>,
    number_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_NumberExpression>>,
    string_expression: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_StringExpression>>,
    verdict_expression: std
    ::option
    ::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless_VerdictExpression>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule_Unless {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    boolean_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_BooleanExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dmarc_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_DmarcExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ip_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_IpExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    number_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_NumberExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    string_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_StringExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verdict_expression: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless_VerdictExpression>>,
    dynamic: MailmanagerRuleSet_Rule_UnlessDynamic,
}

impl MailmanagerRuleSet_Rule_Unless {
    #[doc = "Set the field `boolean_expression`.\n"]
    pub fn set_boolean_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_BooleanExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.boolean_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.boolean_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dmarc_expression`.\n"]
    pub fn set_dmarc_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_DmarcExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dmarc_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dmarc_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ip_expression`.\n"]
    pub fn set_ip_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_IpExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ip_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ip_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `number_expression`.\n"]
    pub fn set_number_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_NumberExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.number_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.number_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `string_expression`.\n"]
    pub fn set_string_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_StringExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.string_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.string_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verdict_expression`.\n"]
    pub fn set_verdict_expression(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless_VerdictExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verdict_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verdict_expression = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule_Unless {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule_Unless {}

impl BuildMailmanagerRuleSet_Rule_Unless {
    pub fn build(self) -> MailmanagerRuleSet_Rule_Unless {
        MailmanagerRuleSet_Rule_Unless {
            boolean_expression: core::default::Default::default(),
            dmarc_expression: core::default::Default::default(),
            ip_expression: core::default::Default::default(),
            number_expression: core::default::Default::default(),
            string_expression: core::default::Default::default(),
            verdict_expression: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_Rule_UnlessRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_Rule_UnlessRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_Rule_UnlessRef {
        MailmanagerRuleSet_Rule_UnlessRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_Rule_UnlessRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `boolean_expression` after provisioning.\n"]
    pub fn boolean_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_BooleanExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.boolean_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `dmarc_expression` after provisioning.\n"]
    pub fn dmarc_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_DmarcExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dmarc_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `ip_expression` after provisioning.\n"]
    pub fn ip_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_IpExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ip_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `number_expression` after provisioning.\n"]
    pub fn number_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_NumberExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.number_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `string_expression` after provisioning.\n"]
    pub fn string_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_StringExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.string_expression", self.base))
    }

    #[doc = "Get a reference to the value of field `verdict_expression` after provisioning.\n"]
    pub fn verdict_expression(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_Unless_VerdictExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.verdict_expression", self.base))
    }
}

#[derive(Serialize, Default)]
struct MailmanagerRuleSet_RuleDynamic {
    action: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Action>>,
    condition: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Condition>>,
    unless: std::option::Option<terrars::DynamicBlock<MailmanagerRuleSet_Rule_Unless>>,
}

#[derive(Serialize)]
pub struct MailmanagerRuleSet_Rule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Action>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    condition: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Condition>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    unless: std::option::Option<std::vec::Vec<MailmanagerRuleSet_Rule_Unless>>,
    dynamic: MailmanagerRuleSet_RuleDynamic,
}

impl MailmanagerRuleSet_Rule {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `action`.\n"]
    pub fn set_action(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Action>>,
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

    #[doc = "Set the field `condition`.\n"]
    pub fn set_condition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Condition>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.condition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.condition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `unless`.\n"]
    pub fn set_unless(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<MailmanagerRuleSet_Rule_Unless>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.unless = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.unless = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for MailmanagerRuleSet_Rule {
    type O = terrars::BlockAssignable<MailmanagerRuleSet_Rule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildMailmanagerRuleSet_Rule {}

impl BuildMailmanagerRuleSet_Rule {
    pub fn build(self) -> MailmanagerRuleSet_Rule {
        MailmanagerRuleSet_Rule {
            name: core::default::Default::default(),
            action: core::default::Default::default(),
            condition: core::default::Default::default(),
            unless: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct MailmanagerRuleSet_RuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for MailmanagerRuleSet_RuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> MailmanagerRuleSet_RuleRef {
        MailmanagerRuleSet_RuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl MailmanagerRuleSet_RuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_ActionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `condition` after provisioning.\n"]
    pub fn condition(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_ConditionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.condition", self.base))
    }

    #[doc = "Get a reference to the value of field `unless` after provisioning.\n"]
    pub fn unless(&self) -> terrars::ListRef<MailmanagerRuleSet_Rule_UnlessRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.unless", self.base))
    }
}
