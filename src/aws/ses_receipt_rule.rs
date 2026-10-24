use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SesReceiptRuleData {
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
    after: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recipients: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    rule_set_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    scan_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tls_policy: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    add_header_action: std::option::Option<std::vec::Vec<SesReceiptRuleAddHeaderActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bounce_action: std::option::Option<std::vec::Vec<SesReceiptRuleBounceActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_action: std::option::Option<std::vec::Vec<SesReceiptRuleLambdaActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_action: std::option::Option<std::vec::Vec<SesReceiptRuleS3ActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sns_action: std::option::Option<std::vec::Vec<SesReceiptRuleSnsActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    stop_action: std::option::Option<std::vec::Vec<SesReceiptRuleStopActionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workmail_action: std::option::Option<std::vec::Vec<SesReceiptRuleWorkmailActionEl>>,
    dynamic: SesReceiptRuleDynamic,
}

struct SesReceiptRule_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SesReceiptRuleData>,
}

#[derive(Clone)]
pub struct SesReceiptRule(std::rc::Rc<SesReceiptRule_>);

impl SesReceiptRule {
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

    #[doc = "Set the field `after`.\n"]
    pub fn set_after(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().after = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `enabled`.\n"]
    pub fn set_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recipients`.\n"]
    pub fn set_recipients(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().recipients = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `scan_enabled`.\n"]
    pub fn set_scan_enabled(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().scan_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `tls_policy`.\n"]
    pub fn set_tls_policy(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().tls_policy = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `add_header_action`.\n"]
    pub fn set_add_header_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleAddHeaderActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().add_header_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.add_header_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `bounce_action`.\n"]
    pub fn set_bounce_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleBounceActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().bounce_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.bounce_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_action`.\n"]
    pub fn set_lambda_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleLambdaActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lambda_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lambda_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `s3_action`.\n"]
    pub fn set_s3_action(self, v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleS3ActionEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().s3_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.s3_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `sns_action`.\n"]
    pub fn set_sns_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleSnsActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().sns_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.sns_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `stop_action`.\n"]
    pub fn set_stop_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleStopActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().stop_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.stop_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workmail_action`.\n"]
    pub fn set_workmail_action(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<SesReceiptRuleWorkmailActionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().workmail_action = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.workmail_action = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `after` after provisioning.\n"]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_set_name` after provisioning.\n"]
    pub fn rule_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_enabled` after provisioning.\n"]
    pub fn scan_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.extract_ref()))
    }
}

impl terrars::Referable for SesReceiptRule {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SesReceiptRule { }

impl terrars::ToListMappable for SesReceiptRule {
    type O = terrars::ListRef<SesReceiptRuleRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SesReceiptRule_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ses_receipt_rule".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSesReceiptRule {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rule_set_name: terrars::PrimField<std::string::String>,
}

impl BuildSesReceiptRule {
    pub fn build(self, stack: &mut terrars::Stack) -> SesReceiptRule {
        let out = SesReceiptRule(std::rc::Rc::new(SesReceiptRule_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SesReceiptRuleData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                after: core::default::Default::default(),
                enabled: core::default::Default::default(),
                id: core::default::Default::default(),
                name: self.name,
                recipients: core::default::Default::default(),
                rule_set_name: self.rule_set_name,
                scan_enabled: core::default::Default::default(),
                tls_policy: core::default::Default::default(),
                add_header_action: core::default::Default::default(),
                bounce_action: core::default::Default::default(),
                lambda_action: core::default::Default::default(),
                s3_action: core::default::Default::default(),
                sns_action: core::default::Default::default(),
                stop_action: core::default::Default::default(),
                workmail_action: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SesReceiptRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SesReceiptRuleRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `after` after provisioning.\n"]
    pub fn after(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.after", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recipients` after provisioning.\n"]
    pub fn recipients(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.recipients", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_set_name` after provisioning.\n"]
    pub fn rule_set_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_set_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `scan_enabled` after provisioning.\n"]
    pub fn scan_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scan_enabled", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tls_policy` after provisioning.\n"]
    pub fn tls_policy(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.tls_policy", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleAddHeaderActionEl {
    header_name: terrars::PrimField<std::string::String>,
    header_value: terrars::PrimField<std::string::String>,
    position: terrars::PrimField<f64>,
}

impl SesReceiptRuleAddHeaderActionEl { }

impl terrars::ToListMappable for SesReceiptRuleAddHeaderActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleAddHeaderActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleAddHeaderActionEl {
    #[doc = ""]
    pub header_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub header_value: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
}

impl BuildSesReceiptRuleAddHeaderActionEl {
    pub fn build(self) -> SesReceiptRuleAddHeaderActionEl {
        SesReceiptRuleAddHeaderActionEl {
            header_name: self.header_name,
            header_value: self.header_value,
            position: self.position,
        }
    }
}

pub struct SesReceiptRuleAddHeaderActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleAddHeaderActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleAddHeaderActionElRef {
        SesReceiptRuleAddHeaderActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleAddHeaderActionElRef {
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

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleBounceActionEl {
    message: terrars::PrimField<std::string::String>,
    position: terrars::PrimField<f64>,
    sender: terrars::PrimField<std::string::String>,
    smtp_reply_code: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesReceiptRuleBounceActionEl {
    #[doc = "Set the field `status_code`.\n"]
    pub fn set_status_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesReceiptRuleBounceActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleBounceActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleBounceActionEl {
    #[doc = ""]
    pub message: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
    #[doc = ""]
    pub sender: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub smtp_reply_code: terrars::PrimField<std::string::String>,
}

impl BuildSesReceiptRuleBounceActionEl {
    pub fn build(self) -> SesReceiptRuleBounceActionEl {
        SesReceiptRuleBounceActionEl {
            message: self.message,
            position: self.position,
            sender: self.sender,
            smtp_reply_code: self.smtp_reply_code,
            status_code: core::default::Default::default(),
            topic_arn: core::default::Default::default(),
        }
    }
}

pub struct SesReceiptRuleBounceActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleBounceActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleBounceActionElRef {
        SesReceiptRuleBounceActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleBounceActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `message` after provisioning.\n"]
    pub fn message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.message", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
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

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleLambdaActionEl {
    function_arn: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    invocation_type: std::option::Option<terrars::PrimField<std::string::String>>,
    position: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesReceiptRuleLambdaActionEl {
    #[doc = "Set the field `invocation_type`.\n"]
    pub fn set_invocation_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.invocation_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesReceiptRuleLambdaActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleLambdaActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleLambdaActionEl {
    #[doc = ""]
    pub function_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
}

impl BuildSesReceiptRuleLambdaActionEl {
    pub fn build(self) -> SesReceiptRuleLambdaActionEl {
        SesReceiptRuleLambdaActionEl {
            function_arn: self.function_arn,
            invocation_type: core::default::Default::default(),
            position: self.position,
            topic_arn: core::default::Default::default(),
        }
    }
}

pub struct SesReceiptRuleLambdaActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleLambdaActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleLambdaActionElRef {
        SesReceiptRuleLambdaActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleLambdaActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `function_arn` after provisioning.\n"]
    pub fn function_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.function_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `invocation_type` after provisioning.\n"]
    pub fn invocation_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.invocation_type", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleS3ActionEl {
    bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    kms_key_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_key_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    position: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesReceiptRuleS3ActionEl {
    #[doc = "Set the field `kms_key_arn`.\n"]
    pub fn set_kms_key_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.kms_key_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_key_prefix`.\n"]
    pub fn set_object_key_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.object_key_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesReceiptRuleS3ActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleS3ActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleS3ActionEl {
    #[doc = ""]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
}

impl BuildSesReceiptRuleS3ActionEl {
    pub fn build(self) -> SesReceiptRuleS3ActionEl {
        SesReceiptRuleS3ActionEl {
            bucket_name: self.bucket_name,
            kms_key_arn: core::default::Default::default(),
            object_key_prefix: core::default::Default::default(),
            position: self.position,
            topic_arn: core::default::Default::default(),
        }
    }
}

pub struct SesReceiptRuleS3ActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleS3ActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleS3ActionElRef {
        SesReceiptRuleS3ActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleS3ActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `kms_key_arn` after provisioning.\n"]
    pub fn kms_key_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.kms_key_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `object_key_prefix` after provisioning.\n"]
    pub fn object_key_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_key_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleSnsActionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    encoding: std::option::Option<terrars::PrimField<std::string::String>>,
    position: terrars::PrimField<f64>,
    topic_arn: terrars::PrimField<std::string::String>,
}

impl SesReceiptRuleSnsActionEl {
    #[doc = "Set the field `encoding`.\n"]
    pub fn set_encoding(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.encoding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesReceiptRuleSnsActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleSnsActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleSnsActionEl {
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
    #[doc = ""]
    pub topic_arn: terrars::PrimField<std::string::String>,
}

impl BuildSesReceiptRuleSnsActionEl {
    pub fn build(self) -> SesReceiptRuleSnsActionEl {
        SesReceiptRuleSnsActionEl {
            encoding: core::default::Default::default(),
            position: self.position,
            topic_arn: self.topic_arn,
        }
    }
}

pub struct SesReceiptRuleSnsActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleSnsActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleSnsActionElRef {
        SesReceiptRuleSnsActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleSnsActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `encoding` after provisioning.\n"]
    pub fn encoding(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.encoding", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleStopActionEl {
    position: terrars::PrimField<f64>,
    scope: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesReceiptRuleStopActionEl {
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesReceiptRuleStopActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleStopActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleStopActionEl {
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
    #[doc = ""]
    pub scope: terrars::PrimField<std::string::String>,
}

impl BuildSesReceiptRuleStopActionEl {
    pub fn build(self) -> SesReceiptRuleStopActionEl {
        SesReceiptRuleStopActionEl {
            position: self.position,
            scope: self.scope,
            topic_arn: core::default::Default::default(),
        }
    }
}

pub struct SesReceiptRuleStopActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleStopActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleStopActionElRef {
        SesReceiptRuleStopActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleStopActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc = "Get a reference to the value of field `scope` after provisioning.\n"]
    pub fn scope(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.scope", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize)]
pub struct SesReceiptRuleWorkmailActionEl {
    organization_arn: terrars::PrimField<std::string::String>,
    position: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    topic_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SesReceiptRuleWorkmailActionEl {
    #[doc = "Set the field `topic_arn`.\n"]
    pub fn set_topic_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.topic_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SesReceiptRuleWorkmailActionEl {
    type O = terrars::BlockAssignable<SesReceiptRuleWorkmailActionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSesReceiptRuleWorkmailActionEl {
    #[doc = ""]
    pub organization_arn: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub position: terrars::PrimField<f64>,
}

impl BuildSesReceiptRuleWorkmailActionEl {
    pub fn build(self) -> SesReceiptRuleWorkmailActionEl {
        SesReceiptRuleWorkmailActionEl {
            organization_arn: self.organization_arn,
            position: self.position,
            topic_arn: core::default::Default::default(),
        }
    }
}

pub struct SesReceiptRuleWorkmailActionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SesReceiptRuleWorkmailActionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SesReceiptRuleWorkmailActionElRef {
        SesReceiptRuleWorkmailActionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SesReceiptRuleWorkmailActionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `organization_arn` after provisioning.\n"]
    pub fn organization_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.organization_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `position` after provisioning.\n"]
    pub fn position(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.position", self.base))
    }

    #[doc = "Get a reference to the value of field `topic_arn` after provisioning.\n"]
    pub fn topic_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.topic_arn", self.base))
    }
}

#[derive(Serialize, Default)]
struct SesReceiptRuleDynamic {
    add_header_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleAddHeaderActionEl>>,
    bounce_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleBounceActionEl>>,
    lambda_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleLambdaActionEl>>,
    s3_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleS3ActionEl>>,
    sns_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleSnsActionEl>>,
    stop_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleStopActionEl>>,
    workmail_action: std::option::Option<terrars::DynamicBlock<SesReceiptRuleWorkmailActionEl>>,
}
