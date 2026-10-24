use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderAws;

#[derive(Serialize)]
struct DataEc2NetworkInsightsAnalysisData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_insights_analysis_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    tags: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    filter: std::option::Option<std::vec::Vec<DataEc2NetworkInsightsAnalysisFilterEl>>,
    dynamic: DataEc2NetworkInsightsAnalysisDynamic,
}

struct DataEc2NetworkInsightsAnalysis_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataEc2NetworkInsightsAnalysisData>,
}

#[derive(Clone)]
pub struct DataEc2NetworkInsightsAnalysis(std::rc::Rc<DataEc2NetworkInsightsAnalysis_>);

impl DataEc2NetworkInsightsAnalysis {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_insights_analysis_id`.\n"]
    pub fn set_network_insights_analysis_id(
        self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().network_insights_analysis_id = std::option::Option::Some(v.into());
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

    #[doc = "Set the field `filter`.\n"]
    pub fn set_filter(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisFilterEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().filter = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.filter = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Get a reference to the value of field `alternate_path_hints` after provisioning.\n"]
    pub fn alternate_path_hints(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_analysis_id` after provisioning.\n"]
    pub fn network_insights_analysis_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_insights_analysis_id", self.extract_ref()))
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
    pub fn return_path_components(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef> {
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

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }
}

impl terrars::Referable for DataEc2NetworkInsightsAnalysis {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataEc2NetworkInsightsAnalysis { }

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysis {
    type O = terrars::ListRef<DataEc2NetworkInsightsAnalysisRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataEc2NetworkInsightsAnalysis_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "aws_ec2_network_insights_analysis".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysis {
    pub tf_id: std::string::String,
}

impl BuildDataEc2NetworkInsightsAnalysis {
    pub fn build(self, stack: &mut terrars::Stack) -> DataEc2NetworkInsightsAnalysis {
        let out = DataEc2NetworkInsightsAnalysis(std::rc::Rc::new(DataEc2NetworkInsightsAnalysis_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataEc2NetworkInsightsAnalysisData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
                network_insights_analysis_id: core::default::Default::default(),
                tags: core::default::Default::default(),
                filter: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataEc2NetworkInsightsAnalysisRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataEc2NetworkInsightsAnalysisRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `alternate_path_hints` after provisioning.\n"]
    pub fn alternate_path_hints(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.alternate_path_hints", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `arn` after provisioning.\n"]
    pub fn arn(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.arn", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `explanations` after provisioning.\n"]
    pub fn explanations(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.explanations", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `filter_in_arns` after provisioning.\n"]
    pub fn filter_in_arns(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.filter_in_arns", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `forward_path_components` after provisioning.\n"]
    pub fn forward_path_components(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.forward_path_components", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `network_insights_analysis_id` after provisioning.\n"]
    pub fn network_insights_analysis_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.network_insights_analysis_id", self.extract_ref()))
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
    pub fn return_path_components(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef> {
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

    #[doc = "Get a reference to the value of field `warning_message` after provisioning.\n"]
    pub fn warning_message(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.warning_message", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisAlternatePathHintsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisAlternatePathHintsEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisAlternatePathHintsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisAlternatePathHintsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisAlternatePathHintsEl {}

impl BuildDataEc2NetworkInsightsAnalysisAlternatePathHintsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisAlternatePathHintsEl {
        DataEc2NetworkInsightsAnalysisAlternatePathHintsEl {
            component_arn: core::default::Default::default(),
            component_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef {
        DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisAlternatePathHintsElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElAclEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAclEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElAclEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElAclEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElAclEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElAclEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElAclEl {
        DataEc2NetworkInsightsAnalysisExplanationsElAclEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElAclElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElAclElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElAclElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElAclElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAclElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
        DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeEl>>,
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
        DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElRef {
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
    pub fn port_range(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElPortRangeElRef> {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
        DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElAttachedToElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    instance_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_port: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
        DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl {
            instance_port: core::default::Default::default(),
            load_balancer_port: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElComponentEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElComponentEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElComponentEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElComponentEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElComponentEl {
        DataEc2NetworkInsightsAnalysisExplanationsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElComponentElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElComponentElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
        DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
        DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElDestinationElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElDestinationElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElDestinationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElDestinationElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
        DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
        DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
        DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
        DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
        DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
        DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
        DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
        DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
        DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElPortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
        DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElPrefixListElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
        DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElRouteTableElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
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

impl DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
        DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl {
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

pub struct DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
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
                        terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeEl>,
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElPortRangeElRef> {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSubnetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSubnetElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSubnetElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
        DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
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

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl {
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

pub struct DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElVpcEl {
        DataEc2NetworkInsightsAnalysisExplanationsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElVpcElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
        DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
        DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
        DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
        DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisExplanationsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAclEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    address: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    availability_zones: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidrs: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    classic_load_balancer_listener: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    customer_gateway: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    elastic_load_balancer_listener: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    explanation_code: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    ingress_route_table: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    internet_gateway: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_listener_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_group: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_groups: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    load_balancer_target_port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    missing_component: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    nat_gateway: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    network_interface: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    packet_field: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_ranges: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocols: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_groups: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    state: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet_route_table: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_attachment: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_endpoint: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc_peering_connection: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_connection: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpn_gateway: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>>,
}

impl DataEc2NetworkInsightsAnalysisExplanationsEl {
    #[doc = "Set the field `acl`.\n"]
    pub fn set_acl(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAclEl>>,
    ) -> Self {
        self.acl = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElAttachedToEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerEl>,
                    >,
    ) -> Self {
        self.classic_load_balancer_listener = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElComponentEl>>,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `customer_gateway`.\n"]
    pub fn set_customer_gateway(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayEl>>,
    ) -> Self {
        self.customer_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination`.\n"]
    pub fn set_destination(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElDestinationEl>>,
    ) -> Self {
        self.destination = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `destination_vpc`.\n"]
    pub fn set_destination_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableEl>>,
    ) -> Self {
        self.ingress_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `internet_gateway`.\n"]
    pub fn set_internet_gateway(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsEl>,
                    >,
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
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayEl>>,
    ) -> Self {
        self.nat_gateway = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `network_interface`.\n"]
    pub fn set_network_interface(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElPortRangesEl>>,
    ) -> Self {
        self.port_ranges = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `prefix_list`.\n"]
    pub fn set_prefix_list(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElPrefixListEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableEl>>,
    ) -> Self {
        self.route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `route_table_route`.\n"]
    pub fn set_route_table_route(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteEl>>,
    ) -> Self {
        self.route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group`.\n"]
    pub fn set_security_group(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupEl>>,
    ) -> Self {
        self.security_group = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_group_rule`.\n"]
    pub fn set_security_group_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleEl>>,
    ) -> Self {
        self.security_group_rule = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `security_groups`.\n"]
    pub fn set_security_groups(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsEl>>,
    ) -> Self {
        self.security_groups = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `source_vpc`.\n"]
    pub fn set_source_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcEl>>,
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
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSubnetEl>>,
    ) -> Self {
        self.subnet = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet_route_table`.\n"]
    pub fn set_subnet_route_table(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableEl>>,
    ) -> Self {
        self.subnet_route_table = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `transit_gateway`.\n"]
    pub fn set_transit_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentEl>,
                    >,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableEl>,
                    >,
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
                        terrars
                        ::ListField<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteEl>,
                    >,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpcEl>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc_endpoint`.\n"]
    pub fn set_vpc_endpoint(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionEl>>,
    ) -> Self {
        self.vpc_peering_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_connection`.\n"]
    pub fn set_vpn_connection(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionEl>>,
    ) -> Self {
        self.vpn_connection = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpn_gateway`.\n"]
    pub fn set_vpn_gateway(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayEl>>,
    ) -> Self {
        self.vpn_gateway = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisExplanationsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisExplanationsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisExplanationsEl {}

impl BuildDataEc2NetworkInsightsAnalysisExplanationsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisExplanationsEl {
        DataEc2NetworkInsightsAnalysisExplanationsEl {
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

pub struct DataEc2NetworkInsightsAnalysisExplanationsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisExplanationsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisExplanationsElRef {
        DataEc2NetworkInsightsAnalysisExplanationsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisExplanationsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl` after provisioning.\n"]
    pub fn acl(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElAclElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl", self.base))
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElAclRuleElRef> {
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
    pub fn attached_to(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElAttachedToElRef> {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElClassicLoadBalancerListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.classic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `customer_gateway` after provisioning.\n"]
    pub fn customer_gateway(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElCustomerGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.customer_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `destination` after provisioning.\n"]
    pub fn destination(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElDestinationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElDestinationVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `direction` after provisioning.\n"]
    pub fn direction(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.direction", self.base))
    }

    #[doc = "Get a reference to the value of field `elastic_load_balancer_listener` after provisioning.\n"]
    pub fn elastic_load_balancer_listener(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElElasticLoadBalancerListenerElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.elastic_load_balancer_listener", self.base))
    }

    #[doc = "Get a reference to the value of field `explanation_code` after provisioning.\n"]
    pub fn explanation_code(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.explanation_code", self.base))
    }

    #[doc = "Get a reference to the value of field `ingress_route_table` after provisioning.\n"]
    pub fn ingress_route_table(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElIngressRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.ingress_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `internet_gateway` after provisioning.\n"]
    pub fn internet_gateway(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElInternetGatewayElRef> {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.load_balancer_target_group", self.base))
    }

    #[doc = "Get a reference to the value of field `load_balancer_target_groups` after provisioning.\n"]
    pub fn load_balancer_target_groups(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElLoadBalancerTargetGroupsElRef> {
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
    pub fn nat_gateway(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElNatGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.nat_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `network_interface` after provisioning.\n"]
    pub fn network_interface(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElNetworkInterfaceElRef> {
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
    pub fn port_ranges(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElPortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.port_ranges", self.base))
    }

    #[doc = "Get a reference to the value of field `prefix_list` after provisioning.\n"]
    pub fn prefix_list(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElPrefixListElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.prefix_list", self.base))
    }

    #[doc = "Get a reference to the value of field `protocols` after provisioning.\n"]
    pub fn protocols(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.protocols", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table` after provisioning.\n"]
    pub fn route_table(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group` after provisioning.\n"]
    pub fn security_group(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `security_groups` after provisioning.\n"]
    pub fn security_groups(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSecurityGroupsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_groups", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSourceVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `state` after provisioning.\n"]
    pub fn state(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.state", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSubnetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet_route_table` after provisioning.\n"]
    pub fn subnet_route_table(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElSubnetRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_attachment` after provisioning.\n"]
    pub fn transit_gateway_attachment(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayAttachmentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_attachment", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table` after provisioning.\n"]
    pub fn transit_gateway_route_table(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElTransitGatewayRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_endpoint` after provisioning.\n"]
    pub fn vpc_endpoint(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElVpcEndpointElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_endpoint", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc_peering_connection` after provisioning.\n"]
    pub fn vpc_peering_connection(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElVpcPeeringConnectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc_peering_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_connection` after provisioning.\n"]
    pub fn vpn_connection(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElVpnConnectionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpn_connection", self.base))
    }

    #[doc = "Get a reference to the value of field `vpn_gateway` after provisioning.\n"]
    pub fn vpn_gateway(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisExplanationsElVpnGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpn_gateway", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeEl>,
                    >,
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElPortRangeElRef> {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_detail_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl>,
    >,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
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
                        ::ListField<
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentEl,
                        >,
                    >,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
    pub fn build(
        self,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars
        ::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl>,
    >,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
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
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesEl,
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
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef {
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
    ::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    pub fn build(
        self,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars
        ::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl>,
    >,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
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
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesEl,
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
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef {
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
    ::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
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

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl {
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

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
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
                        ::ListField<
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeEl,
                        >,
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElPortRangeElRef> {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl {
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

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_header: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_header: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sequence_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>>,
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsEl {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsEl>,
                    >,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteEl>,
                    >,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayEl>>,
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
                        ::ListField<
                            DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteEl,
                        >,
                    >,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcEl>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisForwardPathComponentsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisForwardPathComponentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsEl {}

impl BuildDataEc2NetworkInsightsAnalysisForwardPathComponentsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsEl {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsEl {
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

pub struct DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef {
        DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisForwardPathComponentsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAclRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAdditionalDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElAttachedToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElDestinationVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElInboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElOutboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSecurityGroupRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSourceVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElSubnetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElTransitGatewayRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisForwardPathComponentsElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    egress: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    rule_number: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeEl>,
                    >,
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl {
            cidr: core::default::Default::default(),
            egress: core::default::Default::default(),
            port_range: core::default::Default::default(),
            protocol: core::default::Default::default(),
            rule_action: core::default::Default::default(),
            rule_number: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElPortRangeElRef> {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    type O =
        terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_detail_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
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
                        ::ListField<
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentEl,
                        >,
                    >,
    ) -> Self {
        self.component = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl {
            additional_detail_type: core::default::Default::default(),
            component: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl>,
    >,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
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
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesEl,
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
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef {
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
    ::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    type O =
        terrars
        ::BlockAssignable<
            DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl,
        >;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
    pub fn build(
        self,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
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
::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_port_ranges: std
    ::option
    ::Option<
        terrars
        ::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl>,
    >,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_addresses: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_port_ranges: std
    ::option
    ::Option<
        terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl>,
    >,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
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
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesEl,
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
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesEl,
                        >,
                    >,
    ) -> Self {
        self.source_port_ranges = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl {
            destination_addresses: core::default::Default::default(),
            destination_port_ranges: core::default::Default::default(),
            protocol: core::default::Default::default(),
            source_addresses: core::default::Default::default(),
            source_port_ranges: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef {
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
    ::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElDestinationPortRangesElRef> {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElSourcePortRangesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_port_ranges", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
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

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl {
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

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    from: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    to: std::option::Option<terrars::PrimField<f64>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    type O =
        terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl {
            from: core::default::Default::default(),
            to: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cidr: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    direction: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    port_range: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    prefix_list_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    protocol: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_id: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
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
                        ::ListField<
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeEl,
                        >,
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl {
            cidr: core::default::Default::default(),
            direction: core::default::Default::default(),
            port_range: core::default::Default::default(),
            prefix_list_id: core::default::Default::default(),
            protocol: core::default::Default::default(),
            security_group_id: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef {
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
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElPortRangeElRef> {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    type O =
        terrars
        ::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl {
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

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    arn: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
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

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl {
            arn: core::default::Default::default(),
            id: core::default::Default::default(),
            name: core::default::Default::default(),
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef {
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
pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    acl_rule: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    additional_details: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    attached_to: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    component: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    destination_vpc: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    inbound_header: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    outbound_header: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    route_table_route: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    security_group_rule: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    sequence_number: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    source_vpc: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    subnet: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    transit_gateway_route_table_route: std
    ::option
    ::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    vpc: std::option::Option<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>>,
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsEl {
    #[doc = "Set the field `acl_rule`.\n"]
    pub fn set_acl_rule(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsEl>,
                    >,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToEl>>,
    ) -> Self {
        self.attached_to = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `component`.\n"]
    pub fn set_component(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteEl>>,
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
                    ::Into<
                        terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleEl>,
                    >,
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
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcEl>>,
    ) -> Self {
        self.source_vpc = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `subnet`.\n"]
    pub fn set_subnet(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetEl>>,
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
                    ::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayEl>>,
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
                        ::ListField<
                            DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteEl,
                        >,
                    >,
    ) -> Self {
        self.transit_gateway_route_table_route = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `vpc`.\n"]
    pub fn set_vpc(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcEl>>,
    ) -> Self {
        self.vpc = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisReturnPathComponentsEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisReturnPathComponentsEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsEl {}

impl BuildDataEc2NetworkInsightsAnalysisReturnPathComponentsEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsEl {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsEl {
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

pub struct DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef {
        DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisReturnPathComponentsElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `acl_rule` after provisioning.\n"]
    pub fn acl_rule(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAclRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.acl_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `additional_details` after provisioning.\n"]
    pub fn additional_details(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAdditionalDetailsElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.additional_details", self.base))
    }

    #[doc = "Get a reference to the value of field `attached_to` after provisioning.\n"]
    pub fn attached_to(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElAttachedToElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.attached_to", self.base))
    }

    #[doc = "Get a reference to the value of field `component` after provisioning.\n"]
    pub fn component(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElComponentElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.component", self.base))
    }

    #[doc = "Get a reference to the value of field `destination_vpc` after provisioning.\n"]
    pub fn destination_vpc(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElDestinationVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.destination_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `inbound_header` after provisioning.\n"]
    pub fn inbound_header(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElInboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.inbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `outbound_header` after provisioning.\n"]
    pub fn outbound_header(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElOutboundHeaderElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.outbound_header", self.base))
    }

    #[doc = "Get a reference to the value of field `route_table_route` after provisioning.\n"]
    pub fn route_table_route(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `security_group_rule` after provisioning.\n"]
    pub fn security_group_rule(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSecurityGroupRuleElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.security_group_rule", self.base))
    }

    #[doc = "Get a reference to the value of field `sequence_number` after provisioning.\n"]
    pub fn sequence_number(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.sequence_number", self.base))
    }

    #[doc = "Get a reference to the value of field `source_vpc` after provisioning.\n"]
    pub fn source_vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSourceVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.source_vpc", self.base))
    }

    #[doc = "Get a reference to the value of field `subnet` after provisioning.\n"]
    pub fn subnet(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElSubnetElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.subnet", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway` after provisioning.\n"]
    pub fn transit_gateway(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway", self.base))
    }

    #[doc = "Get a reference to the value of field `transit_gateway_route_table_route` after provisioning.\n"]
    pub fn transit_gateway_route_table_route(
        &self,
    ) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElTransitGatewayRouteTableRouteElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.transit_gateway_route_table_route", self.base))
    }

    #[doc = "Get a reference to the value of field `vpc` after provisioning.\n"]
    pub fn vpc(&self) -> terrars::ListRef<DataEc2NetworkInsightsAnalysisReturnPathComponentsElVpcElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.vpc", self.base))
    }
}

#[derive(Serialize)]
pub struct DataEc2NetworkInsightsAnalysisFilterEl {
    name: terrars::PrimField<std::string::String>,
    values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl DataEc2NetworkInsightsAnalysisFilterEl { }

impl terrars::ToListMappable for DataEc2NetworkInsightsAnalysisFilterEl {
    type O = terrars::BlockAssignable<DataEc2NetworkInsightsAnalysisFilterEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataEc2NetworkInsightsAnalysisFilterEl {
    #[doc = ""]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = ""]
    pub values: terrars::SetField<terrars::PrimField<std::string::String>>,
}

impl BuildDataEc2NetworkInsightsAnalysisFilterEl {
    pub fn build(self) -> DataEc2NetworkInsightsAnalysisFilterEl {
        DataEc2NetworkInsightsAnalysisFilterEl {
            name: self.name,
            values: self.values,
        }
    }
}

pub struct DataEc2NetworkInsightsAnalysisFilterElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataEc2NetworkInsightsAnalysisFilterElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataEc2NetworkInsightsAnalysisFilterElRef {
        DataEc2NetworkInsightsAnalysisFilterElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataEc2NetworkInsightsAnalysisFilterElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `values` after provisioning.\n"]
    pub fn values(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.values", self.base))
    }
}

#[derive(Serialize, Default)]
struct DataEc2NetworkInsightsAnalysisDynamic {
    filter: std::option::Option<terrars::DynamicBlock<DataEc2NetworkInsightsAnalysisFilterEl>>,
}
