use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct SecurityhubInsightData {
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
    group_by_attribute: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filters: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersEl>>,
    dynamic: SecurityhubInsightDynamic,
}

struct SecurityhubInsight_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<SecurityhubInsightData>,
}

#[derive(Clone)]
pub struct SecurityhubInsight(std::rc::Rc<SecurityhubInsight_>);

impl SecurityhubInsight {
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

    #[doc = "Set the field `filters`.\n"]
    pub fn set_filters(self, v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filters = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filters = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `group_by_attribute` after provisioning.\n"]
    pub fn group_by_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<SecurityhubInsightFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

impl terrars::Referable for SecurityhubInsight {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for SecurityhubInsight { }

impl terrars::ToListMappable for SecurityhubInsight {
    type O = terrars::ListRef<SecurityhubInsightRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for SecurityhubInsight_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_securityhub_insight".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildSecurityhubInsight {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub group_by_attribute: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsight {
    pub fn build(self, stack: &mut terrars::Stack) -> SecurityhubInsight {
        let out = SecurityhubInsight(std::rc::Rc::new(SecurityhubInsight_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(SecurityhubInsightData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                group_by_attribute: self.group_by_attribute,
                id: core::default::Default::default(),
                name: self.name,
                filters: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct SecurityhubInsightRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl SecurityhubInsightRef {
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

    #[doc = "Get a reference to the value of field `group_by_attribute` after provisioning.\n"]
    pub fn group_by_attribute(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.group_by_attribute", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filters` after provisioning.\n"]
    pub fn filters(&self) -> terrars::ListRef<SecurityhubInsightFiltersElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filters", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElAwsAccountIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElAwsAccountIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElAwsAccountIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElAwsAccountIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElAwsAccountIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElAwsAccountIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElAwsAccountIdEl {
        SecurityhubInsightFiltersElAwsAccountIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElAwsAccountIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElAwsAccountIdElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElAwsAccountIdElRef {
        SecurityhubInsightFiltersElAwsAccountIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElAwsAccountIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCompanyNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElCompanyNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElCompanyNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElCompanyNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElCompanyNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElCompanyNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCompanyNameEl {
        SecurityhubInsightFiltersElCompanyNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElCompanyNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElCompanyNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElCompanyNameElRef {
        SecurityhubInsightFiltersElCompanyNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElCompanyNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElComplianceStatusEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElComplianceStatusEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElComplianceStatusEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElComplianceStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElComplianceStatusEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElComplianceStatusEl {
    pub fn build(self) -> SecurityhubInsightFiltersElComplianceStatusEl {
        SecurityhubInsightFiltersElComplianceStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElComplianceStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElComplianceStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElComplianceStatusElRef {
        SecurityhubInsightFiltersElComplianceStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElComplianceStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElConfidenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElConfidenceEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElConfidenceEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElConfidenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElConfidenceEl {}

impl BuildSecurityhubInsightFiltersElConfidenceEl {
    pub fn build(self) -> SecurityhubInsightFiltersElConfidenceEl {
        SecurityhubInsightFiltersElConfidenceEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElConfidenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElConfidenceElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElConfidenceElRef {
        SecurityhubInsightFiltersElConfidenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElConfidenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElCreatedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElCreatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
        SecurityhubInsightFiltersElCreatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElCreatedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElCreatedAtElDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCreatedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElCreatedAtElDynamic,
}

impl SecurityhubInsightFiltersElCreatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElCreatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElCreatedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElCreatedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElCreatedAtEl {}

impl BuildSecurityhubInsightFiltersElCreatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCreatedAtEl {
        SecurityhubInsightFiltersElCreatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElCreatedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElCreatedAtElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElCreatedAtElRef {
        SecurityhubInsightFiltersElCreatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElCreatedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElCreatedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElCriticalityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElCriticalityEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElCriticalityEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElCriticalityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElCriticalityEl {}

impl BuildSecurityhubInsightFiltersElCriticalityEl {
    pub fn build(self) -> SecurityhubInsightFiltersElCriticalityEl {
        SecurityhubInsightFiltersElCriticalityEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElCriticalityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElCriticalityElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElCriticalityElRef {
        SecurityhubInsightFiltersElCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElCriticalityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElDescriptionEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElDescriptionEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElDescriptionEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElDescriptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElDescriptionEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElDescriptionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElDescriptionEl {
        SecurityhubInsightFiltersElDescriptionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElDescriptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElDescriptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElDescriptionElRef {
        SecurityhubInsightFiltersElDescriptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElDescriptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
        SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsConfidenceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
        SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsCriticalityElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElFindingProviderFieldsTypesEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
        SecurityhubInsightFiltersElFindingProviderFieldsTypesEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
        SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFindingProviderFieldsTypesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
        SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
        SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElFirstObservedAtElDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElFirstObservedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElFirstObservedAtElDynamic,
}

impl SecurityhubInsightFiltersElFirstObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElFirstObservedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElFirstObservedAtEl {}

impl BuildSecurityhubInsightFiltersElFirstObservedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElFirstObservedAtEl {
        SecurityhubInsightFiltersElFirstObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElFirstObservedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElFirstObservedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElFirstObservedAtElRef {
        SecurityhubInsightFiltersElFirstObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElFirstObservedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElFirstObservedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElGeneratorIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElGeneratorIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElGeneratorIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElGeneratorIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElGeneratorIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElGeneratorIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElGeneratorIdEl {
        SecurityhubInsightFiltersElGeneratorIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElGeneratorIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElGeneratorIdElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElGeneratorIdElRef {
        SecurityhubInsightFiltersElGeneratorIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElGeneratorIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElIdEl {
        SecurityhubInsightFiltersElIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElIdElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElIdElRef {
        SecurityhubInsightFiltersElIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElKeywordEl {
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElKeywordEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElKeywordEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElKeywordEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElKeywordEl {
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElKeywordEl {
    pub fn build(self) -> SecurityhubInsightFiltersElKeywordEl {
        SecurityhubInsightFiltersElKeywordEl { value: self.value }
    }
}

pub struct SecurityhubInsightFiltersElKeywordElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElKeywordElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElKeywordElRef {
        SecurityhubInsightFiltersElKeywordElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElKeywordElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElLastObservedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
        SecurityhubInsightFiltersElLastObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
        SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElLastObservedAtElDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElLastObservedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElLastObservedAtElDynamic,
}

impl SecurityhubInsightFiltersElLastObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElLastObservedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElLastObservedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElLastObservedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElLastObservedAtEl {}

impl BuildSecurityhubInsightFiltersElLastObservedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElLastObservedAtEl {
        SecurityhubInsightFiltersElLastObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElLastObservedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElLastObservedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElLastObservedAtElRef {
        SecurityhubInsightFiltersElLastObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElLastObservedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElLastObservedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwareNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElMalwareNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElMalwareNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElMalwareNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElMalwareNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElMalwareNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwareNameEl {
        SecurityhubInsightFiltersElMalwareNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElMalwareNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElMalwareNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElMalwareNameElRef {
        SecurityhubInsightFiltersElMalwareNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElMalwareNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwarePathEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElMalwarePathEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElMalwarePathEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElMalwarePathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElMalwarePathEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElMalwarePathEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwarePathEl {
        SecurityhubInsightFiltersElMalwarePathEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElMalwarePathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElMalwarePathElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElMalwarePathElRef {
        SecurityhubInsightFiltersElMalwarePathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElMalwarePathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwareStateEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElMalwareStateEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElMalwareStateEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElMalwareStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElMalwareStateEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElMalwareStateEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwareStateEl {
        SecurityhubInsightFiltersElMalwareStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElMalwareStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElMalwareStateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElMalwareStateElRef {
        SecurityhubInsightFiltersElMalwareStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElMalwareStateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElMalwareTypeEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElMalwareTypeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElMalwareTypeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElMalwareTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElMalwareTypeEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElMalwareTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElMalwareTypeEl {
        SecurityhubInsightFiltersElMalwareTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElMalwareTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElMalwareTypeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElMalwareTypeElRef {
        SecurityhubInsightFiltersElMalwareTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElMalwareTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationDomainEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkDestinationDomainEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkDestinationDomainEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationDomainEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkDestinationDomainEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkDestinationDomainEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationDomainEl {
        SecurityhubInsightFiltersElNetworkDestinationDomainEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
        SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkDestinationDomainElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationIpv4El {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkDestinationIpv4El { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkDestinationIpv4El {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv4El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkDestinationIpv4El {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkDestinationIpv4El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationIpv4El {
        SecurityhubInsightFiltersElNetworkDestinationIpv4El { cidr: self.cidr }
    }
}

pub struct SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
        SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkDestinationIpv4ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationIpv6El {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkDestinationIpv6El { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkDestinationIpv6El {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv6El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkDestinationIpv6El {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkDestinationIpv6El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationIpv6El {
        SecurityhubInsightFiltersElNetworkDestinationIpv6El { cidr: self.cidr }
    }
}

pub struct SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
        SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkDestinationIpv6ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDestinationPortEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElNetworkDestinationPortEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkDestinationPortEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationPortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkDestinationPortEl {}

impl BuildSecurityhubInsightFiltersElNetworkDestinationPortEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDestinationPortEl {
        SecurityhubInsightFiltersElNetworkDestinationPortEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkDestinationPortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkDestinationPortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkDestinationPortElRef {
        SecurityhubInsightFiltersElNetworkDestinationPortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkDestinationPortElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkDirectionEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkDirectionEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkDirectionEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDirectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkDirectionEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkDirectionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkDirectionEl {
        SecurityhubInsightFiltersElNetworkDirectionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkDirectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkDirectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkDirectionElRef {
        SecurityhubInsightFiltersElNetworkDirectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkDirectionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkProtocolEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkProtocolEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkProtocolEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkProtocolEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkProtocolEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkProtocolEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkProtocolEl {
        SecurityhubInsightFiltersElNetworkProtocolEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkProtocolElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkProtocolElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkProtocolElRef {
        SecurityhubInsightFiltersElNetworkProtocolElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkProtocolElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceDomainEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkSourceDomainEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkSourceDomainEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceDomainEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkSourceDomainEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkSourceDomainEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceDomainEl {
        SecurityhubInsightFiltersElNetworkSourceDomainEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkSourceDomainElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkSourceDomainElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkSourceDomainElRef {
        SecurityhubInsightFiltersElNetworkSourceDomainElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkSourceDomainElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceIpv4El {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkSourceIpv4El { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkSourceIpv4El {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv4El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkSourceIpv4El {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkSourceIpv4El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceIpv4El {
        SecurityhubInsightFiltersElNetworkSourceIpv4El { cidr: self.cidr }
    }
}

pub struct SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
        SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkSourceIpv4ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceIpv6El {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkSourceIpv6El { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkSourceIpv6El {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv6El>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkSourceIpv6El {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkSourceIpv6El {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceIpv6El {
        SecurityhubInsightFiltersElNetworkSourceIpv6El { cidr: self.cidr }
    }
}

pub struct SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
        SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkSourceIpv6ElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourceMacEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNetworkSourceMacEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkSourceMacEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceMacEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkSourceMacEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNetworkSourceMacEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourceMacEl {
        SecurityhubInsightFiltersElNetworkSourceMacEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkSourceMacElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkSourceMacElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkSourceMacElRef {
        SecurityhubInsightFiltersElNetworkSourceMacElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkSourceMacElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNetworkSourcePortEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElNetworkSourcePortEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElNetworkSourcePortEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourcePortEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNetworkSourcePortEl {}

impl BuildSecurityhubInsightFiltersElNetworkSourcePortEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNetworkSourcePortEl {
        SecurityhubInsightFiltersElNetworkSourcePortEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElNetworkSourcePortElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNetworkSourcePortElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNetworkSourcePortElRef {
        SecurityhubInsightFiltersElNetworkSourcePortElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNetworkSourcePortElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteTextEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNoteTextEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNoteTextEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNoteTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNoteTextEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNoteTextEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteTextEl {
        SecurityhubInsightFiltersElNoteTextEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNoteTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNoteTextElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElNoteTextElRef {
        SecurityhubInsightFiltersElNoteTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNoteTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
        SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElNoteUpdatedAtElDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteUpdatedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElNoteUpdatedAtElDynamic,
}

impl SecurityhubInsightFiltersElNoteUpdatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElNoteUpdatedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNoteUpdatedAtEl {}

impl BuildSecurityhubInsightFiltersElNoteUpdatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteUpdatedAtEl {
        SecurityhubInsightFiltersElNoteUpdatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElNoteUpdatedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNoteUpdatedAtElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElNoteUpdatedAtElRef {
        SecurityhubInsightFiltersElNoteUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNoteUpdatedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElNoteUpdatedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElNoteUpdatedByEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElNoteUpdatedByEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElNoteUpdatedByEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedByEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElNoteUpdatedByEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElNoteUpdatedByEl {
    pub fn build(self) -> SecurityhubInsightFiltersElNoteUpdatedByEl {
        SecurityhubInsightFiltersElNoteUpdatedByEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElNoteUpdatedByElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElNoteUpdatedByElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElNoteUpdatedByElRef {
        SecurityhubInsightFiltersElNoteUpdatedByElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElNoteUpdatedByElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
        SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
        SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElProcessLaunchedAtElDynamic {
    date_range: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessLaunchedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElProcessLaunchedAtElDynamic,
}

impl SecurityhubInsightFiltersElProcessLaunchedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessLaunchedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessLaunchedAtEl {}

impl BuildSecurityhubInsightFiltersElProcessLaunchedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessLaunchedAtEl {
        SecurityhubInsightFiltersElProcessLaunchedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessLaunchedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessLaunchedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElProcessLaunchedAtElRef {
        SecurityhubInsightFiltersElProcessLaunchedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessLaunchedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElProcessLaunchedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElProcessNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElProcessNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessNameEl {
        SecurityhubInsightFiltersElProcessNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElProcessNameElRef {
        SecurityhubInsightFiltersElProcessNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessParentPidEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElProcessParentPidEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessParentPidEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessParentPidEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessParentPidEl {}

impl BuildSecurityhubInsightFiltersElProcessParentPidEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessParentPidEl {
        SecurityhubInsightFiltersElProcessParentPidEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessParentPidElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessParentPidElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElProcessParentPidElRef {
        SecurityhubInsightFiltersElProcessParentPidElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessParentPidElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessPathEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElProcessPathEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessPathEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessPathEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessPathEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElProcessPathEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessPathEl {
        SecurityhubInsightFiltersElProcessPathEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessPathElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessPathElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElProcessPathElRef {
        SecurityhubInsightFiltersElProcessPathElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessPathElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessPidEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    eq: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gte: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lte: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl SecurityhubInsightFiltersElProcessPidEl {
    #[doc = "Set the field `eq`.\n"]
    pub fn set_eq(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.eq = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gte`.\n"]
    pub fn set_gte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gte = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `lte`.\n"]
    pub fn set_lte(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.lte = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessPidEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessPidEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessPidEl {}

impl BuildSecurityhubInsightFiltersElProcessPidEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessPidEl {
        SecurityhubInsightFiltersElProcessPidEl {
            eq: core::default::Default::default(),
            gte: core::default::Default::default(),
            lte: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessPidElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessPidElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElProcessPidElRef {
        SecurityhubInsightFiltersElProcessPidElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessPidElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `eq` after provisioning.\n"]
    pub fn eq(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.eq", self.base))
    }

    #[doc = "Get a reference to the value of field `gte` after provisioning.\n"]
    pub fn gte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gte", self.base))
    }

    #[doc = "Get a reference to the value of field `lte` after provisioning.\n"]
    pub fn lte(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lte", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
        SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElProcessTerminatedAtElDynamic {
    date_range: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProcessTerminatedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElProcessTerminatedAtElDynamic,
}

impl SecurityhubInsightFiltersElProcessTerminatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElProcessTerminatedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProcessTerminatedAtEl {}

impl BuildSecurityhubInsightFiltersElProcessTerminatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProcessTerminatedAtEl {
        SecurityhubInsightFiltersElProcessTerminatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElProcessTerminatedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProcessTerminatedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElProcessTerminatedAtElRef {
        SecurityhubInsightFiltersElProcessTerminatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProcessTerminatedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElProcessTerminatedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProductArnEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElProductArnEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProductArnEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProductArnEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProductArnEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElProductArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProductArnEl {
        SecurityhubInsightFiltersElProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProductArnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProductArnElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElProductArnElRef {
        SecurityhubInsightFiltersElProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProductArnElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElProductFieldsEl {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElProductFieldsEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProductFieldsEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProductFieldsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProductFieldsEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElProductFieldsEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProductFieldsEl {
        SecurityhubInsightFiltersElProductFieldsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProductFieldsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProductFieldsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElProductFieldsElRef {
        SecurityhubInsightFiltersElProductFieldsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProductFieldsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
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
pub struct SecurityhubInsightFiltersElProductNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElProductNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElProductNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElProductNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElProductNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElProductNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElProductNameEl {
        SecurityhubInsightFiltersElProductNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElProductNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElProductNameElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElProductNameElRef {
        SecurityhubInsightFiltersElProductNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElProductNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRecommendationTextEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElRecommendationTextEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElRecommendationTextEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElRecommendationTextEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElRecommendationTextEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElRecommendationTextEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRecommendationTextEl {
        SecurityhubInsightFiltersElRecommendationTextEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElRecommendationTextElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElRecommendationTextElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElRecommendationTextElRef {
        SecurityhubInsightFiltersElRecommendationTextElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElRecommendationTextElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRecordStateEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElRecordStateEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElRecordStateEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElRecordStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElRecordStateEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElRecordStateEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRecordStateEl {
        SecurityhubInsightFiltersElRecordStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElRecordStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElRecordStateElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElRecordStateElRef {
        SecurityhubInsightFiltersElRecordStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElRecordStateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRelatedFindingsIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElRelatedFindingsIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElRelatedFindingsIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElRelatedFindingsIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElRelatedFindingsIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRelatedFindingsIdEl {
        SecurityhubInsightFiltersElRelatedFindingsIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElRelatedFindingsIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElRelatedFindingsIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElRelatedFindingsIdElRef {
        SecurityhubInsightFiltersElRelatedFindingsIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElRelatedFindingsIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElRelatedFindingsProductArnEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElRelatedFindingsProductArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
        SecurityhubInsightFiltersElRelatedFindingsProductArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
        SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElRelatedFindingsProductArnElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl { cidr: self.cidr }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    cidr: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    #[doc = ""]
    pub cidr: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl { cidr: self.cidr }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDynamic {
    date_range: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDynamic,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> terrars::ListRef<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
        SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDynamic {
    date_range: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDynamic,
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {}

impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> terrars::ListRef<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
        SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
        SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerImageIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceContainerImageIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceContainerImageIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceContainerImageIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceContainerImageIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerImageIdEl {
        SecurityhubInsightFiltersElResourceContainerImageIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceContainerImageIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceContainerImageIdElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceContainerImageIdElRef {
        SecurityhubInsightFiltersElResourceContainerImageIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceContainerImageIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerImageNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceContainerImageNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceContainerImageNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceContainerImageNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceContainerImageNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerImageNameEl {
        SecurityhubInsightFiltersElResourceContainerImageNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceContainerImageNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceContainerImageNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceContainerImageNameElRef {
        SecurityhubInsightFiltersElResourceContainerImageNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceContainerImageNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElDynamic {
    date_range: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElResourceContainerLaunchedAtElDynamic,
}

impl SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtEl {}

impl BuildSecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
        SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceContainerLaunchedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElResourceContainerLaunchedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceContainerNameEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceContainerNameEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceContainerNameEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerNameEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceContainerNameEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceContainerNameEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceContainerNameEl {
        SecurityhubInsightFiltersElResourceContainerNameEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceContainerNameElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceContainerNameElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceContainerNameElRef {
        SecurityhubInsightFiltersElResourceContainerNameElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceContainerNameElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceDetailsOtherEl {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceDetailsOtherEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceDetailsOtherEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceDetailsOtherEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceDetailsOtherEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceDetailsOtherEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceDetailsOtherEl {
        SecurityhubInsightFiltersElResourceDetailsOtherEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceDetailsOtherElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceDetailsOtherElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceDetailsOtherElRef {
        SecurityhubInsightFiltersElResourceDetailsOtherElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceDetailsOtherElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
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
pub struct SecurityhubInsightFiltersElResourceIdEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceIdEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceIdEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceIdEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceIdEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceIdEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceIdEl {
        SecurityhubInsightFiltersElResourceIdEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceIdElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceIdElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElResourceIdElRef {
        SecurityhubInsightFiltersElResourceIdElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceIdElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourcePartitionEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourcePartitionEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourcePartitionEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourcePartitionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourcePartitionEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourcePartitionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourcePartitionEl {
        SecurityhubInsightFiltersElResourcePartitionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourcePartitionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourcePartitionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourcePartitionElRef {
        SecurityhubInsightFiltersElResourcePartitionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourcePartitionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceRegionEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceRegionEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceRegionEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceRegionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceRegionEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceRegionEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceRegionEl {
        SecurityhubInsightFiltersElResourceRegionEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceRegionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceRegionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElResourceRegionElRef {
        SecurityhubInsightFiltersElResourceRegionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceRegionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElResourceTagsEl {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceTagsEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceTagsEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceTagsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceTagsEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceTagsEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceTagsEl {
        SecurityhubInsightFiltersElResourceTagsEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceTagsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceTagsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElResourceTagsElRef {
        SecurityhubInsightFiltersElResourceTagsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceTagsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
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
pub struct SecurityhubInsightFiltersElResourceTypeEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElResourceTypeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElResourceTypeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElResourceTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElResourceTypeEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElResourceTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElResourceTypeEl {
        SecurityhubInsightFiltersElResourceTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElResourceTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElResourceTypeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElResourceTypeElRef {
        SecurityhubInsightFiltersElResourceTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElResourceTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElSeverityLabelEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElSeverityLabelEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElSeverityLabelEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElSeverityLabelEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElSeverityLabelEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElSeverityLabelEl {
    pub fn build(self) -> SecurityhubInsightFiltersElSeverityLabelEl {
        SecurityhubInsightFiltersElSeverityLabelEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElSeverityLabelElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElSeverityLabelElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElSeverityLabelElRef {
        SecurityhubInsightFiltersElSeverityLabelElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElSeverityLabelElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElSourceUrlEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElSourceUrlEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElSourceUrlEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElSourceUrlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElSourceUrlEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElSourceUrlEl {
    pub fn build(self) -> SecurityhubInsightFiltersElSourceUrlEl {
        SecurityhubInsightFiltersElSourceUrlEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElSourceUrlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElSourceUrlElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElSourceUrlElRef {
        SecurityhubInsightFiltersElSourceUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElSourceUrlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorCategoryElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDynamic {
    date_range: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDynamic,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(
        &self,
    ) -> terrars::ListRef<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorValueEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
    pub fn build(self) -> SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
        SecurityhubInsightFiltersElThreatIntelIndicatorValueEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
        SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElThreatIntelIndicatorValueElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElTitleEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElTitleEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElTitleEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElTitleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElTitleEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElTitleEl {
    pub fn build(self) -> SecurityhubInsightFiltersElTitleEl {
        SecurityhubInsightFiltersElTitleEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElTitleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElTitleElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElTitleElRef {
        SecurityhubInsightFiltersElTitleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElTitleElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElTypeEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElTypeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElTypeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElTypeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElTypeEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElTypeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElTypeEl {
        SecurityhubInsightFiltersElTypeEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElTypeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElTypeElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElTypeElRef {
        SecurityhubInsightFiltersElTypeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElTypeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    unit: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<f64>,
}

impl SecurityhubInsightFiltersElUpdatedAtElDateRangeEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    #[doc = ""]
    pub unit: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<f64>,
}

impl BuildSecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
    pub fn build(self) -> SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
        SecurityhubInsightFiltersElUpdatedAtElDateRangeEl {
            unit: self.unit,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
        SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElUpdatedAtElDynamic {
    date_range: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElUpdatedAtEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    date_range: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>>,
    dynamic: SecurityhubInsightFiltersElUpdatedAtElDynamic,
}

impl SecurityhubInsightFiltersElUpdatedAtEl {
    #[doc = "Set the field `end`.\n"]
    pub fn set_end(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start`.\n"]
    pub fn set_start(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `date_range`.\n"]
    pub fn set_date_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElUpdatedAtElDateRangeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.date_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.date_range = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersElUpdatedAtEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElUpdatedAtEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElUpdatedAtEl {}

impl BuildSecurityhubInsightFiltersElUpdatedAtEl {
    pub fn build(self) -> SecurityhubInsightFiltersElUpdatedAtEl {
        SecurityhubInsightFiltersElUpdatedAtEl {
            end: core::default::Default::default(),
            start: core::default::Default::default(),
            date_range: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElUpdatedAtElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElUpdatedAtElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElUpdatedAtElRef {
        SecurityhubInsightFiltersElUpdatedAtElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElUpdatedAtElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end` after provisioning.\n"]
    pub fn end(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end", self.base))
    }

    #[doc = "Get a reference to the value of field `start` after provisioning.\n"]
    pub fn start(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start", self.base))
    }

    #[doc = "Get a reference to the value of field `date_range` after provisioning.\n"]
    pub fn date_range(&self) -> terrars::ListRef<SecurityhubInsightFiltersElUpdatedAtElDateRangeElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.date_range", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElUserDefinedValuesEl {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElUserDefinedValuesEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElUserDefinedValuesEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElUserDefinedValuesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElUserDefinedValuesEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElUserDefinedValuesEl {
    pub fn build(self) -> SecurityhubInsightFiltersElUserDefinedValuesEl {
        SecurityhubInsightFiltersElUserDefinedValuesEl {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElUserDefinedValuesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElUserDefinedValuesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElUserDefinedValuesElRef {
        SecurityhubInsightFiltersElUserDefinedValuesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElUserDefinedValuesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
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
pub struct SecurityhubInsightFiltersElVerificationStateEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElVerificationStateEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElVerificationStateEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElVerificationStateEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElVerificationStateEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElVerificationStateEl {
    pub fn build(self) -> SecurityhubInsightFiltersElVerificationStateEl {
        SecurityhubInsightFiltersElVerificationStateEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElVerificationStateElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElVerificationStateElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElVerificationStateElRef {
        SecurityhubInsightFiltersElVerificationStateElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElVerificationStateElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersElWorkflowStatusEl {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl SecurityhubInsightFiltersElWorkflowStatusEl { }

impl terrars::ToListMappable for SecurityhubInsightFiltersElWorkflowStatusEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersElWorkflowStatusEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersElWorkflowStatusEl {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildSecurityhubInsightFiltersElWorkflowStatusEl {
    pub fn build(self) -> SecurityhubInsightFiltersElWorkflowStatusEl {
        SecurityhubInsightFiltersElWorkflowStatusEl {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

pub struct SecurityhubInsightFiltersElWorkflowStatusElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElWorkflowStatusElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> SecurityhubInsightFiltersElWorkflowStatusElRef {
        SecurityhubInsightFiltersElWorkflowStatusElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElWorkflowStatusElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `comparison` after provisioning.\n"]
    pub fn comparison(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.comparison", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightFiltersElDynamic {
    aws_account_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElAwsAccountIdEl>>,
    company_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElCompanyNameEl>>,
    compliance_status: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElComplianceStatusEl>>,
    confidence: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElConfidenceEl>>,
    created_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElCreatedAtEl>>,
    criticality: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElCriticalityEl>>,
    description: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElDescriptionEl>>,
    finding_provider_fields_confidence: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>>,
    finding_provider_fields_criticality: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>>,
    finding_provider_fields_related_findings_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>>,
    finding_provider_fields_related_findings_product_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl>>,
    finding_provider_fields_severity_label: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>>,
    finding_provider_fields_severity_original: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>>,
    finding_provider_fields_types: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>>,
    first_observed_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElFirstObservedAtEl>>,
    generator_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElGeneratorIdEl>>,
    id: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElIdEl>>,
    keyword: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElKeywordEl>>,
    last_observed_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElLastObservedAtEl>>,
    malware_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElMalwareNameEl>>,
    malware_path: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElMalwarePathEl>>,
    malware_state: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElMalwareStateEl>>,
    malware_type: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElMalwareTypeEl>>,
    network_destination_domain: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationDomainEl>>,
    network_destination_ipv4: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationIpv4El>>,
    network_destination_ipv6: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationIpv6El>>,
    network_destination_port: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkDestinationPortEl>>,
    network_direction: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkDirectionEl>>,
    network_protocol: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkProtocolEl>>,
    network_source_domain: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkSourceDomainEl>>,
    network_source_ipv4: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkSourceIpv4El>>,
    network_source_ipv6: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkSourceIpv6El>>,
    network_source_mac: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkSourceMacEl>>,
    network_source_port: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNetworkSourcePortEl>>,
    note_text: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNoteTextEl>>,
    note_updated_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNoteUpdatedAtEl>>,
    note_updated_by: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElNoteUpdatedByEl>>,
    process_launched_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessLaunchedAtEl>>,
    process_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessNameEl>>,
    process_parent_pid: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessParentPidEl>>,
    process_path: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessPathEl>>,
    process_pid: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessPidEl>>,
    process_terminated_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProcessTerminatedAtEl>>,
    product_arn: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProductArnEl>>,
    product_fields: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProductFieldsEl>>,
    product_name: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElProductNameEl>>,
    recommendation_text: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElRecommendationTextEl>>,
    record_state: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElRecordStateEl>>,
    related_findings_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElRelatedFindingsIdEl>>,
    related_findings_product_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>>,
    resource_aws_ec2_instance_iam_instance_profile_arn: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>>,
    resource_aws_ec2_instance_image_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>>,
    resource_aws_ec2_instance_ipv4_addresses: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>>,
    resource_aws_ec2_instance_ipv6_addresses: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>>,
    resource_aws_ec2_instance_key_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>>,
    resource_aws_ec2_instance_launched_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>>,
    resource_aws_ec2_instance_subnet_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>>,
    resource_aws_ec2_instance_type: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>>,
    resource_aws_ec2_instance_vpc_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>>,
    resource_aws_iam_access_key_created_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>>,
    resource_aws_iam_access_key_status: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>>,
    resource_aws_iam_access_key_user_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>>,
    resource_aws_s3_bucket_owner_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>>,
    resource_aws_s3_bucket_owner_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>>,
    resource_container_image_id: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceContainerImageIdEl>>,
    resource_container_image_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceContainerImageNameEl>>,
    resource_container_launched_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>>,
    resource_container_name: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceContainerNameEl>>,
    resource_details_other: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceDetailsOtherEl>>,
    resource_id: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceIdEl>>,
    resource_partition: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourcePartitionEl>>,
    resource_region: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceRegionEl>>,
    resource_tags: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceTagsEl>>,
    resource_type: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElResourceTypeEl>>,
    severity_label: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElSeverityLabelEl>>,
    source_url: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElSourceUrlEl>>,
    threat_intel_indicator_category: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>>,
    threat_intel_indicator_last_observed_at: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>>,
    threat_intel_indicator_source: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>>,
    threat_intel_indicator_source_url: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>>,
    threat_intel_indicator_type: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>>,
    threat_intel_indicator_value: std
    ::option
    ::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>>,
    title: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElTitleEl>>,
    type_: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElTypeEl>>,
    updated_at: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElUpdatedAtEl>>,
    user_defined_values: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElUserDefinedValuesEl>>,
    verification_state: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElVerificationStateEl>>,
    workflow_status: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersElWorkflowStatusEl>>,
}

#[derive(Serialize)]
pub struct SecurityhubInsightFiltersEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElAwsAccountIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    company_name: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElCompanyNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    compliance_status: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElComplianceStatusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    confidence: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElConfidenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    created_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElCreatedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criticality: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElCriticalityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElDescriptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_confidence: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_criticality: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_related_findings_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_related_findings_product_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_severity_label: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_severity_original: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_provider_fields_types: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_observed_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElFirstObservedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    generator_id: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElGeneratorIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    keyword: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElKeywordEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_observed_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElLastObservedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_name: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElMalwareNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_path: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElMalwarePathEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_state: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElMalwareStateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    malware_type: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElMalwareTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_domain: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkDestinationDomainEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_ipv4: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkDestinationIpv4El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_ipv6: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkDestinationIpv6El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_destination_port: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkDestinationPortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_direction: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkDirectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_protocol: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkProtocolEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_domain: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkSourceDomainEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_ipv4: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkSourceIpv4El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_ipv6: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkSourceIpv6El>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_mac: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkSourceMacEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_source_port: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNetworkSourcePortEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_text: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNoteTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_updated_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNoteUpdatedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    note_updated_by: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElNoteUpdatedByEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_launched_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessLaunchedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_name: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_parent_pid: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessParentPidEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_path: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessPathEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_pid: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessPidEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    process_terminated_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProcessTerminatedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_arn: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProductArnEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_fields: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProductFieldsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    product_name: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElProductNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recommendation_text: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElRecommendationTextEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    record_state: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElRecordStateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings_id: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElRelatedFindingsIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_findings_product_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_iam_instance_profile_arn: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_image_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_ipv4_addresses: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_ipv6_addresses: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_key_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_launched_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_subnet_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_type: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_ec2_instance_vpc_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_iam_access_key_created_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_iam_access_key_status: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_iam_access_key_user_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_s3_bucket_owner_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_aws_s3_bucket_owner_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_image_id: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceContainerImageIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_image_name: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceContainerImageNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_launched_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_container_name: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceContainerNameEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_details_other: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceDetailsOtherEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceIdEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_partition: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourcePartitionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_region: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceRegionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceTagsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElResourceTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity_label: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElSeverityLabelEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_url: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElSourceUrlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_category: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_last_observed_at: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_source: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_source_url: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_type: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    threat_intel_indicator_value: std
    ::option
    ::Option<std::vec::Vec<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElTitleEl>>,
    #[serde(rename = "type", skip_serializing_if = "std::option::Option::is_none")]
    type_: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElTypeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElUpdatedAtEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    user_defined_values: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElUserDefinedValuesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    verification_state: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElVerificationStateEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    workflow_status: std::option::Option<std::vec::Vec<SecurityhubInsightFiltersElWorkflowStatusEl>>,
    dynamic: SecurityhubInsightFiltersElDynamic,
}

impl SecurityhubInsightFiltersEl {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElAwsAccountIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.aws_account_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.aws_account_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `company_name`.\n"]
    pub fn set_company_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElCompanyNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.company_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.company_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `compliance_status`.\n"]
    pub fn set_compliance_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElComplianceStatusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.compliance_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.compliance_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `confidence`.\n"]
    pub fn set_confidence(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElConfidenceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.confidence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.confidence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `created_at`.\n"]
    pub fn set_created_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElCreatedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.created_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.created_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `criticality`.\n"]
    pub fn set_criticality(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElCriticalityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.criticality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.criticality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `description`.\n"]
    pub fn set_description(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElDescriptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.description = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.description = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_confidence`.\n"]
    pub fn set_finding_provider_fields_confidence(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsConfidenceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_confidence = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_confidence = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_criticality`.\n"]
    pub fn set_finding_provider_fields_criticality(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsCriticalityEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_criticality = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_criticality = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_related_findings_id`.\n"]
    pub fn set_finding_provider_fields_related_findings_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsIdEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_related_findings_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_related_findings_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_related_findings_product_arn`.\n"]
    pub fn set_finding_provider_fields_related_findings_product_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsRelatedFindingsProductArnEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_related_findings_product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_related_findings_product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_severity_label`.\n"]
    pub fn set_finding_provider_fields_severity_label(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityLabelEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_severity_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_severity_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_severity_original`.\n"]
    pub fn set_finding_provider_fields_severity_original(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsSeverityOriginalEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_severity_original = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_severity_original = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_provider_fields_types`.\n"]
    pub fn set_finding_provider_fields_types(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElFindingProviderFieldsTypesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_provider_fields_types = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_provider_fields_types = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElFirstObservedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.first_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.first_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `generator_id`.\n"]
    pub fn set_generator_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElGeneratorIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.generator_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.generator_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `keyword`.\n"]
    pub fn set_keyword(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElKeywordEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.keyword = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.keyword = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElLastObservedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.last_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.last_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_name`.\n"]
    pub fn set_malware_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElMalwareNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_path`.\n"]
    pub fn set_malware_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElMalwarePathEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_state`.\n"]
    pub fn set_malware_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElMalwareStateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `malware_type`.\n"]
    pub fn set_malware_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElMalwareTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.malware_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.malware_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_domain`.\n"]
    pub fn set_network_destination_domain(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationDomainEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_ipv4`.\n"]
    pub fn set_network_destination_ipv4(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv4El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_ipv4 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_ipv4 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_ipv6`.\n"]
    pub fn set_network_destination_ipv6(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationIpv6El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_ipv6 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_ipv6 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_destination_port`.\n"]
    pub fn set_network_destination_port(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDestinationPortEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_destination_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_destination_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_direction`.\n"]
    pub fn set_network_direction(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkDirectionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_direction = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_direction = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_protocol`.\n"]
    pub fn set_network_protocol(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkProtocolEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_protocol = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_protocol = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_domain`.\n"]
    pub fn set_network_source_domain(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceDomainEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_domain = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_domain = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_ipv4`.\n"]
    pub fn set_network_source_ipv4(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv4El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_ipv4 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_ipv4 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_ipv6`.\n"]
    pub fn set_network_source_ipv6(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceIpv6El>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_ipv6 = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_ipv6 = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_mac`.\n"]
    pub fn set_network_source_mac(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourceMacEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_mac = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_mac = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `network_source_port`.\n"]
    pub fn set_network_source_port(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNetworkSourcePortEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.network_source_port = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.network_source_port = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_text`.\n"]
    pub fn set_note_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNoteTextEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_updated_at`.\n"]
    pub fn set_note_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_updated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `note_updated_by`.\n"]
    pub fn set_note_updated_by(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElNoteUpdatedByEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.note_updated_by = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.note_updated_by = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_launched_at`.\n"]
    pub fn set_process_launched_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessLaunchedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_launched_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_launched_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_name`.\n"]
    pub fn set_process_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_parent_pid`.\n"]
    pub fn set_process_parent_pid(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessParentPidEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_parent_pid = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_parent_pid = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_path`.\n"]
    pub fn set_process_path(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessPathEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_pid`.\n"]
    pub fn set_process_pid(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessPidEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_pid = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_pid = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `process_terminated_at`.\n"]
    pub fn set_process_terminated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProcessTerminatedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.process_terminated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.process_terminated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_arn`.\n"]
    pub fn set_product_arn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProductArnEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_fields`.\n"]
    pub fn set_product_fields(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProductFieldsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_fields = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_fields = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `product_name`.\n"]
    pub fn set_product_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElProductNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.product_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.product_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `recommendation_text`.\n"]
    pub fn set_recommendation_text(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElRecommendationTextEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.recommendation_text = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.recommendation_text = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `record_state`.\n"]
    pub fn set_record_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElRecordStateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.record_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.record_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings_id`.\n"]
    pub fn set_related_findings_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_findings_product_arn`.\n"]
    pub fn set_related_findings_product_arn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElRelatedFindingsProductArnEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_findings_product_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_findings_product_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_iam_instance_profile_arn`.\n"]
    pub fn set_resource_aws_ec2_instance_iam_instance_profile_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIamInstanceProfileArnEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_iam_instance_profile_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_iam_instance_profile_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_image_id`.\n"]
    pub fn set_resource_aws_ec2_instance_image_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceImageIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_image_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_image_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_ipv4_addresses`.\n"]
    pub fn set_resource_aws_ec2_instance_ipv4_addresses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv4AddressesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_ipv4_addresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_ipv4_addresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_ipv6_addresses`.\n"]
    pub fn set_resource_aws_ec2_instance_ipv6_addresses(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceIpv6AddressesEl>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_ipv6_addresses = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_ipv6_addresses = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_key_name`.\n"]
    pub fn set_resource_aws_ec2_instance_key_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceKeyNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_key_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_key_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_launched_at`.\n"]
    pub fn set_resource_aws_ec2_instance_launched_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceLaunchedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_launched_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_launched_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_subnet_id`.\n"]
    pub fn set_resource_aws_ec2_instance_subnet_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceSubnetIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_subnet_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_subnet_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_type`.\n"]
    pub fn set_resource_aws_ec2_instance_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_ec2_instance_vpc_id`.\n"]
    pub fn set_resource_aws_ec2_instance_vpc_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsEc2InstanceVpcIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_ec2_instance_vpc_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_ec2_instance_vpc_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_iam_access_key_created_at`.\n"]
    pub fn set_resource_aws_iam_access_key_created_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyCreatedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_created_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_created_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_iam_access_key_status`.\n"]
    pub fn set_resource_aws_iam_access_key_status(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyStatusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_iam_access_key_user_name`.\n"]
    pub fn set_resource_aws_iam_access_key_user_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsIamAccessKeyUserNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_iam_access_key_user_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_iam_access_key_user_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_s3_bucket_owner_id`.\n"]
    pub fn set_resource_aws_s3_bucket_owner_id(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_s3_bucket_owner_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_s3_bucket_owner_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_aws_s3_bucket_owner_name`.\n"]
    pub fn set_resource_aws_s3_bucket_owner_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceAwsS3BucketOwnerNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_aws_s3_bucket_owner_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_aws_s3_bucket_owner_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_image_id`.\n"]
    pub fn set_resource_container_image_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_image_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_image_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_image_name`.\n"]
    pub fn set_resource_container_image_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerImageNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_image_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_image_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_launched_at`.\n"]
    pub fn set_resource_container_launched_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerLaunchedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_launched_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_launched_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_container_name`.\n"]
    pub fn set_resource_container_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceContainerNameEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_container_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_container_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_details_other`.\n"]
    pub fn set_resource_details_other(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceDetailsOtherEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_details_other = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_details_other = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceIdEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_partition`.\n"]
    pub fn set_resource_partition(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourcePartitionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_partition = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_partition = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_region`.\n"]
    pub fn set_resource_region(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceRegionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_region = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_region = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceTagsEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElResourceTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.resource_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.resource_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `severity_label`.\n"]
    pub fn set_severity_label(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElSeverityLabelEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.severity_label = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.severity_label = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_url`.\n"]
    pub fn set_source_url(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElSourceUrlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_category`.\n"]
    pub fn set_threat_intel_indicator_category(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorCategoryEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_category = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_category = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_last_observed_at`.\n"]
    pub fn set_threat_intel_indicator_last_observed_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorLastObservedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_last_observed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_last_observed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_source`.\n"]
    pub fn set_threat_intel_indicator_source(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_source_url`.\n"]
    pub fn set_threat_intel_indicator_source_url(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorSourceUrlEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_source_url = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_source_url = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_type`.\n"]
    pub fn set_threat_intel_indicator_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `threat_intel_indicator_value`.\n"]
    pub fn set_threat_intel_indicator_value(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElThreatIntelIndicatorValueEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.threat_intel_indicator_value = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.threat_intel_indicator_value = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElTitleEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.title = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.title = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `type_`.\n"]
    pub fn set_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElTypeEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.type_ = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.type_ = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElUpdatedAtEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.updated_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.updated_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `user_defined_values`.\n"]
    pub fn set_user_defined_values(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElUserDefinedValuesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.user_defined_values = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.user_defined_values = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `verification_state`.\n"]
    pub fn set_verification_state(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElVerificationStateEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.verification_state = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.verification_state = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `workflow_status`.\n"]
    pub fn set_workflow_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<SecurityhubInsightFiltersElWorkflowStatusEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.workflow_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.workflow_status = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for SecurityhubInsightFiltersEl {
    type O = terrars::BlockAssignable<SecurityhubInsightFiltersEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildSecurityhubInsightFiltersEl {}

impl BuildSecurityhubInsightFiltersEl {
    pub fn build(self) -> SecurityhubInsightFiltersEl {
        SecurityhubInsightFiltersEl {
            aws_account_id: core::default::Default::default(),
            company_name: core::default::Default::default(),
            compliance_status: core::default::Default::default(),
            confidence: core::default::Default::default(),
            created_at: core::default::Default::default(),
            criticality: core::default::Default::default(),
            description: core::default::Default::default(),
            finding_provider_fields_confidence: core::default::Default::default(),
            finding_provider_fields_criticality: core::default::Default::default(),
            finding_provider_fields_related_findings_id: core::default::Default::default(),
            finding_provider_fields_related_findings_product_arn: core::default::Default::default(),
            finding_provider_fields_severity_label: core::default::Default::default(),
            finding_provider_fields_severity_original: core::default::Default::default(),
            finding_provider_fields_types: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            generator_id: core::default::Default::default(),
            id: core::default::Default::default(),
            keyword: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            malware_name: core::default::Default::default(),
            malware_path: core::default::Default::default(),
            malware_state: core::default::Default::default(),
            malware_type: core::default::Default::default(),
            network_destination_domain: core::default::Default::default(),
            network_destination_ipv4: core::default::Default::default(),
            network_destination_ipv6: core::default::Default::default(),
            network_destination_port: core::default::Default::default(),
            network_direction: core::default::Default::default(),
            network_protocol: core::default::Default::default(),
            network_source_domain: core::default::Default::default(),
            network_source_ipv4: core::default::Default::default(),
            network_source_ipv6: core::default::Default::default(),
            network_source_mac: core::default::Default::default(),
            network_source_port: core::default::Default::default(),
            note_text: core::default::Default::default(),
            note_updated_at: core::default::Default::default(),
            note_updated_by: core::default::Default::default(),
            process_launched_at: core::default::Default::default(),
            process_name: core::default::Default::default(),
            process_parent_pid: core::default::Default::default(),
            process_path: core::default::Default::default(),
            process_pid: core::default::Default::default(),
            process_terminated_at: core::default::Default::default(),
            product_arn: core::default::Default::default(),
            product_fields: core::default::Default::default(),
            product_name: core::default::Default::default(),
            recommendation_text: core::default::Default::default(),
            record_state: core::default::Default::default(),
            related_findings_id: core::default::Default::default(),
            related_findings_product_arn: core::default::Default::default(),
            resource_aws_ec2_instance_iam_instance_profile_arn: core::default::Default::default(),
            resource_aws_ec2_instance_image_id: core::default::Default::default(),
            resource_aws_ec2_instance_ipv4_addresses: core::default::Default::default(),
            resource_aws_ec2_instance_ipv6_addresses: core::default::Default::default(),
            resource_aws_ec2_instance_key_name: core::default::Default::default(),
            resource_aws_ec2_instance_launched_at: core::default::Default::default(),
            resource_aws_ec2_instance_subnet_id: core::default::Default::default(),
            resource_aws_ec2_instance_type: core::default::Default::default(),
            resource_aws_ec2_instance_vpc_id: core::default::Default::default(),
            resource_aws_iam_access_key_created_at: core::default::Default::default(),
            resource_aws_iam_access_key_status: core::default::Default::default(),
            resource_aws_iam_access_key_user_name: core::default::Default::default(),
            resource_aws_s3_bucket_owner_id: core::default::Default::default(),
            resource_aws_s3_bucket_owner_name: core::default::Default::default(),
            resource_container_image_id: core::default::Default::default(),
            resource_container_image_name: core::default::Default::default(),
            resource_container_launched_at: core::default::Default::default(),
            resource_container_name: core::default::Default::default(),
            resource_details_other: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_partition: core::default::Default::default(),
            resource_region: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity_label: core::default::Default::default(),
            source_url: core::default::Default::default(),
            threat_intel_indicator_category: core::default::Default::default(),
            threat_intel_indicator_last_observed_at: core::default::Default::default(),
            threat_intel_indicator_source: core::default::Default::default(),
            threat_intel_indicator_source_url: core::default::Default::default(),
            threat_intel_indicator_type: core::default::Default::default(),
            threat_intel_indicator_value: core::default::Default::default(),
            title: core::default::Default::default(),
            type_: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            user_defined_values: core::default::Default::default(),
            verification_state: core::default::Default::default(),
            workflow_status: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct SecurityhubInsightFiltersElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for SecurityhubInsightFiltersElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> SecurityhubInsightFiltersElRef {
        SecurityhubInsightFiltersElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl SecurityhubInsightFiltersElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}

#[derive(Serialize, Default)]
struct SecurityhubInsightDynamic {
    filters: std::option::Option<terrars::DynamicBlock<SecurityhubInsightFiltersEl>>,
}
