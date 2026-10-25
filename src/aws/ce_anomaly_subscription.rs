#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CeAnomalySubscriptionData_ {
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
    account_id: std::option::Option<terrars::PrimField<std::string::String>>,
    frequency: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    monitor_arn_list: terrars::ListField<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber: std::option::Option<std::vec::Vec<CeAnomalySubscription_Subscriber>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_expression: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression>>,
    dynamic: CeAnomalySubscriptionDynamic,
}

struct CeAnomalySubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CeAnomalySubscriptionData_>,
}

#[derive(Clone)]
pub struct CeAnomalySubscription(std::rc::Rc<CeAnomalySubscription_>);

impl CeAnomalySubscription {
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

    #[doc = "Set the field `account_id`.\n"]
    pub fn set_account_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().account_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `subscriber`.\n"]
    pub fn set_subscriber(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_Subscriber>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().subscriber = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.subscriber = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threshold_expression`.\n"]
    pub fn set_threshold_expression(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().threshold_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.threshold_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_arn_list` after provisioning.\n"]
    pub fn monitor_arn_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitor_arn_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_expression` after provisioning.\n"]
    pub fn threshold_expression(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_expression", self.extract_ref()))
    }
}

impl terrars::Referable for CeAnomalySubscription {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CeAnomalySubscription { }

impl terrars::ToListMappable for CeAnomalySubscription {
    type O = terrars::ListRef<CeAnomalySubscriptionRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CeAnomalySubscription_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ce_anomaly_subscription".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCeAnomalySubscription {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub frequency: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub monitor_arn_list: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildCeAnomalySubscription {
    pub fn build(self, stack: &mut terrars::Stack) -> CeAnomalySubscription {
        let out = CeAnomalySubscription(std::rc::Rc::new(CeAnomalySubscription_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CeAnomalySubscriptionData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                account_id: core::default::Default::default(),
                frequency: self.frequency,
                id: core::default::Default::default(),
                monitor_arn_list: self.monitor_arn_list,
                name: self.name,
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                subscriber: core::default::Default::default(),
                threshold_expression: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CeAnomalySubscriptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CeAnomalySubscriptionRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\n"]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `frequency` after provisioning.\n"]
    pub fn frequency(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.frequency", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `monitor_arn_list` after provisioning.\n"]
    pub fn monitor_arn_list(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.monitor_arn_list", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_expression` after provisioning.\n"]
    pub fn threshold_expression(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_expression", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionDynamic {
    subscriber: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_Subscriber>>,
    threshold_expression: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_Subscriber {
    address: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CeAnomalySubscription_Subscriber { }

impl terrars::ToListMappable for CeAnomalySubscription_Subscriber {
    type O = terrars::BlockAssignable<CeAnomalySubscription_Subscriber>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_Subscriber {
    #[doc = ""]
    pub address: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCeAnomalySubscription_Subscriber {
    pub fn build(self) -> CeAnomalySubscription_Subscriber {
        CeAnomalySubscription_Subscriber {
            address: self.address,
            type_: self.type_,
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_SubscriberRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_SubscriberRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeAnomalySubscription_SubscriberRef {
        CeAnomalySubscription_SubscriberRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_SubscriberRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `type_` after provisioning.\n"]
    pub fn type_(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.type", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression_CostCategory {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscription_ThresholdExpression_CostCategory {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression_CostCategory {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_CostCategory>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression_CostCategory {}

impl BuildCeAnomalySubscription_ThresholdExpression_CostCategory {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression_CostCategory {
        CeAnomalySubscription_ThresholdExpression_CostCategory {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpression_CostCategoryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpression_CostCategoryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscription_ThresholdExpression_CostCategoryRef {
        CeAnomalySubscription_ThresholdExpression_CostCategoryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpression_CostCategoryRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression_Dimension {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscription_ThresholdExpression_Dimension {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression_Dimension {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Dimension>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression_Dimension {}

impl BuildCeAnomalySubscription_ThresholdExpression_Dimension {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression_Dimension {
        CeAnomalySubscription_ThresholdExpression_Dimension {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpression_DimensionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpression_DimensionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscription_ThresholdExpression_DimensionRef {
        CeAnomalySubscription_ThresholdExpression_DimensionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpression_DimensionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression_Tags {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscription_ThresholdExpression_Tags {
    #[doc = "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `match_options`.\n"]
    pub fn set_match_options(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.match_options = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `values`.\n"]
    pub fn set_values(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.values = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression_Tags {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression_Tags {}

impl BuildCeAnomalySubscription_ThresholdExpression_Tags {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression_Tags {
        CeAnomalySubscription_ThresholdExpression_Tags {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpression_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpression_TagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscription_ThresholdExpression_TagsRef {
        CeAnomalySubscription_ThresholdExpression_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpression_TagsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `match_options` after provisioning.\n"]
    pub fn match_options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.match_options", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscription_ThresholdExpression_AndDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Tags>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression_And {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Tags>>,
    dynamic: CeAnomalySubscription_ThresholdExpression_AndDynamic,
}

impl CeAnomalySubscription_ThresholdExpression_And {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression_And {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_And>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression_And {}

impl BuildCeAnomalySubscription_ThresholdExpression_And {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression_And {
        CeAnomalySubscription_ThresholdExpression_And {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpression_AndRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpression_AndRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscription_ThresholdExpression_AndRef {
        CeAnomalySubscription_ThresholdExpression_AndRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpression_AndRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscription_ThresholdExpression_NotDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Tags>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression_Not {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Tags>>,
    dynamic: CeAnomalySubscription_ThresholdExpression_NotDynamic,
}

impl CeAnomalySubscription_ThresholdExpression_Not {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression_Not {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Not>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression_Not {}

impl BuildCeAnomalySubscription_ThresholdExpression_Not {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression_Not {
        CeAnomalySubscription_ThresholdExpression_Not {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpression_NotRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpression_NotRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscription_ThresholdExpression_NotRef {
        CeAnomalySubscription_ThresholdExpression_NotRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpression_NotRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscription_ThresholdExpression_OrDynamic {
    cost_category: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Tags>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression_Or {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Tags>>,
    dynamic: CeAnomalySubscription_ThresholdExpression_OrDynamic,
}

impl CeAnomalySubscription_ThresholdExpression_Or {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression_Or {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Or>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression_Or {}

impl BuildCeAnomalySubscription_ThresholdExpression_Or {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression_Or {
        CeAnomalySubscription_ThresholdExpression_Or {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpression_OrRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpression_OrRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscription_ThresholdExpression_OrRef {
        CeAnomalySubscription_ThresholdExpression_OrRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpression_OrRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscription_ThresholdExpressionDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_And>>,
    cost_category: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    not: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Not>>,
    or: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Or>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscription_ThresholdExpression_Tags>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscription_ThresholdExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_And>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Not>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Or>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscription_ThresholdExpression_Tags>>,
    dynamic: CeAnomalySubscription_ThresholdExpressionDynamic,
}

impl CeAnomalySubscription_ThresholdExpression {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_And>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.and = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.and = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_CostCategory>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.cost_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.cost_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `dimension`.\n"]
    pub fn set_dimension(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Dimension>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimension = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimension = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `not`.\n"]
    pub fn set_not(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Not>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.not = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.not = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `or`.\n"]
    pub fn set_or(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Or>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.or = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.or = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression_Tags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.tags = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CeAnomalySubscription_ThresholdExpression {
    type O = terrars::BlockAssignable<CeAnomalySubscription_ThresholdExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscription_ThresholdExpression {}

impl BuildCeAnomalySubscription_ThresholdExpression {
    pub fn build(self) -> CeAnomalySubscription_ThresholdExpression {
        CeAnomalySubscription_ThresholdExpression {
            and: core::default::Default::default(),
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            not: core::default::Default::default(),
            or: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CeAnomalySubscription_ThresholdExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscription_ThresholdExpressionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeAnomalySubscription_ThresholdExpressionRef {
        CeAnomalySubscription_ThresholdExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscription_ThresholdExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_CostCategoryRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_DimensionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_NotRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscription_ThresholdExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}
