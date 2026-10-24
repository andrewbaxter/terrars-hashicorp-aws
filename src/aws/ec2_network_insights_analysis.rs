use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct Ec2NetworkInsightsAnalysisData {
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
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags_all: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    wait_for_completion: std::option::Option<terrars::PrimField<bool>>,
}

struct Ec2NetworkInsightsAnalysis_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<Ec2NetworkInsightsAnalysisData>,
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
    pub fn alternate_path_hints(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisAlternatePathHintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElRef> {
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

    #[doc = "Get a reference to the value of field `return_path_components` after provisioning.\n"]
    pub fn return_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElRef> {
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
            data: std::cell::RefCell::new(Ec2NetworkInsightsAnalysisData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                filter_in_arns: core::default::Default::default(),
                id: core::default::Default::default(),
                network_insights_path_id: self.network_insights_path_id,
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
    pub fn alternate_path_hints(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisAlternatePathHintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElRef> {
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

    #[doc = "Get a reference to the value of field `return_path_components` after provisioning.\n"]
    pub fn return_path_components(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElRef> {
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
pub struct Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisAlternatePathHintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisAlternatePathHintsEl {}

impl BuildEc2NetworkInsightsAnalysisAlternatePathHintsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
        Ec2NetworkInsightsAnalysisAlternatePathHintsEl {
            component_arn: core::default::Default::default(),
            component_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
        Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisAlternatePathHintsElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElAclEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAclEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAclEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAclEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAclEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAclEl {
        Ec2NetworkInsightsAnalysisExplanationsElAclEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>>,
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
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
    pub fn port_range(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef> {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
        Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
        Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_port: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
        Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
            instance_port: core::default::Default::default(),
            load_balancer_port: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
        Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
        Ec2NetworkInsightsAnalysisExplanationsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
        Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElComponentElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElDestinationEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
        Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
        Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
        Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
        Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
        Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
        Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
        Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
        Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
        Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
        Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
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

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
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

pub struct Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>,
                    >,
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
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
    pub fn port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef> {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
        Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSubnetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSubnetEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
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

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
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

pub struct Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpcEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
        Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
        Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisExplanationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAclEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classic_load_balancer_listener: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_load_balancer_listener: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explanation_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_route_table: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_listener_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_group: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_groups: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_component: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packet_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_ranges: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_route_table: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_connection: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_connection: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_gateway: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>>,
}

impl Ec2NetworkInsightsAnalysisExplanationsEl {
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAclEl>>,
    ) -> Self {
        self.acl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAclRuleEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElAttachedToEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>>,
    ) -> Self {
        self.classic_load_balancer_listener = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElComponentEl>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_gateway`.\n"]
    pub fn set_customer_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>>,
    ) -> Self {
        self.customer_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationEl>>,
    ) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>>,
    ) -> Self {
        self.ingress_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayEl>>,
    ) -> Self {
        self.nat_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElPortRangesEl>>,
    ) -> Self {
        self.port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list`.\n"]
    pub fn set_prefix_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElPrefixListEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableEl>>,
    ) -> Self {
        self.route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group`.\n"]
    pub fn set_security_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>>,
    ) -> Self {
        self.security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>>,
    ) -> Self {
        self.security_group_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetEl>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_route_table`.\n"]
    pub fn set_subnet_route_table(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>>,
    ) -> Self {
        self.subnet_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>>,
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
                    ::Into<
                        terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>,
                    >,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEl>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>>,
    ) -> Self {
        self.vpc_endpoint = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_peering_connection`.\n"]
    pub fn set_vpc_peering_connection(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>>,
    ) -> Self {
        self.vpc_peering_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_connection`.\n"]
    pub fn set_vpn_connection(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>>,
    ) -> Self {
        self.vpn_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_gateway`.\n"]
    pub fn set_vpn_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>>,
    ) -> Self {
        self.vpn_gateway = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisExplanationsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisExplanationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisExplanationsEl {}

impl BuildEc2NetworkInsightsAnalysisExplanationsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisExplanationsEl {
        Ec2NetworkInsightsAnalysisExplanationsEl {
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

pub struct Ec2NetworkInsightsAnalysisExplanationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisExplanationsElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Ec2NetworkInsightsAnalysisExplanationsElRef {
        Ec2NetworkInsightsAnalysisExplanationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisExplanationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElAclElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElAclRuleElRef> {
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
    pub fn attached_to(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElAttachedToElRef> {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_gateway` after provisioning.\n"]
    pub fn customer_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancer_listener` after provisioning.\n"]
    pub fn elastic_load_balancer_listener(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `explanation_code` after provisioning.\n"]
    pub fn explanation_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explanation_code", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_route_table` after provisioning.\n"]
    pub fn ingress_route_table(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef> {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_groups` after provisioning.\n"]
    pub fn load_balancer_target_groups(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef> {
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
    pub fn nat_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElNatGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nat_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef> {
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
    pub fn port_ranges(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElPortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list` after provisioning.\n"]
    pub fn prefix_list(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElPrefixListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_list", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table` after provisioning.\n"]
    pub fn route_table(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSourceVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSubnetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_route_table` after provisioning.\n"]
    pub fn subnet_route_table(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment` after provisioning.\n"]
    pub fn transit_gateway_attachment(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table` after provisioning.\n"]
    pub fn transit_gateway_route_table(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection` after provisioning.\n"]
    pub fn vpc_peering_connection(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_connection` after provisioning.\n"]
    pub fn vpn_connection(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpn_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway` after provisioning.\n"]
    pub fn vpn_gateway(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpn_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>>,
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
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
    pub fn port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef> {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_detail_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>,
                    >,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_detail_type` after provisioning.\n"]
    pub fn additional_detail_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
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

impl terrars
::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
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
                            Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl,
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
                        ::ListField<
                            Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
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

impl terrars
::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    type O =
        terrars
        ::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>,
    >,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
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
                            Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl,
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
                        ::ListField<
                            Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
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
    ) -> terrars
    ::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
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

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
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

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>,
                    >,
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
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
    pub fn port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef> {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
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

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sequence_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>>,
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>>,
    ) -> Self {
        self.additional_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>>,
    ) -> Self {
        self.outbound_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>>,
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
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>,
                    >,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisForwardPathComponentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisForwardPathComponentsEl {}

impl BuildEc2NetworkInsightsAnalysisForwardPathComponentsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
        Ec2NetworkInsightsAnalysisForwardPathComponentsEl {
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

pub struct Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
        Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisForwardPathComponentsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>>,
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
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
    pub fn port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef> {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_detail_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>,
                    >,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `additional_detail_type` after provisioning.\n"]
    pub fn additional_detail_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.additional_detail_type", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
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

impl terrars
::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
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
                            Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl,
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
                        ::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
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

impl terrars
::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
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
                            Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl,
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
                        ::ListField<
                            Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
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

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
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

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
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
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>,
                    >,
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
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
    pub fn port_range(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef> {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    type O =
        terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
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

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
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

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
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
pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_header: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sequence_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>>,
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>>,
    ) -> Self {
        self.additional_details = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `attached_to`.\n"]
    pub fn set_attached_to(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>>,
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
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>>,
    ) -> Self {
        self.outbound_header = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>>,
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
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>>,
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
                    ::Into<
                        terrars
                        ::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>,
                    >,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
    type O = terrars::BlockAssignable<Ec2NetworkInsightsAnalysisReturnPathComponentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildEc2NetworkInsightsAnalysisReturnPathComponentsEl {}

impl BuildEc2NetworkInsightsAnalysisReturnPathComponentsEl {
    pub fn build(self) -> Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
        Ec2NetworkInsightsAnalysisReturnPathComponentsEl {
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

pub struct Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
        Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl Ec2NetworkInsightsAnalysisReturnPathComponentsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<Ec2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}
