use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CeAnomalySubscriptionData {
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
    threshold: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subscriber: std::option::Option<std::vec::Vec<CeAnomalySubscriptionSubscriberEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threshold_expression: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionEl>>,
    dynamic: CeAnomalySubscriptionDynamic,
}

struct CeAnomalySubscription_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CeAnomalySubscriptionData>,
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

    #[doc = "Set the field `threshold`.\n"]
    pub fn set_threshold(self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().threshold = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subscriber`.\n"]
    pub fn set_subscriber(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionSubscriberEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionEl>>,
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

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_expression` after provisioning.\n"]
    pub fn threshold_expression(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElRef> {
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
            data: std::cell::RefCell::new(CeAnomalySubscriptionData {
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
                threshold: core::default::Default::default(),
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

    #[doc = "Get a reference to the value of field `threshold` after provisioning.\n"]
    pub fn threshold(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.threshold", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `threshold_expression` after provisioning.\n"]
    pub fn threshold_expression(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.threshold_expression", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionSubscriberEl {
    address: terrars::PrimField<std::string::String>,
    #[serde(rename = "type")]
    type_: terrars::PrimField<std::string::String>,
}

impl CeAnomalySubscriptionSubscriberEl { }

impl terrars::ToListMappable for CeAnomalySubscriptionSubscriberEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionSubscriberEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionSubscriberEl {
    #[doc = ""]
    pub address: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub type_: terrars::PrimField<std::string::String>,
}

impl BuildCeAnomalySubscriptionSubscriberEl {
    pub fn build(self) -> CeAnomalySubscriptionSubscriberEl {
        CeAnomalySubscriptionSubscriberEl {
            address: self.address,
            type_: self.type_,
        }
    }
}

pub struct CeAnomalySubscriptionSubscriberElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionSubscriberElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeAnomalySubscriptionSubscriberElRef {
        CeAnomalySubscriptionSubscriberElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionSubscriberElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElAndElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef {
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
struct CeAnomalySubscriptionThresholdExpressionElAndElDynamic {
    cost_category: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>>,
    dimension: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElAndEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElAndElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionElAndEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndElTagsEl>>,
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElAndEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElAndEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElAndEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElAndEl {
        CeAnomalySubscriptionThresholdExpressionElAndEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElAndElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElAndElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElAndElRef {
        CeAnomalySubscriptionThresholdExpressionElAndElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElAndElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElAndElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElAndElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElAndElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElDimensionEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElDimensionEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElNotElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef {
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
struct CeAnomalySubscriptionThresholdExpressionElNotElDynamic {
    cost_category: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>>,
    dimension: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElNotEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElNotElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionElNotEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotElTagsEl>>,
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElNotEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElNotEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElNotEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElNotEl {
        CeAnomalySubscriptionThresholdExpressionElNotEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElNotElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElNotElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElNotElRef {
        CeAnomalySubscriptionThresholdExpressionElNotElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElNotElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElNotElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElNotElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElNotElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
        CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
        CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef {
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
pub struct CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElOrElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef {
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
struct CeAnomalySubscriptionThresholdExpressionElOrElDynamic {
    cost_category: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElOrEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElOrElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionElOrEl {
    #[doc = "Set the field `cost_category`.\n"]
    pub fn set_cost_category(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrElTagsEl>>,
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElOrEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElOrEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElOrEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElOrEl {
        CeAnomalySubscriptionThresholdExpressionElOrEl {
            cost_category: core::default::Default::default(),
            dimension: core::default::Default::default(),
            tags: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElOrElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElOrElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElOrElRef {
        CeAnomalySubscriptionThresholdExpressionElOrElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElOrElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElOrElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElOrElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElOrElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionElTagsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    match_options: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    values: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
}

impl CeAnomalySubscriptionThresholdExpressionElTagsEl {
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionElTagsEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionElTagsEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionElTagsEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionElTagsEl {
        CeAnomalySubscriptionThresholdExpressionElTagsEl {
            key: core::default::Default::default(),
            match_options: core::default::Default::default(),
            values: core::default::Default::default(),
        }
    }
}

pub struct CeAnomalySubscriptionThresholdExpressionElTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElTagsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CeAnomalySubscriptionThresholdExpressionElTagsElRef {
        CeAnomalySubscriptionThresholdExpressionElTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElTagsElRef {
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
struct CeAnomalySubscriptionThresholdExpressionElDynamic {
    and: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElAndEl>>,
    cost_category: std
    ::option
    ::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>>,
    dimension: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElDimensionEl>>,
    not: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElNotEl>>,
    or: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElOrEl>>,
    tags: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionElTagsEl>>,
}

#[derive(Serialize)]
pub struct CeAnomalySubscriptionThresholdExpressionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    and: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElAndEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cost_category: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimension: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElDimensionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElNotEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    or: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElOrEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<CeAnomalySubscriptionThresholdExpressionElTagsEl>>,
    dynamic: CeAnomalySubscriptionThresholdExpressionElDynamic,
}

impl CeAnomalySubscriptionThresholdExpressionEl {
    #[doc = "Set the field `and`.\n"]
    pub fn set_and(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElAndEl>>,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElCostCategoryEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElDimensionEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElNotEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElOrEl>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionElTagsEl>>,
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

impl terrars::ToListMappable for CeAnomalySubscriptionThresholdExpressionEl {
    type O = terrars::BlockAssignable<CeAnomalySubscriptionThresholdExpressionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCeAnomalySubscriptionThresholdExpressionEl {}

impl BuildCeAnomalySubscriptionThresholdExpressionEl {
    pub fn build(self) -> CeAnomalySubscriptionThresholdExpressionEl {
        CeAnomalySubscriptionThresholdExpressionEl {
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

pub struct CeAnomalySubscriptionThresholdExpressionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CeAnomalySubscriptionThresholdExpressionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CeAnomalySubscriptionThresholdExpressionElRef {
        CeAnomalySubscriptionThresholdExpressionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CeAnomalySubscriptionThresholdExpressionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cost_category` after provisioning.\n"]
    pub fn cost_category(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElCostCategoryElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cost_category", self.base))
    }

    #[doc = "Get a reference to the value of field `dimension` after provisioning.\n"]
    pub fn dimension(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElDimensionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimension", self.base))
    }

    #[doc = "Get a reference to the value of field `not` after provisioning.\n"]
    pub fn not(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElNotElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<CeAnomalySubscriptionThresholdExpressionElTagsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }
}

#[derive(Serialize, Default)]
struct CeAnomalySubscriptionDynamic {
    subscriber: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionSubscriberEl>>,
    threshold_expression: std::option::Option<terrars::DynamicBlock<CeAnomalySubscriptionThresholdExpressionEl>>,
}
