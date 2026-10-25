#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct BillingViewData_ {
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
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_views: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    data_filter_expression: std::option::Option<std::vec::Vec<BillingView_DataFilterExpression>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    timeouts: std::option::Option<BillingView_Timeouts>,
    dynamic: BillingViewDynamic,
}

struct BillingView_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BillingViewData_>,
}

#[derive(Clone)]
pub struct BillingView(std::rc::Rc<BillingView_>);

impl BillingView {
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

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().description = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_views`.\n"]
    pub fn set_source_views(
        self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().source_views = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `data_filter_expression`.\n"]
    pub fn set_data_filter_expression(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingView_DataFilterExpression>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().data_filter_expression = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.data_filter_expression = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `timeouts`.\n"]
    pub fn set_timeouts(self, v: impl std::convert::Into<BillingView_Timeouts>) -> Self {
        self.0.data.borrow_mut().timeouts = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `billing_view_type` after provisioning.\n"]
    pub fn billing_view_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `derived_view_count` after provisioning.\n"]
    pub fn derived_view_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.derived_view_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_account_id` after provisioning.\n"]
    pub fn source_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_view_count` after provisioning.\n"]
    pub fn source_view_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_view_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_views` after provisioning.\n"]
    pub fn source_views(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_views", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_definition_last_updated_at` after provisioning.\n"]
    pub fn view_definition_last_updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_definition_last_updated_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_filter_expression` after provisioning.\n"]
    pub fn data_filter_expression(&self) -> terrars::ListRef<BillingView_DataFilterExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_filter_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingView_TimeoutsRef {
        BillingView_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

impl terrars::Referable for BillingView {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BillingView { }

impl terrars::ToListMappable for BillingView {
    type O = terrars::ListRef<BillingViewRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BillingView_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_billing_view".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBillingView {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBillingView {
    pub fn build(self, stack: &mut terrars::Stack) -> BillingView {
        let out = BillingView(std::rc::Rc::new(BillingView_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BillingViewData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                description: core::default::Default::default(),
                name: self.name,
                source_views: core::default::Default::default(),
                tags: core::default::Default::default(),
                data_filter_expression: core::default::Default::default(),
                timeouts: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BillingViewRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingViewRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BillingViewRef {
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

    #[doc = "Get a reference to the value of field `billing_view_type` after provisioning.\n"]
    pub fn billing_view_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.billing_view_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `created_at` after provisioning.\n"]
    pub fn created_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.created_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `derived_view_count` after provisioning.\n"]
    pub fn derived_view_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.derived_view_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `owner_account_id` after provisioning.\n"]
    pub fn owner_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.owner_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_account_id` after provisioning.\n"]
    pub fn source_account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_view_count` after provisioning.\n"]
    pub fn source_view_count(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_view_count", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `source_views` after provisioning.\n"]
    pub fn source_views(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_views", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `updated_at` after provisioning.\n"]
    pub fn updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.updated_at", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `view_definition_last_updated_at` after provisioning.\n"]
    pub fn view_definition_last_updated_at(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.view_definition_last_updated_at", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `data_filter_expression` after provisioning.\n"]
    pub fn data_filter_expression(&self) -> terrars::ListRef<BillingView_DataFilterExpressionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.data_filter_expression", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `timeouts` after provisioning.\n"]
    pub fn timeouts(&self) -> BillingView_TimeoutsRef {
        BillingView_TimeoutsRef::new(self.shared().clone(), format!("{}.timeouts", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BillingViewDynamic {
    data_filter_expression: std::option::Option<terrars::DynamicBlock<BillingView_DataFilterExpression>>,
}

#[derive(Serialize)]
pub struct BillingView_DataFilterExpression_Dimensions {
    key: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BillingView_DataFilterExpression_Dimensions { }

impl terrars::ToListMappable for BillingView_DataFilterExpression_Dimensions {
    type O = terrars::BlockAssignable<BillingView_DataFilterExpression_Dimensions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingView_DataFilterExpression_Dimensions {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildBillingView_DataFilterExpression_Dimensions {
    pub fn build(self) -> BillingView_DataFilterExpression_Dimensions {
        BillingView_DataFilterExpression_Dimensions {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct BillingView_DataFilterExpression_DimensionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingView_DataFilterExpression_DimensionsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BillingView_DataFilterExpression_DimensionsRef {
        BillingView_DataFilterExpression_DimensionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingView_DataFilterExpression_DimensionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.key", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingView_DataFilterExpression_Tags {
    key: terrars::PrimField<std::string::String>,
    values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BillingView_DataFilterExpression_Tags { }

impl terrars::ToListMappable for BillingView_DataFilterExpression_Tags {
    type O = terrars::BlockAssignable<BillingView_DataFilterExpression_Tags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingView_DataFilterExpression_Tags {
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildBillingView_DataFilterExpression_Tags {
    pub fn build(self) -> BillingView_DataFilterExpression_Tags {
        BillingView_DataFilterExpression_Tags {
            key: self.key,
            values: self.values,
        }
    }
}

#[allow(dead_code)]
pub struct BillingView_DataFilterExpression_TagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingView_DataFilterExpression_TagsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingView_DataFilterExpression_TagsRef {
        BillingView_DataFilterExpression_TagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingView_DataFilterExpression_TagsRef {
    #[allow(dead_code)]
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
pub struct BillingView_DataFilterExpression_TimeRange {
    begin_date_inclusive: terrars::PrimField<std::string::String>,
    end_date_inclusive: terrars::PrimField<std::string::String>,
}

impl BillingView_DataFilterExpression_TimeRange { }

impl terrars::ToListMappable for BillingView_DataFilterExpression_TimeRange {
    type O = terrars::BlockAssignable<BillingView_DataFilterExpression_TimeRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingView_DataFilterExpression_TimeRange {
    #[doc = ""]
    pub begin_date_inclusive: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub end_date_inclusive: terrars::PrimField<std::string::String>,
}

impl BuildBillingView_DataFilterExpression_TimeRange {
    pub fn build(self) -> BillingView_DataFilterExpression_TimeRange {
        BillingView_DataFilterExpression_TimeRange {
            begin_date_inclusive: self.begin_date_inclusive,
            end_date_inclusive: self.end_date_inclusive,
        }
    }
}

#[allow(dead_code)]
pub struct BillingView_DataFilterExpression_TimeRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingView_DataFilterExpression_TimeRangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingView_DataFilterExpression_TimeRangeRef {
        BillingView_DataFilterExpression_TimeRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingView_DataFilterExpression_TimeRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `begin_date_inclusive` after provisioning.\n"]
    pub fn begin_date_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.begin_date_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `end_date_inclusive` after provisioning.\n"]
    pub fn end_date_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_date_inclusive", self.base))
    }
}

#[derive(Serialize, Default)]
struct BillingView_DataFilterExpressionDynamic {
    dimensions: std::option::Option<terrars::DynamicBlock<BillingView_DataFilterExpression_Dimensions>>,
    tags: std::option::Option<terrars::DynamicBlock<BillingView_DataFilterExpression_Tags>>,
    time_range: std::option::Option<terrars::DynamicBlock<BillingView_DataFilterExpression_TimeRange>>,
}

#[derive(Serialize)]
pub struct BillingView_DataFilterExpression {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    dimensions: std::option::Option<std::vec::Vec<BillingView_DataFilterExpression_Dimensions>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<std::vec::Vec<BillingView_DataFilterExpression_Tags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    time_range: std::option::Option<std::vec::Vec<BillingView_DataFilterExpression_TimeRange>>,
    dynamic: BillingView_DataFilterExpressionDynamic,
}

impl BillingView_DataFilterExpression {
    #[doc = "Set the field `dimensions`.\n"]
    pub fn set_dimensions(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingView_DataFilterExpression_Dimensions>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.dimensions = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.dimensions = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `tags`.\n"]
    pub fn set_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingView_DataFilterExpression_Tags>>,
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

    #[doc = "Set the field `time_range`.\n"]
    pub fn set_time_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BillingView_DataFilterExpression_TimeRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.time_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.time_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BillingView_DataFilterExpression {
    type O = terrars::BlockAssignable<BillingView_DataFilterExpression>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingView_DataFilterExpression {}

impl BuildBillingView_DataFilterExpression {
    pub fn build(self) -> BillingView_DataFilterExpression {
        BillingView_DataFilterExpression {
            dimensions: core::default::Default::default(),
            tags: core::default::Default::default(),
            time_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingView_DataFilterExpressionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingView_DataFilterExpressionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingView_DataFilterExpressionRef {
        BillingView_DataFilterExpressionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingView_DataFilterExpressionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `dimensions` after provisioning.\n"]
    pub fn dimensions(&self) -> terrars::ListRef<BillingView_DataFilterExpression_DimensionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.dimensions", self.base))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::ListRef<BillingView_DataFilterExpression_TagsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.tags", self.base))
    }

    #[doc = "Get a reference to the value of field `time_range` after provisioning.\n"]
    pub fn time_range(&self) -> terrars::ListRef<BillingView_DataFilterExpression_TimeRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.time_range", self.base))
    }
}

#[derive(Serialize)]
pub struct BillingView_Timeouts {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    create: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    delete: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    update: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BillingView_Timeouts {
    #[doc =
        "Set the field `create`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_create(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.create = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `delete`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn set_delete(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.delete = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `update`.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn set_update(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.update = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BillingView_Timeouts {
    type O = terrars::BlockAssignable<BillingView_Timeouts>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBillingView_Timeouts {}

impl BuildBillingView_Timeouts {
    pub fn build(self) -> BillingView_Timeouts {
        BillingView_Timeouts {
            create: core::default::Default::default(),
            delete: core::default::Default::default(),
            update: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BillingView_TimeoutsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BillingView_TimeoutsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BillingView_TimeoutsRef {
        BillingView_TimeoutsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BillingView_TimeoutsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `create` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn create(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.create", self.base))
    }

    #[doc =
        "Get a reference to the value of field `delete` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs."]
    pub fn delete(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.delete", self.base))
    }

    #[doc =
        "Get a reference to the value of field `update` after provisioning.\nA string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as \"30s\" or \"2h45m\". Valid time units are \"s\" (seconds), \"m\" (minutes), \"h\" (hours)."]
    pub fn update(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.update", self.base))
    }
}
