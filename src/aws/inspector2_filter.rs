#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Inspector2FilterData_ {
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
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter_criteria: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria>>,
    dynamic: Inspector2FilterDynamic,
}

struct Inspector2Filter_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Inspector2FilterData_>,
}

#[derive(Clone)]
pub struct Inspector2Filter(std::rc::Rc<Inspector2Filter_>);

impl Inspector2Filter {
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

    #[doc = "Set the field `reason`.\n"]
    pub fn set_reason(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().reason = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `filter_criteria`.\n"]
    pub fn set_filter_criteria(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter_criteria = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter_criteria = std::option::Option::Some(d);
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }
}

impl terrars::Referable for Inspector2Filter {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Inspector2Filter { }

impl terrars::ToListMappable for Inspector2Filter {
    type O = terrars::ListRef<Inspector2FilterRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Inspector2Filter_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_inspector2_filter".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildInspector2Filter {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub action: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter {
    pub fn build(self, stack: &mut terrars::Stack) -> Inspector2Filter {
        let out = Inspector2Filter(std::rc::Rc::new(Inspector2Filter_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Inspector2FilterData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                action: self.action,
                description: core::default::Default::default(),
                name: self.name,
                reason: core::default::Default::default(),
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter_criteria: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Inspector2FilterRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2FilterRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Inspector2FilterRef {
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

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `reason` after provisioning.\n"]
    pub fn reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.reason", self.extract_ref()))
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

    #[doc = "Get a reference to the value of field `filter_criteria` after provisioning.\n"]
    pub fn filter_criteria(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteriaRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_criteria", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct Inspector2FilterDynamic {
    filter_criteria: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria>>,
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_AwsAccountId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_AwsAccountId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_AwsAccountId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_AwsAccountId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_AwsAccountId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_AwsAccountId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_AwsAccountId {
        Inspector2Filter_FilterCriteria_AwsAccountId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_AwsAccountIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_AwsAccountIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_AwsAccountIdRef {
        Inspector2Filter_FilterCriteria_AwsAccountIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_AwsAccountIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_CodeRepositoryProjectName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_CodeRepositoryProjectName { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_CodeRepositoryProjectName {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeRepositoryProjectName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_CodeRepositoryProjectName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_CodeRepositoryProjectName {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_CodeRepositoryProjectName {
        Inspector2Filter_FilterCriteria_CodeRepositoryProjectName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_CodeRepositoryProjectNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_CodeRepositoryProjectNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_CodeRepositoryProjectNameRef {
        Inspector2Filter_FilterCriteria_CodeRepositoryProjectNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_CodeRepositoryProjectNameRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_CodeRepositoryProviderType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_CodeRepositoryProviderType { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_CodeRepositoryProviderType {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeRepositoryProviderType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_CodeRepositoryProviderType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_CodeRepositoryProviderType {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_CodeRepositoryProviderType {
        Inspector2Filter_FilterCriteria_CodeRepositoryProviderType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_CodeRepositoryProviderTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_CodeRepositoryProviderTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_CodeRepositoryProviderTypeRef {
        Inspector2Filter_FilterCriteria_CodeRepositoryProviderTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_CodeRepositoryProviderTypeRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName {
        Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorNameRef {
        Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorNameRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags {
        Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTagsRef {
        Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTagsRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_CodeVulnerabilityFilePath {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_CodeVulnerabilityFilePath {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath {
        Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePathRef {
        Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePathRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_ComponentId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_ComponentId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_ComponentId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ComponentId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_ComponentId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_ComponentId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_ComponentId {
        Inspector2Filter_FilterCriteria_ComponentId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_ComponentIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_ComponentIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_ComponentIdRef {
        Inspector2Filter_FilterCriteria_ComponentIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_ComponentIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_ComponentType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_ComponentType { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_ComponentType {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ComponentType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_ComponentType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_ComponentType {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_ComponentType {
        Inspector2Filter_FilterCriteria_ComponentType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_ComponentTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_ComponentTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_ComponentTypeRef {
        Inspector2Filter_FilterCriteria_ComponentTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_ComponentTypeRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_Ec2InstanceImageId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_Ec2InstanceImageId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_Ec2InstanceImageId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Ec2InstanceImageId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_Ec2InstanceImageId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_Ec2InstanceImageId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_Ec2InstanceImageId {
        Inspector2Filter_FilterCriteria_Ec2InstanceImageId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_Ec2InstanceImageIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_Ec2InstanceImageIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_Ec2InstanceImageIdRef {
        Inspector2Filter_FilterCriteria_Ec2InstanceImageIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_Ec2InstanceImageIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_Ec2InstanceSubnetId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_Ec2InstanceSubnetId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId {
        Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_Ec2InstanceSubnetIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_Ec2InstanceSubnetIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_Ec2InstanceSubnetIdRef {
        Inspector2Filter_FilterCriteria_Ec2InstanceSubnetIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_Ec2InstanceSubnetIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_Ec2InstanceVpcId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_Ec2InstanceVpcId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_Ec2InstanceVpcId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Ec2InstanceVpcId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_Ec2InstanceVpcId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_Ec2InstanceVpcId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_Ec2InstanceVpcId {
        Inspector2Filter_FilterCriteria_Ec2InstanceVpcId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_Ec2InstanceVpcIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_Ec2InstanceVpcIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_Ec2InstanceVpcIdRef {
        Inspector2Filter_FilterCriteria_Ec2InstanceVpcIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_Ec2InstanceVpcIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_EcrImageArchitecture {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_EcrImageArchitecture { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageArchitecture {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageArchitecture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageArchitecture {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_EcrImageArchitecture {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageArchitecture {
        Inspector2Filter_FilterCriteria_EcrImageArchitecture {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageArchitectureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageArchitectureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageArchitectureRef {
        Inspector2Filter_FilterCriteria_EcrImageArchitectureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageArchitectureRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_EcrImageHash {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_EcrImageHash { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageHash {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageHash>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageHash {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_EcrImageHash {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageHash {
        Inspector2Filter_FilterCriteria_EcrImageHash {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageHashRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageHashRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageHashRef {
        Inspector2Filter_FilterCriteria_EcrImageHashRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageHashRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_EcrImageInUseCount {
    lower_inclusive: terrars::PrimField<f64>,
    upper_inclusive: terrars::PrimField<f64>,
}

impl Inspector2Filter_FilterCriteria_EcrImageInUseCount { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageInUseCount {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageInUseCount>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageInUseCount {
    #[doc = ""]
    pub lower_inclusive: terrars::PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: terrars::PrimField<f64>,
}

impl BuildInspector2Filter_FilterCriteria_EcrImageInUseCount {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageInUseCount {
        Inspector2Filter_FilterCriteria_EcrImageInUseCount {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageInUseCountRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageInUseCountRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageInUseCountRef {
        Inspector2Filter_FilterCriteria_EcrImageInUseCountRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageInUseCountRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_EcrImageLastInUseAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2Filter_FilterCriteria_EcrImageLastInUseAt {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_inclusive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageLastInUseAt {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageLastInUseAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageLastInUseAt {}

impl BuildInspector2Filter_FilterCriteria_EcrImageLastInUseAt {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageLastInUseAt {
        Inspector2Filter_FilterCriteria_EcrImageLastInUseAt {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageLastInUseAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageLastInUseAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageLastInUseAtRef {
        Inspector2Filter_FilterCriteria_EcrImageLastInUseAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageLastInUseAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_EcrImagePushedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2Filter_FilterCriteria_EcrImagePushedAt {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_inclusive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImagePushedAt {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImagePushedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImagePushedAt {}

impl BuildInspector2Filter_FilterCriteria_EcrImagePushedAt {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImagePushedAt {
        Inspector2Filter_FilterCriteria_EcrImagePushedAt {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImagePushedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImagePushedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImagePushedAtRef {
        Inspector2Filter_FilterCriteria_EcrImagePushedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImagePushedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_EcrImageRegistry {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_EcrImageRegistry { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageRegistry {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageRegistry>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageRegistry {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_EcrImageRegistry {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageRegistry {
        Inspector2Filter_FilterCriteria_EcrImageRegistry {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageRegistryRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageRegistryRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageRegistryRef {
        Inspector2Filter_FilterCriteria_EcrImageRegistryRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageRegistryRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_EcrImageRepositoryName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_EcrImageRepositoryName { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageRepositoryName {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageRepositoryName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageRepositoryName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_EcrImageRepositoryName {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageRepositoryName {
        Inspector2Filter_FilterCriteria_EcrImageRepositoryName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageRepositoryNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageRepositoryNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageRepositoryNameRef {
        Inspector2Filter_FilterCriteria_EcrImageRepositoryNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageRepositoryNameRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_EcrImageTags {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_EcrImageTags { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EcrImageTags {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EcrImageTags {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_EcrImageTags {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EcrImageTags {
        Inspector2Filter_FilterCriteria_EcrImageTags {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EcrImageTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EcrImageTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_EcrImageTagsRef {
        Inspector2Filter_FilterCriteria_EcrImageTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EcrImageTagsRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_EpssScore {
    lower_inclusive: terrars::PrimField<f64>,
    upper_inclusive: terrars::PrimField<f64>,
}

impl Inspector2Filter_FilterCriteria_EpssScore { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_EpssScore {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EpssScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_EpssScore {
    #[doc = ""]
    pub lower_inclusive: terrars::PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: terrars::PrimField<f64>,
}

impl BuildInspector2Filter_FilterCriteria_EpssScore {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_EpssScore {
        Inspector2Filter_FilterCriteria_EpssScore {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_EpssScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_EpssScoreRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_EpssScoreRef {
        Inspector2Filter_FilterCriteria_EpssScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_EpssScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_ExploitAvailable {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_ExploitAvailable { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_ExploitAvailable {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ExploitAvailable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_ExploitAvailable {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_ExploitAvailable {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_ExploitAvailable {
        Inspector2Filter_FilterCriteria_ExploitAvailable {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_ExploitAvailableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_ExploitAvailableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_ExploitAvailableRef {
        Inspector2Filter_FilterCriteria_ExploitAvailableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_ExploitAvailableRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_FindingArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_FindingArn { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_FindingArn {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FindingArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_FindingArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_FindingArn {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_FindingArn {
        Inspector2Filter_FilterCriteria_FindingArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_FindingArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_FindingArnRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_FindingArnRef {
        Inspector2Filter_FilterCriteria_FindingArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_FindingArnRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_FindingStatus {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_FindingStatus { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_FindingStatus {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FindingStatus>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_FindingStatus {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_FindingStatus {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_FindingStatus {
        Inspector2Filter_FilterCriteria_FindingStatus {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_FindingStatusRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_FindingStatusRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_FindingStatusRef {
        Inspector2Filter_FilterCriteria_FindingStatusRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_FindingStatusRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_FindingType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_FindingType { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_FindingType {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FindingType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_FindingType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_FindingType {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_FindingType {
        Inspector2Filter_FilterCriteria_FindingType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_FindingTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_FindingTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_FindingTypeRef {
        Inspector2Filter_FilterCriteria_FindingTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_FindingTypeRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_FirstObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2Filter_FilterCriteria_FirstObservedAt {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_inclusive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_FirstObservedAt {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FirstObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_FirstObservedAt {}

impl BuildInspector2Filter_FilterCriteria_FirstObservedAt {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_FirstObservedAt {
        Inspector2Filter_FilterCriteria_FirstObservedAt {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_FirstObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_FirstObservedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_FirstObservedAtRef {
        Inspector2Filter_FilterCriteria_FirstObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_FirstObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_FixAvailable {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_FixAvailable { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_FixAvailable {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FixAvailable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_FixAvailable {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_FixAvailable {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_FixAvailable {
        Inspector2Filter_FilterCriteria_FixAvailable {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_FixAvailableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_FixAvailableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_FixAvailableRef {
        Inspector2Filter_FilterCriteria_FixAvailableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_FixAvailableRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_InspectorScore {
    lower_inclusive: terrars::PrimField<f64>,
    upper_inclusive: terrars::PrimField<f64>,
}

impl Inspector2Filter_FilterCriteria_InspectorScore { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_InspectorScore {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_InspectorScore>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_InspectorScore {
    #[doc = ""]
    pub lower_inclusive: terrars::PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: terrars::PrimField<f64>,
}

impl BuildInspector2Filter_FilterCriteria_InspectorScore {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_InspectorScore {
        Inspector2Filter_FilterCriteria_InspectorScore {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_InspectorScoreRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_InspectorScoreRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_InspectorScoreRef {
        Inspector2Filter_FilterCriteria_InspectorScoreRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_InspectorScoreRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn {
        Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArnRef {
        Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArnRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_inclusive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {}

impl BuildInspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {
        Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAtRef {
        Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionLayers {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionLayers { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_LambdaFunctionLayers {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionLayers>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_LambdaFunctionLayers {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_LambdaFunctionLayers {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_LambdaFunctionLayers {
        Inspector2Filter_FilterCriteria_LambdaFunctionLayers {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionLayersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_LambdaFunctionLayersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_LambdaFunctionLayersRef {
        Inspector2Filter_FilterCriteria_LambdaFunctionLayersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionLayersRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionName {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionName { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_LambdaFunctionName {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionName>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_LambdaFunctionName {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_LambdaFunctionName {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_LambdaFunctionName {
        Inspector2Filter_FilterCriteria_LambdaFunctionName {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionNameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_LambdaFunctionNameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_LambdaFunctionNameRef {
        Inspector2Filter_FilterCriteria_LambdaFunctionNameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionNameRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionRuntime {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionRuntime { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_LambdaFunctionRuntime {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionRuntime>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_LambdaFunctionRuntime {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_LambdaFunctionRuntime {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_LambdaFunctionRuntime {
        Inspector2Filter_FilterCriteria_LambdaFunctionRuntime {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_LambdaFunctionRuntimeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_LambdaFunctionRuntimeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_LambdaFunctionRuntimeRef {
        Inspector2Filter_FilterCriteria_LambdaFunctionRuntimeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_LambdaFunctionRuntimeRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_LastObservedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2Filter_FilterCriteria_LastObservedAt {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_inclusive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_LastObservedAt {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LastObservedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_LastObservedAt {}

impl BuildInspector2Filter_FilterCriteria_LastObservedAt {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_LastObservedAt {
        Inspector2Filter_FilterCriteria_LastObservedAt {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_LastObservedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_LastObservedAtRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_LastObservedAtRef {
        Inspector2Filter_FilterCriteria_LastObservedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_LastObservedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_NetworkProtocol {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_NetworkProtocol { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_NetworkProtocol {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_NetworkProtocol>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_NetworkProtocol {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_NetworkProtocol {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_NetworkProtocol {
        Inspector2Filter_FilterCriteria_NetworkProtocol {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_NetworkProtocolRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_NetworkProtocolRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_NetworkProtocolRef {
        Inspector2Filter_FilterCriteria_NetworkProtocolRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_NetworkProtocolRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_PortRange {
    begin_inclusive: terrars::PrimField<f64>,
    end_inclusive: terrars::PrimField<f64>,
}

impl Inspector2Filter_FilterCriteria_PortRange { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_PortRange {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_PortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_PortRange {
    #[doc = ""]
    pub begin_inclusive: terrars::PrimField<f64>,
    #[doc = ""]
    pub end_inclusive: terrars::PrimField<f64>,
}

impl BuildInspector2Filter_FilterCriteria_PortRange {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_PortRange {
        Inspector2Filter_FilterCriteria_PortRange {
            begin_inclusive: self.begin_inclusive,
            end_inclusive: self.end_inclusive,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_PortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_PortRangeRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_PortRangeRef {
        Inspector2Filter_FilterCriteria_PortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_PortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `begin_inclusive` after provisioning.\n"]
    pub fn begin_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.begin_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_RelatedVulnerabilities {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_RelatedVulnerabilities { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_RelatedVulnerabilities {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_RelatedVulnerabilities>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_RelatedVulnerabilities {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_RelatedVulnerabilities {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_RelatedVulnerabilities {
        Inspector2Filter_FilterCriteria_RelatedVulnerabilities {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_RelatedVulnerabilitiesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_RelatedVulnerabilitiesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_RelatedVulnerabilitiesRef {
        Inspector2Filter_FilterCriteria_RelatedVulnerabilitiesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_RelatedVulnerabilitiesRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_ResourceId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_ResourceId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_ResourceId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ResourceId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_ResourceId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_ResourceId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_ResourceId {
        Inspector2Filter_FilterCriteria_ResourceId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_ResourceIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_ResourceIdRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_ResourceIdRef {
        Inspector2Filter_FilterCriteria_ResourceIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_ResourceIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_ResourceTags {
    comparison: terrars::PrimField<std::string::String>,
    key: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_ResourceTags { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_ResourceTags {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ResourceTags>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_ResourceTags {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub key: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_ResourceTags {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_ResourceTags {
        Inspector2Filter_FilterCriteria_ResourceTags {
            comparison: self.comparison,
            key: self.key,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_ResourceTagsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_ResourceTagsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_ResourceTagsRef {
        Inspector2Filter_FilterCriteria_ResourceTagsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_ResourceTagsRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_ResourceType {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_ResourceType { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_ResourceType {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ResourceType>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_ResourceType {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_ResourceType {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_ResourceType {
        Inspector2Filter_FilterCriteria_ResourceType {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_ResourceTypeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_ResourceTypeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_ResourceTypeRef {
        Inspector2Filter_FilterCriteria_ResourceTypeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_ResourceTypeRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_Severity {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_Severity { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_Severity {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Severity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_Severity {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_Severity {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_Severity {
        Inspector2Filter_FilterCriteria_Severity {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_SeverityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_SeverityRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_SeverityRef {
        Inspector2Filter_FilterCriteria_SeverityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_SeverityRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_Title {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_Title { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_Title {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Title>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_Title {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_Title {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_Title {
        Inspector2Filter_FilterCriteria_Title {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_TitleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_TitleRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_TitleRef {
        Inspector2Filter_FilterCriteria_TitleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_TitleRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_UpdatedAt {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    end_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    start_inclusive: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Inspector2Filter_FilterCriteria_UpdatedAt {
    #[doc = "Set the field `end_inclusive`.\n"]
    pub fn set_end_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.end_inclusive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `start_inclusive`.\n"]
    pub fn set_start_inclusive(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.start_inclusive = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_UpdatedAt {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_UpdatedAt>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_UpdatedAt {}

impl BuildInspector2Filter_FilterCriteria_UpdatedAt {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_UpdatedAt {
        Inspector2Filter_FilterCriteria_UpdatedAt {
            end_inclusive: core::default::Default::default(),
            start_inclusive: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_UpdatedAtRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_UpdatedAtRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteria_UpdatedAtRef {
        Inspector2Filter_FilterCriteria_UpdatedAtRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_UpdatedAtRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `end_inclusive` after provisioning.\n"]
    pub fn end_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.end_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `start_inclusive` after provisioning.\n"]
    pub fn start_inclusive(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_VendorSeverity {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VendorSeverity { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VendorSeverity {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VendorSeverity>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VendorSeverity {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VendorSeverity {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VendorSeverity {
        Inspector2Filter_FilterCriteria_VendorSeverity {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VendorSeverityRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VendorSeverityRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VendorSeverityRef {
        Inspector2Filter_FilterCriteria_VendorSeverityRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VendorSeverityRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerabilityId {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerabilityId { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerabilityId {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerabilityId>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerabilityId {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerabilityId {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerabilityId {
        Inspector2Filter_FilterCriteria_VulnerabilityId {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerabilityIdRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerabilityIdRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerabilityIdRef {
        Inspector2Filter_FilterCriteria_VulnerabilityIdRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerabilityIdRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerabilitySource {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerabilitySource { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerabilitySource {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerabilitySource>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerabilitySource {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerabilitySource {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerabilitySource {
        Inspector2Filter_FilterCriteria_VulnerabilitySource {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerabilitySourceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerabilitySourceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerabilitySourceRef {
        Inspector2Filter_FilterCriteria_VulnerabilitySourceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerabilitySourceRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_Architecture {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_Architecture {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture {
        Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_ArchitectureRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_ArchitectureRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_ArchitectureRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_ArchitectureRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_ArchitectureRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch {
    lower_inclusive: terrars::PrimField<f64>,
    upper_inclusive: terrars::PrimField<f64>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_Epoch {
    #[doc = ""]
    pub lower_inclusive: terrars::PrimField<f64>,
    #[doc = ""]
    pub upper_inclusive: terrars::PrimField<f64>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_Epoch {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch {
        Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch {
            lower_inclusive: self.lower_inclusive,
            upper_inclusive: self.upper_inclusive,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_EpochRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_EpochRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_EpochRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_EpochRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_EpochRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `lower_inclusive` after provisioning.\n"]
    pub fn lower_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.lower_inclusive", self.base))
    }

    #[doc = "Get a reference to the value of field `upper_inclusive` after provisioning.\n"]
    pub fn upper_inclusive(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upper_inclusive", self.base))
    }
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_FilePath {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_FilePath {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath {
        Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_FilePathRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_FilePathRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_FilePathRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_FilePathRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_FilePathRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_Name {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_Name { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_Name {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Name>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_Name {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_Name {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_Name {
        Inspector2Filter_FilterCriteria_VulnerablePackages_Name {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_NameRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_NameRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_NameRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_NameRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_NameRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_Release {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_Release { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_Release {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Release>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_Release {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_Release {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_Release {
        Inspector2Filter_FilterCriteria_VulnerablePackages_Release {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_ReleaseRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_ReleaseRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_ReleaseRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_ReleaseRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_ReleaseRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn {
        Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArnRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArnRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArnRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArnRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArnRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash {
        Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHashRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHashRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHashRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHashRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHashRef {
    #[allow(dead_code)]
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
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_Version {
    comparison: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_Version { }

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages_Version {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Version>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages_Version {
    #[doc = ""]
    pub comparison: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages_Version {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages_Version {
        Inspector2Filter_FilterCriteria_VulnerablePackages_Version {
            comparison: self.comparison,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages_VersionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackages_VersionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackages_VersionRef {
        Inspector2Filter_FilterCriteria_VulnerablePackages_VersionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages_VersionRef {
    #[allow(dead_code)]
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
struct Inspector2Filter_FilterCriteria_VulnerablePackagesDynamic {
    architecture: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture>>,
    epoch: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch>>,
    file_path: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath>>,
    name: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_Name>>,
    release: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_Release>>,
    source_lambda_layer_arn: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn>>,
    source_layer_hash: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash>>,
    version: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages_Version>>,
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackages {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    architecture: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    epoch: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_path: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_Name>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    release: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_Release>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_lambda_layer_arn: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_layer_hash: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    version: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages_Version>>,
    dynamic: Inspector2Filter_FilterCriteria_VulnerablePackagesDynamic,
}

impl Inspector2Filter_FilterCriteria_VulnerablePackages {
    #[doc = "Set the field `architecture`.\n"]
    pub fn set_architecture(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Architecture>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.architecture = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.architecture = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `epoch`.\n"]
    pub fn set_epoch(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Epoch>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.epoch = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.epoch = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_path`.\n"]
    pub fn set_file_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_FilePath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.file_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.file_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Name>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `release`.\n"]
    pub fn set_release(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Release>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.release = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.release = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_lambda_layer_arn`.\n"]
    pub fn set_source_lambda_layer_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArn>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_lambda_layer_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_lambda_layer_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `source_layer_hash`.\n"]
    pub fn set_source_layer_hash(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHash>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.source_layer_hash = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.source_layer_hash = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `version`.\n"]
    pub fn set_version(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages_Version>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.version = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.version = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria_VulnerablePackages {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria_VulnerablePackages {}

impl BuildInspector2Filter_FilterCriteria_VulnerablePackages {
    pub fn build(self) -> Inspector2Filter_FilterCriteria_VulnerablePackages {
        Inspector2Filter_FilterCriteria_VulnerablePackages {
            architecture: core::default::Default::default(),
            epoch: core::default::Default::default(),
            file_path: core::default::Default::default(),
            name: core::default::Default::default(),
            release: core::default::Default::default(),
            source_lambda_layer_arn: core::default::Default::default(),
            source_layer_hash: core::default::Default::default(),
            version: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteria_VulnerablePackagesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteria_VulnerablePackagesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Inspector2Filter_FilterCriteria_VulnerablePackagesRef {
        Inspector2Filter_FilterCriteria_VulnerablePackagesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteria_VulnerablePackagesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `architecture` after provisioning.\n"]
    pub fn architecture(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_ArchitectureRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.architecture", self.base))
    }

    #[doc = "Get a reference to the value of field `epoch` after provisioning.\n"]
    pub fn epoch(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_EpochRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.epoch", self.base))
    }

    #[doc = "Get a reference to the value of field `file_path` after provisioning.\n"]
    pub fn file_path(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_FilePathRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_path", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_NameRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `release` after provisioning.\n"]
    pub fn release(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_ReleaseRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.release", self.base))
    }

    #[doc = "Get a reference to the value of field `source_lambda_layer_arn` after provisioning.\n"]
    pub fn source_lambda_layer_arn(
        &self,
    ) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLambdaLayerArnRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_lambda_layer_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `source_layer_hash` after provisioning.\n"]
    pub fn source_layer_hash(
        &self,
    ) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_SourceLayerHashRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_layer_hash", self.base))
    }

    #[doc = "Get a reference to the value of field `version` after provisioning.\n"]
    pub fn version(&self) -> terrars::ListRef<Inspector2Filter_FilterCriteria_VulnerablePackages_VersionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.version", self.base))
    }
}

#[derive(Serialize, Default)]
struct Inspector2Filter_FilterCriteriaDynamic {
    aws_account_id: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_AwsAccountId>>,
    code_repository_project_name: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_CodeRepositoryProjectName>>,
    code_repository_provider_type: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_CodeRepositoryProviderType>>,
    code_vulnerability_detector_name: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName>>,
    code_vulnerability_detector_tags: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags>>,
    code_vulnerability_file_path: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath>>,
    component_id: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_ComponentId>>,
    component_type: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_ComponentType>>,
    ec2_instance_image_id: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_Ec2InstanceImageId>>,
    ec2_instance_subnet_id: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId>>,
    ec2_instance_vpc_id: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_Ec2InstanceVpcId>>,
    ecr_image_architecture: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageArchitecture>>,
    ecr_image_hash: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageHash>>,
    ecr_image_in_use_count: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageInUseCount>>,
    ecr_image_last_in_use_at: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageLastInUseAt>>,
    ecr_image_pushed_at: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImagePushedAt>>,
    ecr_image_registry: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageRegistry>>,
    ecr_image_repository_name: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageRepositoryName>>,
    ecr_image_tags: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EcrImageTags>>,
    epss_score: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_EpssScore>>,
    exploit_available: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_ExploitAvailable>>,
    finding_arn: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_FindingArn>>,
    finding_status: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_FindingStatus>>,
    finding_type: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_FindingType>>,
    first_observed_at: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_FirstObservedAt>>,
    fix_available: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_FixAvailable>>,
    inspector_score: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_InspectorScore>>,
    lambda_function_execution_role_arn: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn>>,
    lambda_function_last_modified_at: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt>>,
    lambda_function_layers: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_LambdaFunctionLayers>>,
    lambda_function_name: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_LambdaFunctionName>>,
    lambda_function_runtime: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_LambdaFunctionRuntime>>,
    last_observed_at: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_LastObservedAt>>,
    network_protocol: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_NetworkProtocol>>,
    port_range: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_PortRange>>,
    related_vulnerabilities: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_RelatedVulnerabilities>>,
    resource_id: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_ResourceId>>,
    resource_tags: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_ResourceTags>>,
    resource_type: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_ResourceType>>,
    severity: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_Severity>>,
    title: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_Title>>,
    updated_at: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_UpdatedAt>>,
    vendor_severity: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VendorSeverity>>,
    vulnerability_id: std::option::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerabilityId>>,
    vulnerability_source: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerabilitySource>>,
    vulnerable_packages: std
    ::option
    ::Option<terrars::DynamicBlock<Inspector2Filter_FilterCriteria_VulnerablePackages>>,
}

#[derive(Serialize)]
pub struct Inspector2Filter_FilterCriteria {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    aws_account_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_AwsAccountId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository_project_name: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_CodeRepositoryProjectName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_repository_provider_type: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_CodeRepositoryProviderType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_vulnerability_detector_name: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_vulnerability_detector_tags: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    code_vulnerability_file_path: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_ComponentId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_type: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_ComponentType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_instance_image_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_Ec2InstanceImageId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_instance_subnet_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ec2_instance_vpc_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_Ec2InstanceVpcId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_architecture: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageArchitecture>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_hash: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageHash>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_in_use_count: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageInUseCount>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_last_in_use_at: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageLastInUseAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_pushed_at: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImagePushedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_registry: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageRegistry>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_repository_name: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageRepositoryName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ecr_image_tags: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EcrImageTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    epss_score: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_EpssScore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    exploit_available: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_ExploitAvailable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_arn: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_FindingArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_status: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_FindingStatus>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    finding_type: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_FindingType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    first_observed_at: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_FirstObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    fix_available: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_FixAvailable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inspector_score: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_InspectorScore>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_execution_role_arn: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_last_modified_at: std
    ::option
    ::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_layers: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_LambdaFunctionLayers>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_name: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_LambdaFunctionName>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lambda_function_runtime: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_LambdaFunctionRuntime>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    last_observed_at: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_LastObservedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_protocol: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_NetworkProtocol>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_PortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    related_vulnerabilities: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_RelatedVulnerabilities>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_ResourceId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_tags: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_ResourceTags>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_ResourceType>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    severity: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_Severity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    title: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_Title>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    updated_at: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_UpdatedAt>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vendor_severity: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VendorSeverity>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_id: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerabilityId>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerability_source: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerabilitySource>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vulnerable_packages: std::option::Option<std::vec::Vec<Inspector2Filter_FilterCriteria_VulnerablePackages>>,
    dynamic: Inspector2Filter_FilterCriteriaDynamic,
}

impl Inspector2Filter_FilterCriteria {
    #[doc = "Set the field `aws_account_id`.\n"]
    pub fn set_aws_account_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_AwsAccountId>>,
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

    #[doc = "Set the field `code_repository_project_name`.\n"]
    pub fn set_code_repository_project_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeRepositoryProjectName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository_project_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository_project_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_repository_provider_type`.\n"]
    pub fn set_code_repository_provider_type(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeRepositoryProviderType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_repository_provider_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_repository_provider_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_vulnerability_detector_name`.\n"]
    pub fn set_code_vulnerability_detector_name(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_vulnerability_detector_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_vulnerability_detector_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_vulnerability_detector_tags`.\n"]
    pub fn set_code_vulnerability_detector_tags(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeVulnerabilityDetectorTags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_vulnerability_detector_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_vulnerability_detector_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `code_vulnerability_file_path`.\n"]
    pub fn set_code_vulnerability_file_path(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_CodeVulnerabilityFilePath>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.code_vulnerability_file_path = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.code_vulnerability_file_path = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `component_id`.\n"]
    pub fn set_component_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ComponentId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.component_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.component_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `component_type`.\n"]
    pub fn set_component_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ComponentType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.component_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.component_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_instance_image_id`.\n"]
    pub fn set_ec2_instance_image_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Ec2InstanceImageId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_instance_image_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_instance_image_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_instance_subnet_id`.\n"]
    pub fn set_ec2_instance_subnet_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Ec2InstanceSubnetId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_instance_subnet_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_instance_subnet_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ec2_instance_vpc_id`.\n"]
    pub fn set_ec2_instance_vpc_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Ec2InstanceVpcId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ec2_instance_vpc_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ec2_instance_vpc_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_architecture`.\n"]
    pub fn set_ecr_image_architecture(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageArchitecture>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_architecture = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_architecture = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_hash`.\n"]
    pub fn set_ecr_image_hash(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageHash>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_hash = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_hash = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_in_use_count`.\n"]
    pub fn set_ecr_image_in_use_count(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageInUseCount>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_in_use_count = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_in_use_count = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_last_in_use_at`.\n"]
    pub fn set_ecr_image_last_in_use_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageLastInUseAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_last_in_use_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_last_in_use_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_pushed_at`.\n"]
    pub fn set_ecr_image_pushed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImagePushedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_pushed_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_pushed_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_registry`.\n"]
    pub fn set_ecr_image_registry(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageRegistry>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_registry = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_registry = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_repository_name`.\n"]
    pub fn set_ecr_image_repository_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageRepositoryName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_repository_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_repository_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `ecr_image_tags`.\n"]
    pub fn set_ecr_image_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EcrImageTags>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.ecr_image_tags = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.ecr_image_tags = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `epss_score`.\n"]
    pub fn set_epss_score(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_EpssScore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.epss_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.epss_score = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `exploit_available`.\n"]
    pub fn set_exploit_available(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ExploitAvailable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.exploit_available = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.exploit_available = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_arn`.\n"]
    pub fn set_finding_arn(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FindingArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_status`.\n"]
    pub fn set_finding_status(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FindingStatus>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_status = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_status = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `finding_type`.\n"]
    pub fn set_finding_type(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FindingType>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.finding_type = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.finding_type = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `first_observed_at`.\n"]
    pub fn set_first_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FirstObservedAt>>,
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

    #[doc = "Set the field `fix_available`.\n"]
    pub fn set_fix_available(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_FixAvailable>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.fix_available = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.fix_available = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `inspector_score`.\n"]
    pub fn set_inspector_score(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_InspectorScore>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.inspector_score = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.inspector_score = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_execution_role_arn`.\n"]
    pub fn set_lambda_function_execution_role_arn(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionExecutionRoleArn>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_execution_role_arn = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_execution_role_arn = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_last_modified_at`.\n"]
    pub fn set_lambda_function_last_modified_at(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionLastModifiedAt>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_last_modified_at = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_last_modified_at = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_layers`.\n"]
    pub fn set_lambda_function_layers(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionLayers>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_layers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_layers = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_name`.\n"]
    pub fn set_lambda_function_name(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionName>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_name = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_name = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lambda_function_runtime`.\n"]
    pub fn set_lambda_function_runtime(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LambdaFunctionRuntime>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.lambda_function_runtime = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.lambda_function_runtime = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `last_observed_at`.\n"]
    pub fn set_last_observed_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_LastObservedAt>>,
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

    #[doc = "Set the field `network_protocol`.\n"]
    pub fn set_network_protocol(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_NetworkProtocol>>,
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

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_PortRange>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.port_range = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.port_range = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `related_vulnerabilities`.\n"]
    pub fn set_related_vulnerabilities(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_RelatedVulnerabilities>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.related_vulnerabilities = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.related_vulnerabilities = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ResourceId>>,
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

    #[doc = "Set the field `resource_tags`.\n"]
    pub fn set_resource_tags(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ResourceTags>>,
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
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_ResourceType>>,
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

    #[doc = "Set the field `severity`.\n"]
    pub fn set_severity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Severity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.severity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.severity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `title`.\n"]
    pub fn set_title(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_Title>>,
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

    #[doc = "Set the field `updated_at`.\n"]
    pub fn set_updated_at(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_UpdatedAt>>,
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

    #[doc = "Set the field `vendor_severity`.\n"]
    pub fn set_vendor_severity(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VendorSeverity>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vendor_severity = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vendor_severity = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vulnerability_id`.\n"]
    pub fn set_vulnerability_id(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerabilityId>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vulnerability_id = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vulnerability_id = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vulnerability_source`.\n"]
    pub fn set_vulnerability_source(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerabilitySource>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vulnerability_source = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vulnerability_source = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `vulnerable_packages`.\n"]
    pub fn set_vulnerable_packages(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<Inspector2Filter_FilterCriteria_VulnerablePackages>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.vulnerable_packages = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.vulnerable_packages = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for Inspector2Filter_FilterCriteria {
    type O = terrars::BlockAssignable<Inspector2Filter_FilterCriteria>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildInspector2Filter_FilterCriteria {}

impl BuildInspector2Filter_FilterCriteria {
    pub fn build(self) -> Inspector2Filter_FilterCriteria {
        Inspector2Filter_FilterCriteria {
            aws_account_id: core::default::Default::default(),
            code_repository_project_name: core::default::Default::default(),
            code_repository_provider_type: core::default::Default::default(),
            code_vulnerability_detector_name: core::default::Default::default(),
            code_vulnerability_detector_tags: core::default::Default::default(),
            code_vulnerability_file_path: core::default::Default::default(),
            component_id: core::default::Default::default(),
            component_type: core::default::Default::default(),
            ec2_instance_image_id: core::default::Default::default(),
            ec2_instance_subnet_id: core::default::Default::default(),
            ec2_instance_vpc_id: core::default::Default::default(),
            ecr_image_architecture: core::default::Default::default(),
            ecr_image_hash: core::default::Default::default(),
            ecr_image_in_use_count: core::default::Default::default(),
            ecr_image_last_in_use_at: core::default::Default::default(),
            ecr_image_pushed_at: core::default::Default::default(),
            ecr_image_registry: core::default::Default::default(),
            ecr_image_repository_name: core::default::Default::default(),
            ecr_image_tags: core::default::Default::default(),
            epss_score: core::default::Default::default(),
            exploit_available: core::default::Default::default(),
            finding_arn: core::default::Default::default(),
            finding_status: core::default::Default::default(),
            finding_type: core::default::Default::default(),
            first_observed_at: core::default::Default::default(),
            fix_available: core::default::Default::default(),
            inspector_score: core::default::Default::default(),
            lambda_function_execution_role_arn: core::default::Default::default(),
            lambda_function_last_modified_at: core::default::Default::default(),
            lambda_function_layers: core::default::Default::default(),
            lambda_function_name: core::default::Default::default(),
            lambda_function_runtime: core::default::Default::default(),
            last_observed_at: core::default::Default::default(),
            network_protocol: core::default::Default::default(),
            port_range: core::default::Default::default(),
            related_vulnerabilities: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_tags: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            severity: core::default::Default::default(),
            title: core::default::Default::default(),
            updated_at: core::default::Default::default(),
            vendor_severity: core::default::Default::default(),
            vulnerability_id: core::default::Default::default(),
            vulnerability_source: core::default::Default::default(),
            vulnerable_packages: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Inspector2Filter_FilterCriteriaRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Inspector2Filter_FilterCriteriaRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Inspector2Filter_FilterCriteriaRef {
        Inspector2Filter_FilterCriteriaRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Inspector2Filter_FilterCriteriaRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }
}
