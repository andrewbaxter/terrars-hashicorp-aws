#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2NetworkInsightsAnalysisData_ {
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
    filter_in_arns: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    network_insights_path_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    region: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_completion: std::option::Option<terrars::PrimField<bool>>,
}

struct Ec2NetworkInsightsAnalysis_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2NetworkInsightsAnalysisData_>,
}

#[derive(Clone)]
pub struct Ec2NetworkInsightsAnalysis(std::rc::Rc<Ec2NetworkInsightsAnalysis_>);

impl Ec2NetworkInsightsAnalysis {
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

    #[doc = "Set the field `filter_in_arns`.\n"]
    pub fn set_filter_in_arns(
        self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().filter_in_arns = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `wait_for_completion`.\n"]
    pub fn set_wait_for_completion(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().wait_for_completion = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `alternate_path_hints` after provisioning.\n"]
    pub fn alternate_path_hints(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_AlternatePathHintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ExplanationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_insights_path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_found` after provisioning.\n"]
    pub fn path_found(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_found", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `return_path_components` after provisioning.\n"]
    pub fn return_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.return_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_completion` after provisioning.\n"]
    pub fn wait_for_completion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }
}

impl terrars::Referable for Ec2NetworkInsightsAnalysis {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Ec2NetworkInsightsAnalysis { }

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis {
    type O = terrars::ListRef<Ec2NetworkInsightsAnalysisRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Ec2NetworkInsightsAnalysis_ {
    fn extract_resource_type(&self) -> std::string::String {
        "aws_ec2_network_insights_analysis".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildEc2NetworkInsightsAnalysis {
    pub tf_id: std::string::String,
    #[doc = ""]
    pub network_insights_path_id: terrars::PrimField<std::string::String>,
}

impl BuildEc2NetworkInsightsAnalysis {
    pub fn build(self, stack: &mut terrars::Stack) -> Ec2NetworkInsightsAnalysis {
        let out = Ec2NetworkInsightsAnalysis(std::rc::Rc::new(Ec2NetworkInsightsAnalysis_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(Ec2NetworkInsightsAnalysisData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                filter_in_arns: core::default::Default::default(),
                id: core::default::Default::default(),
                network_insights_path_id: self.network_insights_path_id,
                region: core::default::Default::default(),
                tags: core::default::Default::default(),
                tags_all: core::default::Default::default(),
                wait_for_completion: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct Ec2NetworkInsightsAnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl Ec2NetworkInsightsAnalysisRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `alternate_path_hints` after provisioning.\n"]
    pub fn alternate_path_hints(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_AlternatePathHintsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ExplanationsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_path_id` after provisioning.\n"]
    pub fn network_insights_path_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_insights_path_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `path_found` after provisioning.\n"]
    pub fn path_found(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.path_found", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `region` after provisioning.\nRegion where this resource will be [managed](https://docs.aws.amazon.com/general/latest/gr/rande.html#regional-endpoints). Defaults to the Region set in the [provider configuration](https://registry.terraform.io/providers/hashicorp/aws/latest/docs#aws-configuration-reference)."]
    pub fn region(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.region", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `return_path_components` after provisioning.\n"]
    pub fn return_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.return_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `start_date` after provisioning.\n"]
    pub fn start_date(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.start_date", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status` after provisioning.\n"]
    pub fn status(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `status_message` after provisioning.\n"]
    pub fn status_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.status_message", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags` after provisioning.\n"]
    pub fn tags(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `tags_all` after provisioning.\n"]
    pub fn tags_all(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.tags_all", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `wait_for_completion` after provisioning.\n"]
    pub fn wait_for_completion(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.wait_for_completion", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_AlternatePathHints {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_AlternatePathHints {
    #[doc = "Set the field `component_arn`.\n"]
    pub fn set_component_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.component_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component_id`.\n"]
    pub fn set_component_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.component_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_AlternatePathHints {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_AlternatePathHints>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_AlternatePathHints {}

impl BuildEc2NetworkInsightsAnalysis_AlternatePathHints {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_AlternatePathHints {
        Ec2NetworkInsightsAnalysis_AlternatePathHints {
            component_arn: core::default::Default::default(),
            component_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_AlternatePathHintsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_AlternatePathHintsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_AlternatePathHintsRef {
        Ec2NetworkInsightsAnalysis_AlternatePathHintsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_AlternatePathHintsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `component_arn` after provisioning.\n"]
    pub fn component_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `component_id` after provisioning.\n"]
    pub fn component_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.component_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_Acl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_Acl {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_Acl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_Acl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_Acl {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_Acl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_Acl {
        Ec2NetworkInsightsAnalysis_Explanations_Acl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AclRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_AclRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_AclRef {
        Ec2NetworkInsightsAnalysis_Explanations_AclRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_AclRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {
        Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef {
        Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AclRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_AclRule {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress`.\n"]
    pub fn set_egress(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.egress = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange>>,
    ) -> Self {
        self.port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_action`.\n"]
    pub fn set_rule_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.rule_action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `rule_number`.\n"]
    pub fn set_rule_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.rule_number = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_AclRule {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_AclRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_AclRule {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_AclRule {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_AclRule {
        Ec2NetworkInsightsAnalysis_Explanations_AclRule {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef {
        Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `egress` after provisioning.\n"]
    pub fn egress(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_action` after provisioning.\n"]
    pub fn rule_action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_action", self.base))
    }

    #[doc = "Get a reference to the value of field `rule_number` after provisioning.\n"]
    pub fn rule_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.rule_number", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AttachedTo {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_AttachedTo {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_AttachedTo {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_AttachedTo {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_AttachedTo {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_AttachedTo {
        Ec2NetworkInsightsAnalysis_Explanations_AttachedTo {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef {
        Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {
    #[doc = "Set the field `instance_port`.\n"]
    pub fn set_instance_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.instance_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_port`.\n"]
    pub fn set_load_balancer_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.load_balancer_port = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {
        Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener {
            instance_port: core::default::Default::default(),
            load_balancer_port: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListenerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListenerRef {
        Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListenerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListenerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `instance_port` after provisioning.\n"]
    pub fn instance_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_port", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_port` after provisioning.\n"]
    pub fn load_balancer_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_port", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_Component {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_Component {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_Component {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_Component>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_Component {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_Component {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_Component {
        Ec2NetworkInsightsAnalysis_Explanations_Component {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_ComponentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_ComponentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_ComponentRef {
        Ec2NetworkInsightsAnalysis_Explanations_ComponentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_ComponentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_CustomerGateway {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_CustomerGateway {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway {
        Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_CustomerGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_CustomerGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_CustomerGatewayRef {
        Ec2NetworkInsightsAnalysis_Explanations_CustomerGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_CustomerGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_Destination {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_Destination {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_Destination {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_Destination>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_Destination {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_Destination {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_Destination {
        Ec2NetworkInsightsAnalysis_Explanations_Destination {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_DestinationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_DestinationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_DestinationRef {
        Ec2NetworkInsightsAnalysis_Explanations_DestinationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_DestinationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_DestinationVpc {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_DestinationVpc {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc {
        Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef {
        Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {
        Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListenerRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListenerRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListenerRef {
        Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListenerRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListenerRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_IngressRouteTable {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_IngressRouteTable {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable {
        Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTableRef {
        Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_InternetGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_InternetGateway {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_InternetGateway {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_InternetGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_InternetGateway {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_InternetGateway {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_InternetGateway {
        Ec2NetworkInsightsAnalysis_Explanations_InternetGateway {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_InternetGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_InternetGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_InternetGatewayRef {
        Ec2NetworkInsightsAnalysis_Explanations_InternetGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_InternetGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {
        Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupRef {
        Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {
        Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupsRef {
        Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_NatGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_NatGateway {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_NatGateway {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_NatGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_NatGateway {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_NatGateway {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_NatGateway {
        Ec2NetworkInsightsAnalysis_Explanations_NatGateway {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_NatGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_NatGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_NatGatewayRef {
        Ec2NetworkInsightsAnalysis_Explanations_NatGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_NatGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_NetworkInterface {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_NetworkInterface {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface {
        Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_NetworkInterfaceRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_NetworkInterfaceRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_NetworkInterfaceRef {
        Ec2NetworkInsightsAnalysis_Explanations_NetworkInterfaceRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_NetworkInterfaceRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_PortRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_PortRanges {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_PortRanges {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_PortRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_PortRanges {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_PortRanges {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_PortRanges {
        Ec2NetworkInsightsAnalysis_Explanations_PortRanges {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_PortRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_PortRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_PortRangesRef {
        Ec2NetworkInsightsAnalysis_Explanations_PortRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_PortRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_PrefixList {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_PrefixList {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_PrefixList {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_PrefixList>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_PrefixList {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_PrefixList {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_PrefixList {
        Ec2NetworkInsightsAnalysis_Explanations_PrefixList {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_PrefixListRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_PrefixListRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_PrefixListRef {
        Ec2NetworkInsightsAnalysis_Explanations_PrefixListRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_PrefixListRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_RouteTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_RouteTable {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_RouteTable {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_RouteTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_RouteTable {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_RouteTable {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_RouteTable {
        Ec2NetworkInsightsAnalysis_Explanations_RouteTable {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_RouteTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_RouteTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_RouteTableRef {
        Ec2NetworkInsightsAnalysis_Explanations_RouteTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_RouteTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress_only_internet_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_connection_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute {
    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_prefix_list_id`.\n"]
    pub fn set_destination_prefix_list_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.destination_prefix_list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `egress_only_internet_gateway_id`.\n"]
    pub fn set_egress_only_internet_gateway_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.egress_only_internet_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `gateway_id`.\n"]
    pub fn set_gateway_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `instance_id`.\n"]
    pub fn set_instance_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.instance_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway_id`.\n"]
    pub fn set_nat_gateway_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.nat_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface_id`.\n"]
    pub fn set_network_interface_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.network_interface_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `origin`.\n"]
    pub fn set_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_id`.\n"]
    pub fn set_transit_gateway_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.transit_gateway_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection_id`.\n"]
    pub fn set_vpc_peering_connection_id(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.vpc_peering_connection_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_RouteTableRoute {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_RouteTableRoute {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute {
        Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute {
            destination_cidr: core::default::Default::default(),
            destination_prefix_list_id: core::default::Default::default(),
            egress_only_internet_gateway_id: core::default::Default::default(),
            gateway_id: core::default::Default::default(),
            instance_id: core::default::Default::default(),
            nat_gateway_id: core::default::Default::default(),
            network_interface_id: core::default::Default::default(),
            origin: core::default::Default::default(),
            transit_gateway_id: core::default::Default::default(),
            vpc_peering_connection_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef {
        Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_prefix_list_id` after provisioning.\n"]
    pub fn destination_prefix_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `egress_only_internet_gateway_id` after provisioning.\n"]
    pub fn egress_only_internet_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.egress_only_internet_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `gateway_id` after provisioning.\n"]
    pub fn gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `instance_id` after provisioning.\n"]
    pub fn instance_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.instance_id", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway_id` after provisioning.\n"]
    pub fn nat_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.nat_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface_id` after provisioning.\n"]
    pub fn network_interface_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_interface_id", self.base))
    }

    #[doc = "Get a reference to the value of field `origin` after provisioning.\n"]
    pub fn origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.origin", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_id` after provisioning.\n"]
    pub fn transit_gateway_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.transit_gateway_id", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection_id` after provisioning.\n"]
    pub fn vpc_peering_connection_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.vpc_peering_connection_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_SecurityGroup {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_SecurityGroup {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup {
        Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRef {
        Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {
    #[doc = "Set the field `cidr`.\n"]
    pub fn set_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_range`.\n"]
    pub fn set_port_range(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRange>>,
    ) -> Self {
        self.port_range = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_id`.\n"]
    pub fn set_security_group_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.security_group_id = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {
        Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef {
        Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `cidr` after provisioning.\n"]
    pub fn cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `port_range` after provisioning.\n"]
    pub fn port_range(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AclRule_PortRangeRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_range", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_id` after provisioning.\n"]
    pub fn security_group_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.security_group_id", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_SecurityGroups {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_SecurityGroups {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups {
        Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupsRef {
        Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SourceVpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_SourceVpc {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_SourceVpc {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_SourceVpc {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_SourceVpc {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_SourceVpc {
        Ec2NetworkInsightsAnalysis_Explanations_SourceVpc {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef {
        Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_Subnet {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_Subnet {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_Subnet {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_Subnet>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_Subnet {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_Subnet {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_Subnet {
        Ec2NetworkInsightsAnalysis_Explanations_Subnet {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SubnetRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_SubnetRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_SubnetRef {
        Ec2NetworkInsightsAnalysis_Explanations_SubnetRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_SubnetRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {
        Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTableRef {
        Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGateway {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_TransitGateway {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_TransitGateway {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_TransitGateway {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGateway {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGateway {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachmentRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachmentRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachmentRef {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachmentRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachmentRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRef {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attachment_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    resource_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_origin: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {
    #[doc = "Set the field `attachment_id`.\n"]
    pub fn set_attachment_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.attachment_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_cidr`.\n"]
    pub fn set_destination_cidr(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.destination_cidr = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list_id`.\n"]
    pub fn set_prefix_list_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.prefix_list_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_id`.\n"]
    pub fn set_resource_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `resource_type`.\n"]
    pub fn set_resource_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.resource_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_origin`.\n"]
    pub fn set_route_origin(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.route_origin = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute {
            attachment_id: core::default::Default::default(),
            destination_cidr: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            resource_id: core::default::Default::default(),
            resource_type: core::default::Default::default(),
            route_origin: core::default::Default::default(),
            state: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef {
        Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `attachment_id` after provisioning.\n"]
    pub fn attachment_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.attachment_id", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_cidr` after provisioning.\n"]
    pub fn destination_cidr(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.destination_cidr", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list_id` after provisioning.\n"]
    pub fn prefix_list_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.prefix_list_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_id` after provisioning.\n"]
    pub fn resource_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_id", self.base))
    }

    #[doc = "Get a reference to the value of field `resource_type` after provisioning.\n"]
    pub fn resource_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.resource_type", self.base))
    }

    #[doc = "Get a reference to the value of field `route_origin` after provisioning.\n"]
    pub fn route_origin(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.route_origin", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_Vpc {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_Vpc {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_Vpc {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_Vpc>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_Vpc {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_Vpc {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_Vpc {
        Ec2NetworkInsightsAnalysis_Explanations_Vpc {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpcRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_VpcRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_VpcRef {
        Ec2NetworkInsightsAnalysis_Explanations_VpcRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpcRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_VpcEndpoint {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_VpcEndpoint {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint {
        Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpcEndpointRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_VpcEndpointRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_VpcEndpointRef {
        Ec2NetworkInsightsAnalysis_Explanations_VpcEndpointRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpcEndpointRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {
        Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnectionRef {
        Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpnConnection {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpnConnection {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_VpnConnection {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_VpnConnection>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_VpnConnection {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_VpnConnection {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_VpnConnection {
        Ec2NetworkInsightsAnalysis_Explanations_VpnConnection {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpnConnectionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_VpnConnectionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_VpnConnectionRef {
        Ec2NetworkInsightsAnalysis_Explanations_VpnConnectionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpnConnectionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpnGateway {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpnGateway {
    #[doc = "Set the field `arn`.\n"]
    pub fn set_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations_VpnGateway {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations_VpnGateway>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations_VpnGateway {}

impl BuildEc2NetworkInsightsAnalysis_Explanations_VpnGateway {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations_VpnGateway {
        Ec2NetworkInsightsAnalysis_Explanations_VpnGateway {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_Explanations_VpnGatewayRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_Explanations_VpnGatewayRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_Explanations_VpnGatewayRef {
        Ec2NetworkInsightsAnalysis_Explanations_VpnGatewayRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_Explanations_VpnGatewayRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.base))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_Explanations {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Acl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classic_load_balancer_listener: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Destination>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_load_balancer_listener: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explanation_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_route_table: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_InternetGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_listener_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_group: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_groups: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_component: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_NatGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packet_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_ranges: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_PortRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_PrefixList>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Subnet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_route_table: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Vpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_connection: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_connection: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpnConnection>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpnGateway>>,
}

impl Ec2NetworkInsightsAnalysis_Explanations {
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Acl>>,
    ) -> Self {
        self.acl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule>>,
    ) -> Self {
        self.acl_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `address`.\n"]
    pub fn set_address(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.address = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `addresses`.\n"]
    pub fn set_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `availability_zones`.\n"]
    pub fn set_availability_zones(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.availability_zones = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cidrs`.\n"]
    pub fn set_cidrs(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.cidrs = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `classic_load_balancer_listener`.\n"]
    pub fn set_classic_load_balancer_listener(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListener>>,
    ) -> Self {
        self.classic_load_balancer_listener = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_gateway`.\n"]
    pub fn set_customer_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_CustomerGateway>>,
    ) -> Self {
        self.customer_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Destination>>,
    ) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>>,
    ) -> Self {
        self.destination_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `direction`.\n"]
    pub fn set_direction(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.direction = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `elastic_load_balancer_listener`.\n"]
    pub fn set_elastic_load_balancer_listener(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListener>>,
    ) -> Self {
        self.elastic_load_balancer_listener = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `explanation_code`.\n"]
    pub fn set_explanation_code(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.explanation_code = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `ingress_route_table`.\n"]
    pub fn set_ingress_route_table(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTable>>,
    ) -> Self {
        self.ingress_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_InternetGateway>>,
    ) -> Self {
        self.internet_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_arn`.\n"]
    pub fn set_load_balancer_arn(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.load_balancer_arn = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_listener_port`.\n"]
    pub fn set_load_balancer_listener_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.load_balancer_listener_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_target_group`.\n"]
    pub fn set_load_balancer_target_group(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroup>>,
    ) -> Self {
        self.load_balancer_target_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_target_groups`.\n"]
    pub fn set_load_balancer_target_groups(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroups>>,
    ) -> Self {
        self.load_balancer_target_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `load_balancer_target_port`.\n"]
    pub fn set_load_balancer_target_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.load_balancer_target_port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `missing_component`.\n"]
    pub fn set_missing_component(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.missing_component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `nat_gateway`.\n"]
    pub fn set_nat_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_NatGateway>>,
    ) -> Self {
        self.nat_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_NetworkInterface>>,
    ) -> Self {
        self.network_interface = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `packet_field`.\n"]
    pub fn set_packet_field(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.packet_field = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port`.\n"]
    pub fn set_port(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.port = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `port_ranges`.\n"]
    pub fn set_port_ranges(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_PortRanges>>,
    ) -> Self {
        self.port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list`.\n"]
    pub fn set_prefix_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_PrefixList>>,
    ) -> Self {
        self.prefix_list = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocols`.\n"]
    pub fn set_protocols(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.protocols = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table`.\n"]
    pub fn set_route_table(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTable>>,
    ) -> Self {
        self.route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group`.\n"]
    pub fn set_security_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroup>>,
    ) -> Self {
        self.security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>>,
    ) -> Self {
        self.security_group_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroups>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `state`.\n"]
    pub fn set_state(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.state = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Subnet>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_route_table`.\n"]
    pub fn set_subnet_route_table(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTable>>,
    ) -> Self {
        self.subnet_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>>,
    ) -> Self {
        self.transit_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_attachment`.\n"]
    pub fn set_transit_gateway_attachment(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachment>>,
    ) -> Self {
        self.transit_gateway_attachment = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table`.\n"]
    pub fn set_transit_gateway_route_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTable>>,
    ) -> Self {
        self.transit_gateway_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_route`.\n"]
    pub fn set_transit_gateway_route_table_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>>,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Vpc>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpcEndpoint>>,
    ) -> Self {
        self.vpc_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection`.\n"]
    pub fn set_vpc_peering_connection(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnection>>,
    ) -> Self {
        self.vpc_peering_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_connection`.\n"]
    pub fn set_vpn_connection(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpnConnection>>,
    ) -> Self {
        self.vpn_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_gateway`.\n"]
    pub fn set_vpn_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_VpnGateway>>,
    ) -> Self {
        self.vpn_gateway = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_Explanations {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_Explanations>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_Explanations {}

impl BuildEc2NetworkInsightsAnalysis_Explanations {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_Explanations {
        Ec2NetworkInsightsAnalysis_Explanations {
            acl: core::default::Default::default(),
            acl_rule: core::default::Default::default(),
            address: core::default::Default::default(),
            addresses: core::default::Default::default(),
            attached_to: core::default::Default::default(),
            availability_zones: core::default::Default::default(),
            cidrs: core::default::Default::default(),
            classic_load_balancer_listener: core::default::Default::default(),
            component: core::default::Default::default(),
            customer_gateway: core::default::Default::default(),
            destination: core::default::Default::default(),
            destination_vpc: core::default::Default::default(),
            direction: core::default::Default::default(),
            elastic_load_balancer_listener: core::default::Default::default(),
            explanation_code: core::default::Default::default(),
            ingress_route_table: core::default::Default::default(),
            internet_gateway: core::default::Default::default(),
            load_balancer_arn: core::default::Default::default(),
            load_balancer_listener_port: core::default::Default::default(),
            load_balancer_target_group: core::default::Default::default(),
            load_balancer_target_groups: core::default::Default::default(),
            load_balancer_target_port: core::default::Default::default(),
            missing_component: core::default::Default::default(),
            nat_gateway: core::default::Default::default(),
            network_interface: core::default::Default::default(),
            packet_field: core::default::Default::default(),
            port: core::default::Default::default(),
            port_ranges: core::default::Default::default(),
            prefix_list: core::default::Default::default(),
            protocols: core::default::Default::default(),
            route_table: core::default::Default::default(),
            route_table_route: core::default::Default::default(),
            security_group: core::default::Default::default(),
            security_group_rule: core::default::Default::default(),
            security_groups: core::default::Default::default(),
            source_vpc: core::default::Default::default(),
            state: core::default::Default::default(),
            subnet: core::default::Default::default(),
            subnet_route_table: core::default::Default::default(),
            transit_gateway: core::default::Default::default(),
            transit_gateway_attachment: core::default::Default::default(),
            transit_gateway_route_table: core::default::Default::default(),
            transit_gateway_route_table_route: core::default::Default::default(),
            vpc: core::default::Default::default(),
            vpc_endpoint: core::default::Default::default(),
            vpc_peering_connection: core::default::Default::default(),
            vpn_connection: core::default::Default::default(),
            vpn_gateway: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ExplanationsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ExplanationsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2NetworkInsightsAnalysis_ExplanationsRef {
        Ec2NetworkInsightsAnalysis_ExplanationsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ExplanationsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AclRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `address` after provisioning.\n"]
    pub fn address(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.address", self.base))
    }

    #[doc = "Get a reference to the value of field `addresses` after provisioning.\n"]
    pub fn addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `availability_zones` after provisioning.\n"]
    pub fn availability_zones(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.availability_zones", self.base))
    }

    #[doc = "Get a reference to the value of field `cidrs` after provisioning.\n"]
    pub fn cidrs(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cidrs", self.base))
    }

    #[doc = "Get a reference to the value of field `classic_load_balancer_listener` after provisioning.\n"]
    pub fn classic_load_balancer_listener(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_ClassicLoadBalancerListenerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_ComponentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_gateway` after provisioning.\n"]
    pub fn customer_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_CustomerGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_DestinationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancer_listener` after provisioning.\n"]
    pub fn elastic_load_balancer_listener(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_ElasticLoadBalancerListenerRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `explanation_code` after provisioning.\n"]
    pub fn explanation_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explanation_code", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_route_table` after provisioning.\n"]
    pub fn ingress_route_table(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_IngressRouteTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_InternetGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.internet_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_arn` after provisioning.\n"]
    pub fn load_balancer_arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_arn", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_listener_port` after provisioning.\n"]
    pub fn load_balancer_listener_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_listener_port", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_group` after provisioning.\n"]
    pub fn load_balancer_target_group(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_groups` after provisioning.\n"]
    pub fn load_balancer_target_groups(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_LoadBalancerTargetGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_target_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_port` after provisioning.\n"]
    pub fn load_balancer_target_port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.load_balancer_target_port", self.base))
    }

    #[doc = "Get a reference to the value of field `missing_component` after provisioning.\n"]
    pub fn missing_component(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.missing_component", self.base))
    }

    #[doc = "Get a reference to the value of field `nat_gateway` after provisioning.\n"]
    pub fn nat_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_NatGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nat_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_NetworkInterfaceRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.network_interface", self.base))
    }

    #[doc = "Get a reference to the value of field `packet_field` after provisioning.\n"]
    pub fn packet_field(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.packet_field", self.base))
    }

    #[doc = "Get a reference to the value of field `port` after provisioning.\n"]
    pub fn port(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.port", self.base))
    }

    #[doc = "Get a reference to the value of field `port_ranges` after provisioning.\n"]
    pub fn port_ranges(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_PortRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list` after provisioning.\n"]
    pub fn prefix_list(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_PrefixListRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_list", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table` after provisioning.\n"]
    pub fn route_table(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SubnetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_route_table` after provisioning.\n"]
    pub fn subnet_route_table(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SubnetRouteTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment` after provisioning.\n"]
    pub fn transit_gateway_attachment(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayAttachmentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table` after provisioning.\n"]
    pub fn transit_gateway_route_table(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpcEndpointRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection` after provisioning.\n"]
    pub fn vpc_peering_connection(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpcPeeringConnectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_connection` after provisioning.\n"]
    pub fn vpn_connection(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpnConnectionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpn_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway` after provisioning.\n"]
    pub fn vpn_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpnGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpn_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_detail_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {
    #[doc = "Set the field `additional_detail_type`.\n"]
    pub fn set_additional_detail_type(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.additional_detail_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {}

impl BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_detail_type` after provisioning.\n"]
    pub fn additional_detail_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_ComponentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {
    type O =
        terrars
        ::BlockAssignable<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {}

impl BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {
    #[doc = "Set the field `from`.\n"]
    pub fn set_from(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.from = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `to`.\n"]
    pub fn set_to(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.to = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {}

impl BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `from` after provisioning.\n"]
    pub fn from(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.from", self.base))
    }

    #[doc = "Get a reference to the value of field `to` after provisioning.\n"]
    pub fn to(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.to", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges>>,
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_ranges`.\n"]
    pub fn set_destination_port_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges,
                        >,
                    >,
    ) -> Self {
        self.destination_port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_ranges`.\n"]
    pub fn set_source_port_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges>,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {}

impl BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_ranges` after provisioning.\n"]
    pub fn destination_port_ranges(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_ranges` after provisioning.\n"]
    pub fn source_port_ranges(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges>>,
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {
    #[doc = "Set the field `destination_addresses`.\n"]
    pub fn set_destination_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.destination_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_port_ranges`.\n"]
    pub fn set_destination_port_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<
                            Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRanges,
                        >,
                    >,
    ) -> Self {
        self.destination_port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `protocol`.\n"]
    pub fn set_protocol(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.protocol = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_addresses`.\n"]
    pub fn set_source_addresses(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.source_addresses = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_port_ranges`.\n"]
    pub fn set_source_port_ranges(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRanges>,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {}

impl BuildEc2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `destination_addresses` after provisioning.\n"]
    pub fn destination_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_port_ranges` after provisioning.\n"]
    pub fn destination_port_ranges(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_DestinationPortRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `protocol` after provisioning.\n"]
    pub fn protocol(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.protocol", self.base))
    }

    #[doc = "Get a reference to the value of field `source_addresses` after provisioning.\n"]
    pub fn source_addresses(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_addresses", self.base))
    }

    #[doc = "Get a reference to the value of field `source_port_ranges` after provisioning.\n"]
    pub fn source_port_ranges(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader_SourcePortRangesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ForwardPathComponents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sequence_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Subnet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Vpc>>,
}

impl Ec2NetworkInsightsAnalysis_ForwardPathComponents {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule>>,
    ) -> Self {
        self.acl_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_details`.\n"]
    pub fn set_additional_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails>>,
    ) -> Self {
        self.additional_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>>,
    ) -> Self {
        self.destination_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inbound_header`.\n"]
    pub fn set_inbound_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader>>,
    ) -> Self {
        self.inbound_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outbound_header`.\n"]
    pub fn set_outbound_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader>>,
    ) -> Self {
        self.outbound_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>>,
    ) -> Self {
        self.security_group_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sequence_number`.\n"]
    pub fn set_sequence_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sequence_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Subnet>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>>,
    ) -> Self {
        self.transit_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_route`.\n"]
    pub fn set_transit_gateway_route_table_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>>,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Vpc>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ForwardPathComponents {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_ForwardPathComponents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ForwardPathComponents {}

impl BuildEc2NetworkInsightsAnalysis_ForwardPathComponents {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ForwardPathComponents {
        Ec2NetworkInsightsAnalysis_ForwardPathComponents {
            acl_rule: core::default::Default::default(),
            additional_details: core::default::Default::default(),
            attached_to: core::default::Default::default(),
            component: core::default::Default::default(),
            destination_vpc: core::default::Default::default(),
            inbound_header: core::default::Default::default(),
            outbound_header: core::default::Default::default(),
            route_table_route: core::default::Default::default(),
            security_group_rule: core::default::Default::default(),
            sequence_number: core::default::Default::default(),
            source_vpc: core::default::Default::default(),
            subnet: core::default::Default::default(),
            transit_gateway: core::default::Default::default(),
            transit_gateway_route_table_route: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef {
        Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ForwardPathComponentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_ComponentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SubnetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponents {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sequence_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Subnet>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Vpc>>,
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponents {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AclRule>>,
    ) -> Self {
        self.acl_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `additional_details`.\n"]
    pub fn set_additional_details(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetails>>,
    ) -> Self {
        self.additional_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_AttachedTo>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Component>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpc>>,
    ) -> Self {
        self.destination_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `inbound_header`.\n"]
    pub fn set_inbound_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeader>>,
    ) -> Self {
        self.inbound_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `outbound_header`.\n"]
    pub fn set_outbound_header(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeader>>,
    ) -> Self {
        self.outbound_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRoute>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRule>>,
    ) -> Self {
        self.security_group_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `sequence_number`.\n"]
    pub fn set_sequence_number(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.sequence_number = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_SourceVpc>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Subnet>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGateway>>,
    ) -> Self {
        self.transit_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway_route_table_route`.\n"]
    pub fn set_transit_gateway_route_table_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRoute>>,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysis_Explanations_Vpc>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysis_ReturnPathComponents {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysis_ReturnPathComponents>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysis_ReturnPathComponents {}

impl BuildEc2NetworkInsightsAnalysis_ReturnPathComponents {
    pub fn build(self) -> Ec2NetworkInsightsAnalysis_ReturnPathComponents {
        Ec2NetworkInsightsAnalysis_ReturnPathComponents {
            acl_rule: core::default::Default::default(),
            additional_details: core::default::Default::default(),
            attached_to: core::default::Default::default(),
            component: core::default::Default::default(),
            destination_vpc: core::default::Default::default(),
            inbound_header: core::default::Default::default(),
            outbound_header: core::default::Default::default(),
            route_table_route: core::default::Default::default(),
            security_group_rule: core::default::Default::default(),
            sequence_number: core::default::Default::default(),
            source_vpc: core::default::Default::default(),
            subnet: core::default::Default::default(),
            transit_gateway: core::default::Default::default(),
            transit_gateway_route_table_route: core::default::Default::default(),
            vpc: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef {
        Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysis_ReturnPathComponentsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AclRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_AdditionalDetailsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_AttachedToRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_ComponentRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_DestinationVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_InboundHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_ReturnPathComponents_OutboundHeaderRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_RouteTableRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SecurityGroupRuleRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SourceVpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_SubnetRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_TransitGatewayRouteTableRouteRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysis_Explanations_VpcRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
