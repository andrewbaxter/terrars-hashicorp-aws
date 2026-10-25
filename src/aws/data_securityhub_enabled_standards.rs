#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataSecurityhubEnabledStandardsData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_subscription_arns: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

struct DataSecurityhubEnabledStandards_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataSecurityhubEnabledStandardsData_>,
}

#[derive(Clone)]
pub struct DataSecurityhubEnabledStandards(std::rc::Rc<DataSecurityhubEnabledStandards_>);

impl DataSecurityhubEnabledStandards {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderAws) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
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

    #[doc = "Set the field `standards_subscription_arns`.\n"]
    pub fn set_standards_subscription_arns(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().standards_subscription_arns = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_subscription_arns` after provisioning.\n"]
    pub fn standards_subscription_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standards_subscription_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_subscriptions` after provisioning.\n"]
    pub fn standards_subscriptions(&self) -> terrars::ListRef<DataSecurityhubEnabledStandards_StandardsSubscriptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standards_subscriptions", self.extract_ref()))
    }
}

impl terrars::Referable for DataSecurityhubEnabledStandards {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataSecurityhubEnabledStandards { }

impl terrars::ToListMappable for DataSecurityhubEnabledStandards {
    type O = terrars::ListRef<DataSecurityhubEnabledStandardsRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataSecurityhubEnabledStandards_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_securityhub_enabled_standards".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataSecurityhubEnabledStandards {
    pub tf_id: std::string::String,
}

impl BuildDataSecurityhubEnabledStandards {
    pub fn build(self, stack: &mut terrars::Stack) -> DataSecurityhubEnabledStandards {
        let out = DataSecurityhubEnabledStandards(std::rc::Rc::new(DataSecurityhubEnabledStandards_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataSecurityhubEnabledStandardsData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                region: core::default::Default::default(),
                standards_subscription_arns: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataSecurityhubEnabledStandardsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubEnabledStandardsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataSecurityhubEnabledStandardsRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_subscription_arns` after provisioning.\n"]
    pub fn standards_subscription_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standards_subscription_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `standards_subscriptions` after provisioning.\n"]
    pub fn standards_subscriptions(&self) -> terrars::ListRef<DataSecurityhubEnabledStandards_StandardsSubscriptionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standards_subscriptions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status_reason_code: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {
    #[doc = "Set the field `status_reason_code`.\n"]
    pub fn set_status_reason_code(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.status_reason_code = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {
    type O = terrars::BlockAssignable<DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {}

impl BuildDataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {
    pub fn build(self) -> DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {
        DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason {
            status_reason_code: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReasonRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReasonRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReasonRef {
        DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReasonRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReasonRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `status_reason_code` after provisioning.\n"]
    pub fn status_reason_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_reason_code", self.base))
    }
}

#[derive(Serialize)]
pub struct DataSecurityhubEnabledStandards_StandardsSubscriptions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_controls_updatable: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_inputs: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_status_reason: std
    ::option
    ::Option<terrars::ListField<DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    standards_subscription_arn: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataSecurityhubEnabledStandards_StandardsSubscriptions {
    #[doc = "Set the field `standards_arn`.\n"]
    pub fn set_standards_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standards_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_controls_updatable`.\n"]
    pub fn set_standards_controls_updatable(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.standards_controls_updatable = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_inputs`.\n"]
    pub fn set_standards_inputs(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.standards_inputs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_status`.\n"]
    pub fn set_standards_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.standards_status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_status_reason`.\n"]
    pub fn set_standards_status_reason(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReason>,
                    >,
    ) -> Self {
        self.standards_status_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `standards_subscription_arn`.\n"]
    pub fn set_standards_subscription_arn(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.standards_subscription_arn = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataSecurityhubEnabledStandards_StandardsSubscriptions {
    type O = terrars::BlockAssignable<DataSecurityhubEnabledStandards_StandardsSubscriptions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataSecurityhubEnabledStandards_StandardsSubscriptions {}

impl BuildDataSecurityhubEnabledStandards_StandardsSubscriptions {
    pub fn build(self) -> DataSecurityhubEnabledStandards_StandardsSubscriptions {
        DataSecurityhubEnabledStandards_StandardsSubscriptions {
            standards_arn: core::default::Default::default(),
            standards_controls_updatable: core::default::Default::default(),
            standards_inputs: core::default::Default::default(),
            standards_status: core::default::Default::default(),
            standards_status_reason: core::default::Default::default(),
            standards_subscription_arn: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataSecurityhubEnabledStandards_StandardsSubscriptionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataSecurityhubEnabledStandards_StandardsSubscriptionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataSecurityhubEnabledStandards_StandardsSubscriptionsRef {
        DataSecurityhubEnabledStandards_StandardsSubscriptionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataSecurityhubEnabledStandards_StandardsSubscriptionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `standards_arn` after provisioning.\n"]
    pub fn standards_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_controls_updatable` after provisioning.\n"]
    pub fn standards_controls_updatable(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_controls_updatable", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_inputs` after provisioning.\n"]
    pub fn standards_inputs(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.standards_inputs", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_status` after provisioning.\n"]
    pub fn standards_status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_status", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_status_reason` after provisioning.\n"]
    pub fn standards_status_reason(
        &self,
    ) -> terrars::ListRef<DataSecurityhubEnabledStandards_StandardsSubscriptions_StandardsStatusReasonRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.standards_status_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `standards_subscription_arn` after provisioning.\n"]
    pub fn standards_subscription_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.standards_subscription_arn", self.base))
    }
}
