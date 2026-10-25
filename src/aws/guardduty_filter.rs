#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct GuarddutyFilterData_ {
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
    action: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    description: std::option::Option<terrars::PrimField<std::string::String>>,
    detector_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    name: terrars::PrimField<std::string::String>,
    rank: terrars::PrimField<f64>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_criteria: std::option::Option<std::vec::Vec<GuarddutyFilter_FindingCriteria>>,
    dynamic: GuarddutyFilterDynamic,
}

struct GuarddutyFilter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<GuarddutyFilterData_>,
}

#[derive(Clone)]
pub struct GuarddutyFilter(std::rc::Rc<GuarddutyFilter_>);

impl GuarddutyFilter {
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

    #[doc = "Set the field `finding_criteria`.\n"]
    pub fn set_finding_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<GuarddutyFilter_FindingCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().finding_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.finding_criteria = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rank` after provisioning.\n"]
    pub fn rank(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `finding_criteria` after provisioning.\n"]
    pub fn finding_criteria(&self) -> terrars::ListRef<GuarddutyFilter_FindingCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.finding_criteria", self.extract_ref()))
    }
}

impl terrars::Referable for GuarddutyFilter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for GuarddutyFilter { }

impl terrars::ToListMappable for GuarddutyFilter {
    type O = terrars::ListRef<GuarddutyFilterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for GuarddutyFilter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_guardduty_filter".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildGuarddutyFilter {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub detector_id: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub rank: terrars::PrimField<f64>,
}

impl BuildGuarddutyFilter {
    pub fn build(self, stack: &mut terrars::Stack) -> GuarddutyFilter {
        let out = GuarddutyFilter(std::rc::Rc::new(GuarddutyFilter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(GuarddutyFilterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                description: core::default::Default::default(),
                detector_id: self.detector_id,
                id: core::default::Default::default(),
                name: self.name,
                rank: self.rank,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                finding_criteria: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct GuarddutyFilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyFilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl GuarddutyFilterRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `description` after provisioning.\n"]
    pub fn description(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.description", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `detector_id` after provisioning.\n"]
    pub fn detector_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.detector_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rank` after provisioning.\n"]
    pub fn rank(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rank", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `finding_criteria` after provisioning.\n"]
    pub fn finding_criteria(&self) -> terrars::ListRef<GuarddutyFilter_FindingCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.finding_criteria", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyFilterDynamic {
    finding_criteria: std::option::Option<terrars::DynamicBlock<GuarddutyFilter_FindingCriteria>>,
}

#[derive(Serialize)]
pub struct GuarddutyFilter_FindingCriteria_Criterion {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    field: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    greater_than: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    greater_than_or_equal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    less_than: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    less_than_or_equal: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matches: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_equals: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    not_matches: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl GuarddutyFilter_FindingCriteria_Criterion {
    #[doc = "Set the field `equals`.\n"]
    pub fn set_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `greater_than`.\n"]
    pub fn set_greater_than(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.greater_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `greater_than_or_equal`.\n"]
    pub fn set_greater_than_or_equal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.greater_than_or_equal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `less_than`.\n"]
    pub fn set_less_than(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.less_than = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `less_than_or_equal`.\n"]
    pub fn set_less_than_or_equal(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.less_than_or_equal = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `matches`.\n"]
    pub fn set_matches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.matches = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_equals`.\n"]
    pub fn set_not_equals(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_equals = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `not_matches`.\n"]
    pub fn set_not_matches(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.not_matches = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for GuarddutyFilter_FindingCriteria_Criterion {
    type O = terrars::BlockAssignable<GuarddutyFilter_FindingCriteria_Criterion>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyFilter_FindingCriteria_Criterion {
    #[doc = ""]
    pub field: terrars::PrimField<std::string::String>,
}

impl BuildGuarddutyFilter_FindingCriteria_Criterion {
    pub fn build(self) -> GuarddutyFilter_FindingCriteria_Criterion {
        GuarddutyFilter_FindingCriteria_Criterion {
            equals: core::default::Default::default(),
            field: self.field,
            greater_than: core::default::Default::default(),
            greater_than_or_equal: core::default::Default::default(),
            less_than: core::default::Default::default(),
            less_than_or_equal: core::default::Default::default(),
            matches: core::default::Default::default(),
            not_equals: core::default::Default::default(),
            not_matches: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyFilter_FindingCriteria_CriterionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyFilter_FindingCriteria_CriterionRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GuarddutyFilter_FindingCriteria_CriterionRef {
        GuarddutyFilter_FindingCriteria_CriterionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyFilter_FindingCriteria_CriterionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `equals` after provisioning.\n"]
    pub fn equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.equals", self.base))
    }

    #[doc = "Get a reference to the value of field `field` after provisioning.\n"]
    pub fn field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.field", self.base))
    }

    #[doc = "Get a reference to the value of field `greater_than` after provisioning.\n"]
    pub fn greater_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.greater_than", self.base))
    }

    #[doc = "Get a reference to the value of field `greater_than_or_equal` after provisioning.\n"]
    pub fn greater_than_or_equal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.greater_than_or_equal", self.base))
    }

    #[doc = "Get a reference to the value of field `less_than` after provisioning.\n"]
    pub fn less_than(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.less_than", self.base))
    }

    #[doc = "Get a reference to the value of field `less_than_or_equal` after provisioning.\n"]
    pub fn less_than_or_equal(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.less_than_or_equal", self.base))
    }

    #[doc = "Get a reference to the value of field `matches` after provisioning.\n"]
    pub fn matches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matches", self.base))
    }

    #[doc = "Get a reference to the value of field `not_equals` after provisioning.\n"]
    pub fn not_equals(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_equals", self.base))
    }

    #[doc = "Get a reference to the value of field `not_matches` after provisioning.\n"]
    pub fn not_matches(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.not_matches", self.base))
    }
}

#[derive(Serialize, Default)]
struct GuarddutyFilter_FindingCriteriaDynamic {
    criterion: std::option::Option<terrars::DynamicBlock<GuarddutyFilter_FindingCriteria_Criterion>>,
}

#[derive(Serialize)]
pub struct GuarddutyFilter_FindingCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    criterion: std::option::Option<std::vec::Vec<GuarddutyFilter_FindingCriteria_Criterion>>,
    dynamic: GuarddutyFilter_FindingCriteriaDynamic,
}

impl GuarddutyFilter_FindingCriteria {
    #[doc = "Set the field `criterion`.\n"]
    pub fn set_criterion(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<GuarddutyFilter_FindingCriteria_Criterion>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.criterion = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.criterion = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for GuarddutyFilter_FindingCriteria {
    type O = terrars::BlockAssignable<GuarddutyFilter_FindingCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildGuarddutyFilter_FindingCriteria {}

impl BuildGuarddutyFilter_FindingCriteria {
    pub fn build(self) -> GuarddutyFilter_FindingCriteria {
        GuarddutyFilter_FindingCriteria {
            criterion: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct GuarddutyFilter_FindingCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for GuarddutyFilter_FindingCriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> GuarddutyFilter_FindingCriteriaRef {
        GuarddutyFilter_FindingCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl GuarddutyFilter_FindingCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}
