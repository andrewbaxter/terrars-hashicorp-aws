#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct CustomerprofilesDomainData_ {
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
    dead_letter_queue_url: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_encryption_key: std::option::Option<terrars::PrimField<std::string::String>>,
    default_expiration_days: terrars::PrimField<f64>,
    domain_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matching: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_based_matching: std::option::Option<std::vec::Vec<CustomerprofilesDomain_RuleBasedMatching>>,
    dynamic: CustomerprofilesDomainDynamic,
}

struct CustomerprofilesDomain_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<CustomerprofilesDomainData_>,
}

#[derive(Clone)]
pub struct CustomerprofilesDomain(std::rc::Rc<CustomerprofilesDomain_>);

impl CustomerprofilesDomain {
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

    #[doc = "Set the field `dead_letter_queue_url`.\n"]
    pub fn set_dead_letter_queue_url(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().dead_letter_queue_url = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_encryption_key`.\n"]
    pub fn set_default_encryption_key(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().default_encryption_key = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `matching`.\n"]
    pub fn set_matching(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().matching = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.matching = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `rule_based_matching`.\n"]
    pub fn set_rule_based_matching(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().rule_based_matching = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.rule_based_matching = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `dead_letter_queue_url` after provisioning.\n"]
    pub fn dead_letter_queue_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_queue_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_encryption_key` after provisioning.\n"]
    pub fn default_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_expiration_days` after provisioning.\n"]
    pub fn default_expiration_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_expiration_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `matching` after provisioning.\n"]
    pub fn matching(&self) -> terrars::ListRef<CustomerprofilesDomain_MatchingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matching", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_based_matching` after provisioning.\n"]
    pub fn rule_based_matching(&self) -> terrars::ListRef<CustomerprofilesDomain_RuleBasedMatchingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_based_matching", self.extract_ref()))
    }
}

impl terrars::Referable for CustomerprofilesDomain {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for CustomerprofilesDomain { }

impl terrars::ToListMappable for CustomerprofilesDomain {
    type O = terrars::ListRef<CustomerprofilesDomainRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for CustomerprofilesDomain_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_customerprofiles_domain".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildCustomerprofilesDomain {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub default_expiration_days: terrars::PrimField<f64>,
    #[doc = ""]
    pub domain_name: terrars::PrimField<std::string::String>,
}

impl BuildCustomerprofilesDomain {
    pub fn build(self, stack: &mut terrars::Stack) -> CustomerprofilesDomain {
        let out = CustomerprofilesDomain(std::rc::Rc::new(CustomerprofilesDomain_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(CustomerprofilesDomainData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                dead_letter_queue_url: core::default::Default::default(),
                default_encryption_key: core::default::Default::default(),
                default_expiration_days: self.default_expiration_days,
                domain_name: self.domain_name,
                id: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                matching: core::default::Default::default(),
                rule_based_matching: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct CustomerprofilesDomainRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomainRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl CustomerprofilesDomainRef {
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

    #[doc = "Get a reference to the value of field `dead_letter_queue_url` after provisioning.\n"]
    pub fn dead_letter_queue_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.dead_letter_queue_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_encryption_key` after provisioning.\n"]
    pub fn default_encryption_key(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_encryption_key", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_expiration_days` after provisioning.\n"]
    pub fn default_expiration_days(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.default_expiration_days", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `domain_name` after provisioning.\n"]
    pub fn domain_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.domain_name", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `matching` after provisioning.\n"]
    pub fn matching(&self) -> terrars::ListRef<CustomerprofilesDomain_MatchingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matching", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `rule_based_matching` after provisioning.\n"]
    pub fn rule_based_matching(&self) -> terrars::ListRef<CustomerprofilesDomain_RuleBasedMatchingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule_based_matching", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomainDynamic {
    matching: std::option::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching>>,
    rule_based_matching: std::option::Option<terrars::DynamicBlock<CustomerprofilesDomain_RuleBasedMatching>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
    conflict_resolving_model: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
    #[doc = "Set the field `source_name`.\n"]
    pub fn set_source_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.source_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
    #[doc = ""]
    pub conflict_resolving_model: terrars::PrimField<std::string::String>,
}

impl BuildCustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
    pub fn build(self) -> CustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
        CustomerprofilesDomain_RuleBasedMatching_ConflictResolution {
            conflict_resolving_model: self.conflict_resolving_model,
            source_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef {
        CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `conflict_resolving_model` after provisioning.\n"]
    pub fn conflict_resolving_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.conflict_resolving_model", self.base))
    }

    #[doc = "Get a reference to the value of field `source_name` after provisioning.\n"]
    pub fn source_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.source_name", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_Matching_AutoMerging_Consolidation {
    matching_attributes_list: terrars::ListField<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CustomerprofilesDomain_Matching_AutoMerging_Consolidation { }

impl terrars::ToListMappable for CustomerprofilesDomain_Matching_AutoMerging_Consolidation {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_Matching_AutoMerging_Consolidation>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_Matching_AutoMerging_Consolidation {
    #[doc = ""]
    pub matching_attributes_list: terrars::ListField<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl BuildCustomerprofilesDomain_Matching_AutoMerging_Consolidation {
    pub fn build(self) -> CustomerprofilesDomain_Matching_AutoMerging_Consolidation {
        CustomerprofilesDomain_Matching_AutoMerging_Consolidation {
            matching_attributes_list: self.matching_attributes_list,
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_Matching_AutoMerging_ConsolidationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_Matching_AutoMerging_ConsolidationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_Matching_AutoMerging_ConsolidationRef {
        CustomerprofilesDomain_Matching_AutoMerging_ConsolidationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_Matching_AutoMerging_ConsolidationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `matching_attributes_list` after provisioning.\n"]
    pub fn matching_attributes_list(
        &self,
    ) -> terrars::ListRef<terrars::ListRef<terrars::PrimExpr<std::string::String>>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.matching_attributes_list", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomain_Matching_AutoMergingDynamic {
    conflict_resolution: std
    ::option
    ::Option<terrars::DynamicBlock<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>>,
    consolidation: std
    ::option
    ::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching_AutoMerging_Consolidation>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_Matching_AutoMerging {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    min_allowed_confidence_score_for_merging: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conflict_resolution: std::option::Option<std::vec::Vec<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    consolidation: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching_AutoMerging_Consolidation>>,
    dynamic: CustomerprofilesDomain_Matching_AutoMergingDynamic,
}

impl CustomerprofilesDomain_Matching_AutoMerging {
    #[doc = "Set the field `min_allowed_confidence_score_for_merging`.\n"]
    pub fn set_min_allowed_confidence_score_for_merging(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.min_allowed_confidence_score_for_merging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `conflict_resolution`.\n"]
    pub fn set_conflict_resolution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conflict_resolution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conflict_resolution = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `consolidation`.\n"]
    pub fn set_consolidation(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching_AutoMerging_Consolidation>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.consolidation = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.consolidation = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_Matching_AutoMerging {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_Matching_AutoMerging>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_Matching_AutoMerging {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCustomerprofilesDomain_Matching_AutoMerging {
    pub fn build(self) -> CustomerprofilesDomain_Matching_AutoMerging {
        CustomerprofilesDomain_Matching_AutoMerging {
            enabled: self.enabled,
            min_allowed_confidence_score_for_merging: core::default::Default::default(),
            conflict_resolution: core::default::Default::default(),
            consolidation: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_Matching_AutoMergingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_Matching_AutoMergingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_Matching_AutoMergingRef {
        CustomerprofilesDomain_Matching_AutoMergingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_Matching_AutoMergingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `min_allowed_confidence_score_for_merging` after provisioning.\n"]
    pub fn min_allowed_confidence_score_for_merging(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(
            self.shared().clone(),
            format!("{}.min_allowed_confidence_score_for_merging", self.base),
        )
    }

    #[doc = "Get a reference to the value of field `conflict_resolution` after provisioning.\n"]
    pub fn conflict_resolution(
        &self,
    ) -> terrars::ListRef<CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conflict_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `consolidation` after provisioning.\n"]
    pub fn consolidation(&self) -> terrars::ListRef<CustomerprofilesDomain_Matching_AutoMerging_ConsolidationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.consolidation", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
    s3_bucket_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_key_name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
    #[doc = "Set the field `s3_key_name`.\n"]
    pub fn set_s3_key_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.s3_key_name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
    #[doc = ""]
    pub s3_bucket_name: terrars::PrimField<std::string::String>,
}

impl BuildCustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
    pub fn build(self) -> CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
        CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting {
            s3_bucket_name: self.s3_bucket_name,
            s3_key_name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_Matching_ExportingConfig_S3ExportingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_Matching_ExportingConfig_S3ExportingRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_Matching_ExportingConfig_S3ExportingRef {
        CustomerprofilesDomain_Matching_ExportingConfig_S3ExportingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_Matching_ExportingConfig_S3ExportingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_bucket_name` after provisioning.\n"]
    pub fn s3_bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `s3_key_name` after provisioning.\n"]
    pub fn s3_key_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_key_name", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomain_Matching_ExportingConfigDynamic {
    s3_exporting: std
    ::option
    ::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_Matching_ExportingConfig {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    s3_exporting: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting>>,
    dynamic: CustomerprofilesDomain_Matching_ExportingConfigDynamic,
}

impl CustomerprofilesDomain_Matching_ExportingConfig {
    #[doc = "Set the field `s3_exporting`.\n"]
    pub fn set_s3_exporting(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching_ExportingConfig_S3Exporting>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.s3_exporting = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.s3_exporting = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_Matching_ExportingConfig {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_Matching_ExportingConfig>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_Matching_ExportingConfig {}

impl BuildCustomerprofilesDomain_Matching_ExportingConfig {
    pub fn build(self) -> CustomerprofilesDomain_Matching_ExportingConfig {
        CustomerprofilesDomain_Matching_ExportingConfig {
            s3_exporting: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_Matching_ExportingConfigRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_Matching_ExportingConfigRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_Matching_ExportingConfigRef {
        CustomerprofilesDomain_Matching_ExportingConfigRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_Matching_ExportingConfigRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `s3_exporting` after provisioning.\n"]
    pub fn s3_exporting(&self) -> terrars::ListRef<CustomerprofilesDomain_Matching_ExportingConfig_S3ExportingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.s3_exporting", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_Matching_JobSchedule {
    day_of_the_week: terrars::PrimField<std::string::String>,
    time: terrars::PrimField<std::string::String>,
}

impl CustomerprofilesDomain_Matching_JobSchedule { }

impl terrars::ToListMappable for CustomerprofilesDomain_Matching_JobSchedule {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_Matching_JobSchedule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_Matching_JobSchedule {
    #[doc = ""]
    pub day_of_the_week: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub time: terrars::PrimField<std::string::String>,
}

impl BuildCustomerprofilesDomain_Matching_JobSchedule {
    pub fn build(self) -> CustomerprofilesDomain_Matching_JobSchedule {
        CustomerprofilesDomain_Matching_JobSchedule {
            day_of_the_week: self.day_of_the_week,
            time: self.time,
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_Matching_JobScheduleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_Matching_JobScheduleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_Matching_JobScheduleRef {
        CustomerprofilesDomain_Matching_JobScheduleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_Matching_JobScheduleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `day_of_the_week` after provisioning.\n"]
    pub fn day_of_the_week(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.day_of_the_week", self.base))
    }

    #[doc = "Get a reference to the value of field `time` after provisioning.\n"]
    pub fn time(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.time", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomain_MatchingDynamic {
    auto_merging: std::option::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching_AutoMerging>>,
    exporting_config: std::option::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching_ExportingConfig>>,
    job_schedule: std::option::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching_JobSchedule>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_Matching {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    auto_merging: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching_AutoMerging>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exporting_config: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching_ExportingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    job_schedule: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching_JobSchedule>>,
    dynamic: CustomerprofilesDomain_MatchingDynamic,
}

impl CustomerprofilesDomain_Matching {
    #[doc = "Set the field `auto_merging`.\n"]
    pub fn set_auto_merging(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching_AutoMerging>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.auto_merging = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.auto_merging = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exporting_config`.\n"]
    pub fn set_exporting_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching_ExportingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exporting_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exporting_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `job_schedule`.\n"]
    pub fn set_job_schedule(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching_JobSchedule>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.job_schedule = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.job_schedule = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_Matching {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_Matching>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_Matching {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCustomerprofilesDomain_Matching {
    pub fn build(self) -> CustomerprofilesDomain_Matching {
        CustomerprofilesDomain_Matching {
            enabled: self.enabled,
            auto_merging: core::default::Default::default(),
            exporting_config: core::default::Default::default(),
            job_schedule: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_MatchingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_MatchingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerprofilesDomain_MatchingRef {
        CustomerprofilesDomain_MatchingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_MatchingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `auto_merging` after provisioning.\n"]
    pub fn auto_merging(&self) -> terrars::ListRef<CustomerprofilesDomain_Matching_AutoMergingRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.auto_merging", self.base))
    }

    #[doc = "Get a reference to the value of field `exporting_config` after provisioning.\n"]
    pub fn exporting_config(&self) -> terrars::ListRef<CustomerprofilesDomain_Matching_ExportingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exporting_config", self.base))
    }

    #[doc = "Get a reference to the value of field `job_schedule` after provisioning.\n"]
    pub fn job_schedule(&self) -> terrars::ListRef<CustomerprofilesDomain_Matching_JobScheduleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.job_schedule", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    attribute_matching_model: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    email_address: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    phone_number: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
}

impl CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
    #[doc = "Set the field `address`.\n"]
    pub fn set_address(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `email_address`.\n"]
    pub fn set_email_address(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.email_address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `phone_number`.\n"]
    pub fn set_phone_number(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.phone_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
    #[doc = ""]
    pub attribute_matching_model: terrars::PrimField<std::string::String>,
}

impl BuildCustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
    pub fn build(self) -> CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
        CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector {
            address: core::default::Default::default(),
            attribute_matching_model: self.attribute_matching_model,
            email_address: core::default::Default::default(),
            phone_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelectorRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelectorRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelectorRef {
        CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelectorRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelectorRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_matching_model` after provisioning.\n"]
    pub fn attribute_matching_model(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attribute_matching_model", self.base))
    }

    #[doc = "Get a reference to the value of field `email_address` after provisioning.\n"]
    pub fn email_address(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.email_address", self.base))
    }

    #[doc = "Get a reference to the value of field `phone_number` after provisioning.\n"]
    pub fn phone_number(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.phone_number", self.base))
    }
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_RuleBasedMatching_MatchingRules {
    rule: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl CustomerprofilesDomain_RuleBasedMatching_MatchingRules { }

impl terrars::ToListMappable for CustomerprofilesDomain_RuleBasedMatching_MatchingRules {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_MatchingRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_RuleBasedMatching_MatchingRules {
    #[doc = ""]
    pub rule: terrars::ListField<terrars::PrimField<std::string::String>>,
}

impl BuildCustomerprofilesDomain_RuleBasedMatching_MatchingRules {
    pub fn build(self) -> CustomerprofilesDomain_RuleBasedMatching_MatchingRules {
        CustomerprofilesDomain_RuleBasedMatching_MatchingRules { rule: self.rule }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_RuleBasedMatching_MatchingRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_RuleBasedMatching_MatchingRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> CustomerprofilesDomain_RuleBasedMatching_MatchingRulesRef {
        CustomerprofilesDomain_RuleBasedMatching_MatchingRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_RuleBasedMatching_MatchingRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `rule` after provisioning.\n"]
    pub fn rule(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.rule", self.base))
    }
}

#[derive(Serialize, Default)]
struct CustomerprofilesDomain_RuleBasedMatchingDynamic {
    attribute_types_selector: std
    ::option
    ::Option<terrars::DynamicBlock<CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector>>,
    conflict_resolution: std
    ::option
    ::Option<terrars::DynamicBlock<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>>,
    exporting_config: std::option::Option<terrars::DynamicBlock<CustomerprofilesDomain_Matching_ExportingConfig>>,
    matching_rules: std
    ::option
    ::Option<terrars::DynamicBlock<CustomerprofilesDomain_RuleBasedMatching_MatchingRules>>,
}

#[derive(Serialize)]
pub struct CustomerprofilesDomain_RuleBasedMatching {
    enabled: terrars::PrimField<bool>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_allowed_rule_level_for_matching: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    max_allowed_rule_level_for_merging: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    status: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attribute_types_selector: std
    ::option
    ::Option<std::vec::Vec<CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    conflict_resolution: std::option::Option<std::vec::Vec<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exporting_config: std::option::Option<std::vec::Vec<CustomerprofilesDomain_Matching_ExportingConfig>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    matching_rules: std::option::Option<std::vec::Vec<CustomerprofilesDomain_RuleBasedMatching_MatchingRules>>,
    dynamic: CustomerprofilesDomain_RuleBasedMatchingDynamic,
}

impl CustomerprofilesDomain_RuleBasedMatching {
    #[doc = "Set the field `max_allowed_rule_level_for_matching`.\n"]
    pub fn set_max_allowed_rule_level_for_matching(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_allowed_rule_level_for_matching = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `max_allowed_rule_level_for_merging`.\n"]
    pub fn set_max_allowed_rule_level_for_merging(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<f64>>,
    ) -> Self {
        self.max_allowed_rule_level_for_merging = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `status`.\n"]
    pub fn set_status(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.status = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attribute_types_selector`.\n"]
    pub fn set_attribute_types_selector(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelector>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.attribute_types_selector = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.attribute_types_selector = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `conflict_resolution`.\n"]
    pub fn set_conflict_resolution(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_ConflictResolution>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.conflict_resolution = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.conflict_resolution = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exporting_config`.\n"]
    pub fn set_exporting_config(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_Matching_ExportingConfig>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exporting_config = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exporting_config = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `matching_rules`.\n"]
    pub fn set_matching_rules(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching_MatchingRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.matching_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.matching_rules = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for CustomerprofilesDomain_RuleBasedMatching {
    type O = terrars::BlockAssignable<CustomerprofilesDomain_RuleBasedMatching>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildCustomerprofilesDomain_RuleBasedMatching {
    #[doc = ""]
    pub enabled: terrars::PrimField<bool>,
}

impl BuildCustomerprofilesDomain_RuleBasedMatching {
    pub fn build(self) -> CustomerprofilesDomain_RuleBasedMatching {
        CustomerprofilesDomain_RuleBasedMatching {
            enabled: self.enabled,
            max_allowed_rule_level_for_matching: core::default::Default::default(),
            max_allowed_rule_level_for_merging: core::default::Default::default(),
            status: core::default::Default::default(),
            attribute_types_selector: core::default::Default::default(),
            conflict_resolution: core::default::Default::default(),
            exporting_config: core::default::Default::default(),
            matching_rules: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct CustomerprofilesDomain_RuleBasedMatchingRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for CustomerprofilesDomain_RuleBasedMatchingRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> CustomerprofilesDomain_RuleBasedMatchingRef {
        CustomerprofilesDomain_RuleBasedMatchingRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl CustomerprofilesDomain_RuleBasedMatchingRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `enabled` after provisioning.\n"]
    pub fn enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `max_allowed_rule_level_for_matching` after provisioning.\n"]
    pub fn max_allowed_rule_level_for_matching(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_allowed_rule_level_for_matching", self.base))
    }

    #[doc = "Get a reference to the value of field `max_allowed_rule_level_for_merging` after provisioning.\n"]
    pub fn max_allowed_rule_level_for_merging(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_allowed_rule_level_for_merging", self.base))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.base))
    }

    #[doc = "Get a reference to the value of field `attribute_types_selector` after provisioning.\n"]
    pub fn attribute_types_selector(
        &self,
    ) -> terrars::ListRef<CustomerprofilesDomain_RuleBasedMatching_AttributeTypesSelectorRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attribute_types_selector", self.base))
    }

    #[doc = "Get a reference to the value of field `conflict_resolution` after provisioning.\n"]
    pub fn conflict_resolution(
        &self,
    ) -> terrars::ListRef<CustomerprofilesDomain_RuleBasedMatching_ConflictResolutionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.conflict_resolution", self.base))
    }

    #[doc = "Get a reference to the value of field `exporting_config` after provisioning.\n"]
    pub fn exporting_config(&self) -> terrars::ListRef<CustomerprofilesDomain_Matching_ExportingConfigRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.exporting_config", self.base))
    }
}
